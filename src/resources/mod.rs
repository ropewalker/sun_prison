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
    pub path: Vec<Vector3>,
    pub current_stage_index: usize,
}

impl Default for SunPath {
    fn default() -> Self {
        SunPath {
            path: vec![
                (1, 0, 0).into(),  //right
                (0, -1, 0).into(), //down
                (0, 0, 1).into(),  //front
                (-1, 0, 0).into(), //left
                (0, 1, 0).into(),  //up
                (0, 0, -1).into(), //back
            ],
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
    Sun,
}

impl Default for GameSide {
    fn default() -> Self {
        GameSide::Player
    }
}
