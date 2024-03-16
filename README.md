# virtual-yaw-axis

---

> [!note]
> This program combines two axes into one.
> 
> If you're looking for a program that combines *controllers*
> into a single virtual controller, this is not the program
> you're looking for.

## Purpose

virtual-yaw-axis simply combines two axes from a joystick / controller
and uses their input to create a combined yaw axis in a virtual controller

For example (and the reason why I made this program), if you have vehicle pedals, 
you can use the clutch and acceleration pedal as if they are rudder pedals.

Though untested, in theory you can use the left and right trigger on an xbox or playstation-like
controller in a similar manner.

## Why?

I have a logitech attack 3 joystick. It works quite well, but does not twist like 
some more modern joysticks do. This means I have no analog rudder control. Using buttons
for rudder control is quite clunky in War Thunder, the game I'm using it for.

So, I just borrow the pedals from my brother's g920 steering wheel. Why not!?
Unfortunately, War Thunder only takes one axis for rudder control. That 
means I need something to remap input. Existing programs do not do the job.

So I thought, "Fine, I'll do it myself."

## UNIX philosophy

This program is simple and will remain that way.
However, there are quality-of-life improvements
to be made.

Note that:

- There are no plans for interactive input.
- This is not a general-purpose axis combiner. It is written with flight simulators in mind.

If there's anything about the program that is annoying you, and the above list does not mention it, create an issue.

I will either add it to the above list :( or my to-do list :) .

## Contributing

This is the first time I've written an actual program that Does The Thing It's Supposed To.
Even moreso, I'm making it available to y'all.

If you're putting in a pull request (PR), make sure to check the issues first.

I still haven't really decided on whether I'm going to even accept any pull requests, so your
likihood of a merge is completely unknown right now.

That said, the program, as it currently exists, is quite simple.

Poke around, the room is small.
