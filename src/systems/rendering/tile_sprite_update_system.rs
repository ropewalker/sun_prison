use crate::components::*;
use bevy::prelude::*;

pub fn tile_sprite_update_system(
    _tile: &Tile,
    insolation: &Insolation,
    is_highlighted: &IsHighlighted,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    sprite.index = insolation_to_sprite_index(insolation, is_highlighted);
}

fn insolation_to_sprite_index(insolation: &Insolation, is_highlighted: &IsHighlighted) -> u32 {
    match (insolation, is_highlighted.0) {
        (Insolation::Day, false) => 0,
        (Insolation::Twilight, false) => 1,
        (Insolation::Night, false) => 2,
        (Insolation::Day, true) => 3,
        (Insolation::Twilight, true) => 4,
        (Insolation::Night, true) => 5,
    }
}
