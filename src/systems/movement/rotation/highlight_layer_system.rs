use crate::algebra::*;
use crate::components::*;
use bevy::prelude::*;

pub fn highlight_layer_system(
    keyboard_input: ChangedRes<Input<KeyCode>>,
    player_query: Query<With<Player, &GameCoordinates>>,
    mut highlights_query: Query<With<Highlight, (&GameCoordinates, &mut TextureAtlasSprite)>>,
) {
    let player_coordinates = player_query.iter().next().unwrap();

    let RotationInfo { axis, layer } = super::calculate_rotation_info(&player_coordinates);

    for (coordinates, mut sprite) in highlights_query.iter_mut() {
        use UnitVector::*;

        let cubelet = coordinates.position.cubelet;

        let current_layer = match axis {
            Right | Left => cubelet.x,
            Up | Down => cubelet.y,
            Front | Back => cubelet.z,
        };

        sprite.color.set_a(
            if current_layer == layer && keyboard_input.pressed(KeyCode::Tab) {
                1.0
            } else {
                0.0
            },
        );
    }
}
