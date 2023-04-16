use super::*;

#[test]
fn test_player_set_pose() {
    let mut player = player::Player::init(0.0, 0.0, 0.0);
    player.set_pose(10.0, 15.0, 30.0);
    assert_eq!(
        player.get_pose(),
        (10.0, 15.0, 30.0),
        "Player pose set to specified state"
    );
}
