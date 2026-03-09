# hello-rgb: HSV-controlled RGB LED demo for Micro:bit v2
Jayabe 2026-03-08

`hello-rgb` is a Rust embedded project for CS-471/571-001 that demonstrates controlling an external RGB LED using HSV input. Button A and Button B select which component (`H`, `S`, or `V`) is currently being edited, the potentiometer on `P2` sets that component value, and the MB2 5x5 display shows the active component letter. HSV is converted to RGB using the instructor `hsv` crate at <https://github.com/pdx-cs-rust-embedded/hsv>, and output is rendered through timer-interrupt PWM.

The PWM path in `src/rgb_display.rs` uses a 100-step brightness frame with 100 us step duration (100 Hz frame rate). To match assignment guidance, the timer is scheduled at the next channel transition instead of interrupting every 100 us. Duty is capped at 50/100 steps per channel for safety.

## What I Did

I implemented an HSV control interface for an external RGB LED on Micro:bit v2 with a superloop-plus-interrupt architecture. The superloop handles button input, potentiometer reads, HSV component updates, and matrix-letter display, while the `TIMER0` interrupt drives PWM transitions for the external LED. I also refactored the control path so hue, saturation, and value update logic live in separate functions in `src/hsv_control.rs`, and component navigation remains isolated in `src/component.rs`.

## How It Went

The main challenge was balancing responsiveness with the blocking matrix display API. Early versions felt slow when changing components, so I reduced display hold time per loop to improve perceived button latency while keeping stable output. Another challenge was getting robust analog behavior from the potentiometer; adding endpoint clamping and light smoothing reduced jitter without making control feel sluggish.

## Interesting and Useful Notes

Transition-based PWM scheduling is the key performance idea in this project. Instead of interrupting every 100 us, the code computes the next RGB transition boundary and schedules the timer directly for that event, which keeps interrupt count low and timing predictable. The implementation also treats the LED as active-low in software, which is easy to forget during debugging. Finally, the current duty cap at 50/100 steps provides a practical safety margin for both LED and GPIO drive limits in this wiring setup.

## Build and Run

Build the project with:

`cargo build --release`

Flash and run on hardware with:

`cargo embed --release`

The expected wiring is Red=`P8`, Green=`P9`, Blue=`P16`, LED common anode=`+3.3V`, and potentiometer wiper=`P2`.

## License

This work is made available under the MIT License. Please see the file `LICENSE.txt` in this repository for license information.

## Acknowledgements

Thanks to Bart Massey for the design and project guidance. Also to my peers for troubleshooting tips on hardware setup and timing behavior.