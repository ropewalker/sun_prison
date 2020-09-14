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
