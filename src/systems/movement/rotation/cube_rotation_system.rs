use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn cube_rotation_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut turn_queue: ResMut<TurnQueue>,
    player_query: Query<&RotationInfo, With<Player>>,
    mut coordinates_query: Query<&mut GameCoordinates, Without<Highlight>>,
) {
    if !keyboard_input.is_changed() {
        return;
    }

    if *game_state == GameState::PlayerTurn && keyboard_input.just_pressed(KeyCode::Space) {
        let RotationInfo { axis, layer } = player_query.iter().next().unwrap();

        for mut coordinates in coordinates_query.iter_mut() {
            use UnitVector::*;

            let cubelet = coordinates.position.cubelet;

            if (*axis == Right || *axis == Left) && *layer == cubelet.x
                || (*axis == Up || *axis == Down) && *layer == cubelet.y
                || (*axis == Front || *axis == Back) && *layer == cubelet.z
            {
                *coordinates = coordinates.rotate(axis);
            }
        }

        let mut queue_element = (*turn_queue).0.peek_mut().unwrap();
        (*queue_element).priority += ROTATE_COST;

        *game_state = GameState::PassingTurn;
    }
}
