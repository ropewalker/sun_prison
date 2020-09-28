use crate::components::*;
use bevy::prelude::*;

pub fn tile_sprite_update_system(
    mut tile_query: Query<With<Tile, (&GameCoordinates, &IsHighlighted, &mut TextureAtlasSprite)>>,
    mut player_query: Query<With<Player, &Viewshed>>,
) {
    let mut player_viewshed_query_borrow = player_query.iter();
    let viewshed = player_viewshed_query_borrow.iter().next().unwrap();

    for (tile_coordinates, is_highlighted, mut sprite) in &mut tile_query.iter() {
        if viewshed
            .visible_positions
            .contains(&tile_coordinates.position)
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
