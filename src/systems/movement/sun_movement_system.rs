use crate::resources::*;
use bevy::prelude::*;

pub fn sun_movement_system(mut current_turn: ResMut<CurrentTurn>, mut sun_path: ResMut<SunPath>) {
    if current_turn.side == GameSide::Sun && current_turn.state == GameState::Playing {
        if current_turn.turn_number % 5 == 0 {
            sun_path.current_stage_index = (sun_path.current_stage_index + 1) % sun_path.path.len();
        }

        current_turn.side = GameSide::Player;
        current_turn.turn_number += 1;
    }
}
