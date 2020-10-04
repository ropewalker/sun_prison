use crate::resources::*;
use bevy::prelude::*;

pub fn sun_movement_system(mut game_state: ResMut<GameState>, mut sun_path: ResMut<SunPath>) {
    if *game_state == GameState::SunTurn {
        sun_path.hour += 1;
        *game_state = GameState::PlayerTurn;
    }
}
