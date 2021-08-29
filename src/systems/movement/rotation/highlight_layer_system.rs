use crate::algebra::*;
use crate::components::*;
use bevy::prelude::*;

pub fn highlight_layer_system(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&RotationInfo, With<Player>>,
    mut highlights_query: Query<(&GameCoordinates, &mut TextureAtlasSprite), With<Highlight>>,
) {
    if !keyboard_input.is_changed() {
        return;
    }

    let RotationInfo { axis, layer } = player_query.iter().next().unwrap();

    for (coordinates, mut sprite) in highlights_query.iter_mut() {
        use UnitVector::*;

        let cubelet = coordinates.position.cubelet;

        let current_layer = match *axis {
            Right | Left => cubelet.x,
            Up | Down => cubelet.y,
            Front | Back => cubelet.z,
        };

        sprite.color.set_a(
            if current_layer == *layer && keyboard_input.pressed(KeyCode::Tab) {
                1.0
            } else {
                0.0
            },
        );
    }
}
