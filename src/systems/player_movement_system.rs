use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;

fn make_move(
    cubelet_position: &mut Mut<CubeletPosition>,
    normal_orientation: &mut Mut<NormalOrientation>,
    tangent_orientation: &mut Mut<TangentOrientation>,
    direction: Vector3,
) {
    let new_cubelet_position = cubelet_position.0 + direction;

    if new_cubelet_position.x.abs() > PLANET_RADIUS
        || new_cubelet_position.y.abs() > PLANET_RADIUS
        || new_cubelet_position.z.abs() > PLANET_RADIUS
    {
        tangent_orientation.0 = match tangent_orientation.0.dot(direction) {
            x if x > 0 => -normal_orientation.0,
            x if x < 0 => normal_orientation.0,
            _ => tangent_orientation.0,
        };

        normal_orientation.0 = direction;
    } else {
        cubelet_position.0 = new_cubelet_position;
    }
}

fn next_tile(
    cubelet_position: &CubeletPosition,
    normal_orientation: &NormalOrientation,
    direction: Vector3,
) -> (CubeletPosition, NormalOrientation, Vector3) {
    let new_cubelet_position = cubelet_position.0 + direction;

    if new_cubelet_position.x.abs() > PLANET_RADIUS
        || new_cubelet_position.y.abs() > PLANET_RADIUS
        || new_cubelet_position.z.abs() > PLANET_RADIUS
    {
        (
            *cubelet_position,
            NormalOrientation(direction),
            -normal_orientation.0,
        )
    } else {
        (
            CubeletPosition(new_cubelet_position),
            *normal_orientation,
            direction,
        )
    }
}

#[allow(clippy::type_complexity)]
pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_turn: ResMut<CurrentTurn>,
    mut player_position_query: Query<(
        &Player,
        &mut CubeletPosition,
        &mut NormalOrientation,
        &mut TangentOrientation,
    )>,
    mut movables_query: Query<
        Without<
            Player,
            (
                Entity,
                &Movable,
                &mut CubeletPosition,
                &mut NormalOrientation,
                &mut TangentOrientation,
            ),
        >,
    >,
    mut immovables_query: Query<(
        Entity,
        &Immovable,
        &mut CubeletPosition,
        &mut NormalOrientation,
        &mut TangentOrientation,
    )>,
) {
    if current_turn.side == GameSide::Player {
        for (_player, mut cubelet_position, mut normal_orientation, mut tangent_orientation) in
            &mut player_position_query.iter()
        {
            let mut direction = None;
            let mut to_move: HashMap<u32, Vector3> = HashMap::new();

            if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
                direction = Some(tangent_orientation.0);
            } else if keyboard_input.just_pressed(KeyCode::Down)
                || keyboard_input.just_pressed(KeyCode::S)
            {
                direction = Some(-tangent_orientation.0);
            } else if keyboard_input.just_pressed(KeyCode::Left)
                || keyboard_input.just_pressed(KeyCode::A)
            {
                tangent_orientation.0 = -tangent_orientation.0.cross(normal_orientation.0);
            } else if keyboard_input.just_pressed(KeyCode::Right)
                || keyboard_input.just_pressed(KeyCode::D)
            {
                tangent_orientation.0 = tangent_orientation.0.cross(normal_orientation.0);
            }

            if let Some(direction) = direction {
                let mov: HashMap<Vector3, u32> = movables_query
                    .iter()
                    .iter()
                    .map(|t| ((t.2).0 + (t.3).0, t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<Vector3, u32> = immovables_query
                    .iter()
                    .iter()
                    .map(|t| ((t.2).0 + (t.3).0, t.0.id()))
                    .collect::<HashMap<_, _>>();

                let (mut new_cubelet_position, mut new_normal_orientation, mut new_direction) =
                    (*cubelet_position, *normal_orientation, direction);

                loop {
                    let tile = next_tile(
                        &new_cubelet_position,
                        &new_normal_orientation,
                        new_direction,
                    );

                    new_cubelet_position = tile.0;
                    new_normal_orientation = tile.1;
                    new_direction = tile.2;

                    let coordinates = new_cubelet_position.0 + new_normal_orientation.0;

                    if let Some(id) = mov.get(&coordinates) {
                        to_move.insert(*id, new_direction);
                    } else if immov.contains_key(&coordinates) {
                        to_move.clear();
                        break;
                    } else {
                        make_move(
                            &mut cubelet_position,
                            &mut normal_orientation,
                            &mut tangent_orientation,
                            direction,
                        );

                        break;
                    }
                }

                for (
                    entity,
                    _movable,
                    mut cubelet_position,
                    mut normal_orientation,
                    mut tangent_orientation,
                ) in &mut movables_query.iter()
                {
                    if let Some(direction) = to_move.remove(&entity.id()) {
                        make_move(
                            &mut cubelet_position,
                            &mut normal_orientation,
                            &mut tangent_orientation,
                            direction,
                        );
                    }
                }

                current_turn.side = GameSide::Sun;
            }
        }
    }
}
