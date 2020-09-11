use crate::algebra::*;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct GameCoordinates {
    pub cubelet_position: Vector3,
    pub normal_orientation: Vector3,
    pub tangent_orientation: Vector3,
}

impl GameCoordinates {
    pub fn rotate(self, axis: &Vector3) -> GameCoordinates {
        GameCoordinates {
            cubelet_position: self.cubelet_position.rotate(axis),
            normal_orientation: self.normal_orientation.rotate(axis),
            tangent_orientation: self.tangent_orientation.rotate(axis),
        }
    }
}
