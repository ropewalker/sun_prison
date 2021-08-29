use super::*;
use std::collections::HashMap;

type EntityWithMutCoordinates<'a> = (Entity, &'a mut GameCoordinates);
type MovableNonPlayer = (With<Movable>, Without<Player>);
type DefinitelyImmovable = (With<Immovable>, Without<Movable>);
type Portal = (With<Exit>, Without<Movable>, Without<Immovable>);

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    (mut game_state, mut turn_queue): (ResMut<GameState>, ResMut<TurnQueue>),
    mut player_position_query: Query<&mut GameCoordinates, (With<Player>, With<Movable>)>,
    mut movables_query: Query<EntityWithMutCoordinates, MovableNonPlayer>,
    immovables_query: Query<(Entity, &GameCoordinates), DefinitelyImmovable>,
    exit_query: Query<&GameCoordinates, Portal>,
    mut label_query: Query<(&mut Text, &Label)>,
) {
    if !keyboard_input.is_changed() {
        return;
    }

    if *game_state == GameState::PlayerTurn {
        let mut player_coordinates = player_position_query.iter_mut().next().unwrap();

        let mut direction = None;
        let mut to_move: HashMap<u32, UnitVector> = HashMap::new();

        let mut queue_element = (*turn_queue).0.peek_mut().unwrap();

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
            (*queue_element).priority += TURN_COST;
            *game_state = GameState::PassingTurn;
        } else if keyboard_input.just_pressed(KeyCode::Right)
            || keyboard_input.just_pressed(KeyCode::D)
        {
            player_coordinates.tangent = Some(
                player_coordinates
                    .tangent
                    .unwrap()
                    .cross(&player_coordinates.position.normal),
            );
            (*queue_element).priority += TURN_COST;
            *game_state = GameState::PassingTurn;
        } else if keyboard_input.just_pressed(KeyCode::E) {
            *game_state = GameState::PassingTurn;
            (*queue_element).priority += WAIT_COST;
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
                    (*queue_element).priority += PUSH_COST;
                    break;
                } else {
                    strafe(&mut player_coordinates, direction);
                    (*queue_element).priority += MOVE_COST;
                    break;
                }
            }

            for (entity, mut coordinates) in movables_query.iter_mut() {
                if let Some(direction) = to_move.remove(&entity.id()) {
                    strafe(&mut coordinates, direction);
                }
            }

            let portal_coordinates = exit_query.iter().next().unwrap();

            if player_coordinates.position == portal_coordinates.position {
                *game_state = GameState::Victory;
                for (mut text, label) in label_query.iter_mut() {
                    if label.label_type == LabelType::GameEvents {
                        (*text).sections[0].value = "You won!".to_string();
                    }
                }
            } else {
                *game_state = GameState::PassingTurn;
            }
        }
    }
}
