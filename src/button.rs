use embedded_hal::digital::InputPin;

pub struct Button<P>
where
    P: InputPin,
{
    pin: P,
    was_down: bool,
}

impl<P> Button<P>
where
    P: InputPin,
{
    pub fn new(pin: P) -> Self {
        Self {
            pin,
            was_down: false,
        }
    }

    pub fn was_pressed(&mut self) -> bool {
        let is_down = self.pin.is_low().unwrap_or(false);
        let was_pressed = is_down && !self.was_down;
        self.was_down = is_down;
        was_pressed
    }
}
