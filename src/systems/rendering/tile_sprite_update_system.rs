use crate::components::*;
use bevy::prelude::*;

type QueryWithTile<'a, T> = Query<'a, With<Tile, T>>;

pub fn tile_sprite_update_system(
    mut tile_query: QueryWithTile<(
        &GameCoordinates,
        &IsHighlighted,
        &mut TextureAtlasSprite,
        &mut Transform,
    )>,
    mut player_query: Query<With<Player, &Viewshed>>,
) {
    let mut player_viewshed_query_borrow = player_query.iter();
    let viewshed = player_viewshed_query_borrow.iter().next().unwrap();

    for (tile_coordinates, is_highlighted, mut sprite, transform) in &mut tile_query.iter() {
        if viewshed
            .visible_positions
            .contains(&tile_coordinates.position)
        {
            match is_highlighted.0 {
                false => {
                    sprite.index = 0;
                    transform.translation().set_z(0.0)
                }
                true => {
                    sprite.index = 3;
                    transform.translation().set_z(0.0)
                }
            }
        } else {
            match is_highlighted.0 {
                false => {
                    sprite.index = 2;
                    transform.translation().set_z(2.0)
                }
                true => {
                    sprite.index = 5;
                    transform.translation().set_z(2.0)
                }
            }
        }
    }
}
