use crate::components::*;
use bevy::prelude::*;

pub fn translation_system(
    mut translation: Mut<Translation>,
    cubelet_position: &CubeletPosition,
    normal_orientation: &NormalOrientation,
) {
    *translation =
        game_coordinates_to_translation(cubelet_position, normal_orientation, translation.0.z());
}
