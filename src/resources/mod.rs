mod constants;

pub use self::constants::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Default)]
pub struct KeyboardState {
    pub event_reader: EventReader<KeyboardInput>,
}

#[derive(Eq, PartialEq)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn(Entity),
    Defeat,
    Victory,
    PassingTurn,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::PlayerTurn
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TurnQueueElement {
    pub entity: Entity,
    pub priority: u32,
}

impl PartialOrd for TurnQueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for TurnQueueElement {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then(other.entity.cmp(&self.entity))
    }
}

pub struct TurnQueue(pub BinaryHeap<TurnQueueElement>);

impl Default for TurnQueue {
    fn default() -> Self {
        TurnQueue(BinaryHeap::new())
    }
}
