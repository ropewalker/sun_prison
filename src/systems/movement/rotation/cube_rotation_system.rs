use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn cube_rotation_system(
    keyboard_input: ChangedRes<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    player_query: Query<With<Player, &RotationInfo>>,
    mut coordinates_query: Query<Without<Highlight, &mut GameCoordinates>>,
) {
    if *game_state == GameState::PlayerTurn && keyboard_input.just_pressed(KeyCode::Space) {
        let RotationInfo { axis, layer } = player_query.iter().next().unwrap();

        for mut coordinates in coordinates_query.iter_mut() {
            use UnitVector::*;

            let cubelet = coordinates.position.cubelet;

            if (*axis == Right || *axis == Left) && *layer == cubelet.x
                || (*axis == Up || *axis == Down) && *layer == cubelet.y
                || (*axis == Front || *axis == Back) && *layer == cubelet.z
            {
                *coordinates = coordinates.rotate(&axis);
            }
        }

        *game_state = GameState::EnemyTurn;
    }
}
