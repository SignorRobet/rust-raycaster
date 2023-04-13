mod controls;
mod dynamic_components;
mod static_components;

use dynamic_components::camera;
use macroquad::prelude::*;
use static_components::game_defs::Player;

#[macroquad::main("Ray-Caster")]
async fn main() {
    let player1: Player = Player::init(0.0, 0.0, 0.0);

    let _value: f32 = player1.get_x() + player1.get_y();

    let player2: Player = Player::init_def();

    // camera::render2();

    let mut i: f32 = 0.0;
    loop {
        controls::handle_keyboard_input(player1);
        clear_background(LIME);

        draw_line(40.0, 40.0, 100.0, 200.0, i, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        draw_text(&get_fps().to_string(), 10.0, 20.0, 20.0, DARKGRAY);

        i += 1.0;
        if i > 100.0 {
            i = 1.0;
            // info!("and info message");
        }
        next_frame().await
    }
}
