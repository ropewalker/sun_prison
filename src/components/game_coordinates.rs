use crate::algebra::*;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct GameCoordinates {
    pub cubelet_position: Vector3,
    pub normal_orientation: UnitVector,
    pub tangent_orientation: Option<UnitVector>,
}

impl GameCoordinates {
    pub fn rotate(self, axis: &UnitVector) -> GameCoordinates {
        GameCoordinates {
            cubelet_position: self.cubelet_position.rotate(axis),
            normal_orientation: self.normal_orientation.rotate(axis),
            tangent_orientation: if let Some(tangent) = self.tangent_orientation {
                Some(tangent.rotate(axis))
            } else {
                None
            },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Position {
    pub cubelet_position: Vector3,
    pub normal_orientation: UnitVector,
}

impl From<GameCoordinates> for Position {
    fn from(game_coordinates: GameCoordinates) -> Position {
        Position {
            cubelet_position: game_coordinates.cubelet_position,
            normal_orientation: game_coordinates.normal_orientation,
        }
    }
}

impl From<Position> for GameCoordinates {
    fn from(position: Position) -> GameCoordinates {
        GameCoordinates {
            cubelet_position: position.cubelet_position,
            normal_orientation: position.normal_orientation,
            tangent_orientation: None,
        }
    }
}
