use crate::game::game_map::GameMap;

use super::player::Player;
use macroquad::logging::*;
use macroquad::{input, input::KeyCode};

pub struct Movement {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
}
impl Movement {
    fn init(forward: bool, back: bool, left: bool, right: bool) -> Movement {
        Movement {
            forward,
            back,
            left,
            right,
        }
    }
}

pub fn handle_keyboard_input(player: &mut Player, game_map: &GameMap) {
    let mut movement = Movement::init(false, false, false, false);

    // Movement Controls
    if input::is_key_down(KeyCode::W) {
        info!("W Down");
        movement.forward = true;
    }
    if input::is_key_down(KeyCode::A) {
        info!("A Down");
        movement.left = true;
    }
    if input::is_key_down(KeyCode::S) {
        info!("S Down");
        movement.back = true;
    }
    if input::is_key_down(KeyCode::D) {
        info!("D Down");
        movement.right = true;
    }
    player.update(
        game_map,
        movement.forward,
        movement.back,
        movement.left,
        movement.right,
    );

    if input::is_key_pressed(KeyCode::Q) {
        info! {"Q Pressed"}
    }
}
