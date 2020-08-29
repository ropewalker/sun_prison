use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

type PlayerQueryArgs<'a> = (
    &'a Player,
    Mut<'a, CubeletPosition>,
    Mut<'a, NormalOrientation>,
    Mut<'a, TangentOrientation>,
);

type NonPlayerQueryArgs<'a> = Without<
    Player,
    (
        Mut<'a, CubeletPosition>,
        Mut<'a, NormalOrientation>,
        Mut<'a, TangentOrientation>,
    ),
>;

pub fn cube_rotation_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_turn: ResMut<CurrentTurn>,
    mut player_query: Query<PlayerQueryArgs>,
    mut coordinates_query: Query<NonPlayerQueryArgs>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (_, mut cubelet_position, mut normal_orientation, mut tangent_orientation) in
            &mut player_query.iter()
        {
            let RotationInfo { axis, layer } = super::calculate_rotation_info(
                &cubelet_position,
                &normal_orientation,
                &tangent_orientation,
            );

            cubelet_position.0 = cubelet_position.0.rotate(&axis);
            normal_orientation.0 = normal_orientation.0.rotate(&axis);
            tangent_orientation.0 = tangent_orientation.0.rotate(&axis);

            for (mut cubelet_position, mut normal_orientation, mut tangent_orientation) in
                &mut coordinates_query.iter()
            {
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
        }

        current_turn.side = GameSide::Sun;
    }
}
