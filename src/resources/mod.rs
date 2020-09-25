mod constants;

pub use self::constants::*;
use crate::algebra::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

#[derive(Default)]
pub struct KeyboardState {
    pub event_reader: EventReader<KeyboardInput>,
}

pub struct SunPath {
    pub path: Vec<UnitVector>,
    pub current_stage_index: usize,
}

impl Default for SunPath {
    fn default() -> Self {
        use UnitVector::*;

        SunPath {
            path: vec![Right, Down, Front, Left, Up, Back],
            current_stage_index: 0,
        }
    }
}

#[derive(Default, Eq, PartialEq)]
pub struct CurrentTurn {
    pub side: GameSide,
    pub turn_number: usize,
}

#[derive(Eq, PartialEq)]
pub enum GameSide {
    Player,
    Enemies,
    Sun,
}

impl Default for GameSide {
    fn default() -> Self {
        GameSide::Player
    }
}
