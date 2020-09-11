use crate::algebra::*;
use crate::components::*;
use bevy::prelude::*;

pub fn player_sprite_update_system(
    _player: &Player,
    player_coordinates: &GameCoordinates,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    const UP: u32 = 3;
    const DOWN: u32 = 0;
    const RIGHT: u32 = 2;
    const LEFT: u32 = 1;

    match player_coordinates.normal_orientation {
        Vector3 { x: 1, y: 0, z: 0 } => match player_coordinates.tangent_orientation {
            Vector3 { x: 0, y: 1, z: 0 } => sprite.index = RIGHT,
            Vector3 { x: 0, y: 0, z: 1 } => sprite.index = UP,
            Vector3 { x: 0, y: -1, z: 0 } => sprite.index = LEFT,
            Vector3 { x: 0, y: 0, z: -1 } => sprite.index = DOWN,
            _ => {}
        },
        Vector3 { x: 0, y: 1, z: 0 } => match player_coordinates.tangent_orientation {
            Vector3 { x: 1, y: 0, z: 0 } => sprite.index = UP,
            Vector3 { x: 0, y: 0, z: 1 } => sprite.index = RIGHT,
            Vector3 { x: -1, y: 0, z: 0 } => sprite.index = DOWN,
            Vector3 { x: 0, y: 0, z: -1 } => sprite.index = LEFT,
            _ => {}
        },
        Vector3 { x: 0, y: 0, z: 1 } => match player_coordinates.tangent_orientation {
            Vector3 { x: 0, y: 1, z: 0 } => sprite.index = UP,
            Vector3 { x: 1, y: 0, z: 0 } => sprite.index = RIGHT,
            Vector3 { x: 0, y: -1, z: 0 } => sprite.index = DOWN,
            Vector3 { x: -1, y: 0, z: 0 } => sprite.index = LEFT,
            _ => {}
        },
        Vector3 { x: -1, y: 0, z: 0 } => match player_coordinates.tangent_orientation {
            Vector3 { x: 0, y: 1, z: 0 } => sprite.index = UP,
            Vector3 { x: 0, y: 0, z: 1 } => sprite.index = RIGHT,
            Vector3 { x: 0, y: -1, z: 0 } => sprite.index = DOWN,
            Vector3 { x: 0, y: 0, z: -1 } => sprite.index = LEFT,
            _ => {}
        },
        Vector3 { x: 0, y: -1, z: 0 } => match player_coordinates.tangent_orientation {
            Vector3 { x: 1, y: 0, z: 0 } => sprite.index = RIGHT,
            Vector3 { x: 0, y: 0, z: 1 } => sprite.index = UP,
            Vector3 { x: -1, y: 0, z: 0 } => sprite.index = LEFT,
            Vector3 { x: 0, y: 0, z: -1 } => sprite.index = DOWN,
            _ => {}
        },
        Vector3 { x: 0, y: 0, z: -1 } => match player_coordinates.tangent_orientation {
            Vector3 { x: 0, y: 1, z: 0 } => sprite.index = RIGHT,
            Vector3 { x: 1, y: 0, z: 0 } => sprite.index = UP,
            Vector3 { x: 0, y: -1, z: 0 } => sprite.index = LEFT,
            Vector3 { x: -1, y: 0, z: 0 } => sprite.index = DOWN,
            _ => {}
        },
        _ => {}
    }
}
