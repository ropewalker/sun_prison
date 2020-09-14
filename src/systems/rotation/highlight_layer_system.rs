use crate::algebra::*;
use crate::components::*;
use bevy::prelude::*;

pub fn highlight_layer_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Player, &GameCoordinates)>,
    mut tiles_query: Query<(&Tile, &GameCoordinates, Mut<IsHighlighted>)>,
) {
    for (_, player_coordinates) in &mut player_query.iter() {
        let RotationInfo { axis, layer } = super::calculate_rotation_info(&player_coordinates);

        for (_, coordinates, mut is_highlighted) in &mut tiles_query.iter() {
            use UnitVector::*;

            let current_layer = match axis {
                Right | Left => coordinates.cubelet_position.x,
                Up | Down => coordinates.cubelet_position.y,
                Front | Back => coordinates.cubelet_position.z,
            };

            *is_highlighted =
                IsHighlighted(current_layer == layer && keyboard_input.pressed(KeyCode::Tab));
        }
    }
}
