mod enemies_movement_system;
mod player_movement_system;
mod rotation;
mod viewshed_update_system;

use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub use self::{
    enemies_movement_system::*, player_movement_system::*, rotation::*, viewshed_update_system::*,
};

pub fn strafe(coordinates: &mut GameCoordinates, direction: UnitVector) {
    let new_coordinates = next_tile(&coordinates.position, direction);

    if let Some(tangent) = coordinates.tangent {
        if tangent == direction {
            *coordinates = new_coordinates;
        } else if tangent == -direction {
            *coordinates = GameCoordinates {
                position: new_coordinates.position,
                tangent: Some(-new_coordinates.tangent.unwrap()),
            }
        } else {
            *coordinates = GameCoordinates {
                position: new_coordinates.position,
                tangent: Some(tangent),
            }
        }
    } else {
        *coordinates = GameCoordinates {
            position: new_coordinates.position,
            tangent: None,
        }
    }
}

pub fn turn_and_move(coordinates: &mut GameCoordinates, direction: UnitVector) {
    let new_coordinates = next_tile(&coordinates.position, direction);

    if coordinates.tangent.is_some() {
        *coordinates = new_coordinates
    } else {
        *coordinates = GameCoordinates {
            position: new_coordinates.position,
            tangent: None,
        }
    }
}

pub fn next_tile(position: &Position, direction: UnitVector) -> GameCoordinates {
    let new_cubelet = position.cubelet + direction;

    if new_cubelet.x.abs() > PLANET_RADIUS
        || new_cubelet.y.abs() > PLANET_RADIUS
        || new_cubelet.z.abs() > PLANET_RADIUS
    {
        GameCoordinates {
            position: Position {
                cubelet: position.cubelet,
                normal: direction,
            },
            tangent: Some(-position.normal),
        }
    } else {
        GameCoordinates {
            position: Position {
                cubelet: new_cubelet,
                normal: position.normal,
            },
            tangent: Some(direction),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub struct PathNode {
    pub coordinates: GameCoordinates,
    pub came_from: UnitVector,
}

pub fn neighbours(position: &Position) -> impl Iterator<Item = PathNode> {
    let (abscissa, ordinate) = position.normal.abscissa_and_ordinate();

    vec![
        PathNode {
            coordinates: next_tile(position, abscissa),
            came_from: abscissa,
        },
        PathNode {
            coordinates: next_tile(position, ordinate),
            came_from: ordinate,
        },
        PathNode {
            coordinates: next_tile(position, -abscissa),
            came_from: -abscissa,
        },
        PathNode {
            coordinates: next_tile(position, -ordinate),
            came_from: -ordinate,
        },
    ]
    .into_iter()
}
