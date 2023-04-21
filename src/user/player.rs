use macroquad::prelude::info;

use super::controls::Input;
use crate::game::game_map::GameMap;

pub struct Player {
    x: f32,
    y: f32,
    theta: f32,
    move_speed: f32,
    rotate_speed: f32,
}

#[allow(dead_code)]
impl Player {
    pub const fn init_def() -> Player {
        Player {
            x: 1.5,
            y: 1.5,
            theta: 0.0,
            move_speed: 0.045,
            rotate_speed: 0.045,
        }
    }
    pub fn init(x: f32, y: f32, theta: f32) -> Player {
        Player {
            x,
            y,
            theta,
            move_speed: 0.045,
            rotate_speed: 2.0,
        }
    }

    // Update player pose by delta increments
    pub fn update(&mut self, game_map: &GameMap, input: &Input) -> bool {
        // store our current position in case we might need it later
        let previous_pos: (f32, f32) = (self.x, self.y);

        let mut dx: f32 = 0.;
        let mut dy: f32 = 0.;

        // sin functions are negative because map y-axis is flipped
        if input.movement_input.forward {
            dx += self.move_speed * self.theta.cos();
            dy += self.move_speed * -self.theta.sin();
        }
        if input.movement_input.back {
            dx -= self.move_speed * self.theta.cos();
            dy -= self.move_speed * -self.theta.sin();
        }
        if input.movement_input.left {
            self.theta += self.rotate_speed;
        }
        if input.movement_input.right {
            self.theta -= self.rotate_speed;
        }
        let (x_coll, y_coll): (bool, bool) =
            Player::detect_collision(&game_map, self.x, self.y, dx, dy);

        if !x_coll {
            self.x += dx;
        }
        if !y_coll {
            self.y += dy;
        }
        // if moving us on this frame put us into a wall just revert it
        // if !game_map.point_in_wall(new_x, new_y) {
        //     (self.x, self.y) = (new_x, new_y);
        // }
        if input.movement_input.left
            || input.movement_input.right
            || (self.x, self.y) != previous_pos
        {
            return true;
        } else {
            return false;
        }
    }

    fn detect_collision(game_map: &GameMap, x: f32, y: f32, dx: f32, dy: f32) -> (bool, bool) {
        let mut collisions: (bool, bool) = (false, false);
        let new_x: f32 = x + dx;
        let new_y: f32 = y + dy;

        if game_map.point_in_wall(new_x, y) {
            collisions.0 = true;
            [info!("X collision")];
        }
        if game_map.point_in_wall(x, new_y) {
            collisions.1 = true;
            [info!("Y collision")];
        }

        return collisions;
    }

    // Set player pose to specified position
    pub fn set_pose(&mut self, x: f32, y: f32, theta: f32) {
        self.x = x;
        self.y = y;
        self.theta = theta;
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_theta(&self) -> f32 {
        self.theta
    }

    // Return x, y and theta in a tuple
    pub fn get_pose(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.theta)
    }
}
