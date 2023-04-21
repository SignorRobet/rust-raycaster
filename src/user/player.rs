use crate::game::game_map::GameMap;
use macroquad::logging::*;

use super::controls::Movement;

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
    pub fn update(&mut self, game_map: &GameMap, movement: &Movement) -> bool {
        // store our current position in case we might need it later
        let previous_pos = (self.x, self.y);

        // sin functions are negative because map y-axis is flipped
        if movement.forward {
            self.x += self.theta.cos() * self.move_speed;
            self.y += -self.theta.sin() * self.move_speed;
        }
        if movement.back {
            self.x -= self.theta.cos() * self.move_speed;
            self.y -= -self.theta.sin() * self.move_speed;
        }
        if movement.left {
            self.theta += self.rotate_speed;
        }
        if movement.right {
            self.theta -= self.rotate_speed;
        }
        // if moving us on this frame put us into a wall just revert it
        if game_map.point_in_wall(self.x, self.y) {
            (self.x, self.y) = previous_pos;
        }
        if movement.left || movement.right || (self.x, self.y) != previous_pos {
            return true;
        } else {
            return false;
        }
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
