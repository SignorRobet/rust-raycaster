use macroquad::prelude::*;

use super::{controls::Input, player::Player};
use crate::game::game_map::*;

use core::f32::consts::{FRAC_PI_2, PI};

const FOV: f32 = PI / 2.7; // The player's field of view.
const HALF_FOV: f32 = FOV * 0.5; // Half the player's field of view.
const RESOLUTION: (usize, usize) = (640, 500);
const ANGLE_STEP: f32 = FOV / RESOLUTION.0 as f32; // The angle between each ray.
const WALL_HEIGHT: f32 = 500.0; // A magic number.

pub struct Camera {
    fisheye: bool,
    // zoom: u8,
}

impl Camera {
    pub fn init() -> Camera {
        Camera {
            fisheye: false,
            // zoom: 1,
        }
    }

    pub fn handle_input(&mut self, input: &Input) {
        if input.game_input.fisheye {
            self.toggle_fisheye();
        }
    }

    pub fn render(&self, player: &Player, game_map: &GameMap) {
        let (x, y, theta): (f32, f32, f32) = player.get_pose();

        clear_background(LIME);
        self.get_view_loop((x, y, theta), game_map);

        draw_rectangle_lines(
            20.0,
            20.0,
            RESOLUTION.0 as f32,
            RESOLUTION.1 as f32,
            2.0,
            GREEN,
        );
        draw_text(&get_fps().to_string(), 15.0, 15.0, 20.0, DARKGRAY);
        draw_text(&theta.to_string(), 50.0, 15.0, 20.0, BLACK);
        draw_text(&format!("({}, {})", x, y), 150.0, 15.0, 20.0, BLACK);
    }

    // go through each column on screen and draw walls in the center.
    fn get_view_loop(&self, (x, y, theta): (f32, f32, f32), game_map: &GameMap) {
        for (x, wall) in self.get_view((x, y, theta), game_map).iter().enumerate() {
            let x_f32: f32 = x as f32 + 20.0;
            let (height, shadow) = wall;
            let wall_color;
            if *shadow {
                wall_color = BLUE;
            } else {
                wall_color = SKYBLUE;
            }
            draw_line(
                x_f32,
                200.0 + (80 - (height / 2)) as f32,
                x_f32,
                200.0 + *height as f32,
                1.0,
                wall_color,
            );
        }
    }

    fn get_view(
        &self,
        (x, y, theta): (f32, f32, f32),
        game_map: &GameMap,
    ) -> [(i32, bool); RESOLUTION.0] {
        // The player's FOV is split in half by their viewing angle.
        // In order to get the ray's first angle we must
        // add half the FOV to the player's angle to get
        // the edge of the player's FOV.
        let starting_angle: f32 = theta + HALF_FOV;
        let mut walls: [(i32, bool); RESOLUTION.0] = [(0, false); RESOLUTION.0];

        for (idx, wall) in walls.iter_mut().enumerate() {
            // `idx` is what number ray we are, `wall` is
            // a mutable reference to a value in `walls`.
            let angle: f32 = starting_angle - idx as f32 * ANGLE_STEP;

            // Get both the closest horizontal and vertical wall
            // intersections for this angle.
            let h_dist: f32 = self.horizontal_intersection((x, y), game_map, angle);
            let v_dist: f32 = self.vertical_intersection((x, y), game_map, angle);

            let (min_dist, shadow) = if h_dist < v_dist {
                (h_dist, false)
            } else {
                (v_dist, true)
            };

            // Get the minimum of the two distances and
            // "convert" it into a wall height.
            if self.fisheye {
                *wall = ((WALL_HEIGHT / min_dist) as i32, shadow);
            } else {
                *wall = (
                    (WALL_HEIGHT / (min_dist * (angle - theta).cos())) as i32,
                    shadow,
                );
            }
        }
        return walls;
    }

    fn horizontal_intersection(
        &self,
        (player_x, player_y): (f32, f32),
        game_map: &GameMap,
        angle: f32,
    ) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let up = ((angle / PI).floor() % 2.0).abs() != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_y = if up {
            player_y.ceil() - player_y
        } else {
            player_y.floor() - player_y
        };
        let first_x = -first_y / angle.tan();

        // dy and dx are the "ray extension" values mentioned earlier.
        let dy = if up { 1.0 } else { -1.0 };
        let dx = -dy / angle.tan();

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 255.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually break and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = next_x + player_x;
            let current_y = if up {
                next_y + player_y
            } else {
                next_y + player_y - 1.0
            };

            // Tell the loop to quit if we've just hit a wall.
            if game_map.point_in_wall(current_x, current_y) {
                break;
            }

            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }
        // return the distance from next_x and next_y to the player.
        return distance(next_x, next_y);
    }

    fn vertical_intersection(
        &self,
        (player_x, player_y): (f32, f32),
        game_map: &GameMap,
        angle: f32,
    ) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let right = (((angle - FRAC_PI_2) / PI).floor() % 2.0).abs() != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_x = if right {
            player_x.ceil() - player_x
        } else {
            player_x.floor() - player_x
        };
        let first_y = -(angle.tan()) * first_x;

        // dy and dx are the "ray extension" values mentioned earlier.
        let dx = if right { 1.0 } else { -1.0 };
        let dy = dx * -(angle.tan());

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 255.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually quit and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = if right {
                next_x + player_x
            } else {
                next_x + player_x - 1.0
            };
            let current_y = next_y + player_y;

            // Tell the loop to quit if we've just hit a wall.
            if game_map.point_in_wall(current_x, current_y) {
                break;
            }
            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }
        // return the distance from next_x and next_y to the player.
        return distance(next_x, next_y);
    }

    pub fn toggle_fisheye(&mut self) {
        self.fisheye = !self.fisheye;
    }
}

fn distance(a: f32, b: f32) -> f32 {
    ((a * a) + (b * b)).sqrt()
}
