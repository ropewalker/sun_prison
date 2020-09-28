use crate::resources::*;
use bevy::prelude::*;

pub fn sun_movement_system(
    _: ChangedRes<CurrentTurn>,
    mut current_turn: ResMut<CurrentTurn>,
    mut sun_path: ResMut<SunPath>,
) {
    if current_turn.side == GameSide::Sun {
        if current_turn.turn_number % 5 == 0 {
            sun_path.current_stage_index = (sun_path.current_stage_index + 1) % sun_path.path.len();
        }

        *current_turn = CurrentTurn {
            side: GameSide::Player,
            turn_number: current_turn.turn_number + 1,
        };
    }
}
