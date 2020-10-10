use crate::components::*;
use bevy::prelude::*;

type QueryWithTile<'a, T> = Query<'a, With<Tile, T>>;

pub fn tile_sprite_update_system(
    mut tile_query: QueryWithTile<(&GameCoordinates, &mut TextureAtlasSprite, &mut Transform)>,
    mut player_query: Query<With<Player, &Viewshed>>,
) {
    let mut player_viewshed_query_borrow = player_query.iter();
    let viewshed = player_viewshed_query_borrow.iter().next().unwrap();

    for (tile_coordinates, mut sprite, transform) in &mut tile_query.iter() {
        if viewshed
            .visible_positions
            .contains(&tile_coordinates.position)
        {
            sprite.index = 0;
            transform.translation().set_z(0.0)
        } else {
            sprite.index = 2;
            transform.translation().set_z(2.0)
        }
    }
}
