mod game_coordinates;

pub use self::game_coordinates::*;
use std::collections::HashSet;

pub struct Player;
#[derive(Clone, Copy)]
pub enum Enemy {
    Zombie,
    Ghoul,
    Demon,
}

pub struct Health(pub i8);

pub struct Tile;
pub struct Highlight;
pub struct Fog;
pub struct Exit;

pub struct Obstacle;
pub struct Movable;
pub struct Immovable;
pub struct Opaque;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum ViewshedShape {
    Circle,
    Quadrant,
    All,
}

pub struct Viewshed {
    pub visible_positions: HashSet<Position>,
    pub shape: ViewshedShape,
}

pub struct LastPlayerPosition(pub Option<Position>);
pub struct RememberedObstacles(pub HashSet<Position>);

#[derive(Eq, PartialEq)]
pub enum LabelType {
    Health,
    GameEvents,
}

#[derive(Eq, PartialEq)]
pub struct Label {
    pub label_type: LabelType,
}
