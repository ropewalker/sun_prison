use crate::resources::*;
use bevy::prelude::*;

pub fn enemies_movement_system(mut current_turn: ResMut<CurrentTurn>) {
    if current_turn.side == GameSide::Enemies {
        current_turn.side = GameSide::Sun
    }
}
