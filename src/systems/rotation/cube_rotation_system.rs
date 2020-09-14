use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn cube_rotation_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut current_turn: ResMut<CurrentTurn>,
    mut player_query: Query<(&Player, Mut<GameCoordinates>)>,
    mut coordinates_query: Query<Without<Player, Mut<GameCoordinates>>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (_, mut coordinates) in &mut player_query.iter() {
            let RotationInfo { axis, layer } = super::calculate_rotation_info(&coordinates);
            *coordinates = coordinates.rotate(&axis);

            for mut coordinates in &mut coordinates_query.iter() {
                let mut rotate = false;

                use UnitVector::*;

                match axis {
                    Right | Left => {
                        if coordinates.cubelet_position.x == layer {
                            rotate = true;
                        }
                    }
                    Up | Down => {
                        if coordinates.cubelet_position.y == layer {
                            rotate = true;
                        }
                    }
                    Front | Back => {
                        if coordinates.cubelet_position.z == layer {
                            rotate = true;
                        }
                    }
                }

                if rotate {
                    *coordinates = coordinates.rotate(&axis);
                }
            }
        }

        current_turn.side = GameSide::Sun;
    }
}
