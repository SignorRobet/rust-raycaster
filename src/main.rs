mod game;
mod user;

use game::game_map::GameMap;
use macroquad::prelude::*;
use user::camera::Camera;
use user::controls::Input;
use user::player::Player;

#[macroquad::main("Ray-Caster")]
async fn main() {
    const GAME_MAP: GameMap = GameMap::init_def();

    let mut player1: Player = Player::init_def();
    let mut camera: Camera = Camera::init();
    camera.render(&player1, &GAME_MAP);

    let mut input: Input = Input::init();
    loop {
        input.handle_input();
        player1.update(&GAME_MAP, &input);
        camera.handle_input(&input);
        camera.render(&player1, &GAME_MAP);

        next_frame().await
    }
}
