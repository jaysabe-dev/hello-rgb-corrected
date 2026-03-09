use embedded_hal::digital::OutputPin;
use hsv::Hsv;
use microbit::hal::{
    gpio::{self, Level},
    pac, Timer,
};

pub const FRAME_STEPS: u32 = 100;
pub const STEP_US: u32 = 100;
const MAX_DUTY_STEPS: u32 = 50;
const LED_ACTIVE_LOW: bool = true;

pub struct RgbDisplay {
    tick: u32,
    schedule: [u32; 3],
    next_schedule: Option<[u32; 3]>,
    rgb_pins: [gpio::Pin<gpio::Output<gpio::PushPull>>; 3],
    timer0: Timer<pac::TIMER0>,
}

impl RgbDisplay {
    pub fn new<T>(rgb_pins: [gpio::Pin<T>; 3], mut timer0: Timer<pac::TIMER0>) -> Self {
        timer0.enable_interrupt();
        Self {
            tick: 0,
            schedule: [0; 3],
            next_schedule: None,
            rgb_pins: rgb_pins.map(|pin| pin.into_push_pull_output(led_off_level())),
            timer0,
        }
    }

    pub fn set(&mut self, hsv: &Hsv) {
        let rgb = hsv.to_rgb();
        self.next_schedule = Some([
            scale_to_duty_steps(rgb.r),
            scale_to_duty_steps(rgb.g),
            scale_to_duty_steps(rgb.b),
        ]);
    }

    pub fn step(&mut self) {
        self.timer0.reset_event();
        if self.tick == 0 {
            self.begin_frame();
        } else {
            self.apply_turnoffs_for_tick();
        }

        let next_tick = self.next_transition_tick();

        let delay_us = (next_tick - self.tick).max(1) * STEP_US;
        self.tick = if next_tick >= FRAME_STEPS {
            0
        } else {
            next_tick
        };
        self.timer0.start(delay_us);
    }

    fn begin_frame(&mut self) {
        if let Some(schedule) = self.next_schedule.take() {
            self.schedule = schedule;
        }

        // Start of frame: each channel is on unless it should turn off at tick 0.
        for (pin, off_tick) in self.rgb_pins.iter_mut().zip(self.schedule.iter()) {
            set_led(pin, *off_tick > 0);
        }
    }

    fn apply_turnoffs_for_tick(&mut self) {
        // On each transition interrupt, turn off any channels scheduled for this tick.
        for (pin, off_tick) in self.rgb_pins.iter_mut().zip(self.schedule.iter()) {
            if *off_tick == self.tick {
                set_led(pin, false);
            }
        }
    }

    fn next_transition_tick(&self) -> u32 {
        self.schedule
            .iter()
            .copied()
            .filter(|off_tick| *off_tick > self.tick)
            .min()
            .unwrap_or(FRAME_STEPS)
    }
}

fn scale_to_duty_steps(value: f32) -> u32 {
    let clamped = value.clamp(0.0, 1.0);
    (clamped * MAX_DUTY_STEPS as f32 + 0.5) as u32
}

fn led_off_level() -> Level {
    if LED_ACTIVE_LOW {
        Level::High
    } else {
        Level::Low
    }
}

fn set_led(pin: &mut gpio::Pin<gpio::Output<gpio::PushPull>>, on: bool) {
    if on {
        if LED_ACTIVE_LOW {
            let _ = pin.set_low();
        } else {
            let _ = pin.set_high();
        }
    } else if LED_ACTIVE_LOW {
        let _ = pin.set_high();
    } else {
        let _ = pin.set_low();
    }
}
