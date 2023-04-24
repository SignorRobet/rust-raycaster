use macroquad::{input, input::KeyCode, input::MouseButton};

pub struct Input {
    pub movement_input: MovementInput,
    pub game_input: GameInput,
    pub player_input: PlayerInput,
}
impl Input {
    pub fn init() -> Input {
        Input {
            movement_input: get_movement_input(),
            player_input: get_player_input(),
            game_input: get_game_input(),
        }
    }

    pub fn get_input(&mut self) {
        self.movement_input = get_movement_input();
        self.player_input = get_player_input();
        self.game_input = get_game_input();
    }
}

pub struct MovementInput {
    pub forward: bool,
    pub back: bool,
    pub left: bool,
    pub right: bool,
    pub rotate_left: bool,
    pub rotate_right: bool,
}
impl MovementInput {
    const fn init() -> MovementInput {
        MovementInput {
            forward: false,
            back: false,
            left: false,
            right: false,
            rotate_left: false,
            rotate_right: false,
        }
    }
}

pub struct GameInput {
    pub menu: bool,
    pub fisheye: bool,
    pub collision_buffer: bool,
}
impl GameInput {
    const fn init() -> GameInput {
        GameInput {
            menu: false,
            fisheye: false,
            collision_buffer: false,
        }
    }
}

pub struct PlayerInput {
    pub interact: bool,
    pub shoot: bool,
}
impl PlayerInput {
    const fn init() -> PlayerInput {
        PlayerInput {
            interact: false,
            shoot: false,
        }
    }
}

fn get_movement_input() -> MovementInput {
    let mut movement: MovementInput = MovementInput::init();

    // MovementInput Controls
    if input::is_key_down(KeyCode::W) {
        movement.forward = true;
    }
    if input::is_key_down(KeyCode::S) {
        movement.back = true;
    }
    if input::is_key_down(KeyCode::A) {
        movement.left = true;
    }
    if input::is_key_down(KeyCode::D) {
        movement.right = true;
    }
    if input::is_key_down(KeyCode::Q) {
        movement.rotate_left = true;
    }
    if input::is_key_down(KeyCode::E) {
        movement.rotate_right = true;
    }
    return movement;
}

fn get_player_input() -> PlayerInput {
    let mut player_input: PlayerInput = PlayerInput::init();

    if input::is_mouse_button_pressed(MouseButton::Left) {
        player_input.shoot = true;
    }
    return player_input;
}

fn get_game_input() -> GameInput {
    let mut game_input: GameInput = GameInput::init();

    if input::is_key_pressed(KeyCode::F) {
        game_input.fisheye = true;
    }
    if input::is_key_pressed(KeyCode::C) {
        game_input.collision_buffer = true;
    }
    return game_input;
}
