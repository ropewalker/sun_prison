use crate::components::*;
use bevy::prelude::*;

type QueryWithTile<'a, T> = Query<'a, With<Tile, T>>;

pub fn tile_sprite_update_system(
    mut tile_query: QueryWithTile<(&GameCoordinates, &mut TextureAtlasSprite, &mut Transform)>,
    player_query: Query<With<Player, &Viewshed>>,
) {
    let viewshed = player_query.iter().next().unwrap();

    for (tile_coordinates, mut sprite, mut transform) in tile_query.iter_mut() {
        if viewshed
            .visible_positions
            .contains(&tile_coordinates.position)
        {
            sprite.index = 0;
            transform.translation.set_z(0.0)
        } else {
            sprite.index = 2;
            transform.translation.set_z(2.0)
        }
    }
}
