#![no_main]
#![no_std]

mod button;
mod rgb_display;

use core::cell::RefCell;

use button::Button;
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

const HUE_MAX_DEGREES: f32 = 300.0;

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
        h: 0.0,
        s: 1.0,
        v: 0.5,
    };
    let mut component = Component::H;

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

        let knob = read_knob_0_1(&mut adc, &mut pot);
        match component {
            Component::H => hsv.h = knob * HUE_MAX_DEGREES,
            Component::S => hsv.s = knob,
            Component::V => hsv.v = knob,
        }

        cortex_m::interrupt::free(|cs| {
            if let Some(rgb_display) = RGB_DISPLAY.borrow(cs).borrow_mut().as_mut() {
                rgb_display.set(&hsv);
            }
        });

        display.show(&mut display_timer, component.letter(), 100);
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

fn read_knob_0_1<P>(adc: &mut Saadc, pin: &mut P) -> f32
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
    (clamped - lo) as f32 / (hi - lo) as f32
}

#[derive(Clone, Copy)]
enum Component {
    H,
    S,
    V,
}

impl Component {
    fn prev(self) -> Self {
        match self {
            Self::H => Self::V,
            Self::S => Self::H,
            Self::V => Self::S,
        }
    }

    fn next(self) -> Self {
        match self {
            Self::H => Self::S,
            Self::S => Self::V,
            Self::V => Self::H,
        }
    }

    fn letter(self) -> [[u8; 5]; 5] {
        match self {
            Self::H => [
                [0, 1, 0, 1, 0],
                [0, 1, 0, 1, 0],
                [0, 1, 1, 1, 0],
                [0, 1, 0, 1, 0],
                [0, 1, 0, 1, 0],
            ],
            Self::S => [
                [0, 1, 1, 1, 0],
                [1, 0, 0, 0, 0],
                [0, 1, 1, 1, 0],
                [0, 0, 0, 1, 0],
                [1, 1, 1, 0, 0],
            ],
            Self::V => [
                [1, 0, 0, 0, 1],
                [1, 0, 0, 0, 1],
                [0, 1, 0, 1, 0],
                [0, 1, 0, 1, 0],
                [0, 0, 1, 0, 0],
            ],
        }
    }
}
