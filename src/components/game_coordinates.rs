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
        if self.normal == other.normal {
            (self.cubelet - other.cubelet).manhattan_length()
        } else if self.normal == -other.normal {
            let (abscissa, ordinate) = self.normal.abscissa_and_ordinate();
            let (self_x, self_y) = (
                self.cubelet.dot(&abscissa.to_vector()),
                self.cubelet.dot(&ordinate.to_vector()),
            );
            let (other_x, other_y) = (
                other.cubelet.dot(&abscissa.to_vector()),
                other.cubelet.dot(&ordinate.to_vector()),
            );

            2 * PLANET_RADIUS
                + 2
                + [
                    (self_x - PLANET_RADIUS).abs()
                        + (other_x - PLANET_RADIUS).abs()
                        + (self_y - other_y).abs(),
                    (self_x + PLANET_RADIUS).abs()
                        + (other_x + PLANET_RADIUS).abs()
                        + (self_y - other_y).abs(),
                    (self_y - PLANET_RADIUS).abs()
                        + (other_y - PLANET_RADIUS).abs()
                        + (self_x - other_x).abs(),
                    (self_y + PLANET_RADIUS).abs()
                        + (other_y + PLANET_RADIUS).abs()
                        + (self_x - other_x).abs(),
                ]
                .iter()
                .min()
                .unwrap()
        } else {
            (self.cubelet - other.cubelet).manhattan_length() + 1
        }
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
