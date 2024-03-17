use std::io::{Result, Error, ErrorKind};
use std::fs::{File, OpenOptions};
use std::env;
use std::vec::Vec;

use input_linux::{EventKind, InputEvent, AbsoluteAxis, AbsoluteInfo, AbsoluteInfoSetup, InputId};
use input_linux::sys::{input_event, timeval, BUS_VIRTUAL, EV_SYN};
use input_linux::uinput::UInputHandle;
use input_linux::evdev::EvdevHandle;

const DEVICE_NAME: &[u8] = b"Virtual Rudder";
const SYN_REPORT: input_event = input_event {
    time: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
    type_: EV_SYN as u16,
    code: 0,
    value: 0,
};

struct Arguments {
    device_path: String,
    left_axis: u16,
    right_axis: u16,
    invert_output: bool,
}
impl TryFrom<&Vec<String>> for Arguments {
    type Error = Error;

    fn try_from(value: &Vec<String>) -> Result<Self> {
        let mut args = value.iter();
        let _ = args.next(); // Throw away program path

        let device_path: String = match args.next() {
            Some(str) => str.to_string(),
            None => return Err(Error::new(ErrorKind::InvalidInput, "Required: device path"))
        };
        let left_axis = match args.next() {
            Some(val) => {
                match val.parse::<u16>() {
                    Ok(val) => val,
                    Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string()))
                }
            }
            None => return Err(Error::new(ErrorKind::InvalidInput, "Required: left axis"))
        };
        let right_axis = match args.next() {
            Some(val) => {
                match val.parse::<u16>() {
                    Ok(val) => val,
                    Err(err) => return Err(Error::new(ErrorKind::InvalidInput, err.to_string()))
                }
            }
            None => return Err(Error::new(ErrorKind::InvalidInput, "Required: right axis"))
        };
        let mut invert_output: bool = false;
        if let Some(val) = args.next() {
            if val.contains('i') { 
               invert_output = true;
            };
        };


        Ok(Self {
            device_path,
            left_axis,
            right_axis,
            invert_output
        })
    }
}

impl Arguments {
    pub fn left_axis(&self) -> u16 {
        self.left_axis
    }
    pub fn right_axis(&self) -> u16 {
        self.right_axis
    }
    pub fn device_path(&self) -> &str {
        self.device_path.as_ref()
    }
    pub fn invert_output(&self) -> bool {
        self.invert_output
    }
}


fn main() -> Result<()> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let args: Arguments = Arguments::try_from(&args)?;

    let input_path: &str = args.device_path();
    let left_axis: u16 = args.left_axis();
    let right_axis: u16 = args.right_axis();
    let invert_output: bool = args.invert_output();
    
    let input_joy = File::open(input_path)?;
    
    let virt_path = "/dev/uinput";
    let virt_joy = OpenOptions::new().read(false).write(true)
        .create(false).open(virt_path)?;

    let input_joy = EvdevHandle::new(input_joy);
    let virt_joy = UInputHandle::new(virt_joy);

    let virt_id = InputId {
        bustype: BUS_VIRTUAL,
        ..Default::default()
    };

    let left_info = input_joy.absolute_info(AbsoluteAxis::from_code(left_axis).unwrap())?;
    let right_info = input_joy.absolute_info(AbsoluteAxis::from_code(right_axis).unwrap())?;

    if left_info.maximum != right_info.maximum {
        eprintln!("Maximum values for input axes are not equal.");
        eprintln!("This may result in unexpected behavior.");
    };

    // Set current values for selected axes.
    // This is important, as some devices' axes
    // have a non-zero resting value.
    let mut left: i32 = -left_info.value;
    let mut right: i32 = right_info.value;

    // TODO: Derive min-max values from 
    // the input device
    let virt_rudder = AbsoluteInfoSetup {
        axis: AbsoluteAxis::Rudder,
        info: AbsoluteInfo {
            value: (left + right),
            minimum: -left_info.maximum,
            maximum: right_info.maximum,
            fuzz: 0,
            flat: 0,
            resolution: 42, // Can this just be zero?
        }
    };

    virt_joy.set_evbit(input_linux::EventKind::Absolute)?; // Required for yaw input
    virt_joy.set_evbit(input_linux::EventKind::Key)?; // Required for ButtonTrigger to register
    virt_joy.set_keybit(input_linux::Key::ButtonTrigger)?; // Required for joystick recognition
    virt_joy.create(&virt_id, DEVICE_NAME, 0, &[virt_rudder])?;

    loop {
        let mut raw_input = [ input_event {
            time: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            type_: 0,
            code: 0,
            value: 0,
        }];
        
        let _ = input_joy.read(&mut raw_input);
        let event = InputEvent::from_raw(&raw_input[0])?;
        
        if event.code == left_axis {
            left = -event.value;
        } else if event.code == right_axis {
            right = event.value;
        } else {
            continue;
        };

        let mut yaw_value = left + right;
        if invert_output { yaw_value = -yaw_value; };
        println!("Yaw: {}", yaw_value);

        // println!("{:?}", event);
        let yaw_input = input_event {
            time: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            type_: EventKind::Absolute.into(),
            code: AbsoluteAxis::Rudder.into(),
            value: yaw_value
        };
        let _ = virt_joy.write(&[yaw_input, SYN_REPORT]);
    };
}
