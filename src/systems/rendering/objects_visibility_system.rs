use crate::components::*;
use bevy::prelude::*;

pub fn objects_visibility_system(
    mut fog_query: Query<With<Fog, (&GameCoordinates, &mut TextureAtlasSprite)>>,
    mut player_query: Query<With<Player, &Viewshed>>,
) {
    let mut player_viewshed_query_borrow = player_query.iter();
    let viewshed = player_viewshed_query_borrow.iter().next().unwrap();

    for (fog_coordinates, mut sprite) in &mut fog_query.iter() {
        if viewshed
            .visible_positions
            .contains(&fog_coordinates.position)
        {
            sprite.color.a = 0.0;
        } else {
            sprite.color.a = 1.0;
        }
    }
}
