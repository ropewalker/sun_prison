use crate::algebra::*;
use crate::components::*;
use bevy::prelude::*;

pub fn enemy_sprite_update_system(
    _enemy: &Enemy,
    coordinates: &GameCoordinates,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    const UP: u32 = 3;
    const DOWN: u32 = 0;
    const RIGHT: u32 = 2;
    const LEFT: u32 = 1;

    use UnitVector::*;

    match coordinates.position.normal {
        Right => match coordinates.tangent {
            Some(Up) => sprite.index = RIGHT,
            Some(Front) => sprite.index = UP,
            Some(Down) => sprite.index = LEFT,
            Some(Back) => sprite.index = DOWN,
            _ => {}
        },
        Up => match coordinates.tangent {
            Some(Right) => sprite.index = UP,
            Some(Front) => sprite.index = RIGHT,
            Some(Left) => sprite.index = DOWN,
            Some(Back) => sprite.index = LEFT,
            _ => {}
        },
        Front => match coordinates.tangent {
            Some(Up) => sprite.index = UP,
            Some(Right) => sprite.index = RIGHT,
            Some(Down) => sprite.index = DOWN,
            Some(Left) => sprite.index = LEFT,
            _ => {}
        },
        Left => match coordinates.tangent {
            Some(Up) => sprite.index = UP,
            Some(Front) => sprite.index = RIGHT,
            Some(Down) => sprite.index = DOWN,
            Some(Back) => sprite.index = LEFT,
            _ => {}
        },
        Down => match coordinates.tangent {
            Some(Right) => sprite.index = RIGHT,
            Some(Front) => sprite.index = UP,
            Some(Left) => sprite.index = LEFT,
            Some(Back) => sprite.index = DOWN,
            _ => {}
        },
        Back => match coordinates.tangent {
            Some(Up) => sprite.index = RIGHT,
            Some(Right) => sprite.index = UP,
            Some(Down) => sprite.index = LEFT,
            Some(Left) => sprite.index = DOWN,
            _ => {}
        },
    }
}
