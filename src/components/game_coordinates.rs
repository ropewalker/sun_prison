use crate::algebra::*;
use crate::resources::PLANET_RADIUS;

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

    pub fn position(&self) -> Position {
        (*self).into()
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct Position {
    pub cubelet: Vector3,
    pub normal: UnitVector,
}

impl Position {
    pub fn on_surface(&self) -> bool {
        self.cubelet.x.abs() <= PLANET_RADIUS
            && self.cubelet.y.abs() <= PLANET_RADIUS
            && self.cubelet.z.abs() <= PLANET_RADIUS
            && self.cubelet.dot(&self.normal.to_vector()) == PLANET_RADIUS
    }

    #[allow(dead_code)]
    pub fn manhattan_distance_to(&self, other: &Position) -> isize {
        (self.cubelet - other.cubelet).manhattan_length()
            - self.normal.to_vector().dot(&other.normal.to_vector())
            + 1
    }
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
