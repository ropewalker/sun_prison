use crate::algebra::*;
use crate::components::*;
use bevy::prelude::*;

pub fn highlight_layer_system(
    keyboard_input: ChangedRes<Input<KeyCode>>,
    mut player_query: Query<With<Player, &mut GameCoordinates>>,
    mut tiles_query: Query<With<Tile, (&GameCoordinates, &mut IsHighlighted)>>,
) {
    let mut player_query_borrow = player_query.iter();
    let player_coordinates = player_query_borrow.iter().next().unwrap();

    let RotationInfo { axis, layer } = super::calculate_rotation_info(&player_coordinates);

    for (coordinates, mut is_highlighted) in &mut tiles_query.iter() {
        use UnitVector::*;

        let cubelet = coordinates.position.cubelet;

        let current_layer = match axis {
            Right | Left => cubelet.x,
            Up | Down => cubelet.y,
            Front | Back => cubelet.z,
        };

        *is_highlighted =
            IsHighlighted(current_layer == layer && keyboard_input.pressed(KeyCode::Tab));
    }
}
