use crate::components::*;
use bevy::prelude::*;

pub fn objects_visibility_system(
    mut objects_query: Query<Without<Tile, (&GameCoordinates, &mut TextureAtlasSprite)>>,
    mut player_query: Query<(&Player, &Viewshed)>,
) {
    for (_, viewshed) in &mut player_query.iter() {
        for (object_coordinates, mut sprite) in &mut objects_query.iter() {
            if viewshed
                .visible_positions
                .contains(&((*object_coordinates).into()))
            {
                sprite.color.a = 1.0;
            } else {
                sprite.color.a = 0.0;
            }
        }
    }
}
