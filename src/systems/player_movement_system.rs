use crate::components::*;
use crate::events::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut rotate_layer_events: ResMut<Events<RotateLayerEvent>>,
    mut current_turn: ResMut<CurrentTurn>,
    _player: &Player,
    mut cubelet_position: Mut<CubeletPosition>,
    mut normal_orientation: Mut<NormalOrientation>,
    mut tangent_orientation: Mut<TangentOrientation>,
) {
    if *current_turn == CurrentTurn(GameSide::Player) {
        if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
            let new_cubelet_position = cubelet_position.0 + tangent_orientation.0;

            if new_cubelet_position.x.abs() > 2
                || new_cubelet_position.y.abs() > 2
                || new_cubelet_position.z.abs() > 2
            {
                let new_tangent_orientation = -normal_orientation.0;
                normal_orientation.0 = tangent_orientation.0;
                tangent_orientation.0 = new_tangent_orientation;
            } else {
                cubelet_position.0 = new_cubelet_position;
            }

            *current_turn = CurrentTurn(GameSide::Sun);
        } else if keyboard_input.just_pressed(KeyCode::Down)
            || keyboard_input.just_pressed(KeyCode::S)
        {
            let new_cubelet_position = cubelet_position.0 - tangent_orientation.0;

            if new_cubelet_position.x.abs() > 2
                || new_cubelet_position.y.abs() > 2
                || new_cubelet_position.z.abs() > 2
            {
                let new_tangent_orientation = normal_orientation.0;
                normal_orientation.0 = -tangent_orientation.0;
                tangent_orientation.0 = new_tangent_orientation;
            } else {
                cubelet_position.0 = new_cubelet_position;
            }

            *current_turn = CurrentTurn(GameSide::Sun);
        } else if keyboard_input.just_pressed(KeyCode::Left)
            || keyboard_input.just_pressed(KeyCode::A)
        {
            tangent_orientation.0 = -tangent_orientation.0.cross(normal_orientation.0);
        } else if keyboard_input.just_pressed(KeyCode::Right)
            || keyboard_input.just_pressed(KeyCode::D)
        {
            tangent_orientation.0 = tangent_orientation.0.cross(normal_orientation.0);
        } else if keyboard_input.just_pressed(KeyCode::Space) {
            let rotation_info = calculate_rotation_info(
                &cubelet_position,
                &normal_orientation,
                &tangent_orientation,
            );

            rotate_layer_events.send(RotateLayerEvent(rotation_info));
            *current_turn = CurrentTurn(GameSide::Sun);
        }
    }
}
