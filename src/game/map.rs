use macroquad::prelude::info;

use crate::user::controls::Input;

const MAP: [u16; 8] = [
    0b1111111111111111,
    0b1000001010000101,
    0b1011100000110101,
    0b1000111010010001,
    0b1010001011110111,
    0b1011101001100001,
    0b1000100000001101,
    0b1111111111111111,
];
const DEF_COLLISION_BUFFER: f32 = 0.075;
const DEF_COLLISION_CORNER: f32 = DEF_COLLISION_BUFFER / 2.;

pub struct GameMap {
    map_array: [u16; 8],
    collision_buffer: f32,
    collision_corner: f32,
    collision_buffer_enabled: bool,
}

#[allow(dead_code)]
impl GameMap {
    pub const fn init_def() -> GameMap {
        GameMap {
            map_array: MAP,
            collision_buffer: DEF_COLLISION_BUFFER,
            collision_corner: DEF_COLLISION_CORNER,
            collision_buffer_enabled: true,
        }
    }
    pub const fn init(map_array: [u16; 8]) -> GameMap {
        GameMap {
            map_array,
            collision_buffer: DEF_COLLISION_BUFFER,
            collision_corner: DEF_COLLISION_CORNER,
            collision_buffer_enabled: true,
        }
    }
    /// Check if the map contains a wall at a point.
    pub fn point_in_wall(&self, x: f32, y: f32) -> bool {
        match self.map_array.get(y as usize) {
            Some(line) => (line & (0b1_u16.checked_shl(x as u32).unwrap_or(0))) != 0,
            None => true,
        }
    }

    pub fn handle_input(&mut self, input: &Input) {
        if input.game_input.collision_buffer {
            self.toggle_collision_buffering();
        }
    }

    pub fn is_collision_buffered(&self) -> bool {
        return self.collision_buffer_enabled;
    }

    pub fn toggle_collision_buffering(&mut self) -> bool {
        self.collision_buffer_enabled = !self.collision_buffer_enabled;
        [info!(
            "{}",
            format!(
                "Collision Buffer Toggled: {}",
                self.collision_buffer_enabled
            )
        )];
        return self.collision_buffer_enabled;
    }

    pub fn get_collision_buffers(&self) -> (f32, f32) {
        return (self.collision_buffer, self.collision_corner);
    }

    pub fn set_collision_buffer(&mut self, collision_buffer: f32) -> (f32, f32) {
        self.collision_buffer = collision_buffer;
        self.collision_corner = collision_buffer / 2.;

        return (self.collision_buffer, self.collision_corner);
    }
}
