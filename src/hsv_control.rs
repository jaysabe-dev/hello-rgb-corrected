use crate::component::Component;
use hsv::Hsv;

const HUE_MAX_TURNS: f32 = 1.0;

pub fn apply_selected_component(hsv: &mut Hsv, component: Component, knob_0_to_1: f32) {
    match component {
        Component::H => apply_hue(hsv, knob_0_to_1),
        Component::S => apply_saturation(hsv, knob_0_to_1),
        Component::V => apply_value(hsv, knob_0_to_1),
    }
}

pub fn apply_hue(hsv: &mut Hsv, knob_0_to_1: f32) {
    hsv.h = knob_0_to_1 * HUE_MAX_TURNS;
}

pub fn apply_saturation(hsv: &mut Hsv, knob_0_to_1: f32) {
    hsv.s = knob_0_to_1;
}

pub fn apply_value(hsv: &mut Hsv, knob_0_to_1: f32) {
    hsv.v = knob_0_to_1;
}
