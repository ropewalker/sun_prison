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

pub fn insolation_to_sprite_index(insolation: &Insolation, is_highlighted: &IsHighlighted) -> u32 {
    match (insolation, is_highlighted.0) {
        (Insolation::Day, false) => 0,
        (Insolation::Twilight, false) => 1,
        (Insolation::Night, false) => 2,
        (Insolation::Day, true) => 3,
        (Insolation::Twilight, true) => 4,
        (Insolation::Night, true) => 5,
    }
}
