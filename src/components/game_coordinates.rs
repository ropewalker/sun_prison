use crate::algebra::*;
use crate::resources::PLANET_RADIUS;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Ord, PartialOrd)]
pub struct GameCoordinates {
    pub position: Position,
    pub tangent: Option<UnitVector>,
}

impl GameCoordinates {
    pub fn rotate(self, axis: &UnitVector) -> GameCoordinates {
        GameCoordinates {
            position: Position {
                cubelet: self.position.cubelet.rotate(axis),
                normal: self.position.normal.rotate(axis),
            },
            tangent: if let Some(tangent) = self.tangent {
                Some(tangent.rotate(axis))
            } else {
                None
            },
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Ord, PartialOrd)]
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

impl From<Position> for GameCoordinates {
    fn from(position: Position) -> GameCoordinates {
        GameCoordinates {
            position,
            tangent: None,
        }
    }
}

pub struct RotationInfo {
    pub axis: UnitVector,
    pub layer: isize,
}

pub fn calculate_rotation_info(coordinates: &GameCoordinates) -> RotationInfo {
    let tangent = coordinates.tangent.unwrap();
    let axis = coordinates.position.normal.cross(&tangent);

    use UnitVector::*;

    let cubelet = coordinates.position.cubelet;

    let layer = match axis {
        Right | Left => cubelet.x,
        Up | Down => cubelet.y,
        Front | Back => cubelet.z,
    };

    RotationInfo { axis, layer }
}
