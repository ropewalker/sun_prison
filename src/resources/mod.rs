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
    path: Vec<UnitVector>,
    pub hour: usize,
}

impl SunPath {
    pub fn sunny_side(&self) -> UnitVector {
        self.path[self.hour / DAY_LENGTH % self.path.len()]
    }

    pub fn morning_side(&self) -> UnitVector {
        self.path[(self.hour / DAY_LENGTH + 1) % self.path.len()]
    }

    pub fn evening_side(&self) -> UnitVector {
        self.path[(self.path.len() + self.hour / DAY_LENGTH - 1) % self.path.len()]
    }
}

impl Default for SunPath {
    fn default() -> Self {
        use UnitVector::*;

        SunPath {
            path: vec![Right, Down, Front, Left, Up, Back],
            hour: 0,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn,
    SunTurn,
    Defeat,
    Victory,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::PlayerTurn
    }
}
