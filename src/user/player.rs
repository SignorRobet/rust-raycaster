use std::f32::consts::{PI, TAU};

use macroquad::prelude::info;

use super::controls::Input;
use crate::game::game_map::GameMap;

const COLLISION_BUFFER: f32 = 0.075;
const COLLISION_CORNER: f32 = COLLISION_BUFFER / 2.;

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

    // Update player pose by delta increments
    pub fn update(&mut self, game_map: &GameMap, input: &Input) {
        // store our current position in case we might need it later
        // (self.x_prev, self.y_prev) = (self.x, self.y);

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
            dx += self.move_speed * -self.theta.sin();
            dy -= self.move_speed * self.theta.cos();
        }
        if input.movement_input.right {
            dx -= self.move_speed * -self.theta.sin();
            dy += self.move_speed * self.theta.cos();
        }
        if input.movement_input.rotate_left {
            self.theta += self.rotate_speed;
        }
        if input.movement_input.rotate_right {
            self.theta -= self.rotate_speed;
        }
        // Place bound on player angle
        if self.theta > TAU {
            self.theta -= TAU;
        } else if self.theta < 0. {
            self.theta += TAU;
        }

        let new_x: f32 = self.x + dx;
        let new_y: f32 = self.y + dy;

        if new_x != self.x || new_y != self.y {
            let (x_coll, y_coll) = self.detect_collision_with_buffer(game_map, new_x, new_y);
            self.handle_coll_buffer_and_move(x_coll, y_coll, new_x, new_y);
        }
    }

    fn detect_collision(&self, game_map: &GameMap, new_x: f32, new_y: f32) -> (bool, bool) {
        let mut collisions: (bool, bool) = (false, false);

        if game_map.point_in_wall(new_x, self.y) {
            collisions.0 = true;
            // [info!("X collision")];
        }
        if game_map.point_in_wall(self.x, new_y) {
            collisions.1 = true;
            // [info!("Y collision")];
        }
        return collisions;
    }

    fn detect_collision_with_buffer(
        &self,
        game_map: &GameMap,
        new_x: f32,
        new_y: f32,
    ) -> ((bool, bool), (bool, bool)) {
        let mut collisions = ((false, false), (false, false));

        // [info!("X collision")];
        if game_map.point_in_wall(new_x + COLLISION_BUFFER, self.y - COLLISION_CORNER)
            | game_map.point_in_wall(new_x + COLLISION_BUFFER, self.y + COLLISION_CORNER)
        {
            collisions.0 .0 = true;
        } else if game_map.point_in_wall(new_x - COLLISION_BUFFER, self.y - COLLISION_CORNER)
            | game_map.point_in_wall(new_x - COLLISION_BUFFER, self.y + COLLISION_CORNER)
        {
            collisions.0 .1 = true;
        }
        // [info!("Y collision")];
        if game_map.point_in_wall(self.x - COLLISION_CORNER, new_y + COLLISION_BUFFER)
            | game_map.point_in_wall(self.x + COLLISION_CORNER, new_y + COLLISION_BUFFER)
        {
            collisions.1 .0 = true;
        } else if game_map.point_in_wall(self.x - COLLISION_CORNER, new_y - COLLISION_BUFFER)
            | game_map.point_in_wall(self.x + COLLISION_CORNER, new_y - COLLISION_BUFFER)
        {
            collisions.1 .1 = true;
        }
        return collisions;
    }

    fn handle_coll_buffer_and_move(
        &mut self,
        x_coll: (bool, bool),
        y_coll: (bool, bool),
        new_x: f32,
        new_y: f32,
    ) {
        if !x_coll.0 & !x_coll.1 {
            self.x = new_x;
        } else {
            if x_coll.0 & (new_x < self.x) {
                self.x = new_x;
            } else if x_coll.1 & (new_x > self.x) {
                self.x = new_x;
            }
        }
        if !y_coll.0 & !y_coll.1 {
            self.y = new_y;
        } else {
            if y_coll.0 & (new_y < self.y) {
                self.y = new_y;
            } else if y_coll.1 & (new_y > self.y) {
                self.y = new_y;
            }
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
