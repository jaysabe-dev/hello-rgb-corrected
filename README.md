# hello-rgb: HSV-controlled RGB LED demo for Micro:bit v2
Jayabe 2026-03-08

`hello-rgb` is a Rust embedded project for CS-471/571-001 that demonstrates controlling an external RGB LED using HSV input. Button A and Button B select which component (`H`, `S`, or `V`) is currently being edited, the potentiometer on `P2` sets that component value, and the MB2 5x5 display shows the active component letter. HSV is converted to RGB using the instructor `hsv` crate at <https://github.com/pdx-cs-rust-embedded/hsv>, and output is rendered through timer-interrupt PWM.

The PWM path in `src/rgb_display.rs` uses a 100-step brightness frame with 100 us step duration (100 Hz frame rate). To match assignment guidance, the timer is scheduled at the next channel transition instead of interrupting every 100 us. Duty is capped at 50/100 steps per channel for safety.

## Build and Run

Build the project with:

`cargo build --release`

Flash and run on hardware with:

`cargo embed --release`

The expected wiring is Red=`P8`, Green=`P9`, Blue=`P16`, LED common anode=`+3.3V`, and potentiometer wiper=`P2`.

## License

This work is made available under the MIT License. Please see the file `LICENSE.txt` in this repository for license information.

## Acknowledgements

Thanks to Bart Massey for the assignment design and project guidance, and to the course staff and classmates for troubleshooting tips on hardware setup and timing behavior.
