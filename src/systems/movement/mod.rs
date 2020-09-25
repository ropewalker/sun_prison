mod enemies_movement_system;
mod player_movement_system;
mod sun_movement_system;

use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub use self::{enemies_movement_system::*, player_movement_system::*, sun_movement_system::*};

pub fn make_move(coordinates: &mut Mut<GameCoordinates>, direction: UnitVector) {
    let new_cubelet = coordinates.cubelet + direction;

    if new_cubelet.x.abs() > PLANET_RADIUS
        || new_cubelet.y.abs() > PLANET_RADIUS
        || new_cubelet.z.abs() > PLANET_RADIUS
    {
        if let Some(tangent) = coordinates.tangent {
            if tangent == direction {
                coordinates.tangent = Some(-coordinates.normal)
            } else if tangent == -direction {
                coordinates.tangent = Some(coordinates.normal)
            }
        }

        coordinates.normal = direction;
    } else {
        coordinates.cubelet = new_cubelet;
    }
}

pub fn next_tile(position: &Position, direction: UnitVector) -> (Position, UnitVector) {
    let new_cubelet = position.cubelet + direction;

    if new_cubelet.x.abs() > PLANET_RADIUS
        || new_cubelet.y.abs() > PLANET_RADIUS
        || new_cubelet.z.abs() > PLANET_RADIUS
    {
        (
            Position {
                cubelet: position.cubelet,
                normal: direction,
            },
            -position.normal,
        )
    } else {
        (
            Position {
                cubelet: new_cubelet,
                normal: position.normal,
            },
            direction,
        )
    }
}
