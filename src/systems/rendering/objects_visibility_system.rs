use crate::components::*;
use bevy::prelude::*;

type QueryWithoutTileAndHighlight<'a, T> = Query<'a, Without<Tile, Without<Highlight, T>>>;

pub fn objects_visibility_system(
    mut objects_query: QueryWithoutTileAndHighlight<(&GameCoordinates, &mut TextureAtlasSprite)>,
    mut player_query: Query<With<Player, &Viewshed>>,
) {
    let mut player_viewshed_query_borrow = player_query.iter();
    let viewshed = player_viewshed_query_borrow.iter().next().unwrap();

    for (object_coordinates, mut sprite) in &mut objects_query.iter() {
        if viewshed
            .visible_positions
            .contains(&object_coordinates.position)
        {
            sprite.color.a = 1.0;
        } else {
            sprite.color.a = 0.0;
        }
    }
}
