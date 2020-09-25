use super::*;
use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_turn: ResMut<CurrentTurn>,
    mut player_position_query: Query<(&Player, &mut GameCoordinates)>,
    mut movables_query: Query<Without<Player, (Entity, &Movable, &mut GameCoordinates)>>,
    mut immovables_query: Query<(Entity, &Immovable, &GameCoordinates)>,
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
                let mov = movables_query
                    .iter()
                    .iter()
                    .map(|t| ((*t.2).position(), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov = immovables_query
                    .iter()
                    .iter()
                    .map(|t| ((*t.2).position(), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let (mut new_position, mut new_direction) =
                    ((*player_coordinates).position(), direction);

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

                current_turn.side = GameSide::Enemies;
            }
        }
    }
}
