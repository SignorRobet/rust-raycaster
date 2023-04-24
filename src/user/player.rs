use std::f32::consts::{PI, TAU};

use macroquad::prelude::info;

use super::controls::{Input, MovementInput};
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
            rotate_speed: 0.045,
        }
    }

    // Update player pose
    pub fn update(&mut self, game_map: &GameMap, input: &Input) {
        // Rotate player, and calculate movement deltas
        self.rotate_player(&input.movement_input);
        let (dx, dy): (f32, f32) = self.calculate_move_deltas(&input.movement_input);
        let (new_x, new_y): (f32, f32) = (self.x + dx, self.y + dy);

        // if player is moving, factor in collisions and move player accordingly
        if new_x != self.x || new_y != self.y {
            let (x_coll, y_coll): ((bool, bool), (bool, bool));
            if game_map.is_collision_buffered() {
                (x_coll, y_coll) = self.detect_collision_with_buffer(game_map, new_x, new_y);
                self.move_player_with_buffer(x_coll, y_coll, new_x, new_y);
            } else {
                (x_coll, y_coll) = self.detect_collision(game_map, new_x, new_y);
                self.move_player(x_coll, y_coll, new_x, new_y);
            }
        }
    }

    fn calculate_move_deltas(&self, movement_input: &MovementInput) -> (f32, f32) {
        let (mut dx, mut dy): (f32, f32) = (0., 0.);

        // sin functions are negative because map y-axis is flipped
        if movement_input.forward {
            dx += self.move_speed * self.theta.cos();
            dy += self.move_speed * -self.theta.sin();
        }
        if movement_input.back {
            dx -= self.move_speed * self.theta.cos();
            dy -= self.move_speed * -self.theta.sin();
        }
        if movement_input.left {
            dx += self.move_speed * -self.theta.sin();
            dy -= self.move_speed * self.theta.cos();
        }
        if movement_input.right {
            dx -= self.move_speed * -self.theta.sin();
            dy += self.move_speed * self.theta.cos();
        }
        return (dx, dy);
    }

    fn rotate_player(&mut self, movement_input: &MovementInput) {
        if movement_input.rotate_left {
            self.theta += self.rotate_speed;
        }
        if movement_input.rotate_right {
            self.theta -= self.rotate_speed;
        }
        // Place bound on player angle
        if self.theta > TAU {
            self.theta -= TAU;
        } else if self.theta < 0. {
            self.theta += TAU;
        }
    }

    fn move_player_with_buffer(
        &mut self,
        x_coll: (bool, bool),
        y_coll: (bool, bool),
        new_x: f32,
        new_y: f32,
    ) {
        if x_coll.0 | x_coll.1 {
            if x_coll.0 & (new_x < self.x) {
                self.x = new_x;
            } else if x_coll.1 & (new_x > self.x) {
                self.x = new_x;
            }
        } else {
            self.x = new_x;
        }
        if y_coll.0 | y_coll.1 {
            if y_coll.0 & (new_y < self.y) {
                self.y = new_y;
            } else if y_coll.1 & (new_y > self.y) {
                self.y = new_y;
            }
        } else {
            self.y = new_y;
        }
    }
    fn move_player(&mut self, x_coll: (bool, bool), y_coll: (bool, bool), new_x: f32, new_y: f32) {
        if !(x_coll.0 & x_coll.1) {
            self.x = new_x;
        }
        if !(y_coll.0 & y_coll.1) {
            self.y = new_y;
        }
    }

    fn detect_collision(
        &self,
        game_map: &GameMap,
        new_x: f32,
        new_y: f32,
    ) -> ((bool, bool), (bool, bool)) {
        let mut collisions: ((bool, bool), (bool, bool)) = ((false, false), (false, false));

        if game_map.point_in_wall(new_x, self.y) {
            collisions.0 .0 = true;
            collisions.0 .1 = true;
        }
        if game_map.point_in_wall(self.x, new_y) {
            collisions.1 .0 = true;
            collisions.1 .1 = true;
        }
        return collisions;
    }

    fn detect_collision_with_buffer(
        &self,
        game_map: &GameMap,
        new_x: f32,
        new_y: f32,
    ) -> ((bool, bool), (bool, bool)) {
        let mut collisions: ((bool, bool), (bool, bool)) = ((false, false), (false, false));
        let coll_buffers: (f32, f32) = game_map.get_collision_buffers();

        let ((new_x_pos, new_x_neg), (new_y_pos, new_y_neg)): ((f32, f32), (f32, f32));
        let ((x_pos, x_neg), (y_pos, y_neg)): ((f32, f32), (f32, f32));

        (new_x_pos, new_x_neg) = (new_x + coll_buffers.0, new_x - coll_buffers.0);
        (new_y_pos, new_y_neg) = (new_y + coll_buffers.0, new_y - coll_buffers.0);
        (x_pos, x_neg) = (self.x + coll_buffers.1, self.x - coll_buffers.1);
        (y_pos, y_neg) = (self.y + coll_buffers.1, self.y - coll_buffers.1);

        // [info!("X collision")];
        if game_map.point_in_wall(new_x_pos, y_neg) | game_map.point_in_wall(new_x_pos, y_pos) {
            collisions.0 .0 = true;
        } else if game_map.point_in_wall(new_x_neg, y_neg)
            | game_map.point_in_wall(new_x_neg, y_pos)
        {
            collisions.0 .1 = true;
        }
        // [info!("Y collision")];
        if game_map.point_in_wall(x_neg, new_y_pos) | game_map.point_in_wall(x_pos, new_y_pos) {
            collisions.1 .0 = true;
        } else if game_map.point_in_wall(x_neg, new_y_neg)
            | game_map.point_in_wall(x_pos, new_y_neg)
        {
            collisions.1 .1 = true;
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
