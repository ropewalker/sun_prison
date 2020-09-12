mod game_coordinates;

pub use self::game_coordinates::*;

pub struct Player;

pub struct Tile;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Insolation {
    Day,
    Twilight,
    Night,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct IsHighlighted(pub bool);

pub struct Movable;
pub struct Immovable;

pub struct Wall;

pub struct Opaque;
