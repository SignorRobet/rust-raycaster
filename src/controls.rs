use super::static_components::game_defs::Player;
use macroquad::logging::*;
use macroquad::{input, input::KeyCode};

pub fn handle_keyboard_input(player_state: Player) {
    let x = player_state.get_x();
    let y = player_state.get_y();
    let theta = player_state.get_theta();

    // info!("Keyboard Input");
    if input::is_key_down(KeyCode::W) {
        info!("W Down");
    }
    if input::is_key_down(KeyCode::A) {
        info!("A Down");
    }
    if input::is_key_down(KeyCode::S) {
        info!("S Down");
    }
    if input::is_key_down(KeyCode::D) {
        info!("D Down");
    }

    if input::is_key_pressed(KeyCode::Q) {
        info! {"Q Pressed"}
    }
}
