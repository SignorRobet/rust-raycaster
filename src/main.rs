mod game;
mod user;

use game::map::GameMap;
use macroquad::prelude::*;
use user::camera::{Camera, RES_F32, VIEWPORT_OFFSET};
use user::controls::Input;
use user::player::Player;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ray-Caster".to_owned(),
        window_width: (RES_F32.0 + (VIEWPORT_OFFSET * 2.)) as i32,
        window_height: (RES_F32.1 + (VIEWPORT_OFFSET * 2.)) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_map: GameMap = GameMap::init_def();
    let mut player1: Player = Player::init_def();
    let mut camera: Camera = Camera::init();
    camera.render(&player1, &game_map);

    let mut input: Input = Input::init();
    loop {
        input.get_input();

        game_map.handle_input(&input);
        camera.handle_input(&input);
        player1.update(&game_map, &input);

        camera.render(&player1, &game_map);

        next_frame().await
    }
}
