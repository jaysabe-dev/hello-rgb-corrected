#![no_main]
#![no_std]

mod button;
mod component;
mod hsv_control;
mod rgb_display;

use core::cell::RefCell;

use button::Button;
use component::Component;
use hsv_control::apply_selected_component;
use rgb_display::RgbDisplay;

use cortex_m::{interrupt::Mutex, peripheral::NVIC};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use hsv::Hsv;
use microbit::hal::pac::interrupt;
use microbit::{
    display::blocking::Display,
    hal::{
        pac,
        saadc::{Channel, Saadc, SaadcConfig},
        Timer,
    },
    Board,
};

const KNOB_SMOOTHING_ALPHA: f32 = 0.2;
const DISPLAY_SHOW_MS: u32 = 20;

static RGB_DISPLAY: Mutex<RefCell<Option<RgbDisplay>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut display = Display::new(board.display_pins);
    let mut display_timer = Timer::new(board.TIMER1);

    let mut button_a = Button::new(board.buttons.button_a);
    let mut button_b = Button::new(board.buttons.button_b);

    let mut adc = Saadc::new(board.ADC, SaadcConfig::default());
    let mut pot = board.edge.e02;

    let timer0 = Timer::new(board.TIMER0);
    let rgb_pins = [
        board.edge.e08.degrade(),
        board.edge.e09.degrade(),
        board.edge.e16.degrade(),
    ];

    let mut rgb_display = RgbDisplay::new(rgb_pins, timer0);

    let mut hsv = Hsv {
        h: 0.8,
        s: 0.2,
        v: 0.5,
    };
    let mut component = Component::H;
    let mut knob_smoothed = read_knob_sample_0_1(&mut adc, &mut pot).1;

    rgb_display.set(&hsv);
    rgb_display.step();

    cortex_m::interrupt::free(|cs| {
        RGB_DISPLAY.borrow(cs).replace(Some(rgb_display));
    });

    unsafe {
        NVIC::unmask(pac::Interrupt::TIMER0);
    }

    loop {
        if button_a.was_pressed() {
            component = component.prev();
        }
        if button_b.was_pressed() {
            component = component.next();
        }

        let (_, knob_raw) = read_knob_sample_0_1(&mut adc, &mut pot);
        knob_smoothed += (knob_raw - knob_smoothed) * KNOB_SMOOTHING_ALPHA;
        let knob = knob_smoothed;
        apply_selected_component(&mut hsv, component, knob);

        cortex_m::interrupt::free(|cs| {
            if let Some(rgb_display) = RGB_DISPLAY.borrow(cs).borrow_mut().as_mut() {
                rgb_display.set(&hsv);
            }
        });

        // Keep UI display responsive by avoiding long blocking intervals.
        display.show(
            &mut display_timer,
            component_letter(component),
            DISPLAY_SHOW_MS,
        );
    }
}

#[interrupt]
fn TIMER0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(rgb_display) = RGB_DISPLAY.borrow(cs).borrow_mut().as_mut() {
            rgb_display.step();
        }
    });
}

fn read_knob_sample_0_1<P>(adc: &mut Saadc, pin: &mut P) -> (i32, f32)
where
    P: Channel,
{
    const ADC_MIN: i32 = -(1 << 13);
    const ADC_MAX: i32 = (1 << 13) - 1;
    const DEAD_ZONE: i32 = 64;

    let raw = adc.read_channel(pin).unwrap_or(0) as i32;
    let lo = ADC_MIN + DEAD_ZONE;
    let hi = ADC_MAX - DEAD_ZONE;
    let clamped = raw.clamp(lo, hi);
    let normalized = (clamped - lo) as f32 / (hi - lo) as f32;
    (raw, normalized)
}

fn component_letter(component: Component) -> [[u8; 5]; 5] {
    match component {
        Component::H => [
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 1, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
        ],
        Component::S => [
            [0, 1, 1, 1, 0],
            [1, 0, 0, 0, 0],
            [0, 1, 1, 1, 0],
            [0, 0, 0, 1, 0],
            [1, 1, 1, 0, 0],
        ],
        Component::V => [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [0, 1, 0, 1, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 1, 0, 0],
        ],
    }
}
