use crate::algebra::*;
use crate::components::*;
use bevy::prelude::*;

pub fn player_sprite_update_system(
    _player: &Player,
    player_coordinates: Changed<GameCoordinates>,
    mut sprite: Mut<TextureAtlasSprite>,
) {
    const UP: u32 = 3;
    const DOWN: u32 = 0;
    const RIGHT: u32 = 2;
    const LEFT: u32 = 1;

    use UnitVector::*;

    match player_coordinates.position.normal {
        Right => match player_coordinates.tangent {
            Some(Up) => sprite.index = RIGHT,
            Some(Front) => sprite.index = UP,
            Some(Down) => sprite.index = LEFT,
            Some(Back) => sprite.index = DOWN,
            _ => {}
        },
        Up => match player_coordinates.tangent {
            Some(Right) => sprite.index = UP,
            Some(Front) => sprite.index = RIGHT,
            Some(Left) => sprite.index = DOWN,
            Some(Back) => sprite.index = LEFT,
            _ => {}
        },
        Front => match player_coordinates.tangent {
            Some(Up) => sprite.index = UP,
            Some(Right) => sprite.index = RIGHT,
            Some(Down) => sprite.index = DOWN,
            Some(Left) => sprite.index = LEFT,
            _ => {}
        },
        Left => match player_coordinates.tangent {
            Some(Up) => sprite.index = UP,
            Some(Front) => sprite.index = RIGHT,
            Some(Down) => sprite.index = DOWN,
            Some(Back) => sprite.index = LEFT,
            _ => {}
        },
        Down => match player_coordinates.tangent {
            Some(Right) => sprite.index = RIGHT,
            Some(Front) => sprite.index = UP,
            Some(Left) => sprite.index = LEFT,
            Some(Back) => sprite.index = DOWN,
            _ => {}
        },
        Back => match player_coordinates.tangent {
            Some(Up) => sprite.index = RIGHT,
            Some(Right) => sprite.index = UP,
            Some(Down) => sprite.index = LEFT,
            Some(Left) => sprite.index = DOWN,
            _ => {}
        },
    }
}
