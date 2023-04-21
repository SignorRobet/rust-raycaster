mod game;
mod user;

use game::game_map::GameMap;
use macroquad::prelude::*;
use user::controls::Movement;
use user::player::Player;
use user::{camera::Camera, controls};

#[macroquad::main("Ray-Caster")]
async fn main() {
    const GAME_MAP: GameMap = GameMap::init_def();

    let mut player1: Player = Player::init_def();
    let camera: Camera = Camera::init_def();
    camera.render(&player1, &GAME_MAP);

    let mut movement: Movement;
    loop {
        movement = controls::handle_keyboard_input();
        player1.update(&GAME_MAP, &movement);
        camera.render(&player1, &GAME_MAP);

        next_frame().await
    }
}
