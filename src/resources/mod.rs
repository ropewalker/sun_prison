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
                Vector3::new(1, 0, 0),  //right
                Vector3::new(0, -1, 0), //down
                Vector3::new(0, 0, 1),  //front
                Vector3::new(-1, 0, 0), //left
                Vector3::new(0, 1, 0),  //up
                Vector3::new(0, 0, -1), //back
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
