use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

fn make_move(coordinates: &mut Mut<GameCoordinates>, direction: Vector3) {
    let new_cubelet_position = &coordinates.cubelet_position + &direction;

    if new_cubelet_position.x.abs() > PLANET_RADIUS
        || new_cubelet_position.y.abs() > PLANET_RADIUS
        || new_cubelet_position.z.abs() > PLANET_RADIUS
    {
        coordinates.tangent_orientation = match coordinates.tangent_orientation.dot(&direction) {
            x if x > 0 => -coordinates.normal_orientation,
            x if x < 0 => coordinates.normal_orientation,
            _ => coordinates.tangent_orientation,
        };

        coordinates.normal_orientation = direction;
    } else {
        coordinates.cubelet_position = new_cubelet_position;
    }
}

fn next_tile(coordinates: &GameCoordinates, direction: Vector3) -> (GameCoordinates, Vector3) {
    let new_cubelet_position = &coordinates.cubelet_position + &direction;

    if new_cubelet_position.x.abs() > PLANET_RADIUS
        || new_cubelet_position.y.abs() > PLANET_RADIUS
        || new_cubelet_position.z.abs() > PLANET_RADIUS
    {
        (
            GameCoordinates {
                cubelet_position: coordinates.cubelet_position,
                normal_orientation: direction,
                tangent_orientation: (0, 0, 0).into(),
            },
            -coordinates.normal_orientation,
        )
    } else {
        (
            GameCoordinates {
                cubelet_position: new_cubelet_position,
                normal_orientation: coordinates.normal_orientation,
                tangent_orientation: (0, 0, 0).into(),
            },
            direction,
        )
    }
}

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_turn: ResMut<CurrentTurn>,
    mut player_position_query: Query<(&Player, &mut GameCoordinates)>,
    mut movables_query: Query<Without<Player, (Entity, &Movable, &mut GameCoordinates)>>,
    mut immovables_query: Query<(Entity, &Immovable, &mut GameCoordinates)>,
) {
    if current_turn.side == GameSide::Player {
        for (_player, mut player_coordinates) in &mut player_position_query.iter() {
            let mut direction = None;
            let mut to_move: HashMap<u32, Vector3> = HashMap::new();

            if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
                direction = Some(player_coordinates.tangent_orientation);
            } else if keyboard_input.just_pressed(KeyCode::Down)
                || keyboard_input.just_pressed(KeyCode::S)
            {
                direction = Some(-player_coordinates.tangent_orientation);
            } else if keyboard_input.just_pressed(KeyCode::Left)
                || keyboard_input.just_pressed(KeyCode::A)
            {
                player_coordinates.tangent_orientation = -player_coordinates
                    .tangent_orientation
                    .cross(&player_coordinates.normal_orientation);
            } else if keyboard_input.just_pressed(KeyCode::Right)
                || keyboard_input.just_pressed(KeyCode::D)
            {
                player_coordinates.tangent_orientation = player_coordinates
                    .tangent_orientation
                    .cross(&player_coordinates.normal_orientation);
            }

            if let Some(direction) = direction {
                let mov: HashMap<Vector3, u32> = movables_query
                    .iter()
                    .iter()
                    .map(|t| {
                        (
                            &(t.2).cubelet_position + &(t.2).normal_orientation,
                            t.0.id(),
                        )
                    })
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<Vector3, u32> = immovables_query
                    .iter()
                    .iter()
                    .map(|t| {
                        (
                            &(t.2).cubelet_position + &(t.2).normal_orientation,
                            t.0.id(),
                        )
                    })
                    .collect::<HashMap<_, _>>();

                let (mut new_coordinates, mut new_direction) = (*player_coordinates, direction);

                loop {
                    let tile = next_tile(&new_coordinates, new_direction);

                    new_coordinates = tile.0;
                    new_direction = tile.1;

                    let coordinate_vec =
                        &new_coordinates.cubelet_position + &new_coordinates.normal_orientation;

                    if let Some(id) = mov.get(&coordinate_vec) {
                        to_move.insert(*id, new_direction);
                    } else if immov.contains_key(&coordinate_vec) {
                        to_move.clear();
                        break;
                    } else {
                        make_move(&mut player_coordinates, direction);
                        break;
                    }
                }

                for (entity, _movable, mut coordinates) in &mut movables_query.iter() {
                    if let Some(direction) = to_move.remove(&entity.id()) {
                        make_move(&mut coordinates, direction);
                    }
                }

                current_turn.side = GameSide::Sun;
            }
        }
    }
}
