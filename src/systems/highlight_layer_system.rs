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
    mut tiles_query: Query<(&Tile, &CubeletPosition, Mut<TextureAtlasSprite>)>,
) {
    for (_, player_cubelet_position, normal_orientation, tangent_orientation) in
        &mut player_query.iter()
    {
        let RotationInfo { axis, layer } = calculate_rotation_info(
            &player_cubelet_position,
            &normal_orientation,
            &tangent_orientation,
        );

        for (_, cubelet_position, mut sprite) in &mut tiles_query.iter() {
            match (axis.x, axis.y, axis.z) {
                (1, 0, 0) | (-1, 0, 0) => {
                    if cubelet_position.0.x == layer {
                        sprite.index = if keyboard_input.pressed(KeyCode::Tab) {
                            1
                        } else {
                            0
                        };
                    } else {
                        sprite.index = 0;
                    }
                }
                (0, 1, 0) | (0, -1, 0) => {
                    if cubelet_position.0.y == layer {
                        sprite.index = if keyboard_input.pressed(KeyCode::Tab) {
                            1
                        } else {
                            0
                        };
                    } else {
                        sprite.index = 0;
                    }
                }
                (0, 0, 1) | (0, 0, -1) => {
                    if cubelet_position.0.z == layer {
                        sprite.index = if keyboard_input.pressed(KeyCode::Tab) {
                            1
                        } else {
                            0
                        };
                    } else {
                        sprite.index = 0;
                    }
                }
                _ => {}
            }
        }
    }
}
