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
