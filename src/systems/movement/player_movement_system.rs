use super::*;
use std::collections::HashMap;

type QueryWithoutPlayer<'a, T> = Query<'a, Without<Player, T>>;

pub fn player_movement_system(
    keyboard_input: ChangedRes<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut player_position_query: Query<With<Player, &mut GameCoordinates>>,
    mut movables_query: QueryWithoutPlayer<With<Movable, (Entity, &mut GameCoordinates)>>,
    immovables_query: Query<With<Immovable, (Entity, &GameCoordinates)>>,
    portal_query: Query<With<Exit, &GameCoordinates>>,
) {
    if *game_state == GameState::PlayerTurn {
        let mut player_coordinates = player_position_query.iter_mut().next().unwrap();

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
                    .cross(&player_coordinates.position.normal),
            );
        } else if keyboard_input.just_pressed(KeyCode::Right)
            || keyboard_input.just_pressed(KeyCode::D)
        {
            player_coordinates.tangent = Some(
                player_coordinates
                    .tangent
                    .unwrap()
                    .cross(&player_coordinates.position.normal),
            );
        } else if keyboard_input.just_pressed(KeyCode::E) {
            *game_state = GameState::EnemyTurn;
        }

        if let Some(direction) = direction {
            let mov = movables_query
                .iter_mut()
                .map(|t| (t.1.position, t.0.id()))
                .collect::<HashMap<_, _>>();
            let immov = immovables_query
                .iter()
                .map(|t| (t.1.position, t.0.id()))
                .collect::<HashMap<_, _>>();

            let (mut new_position, mut new_direction) = (player_coordinates.position, direction);

            loop {
                let tile = next_tile(&new_position, new_direction);

                new_position = tile.position;
                new_direction = tile.tangent.unwrap();

                if let Some(id) = mov.get(&new_position) {
                    to_move.insert(*id, new_direction);
                } else if immov.contains_key(&new_position) {
                    to_move.clear();
                    break;
                } else {
                    strafe(&mut player_coordinates, direction);
                    break;
                }
            }

            for (entity, mut coordinates) in movables_query.iter_mut() {
                if let Some(direction) = to_move.remove(&entity.id()) {
                    strafe(&mut coordinates, direction);
                }
            }

            let portal_coordinates = portal_query.iter().next().unwrap();

            if player_coordinates.position == portal_coordinates.position {
                *game_state = GameState::Victory;
                println!("You won!");
            } else {
                *game_state = GameState::EnemyTurn;
            }
        }
    }
}
