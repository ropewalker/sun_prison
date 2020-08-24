use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn translation_system(
    tile_size: Res<TileSize>,
    mut translation: Mut<Translation>,
    cubelet_position: &CubeletPosition,
    normal_orientation: &NormalOrientation,
) {
    *translation = game_coordinates_to_translation(
        &tile_size,
        &cubelet_position,
        &normal_orientation,
        translation.0.z(),
    );
}
