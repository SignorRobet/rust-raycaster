use super::player::Player;
use crate::game::game_map::*;

struct CameraProps {
    pub fisheye: bool,
    zoom: u8,
}

impl CameraProps {
    pub fn init_def() -> CameraProps {
        CameraProps {
            fisheye: false,
            zoom: 1,
        }
    }
    pub fn init(fisheye: bool, zoom: u8) -> CameraProps {
        CameraProps { fisheye, zoom }
    }
}

pub fn render(player: Player, game_map: GameMap) {
    let (x, y, theta) = player.get_pose();
}
