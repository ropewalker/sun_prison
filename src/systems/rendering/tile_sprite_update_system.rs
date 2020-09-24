use crate::components::*;
use bevy::prelude::*;

pub fn tile_sprite_update_system(
    mut tile_query: Query<(
        &Tile,
        &GameCoordinates,
        &IsHighlighted,
        &mut TextureAtlasSprite,
    )>,
    mut player_query: Query<(&Player, &Viewshed)>,
) {
    for (_, viewshed) in &mut player_query.iter() {
        for (_, tile_coordinates, is_highlighted, mut sprite) in &mut tile_query.iter() {
            if viewshed
                .visible_positions
                .contains(&((*tile_coordinates).into()))
            {
                sprite.index = match is_highlighted.0 {
                    false => 0,
                    true => 3,
                }
            } else {
                sprite.index = match is_highlighted.0 {
                    false => 2,
                    true => 5,
                }
            }
        }
    }
}
