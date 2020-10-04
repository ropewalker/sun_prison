use crate::resources::*;
use bevy::prelude::*;

pub fn sun_movement_system(mut current_turn: ResMut<CurrentTurn>, mut sun_path: ResMut<SunPath>) {
    if current_turn.side == GameSide::Sun && current_turn.state == GameState::Playing {
        sun_path.hour += 1;
        current_turn.side = GameSide::Player;
    }
}
