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

pub struct GameMap {
    map_array: [u16; 8],
}

impl GameMap {
    pub fn init_def() -> GameMap {
        GameMap { map_array: MAP }
    }
    pub fn init(map_array: [u16; 8]) -> GameMap {
        GameMap { map_array }
    }
    /// Check if the map contains a wall at a point.
    pub fn point_in_wall(&self, x: f32, y: f32) -> bool {
        match self.map_array.get(y as usize) {
            Some(line) => (line & (0b1_u16.checked_shl(x as u32).unwrap_or(0))) != 0,
            None => true,
        }
    }
}
