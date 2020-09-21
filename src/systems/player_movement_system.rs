use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

fn make_move(coordinates: &mut Mut<GameCoordinates>, direction: UnitVector) {
    let new_cubelet = coordinates.cubelet + direction;

    if new_cubelet.x.abs() > PLANET_RADIUS
        || new_cubelet.y.abs() > PLANET_RADIUS
        || new_cubelet.z.abs() > PLANET_RADIUS
    {
        if let Some(tangent) = coordinates.tangent {
            if tangent == direction {
                coordinates.tangent = Some(-coordinates.normal)
            } else if tangent == -direction {
                coordinates.tangent = Some(coordinates.normal)
            }
        }

        coordinates.normal = direction;
    } else {
        coordinates.cubelet = new_cubelet;
    }
}

fn next_tile(position: &Position, direction: UnitVector) -> (Position, UnitVector) {
    let new_cubelet = position.cubelet + direction;

    if new_cubelet.x.abs() > PLANET_RADIUS
        || new_cubelet.y.abs() > PLANET_RADIUS
        || new_cubelet.z.abs() > PLANET_RADIUS
    {
        (
            Position {
                cubelet: position.cubelet,
                normal: direction,
            },
            -position.normal,
        )
    } else {
        (
            Position {
                cubelet: new_cubelet,
                normal: position.normal,
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
            let mut to_move: HashMap<u32, UnitVector> = HashMap::new();

            if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
                direction = player_coordinates.tangent;
            } else if keyboard_input.just_pressed(KeyCode::Down)
                || keyboard_input.just_pressed(KeyCode::S)
            {
                direction = Some(-player_coordinates.tangent.unwrap());
            } else if keyboard_input.just_pressed(KeyCode::Left)
                || keyboard_input.just_pressed(KeyCode::A)
            {
                player_coordinates.tangent = Some(
                    -player_coordinates
                        .tangent
                        .unwrap()
                        .cross(&player_coordinates.normal),
                );
            } else if keyboard_input.just_pressed(KeyCode::Right)
                || keyboard_input.just_pressed(KeyCode::D)
            {
                player_coordinates.tangent = Some(
                    player_coordinates
                        .tangent
                        .unwrap()
                        .cross(&player_coordinates.normal),
                );
            }

            if let Some(direction) = direction {
                let mov: HashMap<Position, u32> = movables_query
                    .iter()
                    .iter()
                    .map(|t| ((*t.2).into(), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<Position, u32> = immovables_query
                    .iter()
                    .iter()
                    .map(|t| ((*t.2).into(), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let (mut new_position, mut new_direction) =
                    ((*player_coordinates).into(), direction);

                loop {
                    let tile = next_tile(&new_position, new_direction);

                    new_position = tile.0;
                    new_direction = tile.1;

                    if let Some(id) = mov.get(&new_position) {
                        to_move.insert(*id, new_direction);
                    } else if immov.contains_key(&new_position) {
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
