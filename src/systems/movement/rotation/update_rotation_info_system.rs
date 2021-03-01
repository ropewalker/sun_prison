use crate::components::*;
use bevy::prelude::*;

pub fn update_rotation_info_system(
    mut rotator_query: Query<(&mut RotationInfo, &GameCoordinates), Changed<GameCoordinates>>,
) {
    for (mut rotation_info, coordinates) in rotator_query.iter_mut() {
        *rotation_info = calculate_rotation_info(coordinates);
    }
}
