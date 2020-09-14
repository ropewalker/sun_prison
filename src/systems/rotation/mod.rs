mod cube_rotation_system;
mod highlight_layer_system;

pub use self::cube_rotation_system::*;
pub use self::highlight_layer_system::*;
use crate::algebra::*;
use crate::components::*;

pub fn calculate_rotation_info(coordinates: &GameCoordinates) -> RotationInfo {
    let tangent = coordinates.tangent_orientation.unwrap();
    let axis = coordinates.normal_orientation.cross(&tangent);

    use UnitVector::*;

    let layer = match axis {
        Right | Left => coordinates.cubelet_position.x,
        Up | Down => coordinates.cubelet_position.y,
        Front | Back => coordinates.cubelet_position.z,
    };

    RotationInfo { axis, layer }
}
