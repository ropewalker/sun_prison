use crate::algebra::*;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct GameCoordinates {
    pub cubelet: Vector3,
    pub normal: UnitVector,
    pub tangent: Option<UnitVector>,
}

impl GameCoordinates {
    pub fn rotate(self, axis: &UnitVector) -> GameCoordinates {
        GameCoordinates {
            cubelet: self.cubelet.rotate(axis),
            normal: self.normal.rotate(axis),
            tangent: if let Some(tangent) = self.tangent {
                Some(tangent.rotate(axis))
            } else {
                None
            },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Position {
    pub cubelet: Vector3,
    pub normal: UnitVector,
}

impl From<GameCoordinates> for Position {
    fn from(game_coordinates: GameCoordinates) -> Position {
        Position {
            cubelet: game_coordinates.cubelet,
            normal: game_coordinates.normal,
        }
    }
}

impl From<Position> for GameCoordinates {
    fn from(position: Position) -> GameCoordinates {
        GameCoordinates {
            cubelet: position.cubelet,
            normal: position.normal,
            tangent: None,
        }
    }
}
