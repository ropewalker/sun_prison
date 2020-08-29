use crate::algebra::*;
use bevy::prelude::*;

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct CubeletPosition(pub Vector3);

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct NormalOrientation(pub Vector3);

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct TangentOrientation(pub Vector3);

#[derive(Bundle)]
pub struct GameCoordinates {
    pub cubelet_position: CubeletPosition,
    pub normal_orientation: NormalOrientation,
    pub tangent_orientation: TangentOrientation,
}
