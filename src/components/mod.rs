mod game_coordinates;

pub use self::game_coordinates::*;
use std::collections::HashSet;

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

pub struct Opaque;

#[allow(dead_code)]
pub enum ViewshedShape {
    Circle,
    Quadrant,
}

pub struct Viewshed {
    pub visible_positions: HashSet<Position>,
    pub shape: ViewshedShape,
}
