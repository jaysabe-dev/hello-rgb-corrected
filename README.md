# hello-rgb: MB2 HSV RGB demo

This Micro:bit v2 project implements HSV control of an external RGB LED.

- `Button A`: previous component (`H -> V -> S -> H`)
- `Button B`: next component (`H -> S -> V -> H`)
- Knob (potentiometer): sets the selected component in range `0.0..1.0`
- MB2 LED matrix: shows the currently selected component letter (`H`, `S`, or `V`)

The HSV value is converted to RGB, and RGB is displayed using timer-interrupt PWM.

## What I implemented

- Button-driven component selection for `H`, `S`, and `V`
- Potentiometer-driven update of only the selected HSV component
- HSV to RGB conversion through the `hsv` crate
- Timer-interrupt PWM for external RGB LED output
- Blocking MB2 LED matrix updates in the superloop (`H`, `S`, `V`)
- Refactor: all component-specific logic (`H`/`S`/`V`) moved into `src/component.rs` for easier testing

## PWM behavior

- Frame size: 100 steps
- Step period: 100 µs
- Frame rate: 100 Hz
- Timer interrupts occur only at the next channel transition (not every step)
- Output duty is capped at 50/100 steps per channel to stay within the assignment safety target

## Requirements alignment

- Uses `A`/`B` buttons to move previous/next across `H`, `S`, `V`
- Uses potentiometer on `P2` to set the selected component in `0.0..1.0`
- Displays current selected component letter on MB2 matrix
- Uses timer interrupt (`TIMER0`) to run PWM updates
- PWM scale is exactly `100` steps with `100 us` per step (`10 ms` frame, `100 Hz`)
- Does not interrupt every `100 us`; timer is scheduled only at next LED transition
- Keeps each channel duty capped at 50% (`MAX_DUTY_STEPS = 50`)
- Enables timer interrupt on both timer peripheral and NVIC
- Keeps button processing, ADC reading, HSV update, and LED letter display in superloop

## How it went

The main challenge was keeping PWM timing efficient while preserving a straightforward superloop for UI updates. Scheduling interrupts only at RGB transition times made the timer load low and kept the display logic simple. A second challenge was noisy ADC readings from the potentiometer: clamping and light smoothing (`alpha = 0.2`) gave stable control without making the knob feel unresponsive.

## Interesting notes

- The external LED wiring behaves as active-low from GPIO perspective in this setup, so pin-low turns a channel on.
- Using a pending "next schedule" lets the color update cleanly at frame boundaries instead of mid-frame.

## Wiring

RGB LED (common anode):

- Red to `P8`
- Green to `P9`
- Blue to `P16`
- Common anode (longest lead) to `+3.3V`

Potentiometer:

- Pin 1 to `GND`
- Pin 2 (wiper) to `P2`
- Pin 3 to `+3.3V`

## Build / flash

- Build: `cargo build --release`
- Flash/debug (probe-rs): `cargo embed --release`

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
