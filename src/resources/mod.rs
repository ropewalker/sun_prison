mod constants;

pub use self::constants::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

#[derive(Default)]
pub struct KeyboardState {
    pub event_reader: EventReader<KeyboardInput>,
}

#[derive(Eq, PartialEq)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn,
    Defeat,
    Victory,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::PlayerTurn
    }
}
