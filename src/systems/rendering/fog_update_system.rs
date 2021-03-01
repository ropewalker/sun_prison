use crate::components::*;
use bevy::prelude::*;

pub fn fog_update_system(
    mut fog_query: Query<(&GameCoordinates, &mut TextureAtlasSprite), With<Fog>>,
    player_query: Query<&Viewshed, (Changed<Viewshed>, With<Player>)>,
) {
    let viewshed = player_query.iter().next().unwrap();

    for (fog_coordinates, mut sprite) in fog_query.iter_mut() {
        if viewshed
            .visible_positions
            .contains(&fog_coordinates.position)
        {
            sprite.color.set_a(0.0);
        } else {
            sprite.color.set_a(1.0);
        }
    }
}
