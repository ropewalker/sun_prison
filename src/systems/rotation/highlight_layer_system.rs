use crate::algebra::*;
use crate::components::*;
use bevy::prelude::*;

pub fn highlight_layer_system(
    keyboard_input: ChangedRes<Input<KeyCode>>,
    mut player_query: Query<With<Player, &mut GameCoordinates>>,
    mut highlights_query: Query<With<Highlight, (&GameCoordinates, &mut TextureAtlasSprite)>>,
) {
    let mut player_query_borrow = player_query.iter();
    let player_coordinates = player_query_borrow.iter().next().unwrap();

    let RotationInfo { axis, layer } = super::calculate_rotation_info(&player_coordinates);

    for (coordinates, mut sprite) in &mut highlights_query.iter() {
        use UnitVector::*;

        let cubelet = coordinates.position.cubelet;

        let current_layer = match axis {
            Right | Left => cubelet.x,
            Up | Down => cubelet.y,
            Front | Back => cubelet.z,
        };

        sprite.color.a = if current_layer == layer && keyboard_input.pressed(KeyCode::Tab) {
            1.0
        } else {
            0.0
        };
    }
}
