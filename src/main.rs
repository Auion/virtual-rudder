use std::io::{Result, Error, ErrorKind};
use std::fs::{File, OpenOptions};
use std::env;
use std::vec::Vec;

use input_linux::{EventKind, InputEvent, AbsoluteAxis, AbsoluteInfo, AbsoluteInfoSetup, InputId};
use input_linux::sys::{input_event, timeval, BUS_VIRTUAL, EV_SYN};
use input_linux::uinput::UInputHandle;
use input_linux::evdev::EvdevHandle;

const DEVICE_NAME: &[u8] = b"Virtual Yaw Axis";
const SYN_REPORT: input_event = input_event {
    time: timeval {
        tv_sec: 0,
        tv_usec: 0,
    },
    type_: EV_SYN as u16,
    code: 0,
    value: 0,
};

// TODO: Flags --left {uint} and --right {uint}
struct Arguments {
    device_path: String,
    left_axis: u16,
    right_axis: u16,
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
            None => return Err(Error::new(ErrorKind::InvalidInput, "Required: left axis"))
        };


        Ok(Self {
            device_path,
            left_axis,
            right_axis,
        })
    }

    // fn try_from(value: Vec<String>) -> Result<Self> {
    //     let mut args = value.iter();
    //     let _ = args.next(); // Throw away the program path
    //
    //     // Put arguments here
    //     let mut left_axis: Option<u16> = Option::None;
    //     let mut right_axis: Option<u16> = Option::None;
    //     let mut device_path: Option<String> = Option::None;
    //
    //     while let Some(arg) = args.next() {
    //         match arg.as_ref() {
    //             // Required Arguments
    //             "-d" | "--device" => {
    //                 if let Some(str) = args.next() {
    //                     device_path = Some(str.clone());
    //                 } else {
    //                     return Err(Error::new(ErrorKind::InvalidInput, "Required: --device [path]"));
    //                 };
    //             },
    //             "-l" | "--left" => {
    //                 if let Some(str) = args.next() {
    //                     match str.parse::<u16>() {
    //                         Ok(val) => left_axis = Some(val),
    //                         Err(_err) => return Err(Error::new(ErrorKind::InvalidInput, "Failed to parse right axis value")),
    //                     };
    //                 } else {
    //                     return Err(Error::new(ErrorKind::InvalidInput, "Required: --left [axis_num]"))
    //                 };
    //             },
    //             "-r" | "--right" => {
    //                 if let Some(str) = args.next() {
    //                     match str.parse::<u16>() {
    //                         Ok(val) => right_axis = Some(val),
    //                         Err(_err) => return Err(Error::new(ErrorKind::InvalidInput, "Failed to parse right axis value")),
    //                     };
    //                 } else {
    //                     return Err(Error::new(ErrorKind::InvalidInput, "Required: --right [axis_num]"))
    //                 };
    //             }
    //             // Optional Arguments (none currently)
    //             _ => return Err(Error::new(ErrorKind::InvalidInput, format!("Unknown argument given: {}", arg) ))
    //         };
    //     };
    //
    //     let left_axis = match left_axis {
    //         Some(val) => val,
    //         None => return Err(Error::new(ErrorKind::InvalidInput, "Missing argument: --left [axis_num]"))
    //     };
    //     let right_axis = match right_axis {
    //         Some(val) => val,
    //         None => return Err(Error::new(ErrorKind::InvalidInput, "Missing argument: --right [axis_num]"))
    //     };
    //     let device_path = match device_path {
    //         Some(val) => val,
    //         None => return Err(Error::new(ErrorKind::InvalidInput, "Missing argument: --device [path]"))
    //     };
    //
    //     Ok(Self {
    //         left_axis,
    //         right_axis,
    //         device_path,
    //     })
    // }
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
}


fn main() -> Result<()> {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let args: Arguments = Arguments::try_from(&args)?;

    let input_path: &str = args.device_path();
    let left_axis: u16 = args.left_axis();
    let right_axis: u16 = args.right_axis();
    
    let input_joy = File::open(input_path)?;
    
    let virt_path = "/dev/uinput";
    let virt_joy = OpenOptions::new().read(false).write(true).create(false).open(virt_path)?;

    let input_joy = EvdevHandle::new(input_joy);
    let virt_joy = UInputHandle::new(virt_joy);

    let virt_id = InputId {
        bustype: BUS_VIRTUAL,
        ..Default::default()
    };

    let virt_rudder = AbsoluteInfoSetup {
        axis: AbsoluteAxis::Rudder,
        info: AbsoluteInfo {
            value: 0,
            minimum: -255,
            maximum: 255,
            fuzz: 0,
            flat: 0,
            resolution: 5, // Can this just be zero?
        }
    };


    virt_joy.set_evbit(input_linux::EventKind::Absolute)?; // Required for yaw input
    virt_joy.set_evbit(input_linux::EventKind::Key)?; // Required for ButtonTrigger to register
    virt_joy.set_keybit(input_linux::Key::ButtonTrigger)?; // Required for joystick recognition
    virt_joy.create(&virt_id, DEVICE_NAME, 0, &[virt_rudder])?;

    let mut left: i32 = 0;
    let mut right: i32 = 0;
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
        match event.code {
            _ if left_axis == event.code => left = event.value, // Left Pedal
            _ if right_axis == event.code => right = -event.value, // Right pedal
            _ => continue,
        };  
        // if event.code == left_axis {
        //     left = event.value;
        // } else if event.code == right_axis {
        //     right = -event.value;
        // } else {
        //     continue;
        // };
        let yaw_value = left + right;
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
