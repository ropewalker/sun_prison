mod cube_rotation_system;
mod highlight_layer_system;

pub use self::cube_rotation_system::*;
pub use self::highlight_layer_system::*;
use crate::algebra::*;
use crate::components::*;

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
