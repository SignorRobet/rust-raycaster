use macroquad::logging::*;
use macroquad::{input, input::KeyCode};

pub struct Movement {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}
impl Movement {
    pub const fn init() -> Movement {
        Movement {
            forward: false,
            back: false,
            left: false,
            right: false,
        }
    }
}

pub fn handle_keyboard_input() -> Movement {
    let mut movement = Movement::init();

    // Movement Controls
    if input::is_key_down(KeyCode::W) {
        movement.forward = true;
    }
    if input::is_key_down(KeyCode::A) {
        movement.left = true;
    }
    if input::is_key_down(KeyCode::S) {
        movement.back = true;
    }
    if input::is_key_down(KeyCode::D) {
        movement.right = true;
    }

    if input::is_key_pressed(KeyCode::Q) {
        info! {"Q Pressed"}
    }
    return movement;
}
