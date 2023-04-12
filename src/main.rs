use macroquad::prelude::*;

#[macroquad::main("Ray-Caster")]
async fn main() {
    let mut i: f32 = 0.0;
    loop {
        clear_background(LIME);

        draw_line(40.0, 40.0, 100.0, 200.0, i, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        i += 1.0;
        if i > 50.0 {
            i = 1.0;
        }
        next_frame().await
    }
}
