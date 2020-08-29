use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn cube_rotation_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_turn: ResMut<CurrentTurn>,
    mut coordinates_query: Query<(
        Mut<CubeletPosition>,
        Mut<NormalOrientation>,
        Mut<TangentOrientation>,
    )>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut cubelet_position, mut normal_orientation, mut tangent_orientation) in
            &mut coordinates_query.iter()
        {
            let RotationInfo { layer, axis } = super::calculate_rotation_info(
                &cubelet_position,
                &normal_orientation,
                &tangent_orientation,
            );

            let mut rotate = false;

            match (axis.x, axis.y, axis.z) {
                (1, 0, 0) | (-1, 0, 0) => {
                    if cubelet_position.0.x == layer {
                        rotate = true;
                    }
                }
                (0, 1, 0) | (0, -1, 0) => {
                    if cubelet_position.0.y == layer {
                        rotate = true;
                    }
                }
                (0, 0, 1) | (0, 0, -1) => {
                    if cubelet_position.0.z == layer {
                        rotate = true;
                    }
                }
                _ => {}
            }

            if rotate {
                cubelet_position.0 = cubelet_position.0.rotate(&axis);
                normal_orientation.0 = normal_orientation.0.rotate(&axis);
                tangent_orientation.0 = tangent_orientation.0.rotate(&axis);
            }
        }

        current_turn.side = GameSide::Sun;
    }
}
