use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

fn next_tile(current_tile: GameCoordinates, direction: Vector3) -> GameCoordinates {
    let new_cubelet_position = current_tile.cubelet_position.0 + direction;

    if new_cubelet_position.x.abs() > PLANET_RADIUS
        || new_cubelet_position.y.abs() > PLANET_RADIUS
        || new_cubelet_position.z.abs() > PLANET_RADIUS
    {
        GameCoordinates {
            cubelet_position: current_tile.cubelet_position,
            normal_orientation: NormalOrientation(direction),
            tangent_orientation: TangentOrientation(
                match current_tile.tangent_orientation.0.dot(direction) {
                    x if x > 0 => -current_tile.normal_orientation.0,
                    x if x < 0 => current_tile.normal_orientation.0,
                    _ => current_tile.tangent_orientation.0,
                },
            ),
        }
    } else {
        GameCoordinates {
            cubelet_position: CubeletPosition(new_cubelet_position),
            normal_orientation: current_tile.normal_orientation,
            tangent_orientation: current_tile.tangent_orientation,
        }
    }
}

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_turn: ResMut<CurrentTurn>,
    _player: &Player,
    mut cubelet_position: Mut<CubeletPosition>,
    mut normal_orientation: Mut<NormalOrientation>,
    mut tangent_orientation: Mut<TangentOrientation>,
) {
    if current_turn.side == GameSide::Player {
        if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
            let new_coordinates = next_tile(
                GameCoordinates {
                    cubelet_position: *cubelet_position,
                    normal_orientation: *normal_orientation,
                    tangent_orientation: *tangent_orientation,
                },
                tangent_orientation.0,
            );

            *cubelet_position = new_coordinates.cubelet_position;
            *normal_orientation = new_coordinates.normal_orientation;
            *tangent_orientation = new_coordinates.tangent_orientation;

            current_turn.side = GameSide::Sun;
        } else if keyboard_input.just_pressed(KeyCode::Down)
            || keyboard_input.just_pressed(KeyCode::S)
        {
            let new_coordinates = next_tile(
                GameCoordinates {
                    cubelet_position: *cubelet_position,
                    normal_orientation: *normal_orientation,
                    tangent_orientation: *tangent_orientation,
                },
                -tangent_orientation.0,
            );

            *cubelet_position = new_coordinates.cubelet_position;
            *normal_orientation = new_coordinates.normal_orientation;
            *tangent_orientation = new_coordinates.tangent_orientation;

            current_turn.side = GameSide::Sun;
        } else if keyboard_input.just_pressed(KeyCode::Left)
            || keyboard_input.just_pressed(KeyCode::A)
        {
            tangent_orientation.0 = -tangent_orientation.0.cross(normal_orientation.0);
        } else if keyboard_input.just_pressed(KeyCode::Right)
            || keyboard_input.just_pressed(KeyCode::D)
        {
            tangent_orientation.0 = tangent_orientation.0.cross(normal_orientation.0);
        }
    }
}
