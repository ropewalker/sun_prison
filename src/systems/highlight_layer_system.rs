use crate::algebra::*;
use crate::components::*;
use bevy::prelude::*;

pub fn highlight_layer_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        &Player,
        &CubeletPosition,
        &NormalOrientation,
        &TangentOrientation,
    )>,
    mut tiles_query: Query<(&Tile, &CubeletPosition, Mut<IsHighlighted>)>,
) {
    for (_, player_cubelet_position, normal_orientation, tangent_orientation) in
        &mut player_query.iter()
    {
        let RotationInfo { axis, layer } = calculate_rotation_info(
            &player_cubelet_position,
            &normal_orientation,
            &tangent_orientation,
        );

        for (_, cubelet_position, mut is_highlighted) in &mut tiles_query.iter() {
            let current_layer = match (axis.x, axis.y, axis.z) {
                (1, 0, 0) | (-1, 0, 0) => cubelet_position.0.x,
                (0, 1, 0) | (0, -1, 0) => cubelet_position.0.y,
                (0, 0, 1) | (0, 0, -1) => cubelet_position.0.z,
                _ => panic!("wrong axis!"),
            };

            *is_highlighted =
                IsHighlighted(current_layer == layer && keyboard_input.pressed(KeyCode::Tab));
        }
    }
}
