#[derive(Copy, Clone)]
pub struct Player {
    x: f32,
    y: f32,
    theta: f32,
}

impl Player {
    pub fn init_def() -> Player {
        Player {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
        }
    }
    pub fn init(x: f32, y: f32, theta: f32) -> Player {
        Player { x, y, theta }
    }
    pub fn update(&mut self, delta_x: f32, delta_y: f32, delta_theta: f32) {
        self.x += delta_x;
        self.y += delta_y;
        self.theta += delta_theta;
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
}
