use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn cube_rotation_system(
    keyboard_input: ChangedRes<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut player_query: Query<With<Player, &mut GameCoordinates>>,
    mut coordinates_query: Query<Without<Player, &mut GameCoordinates>>,
) {
    if *game_state == GameState::PlayerTurn && keyboard_input.just_pressed(KeyCode::Space) {
        let mut player_query_borrow = player_query.iter();
        let mut coordinates = player_query_borrow.iter().next().unwrap();

        let RotationInfo { axis, layer } = super::calculate_rotation_info(&coordinates);
        *coordinates = coordinates.rotate(&axis);

        for mut coordinates in &mut coordinates_query.iter() {
            use UnitVector::*;

            let cubelet = coordinates.position.cubelet;

            if (axis == Right || axis == Left) && layer == cubelet.x
                || (axis == Up || axis == Down) && layer == cubelet.y
                || (axis == Front || axis == Back) && layer == cubelet.z
            {
                *coordinates = coordinates.rotate(&axis);
            }
        }

        *game_state = GameState::EnemyTurn;
    }
}
