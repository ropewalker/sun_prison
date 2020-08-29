use crate::algebra::*;
use crate::resources::*;
use bevy::prelude::*;
use std::cmp::Ordering;

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

pub fn game_coordinates_to_translation(
    cubelet_position: &CubeletPosition,
    normal_orientation: &NormalOrientation,
    z: f32,
) -> Translation {
    let edge_len = (PLANET_RADIUS * 2 + 1) as f32;

    let columns = [
        -edge_len * 1.5,
        -edge_len / 2.0,
        edge_len / 2.0,
        edge_len * 1.5,
    ];
    let rows = [edge_len, 0.0, -edge_len];

    let (x, y) = match (
        normal_orientation.0.x,
        normal_orientation.0.y,
        normal_orientation.0.z,
    ) {
        //right
        (1, 0, 0) => (
            (cubelet_position.0.y as f32 + columns[3]) * TILE_SIZE,
            (cubelet_position.0.z as f32 + rows[2]) * TILE_SIZE,
        ),
        //up
        (0, 1, 0) => (
            (cubelet_position.0.z as f32 + columns[1]) * TILE_SIZE,
            (cubelet_position.0.x as f32 + rows[0]) * TILE_SIZE,
        ),
        //front
        (0, 0, 1) => (
            (cubelet_position.0.x as f32 + columns[2]) * TILE_SIZE,
            (cubelet_position.0.y as f32 + rows[1]) * TILE_SIZE,
        ),
        //left
        (-1, 0, 0) => (
            (cubelet_position.0.z as f32 + columns[1]) * TILE_SIZE,
            (cubelet_position.0.y as f32 + rows[1]) * TILE_SIZE,
        ),
        //bottom
        (0, -1, 0) => (
            (cubelet_position.0.x as f32 + columns[2]) * TILE_SIZE,
            (cubelet_position.0.z as f32 + rows[2]) * TILE_SIZE,
        ),
        //back
        (0, 0, -1) => (
            (cubelet_position.0.y as f32 + columns[0]) * TILE_SIZE,
            (cubelet_position.0.x as f32 + rows[0]) * TILE_SIZE,
        ),
        _ => panic!("wrong orientation!"),
    };

    Translation(Vec3::new(x, y, z))
}

enum PlaneAxis {
    X,
    Y,
    NegX,
    NegY,
}

fn coordinates_to_axis(
    position_x: isize,
    position_y: isize,
    orientation_x: isize,
    orientation_y: isize,
) -> PlaneAxis {
    use Ordering::*;

    match (position_x.cmp(&position_y), position_x.cmp(&-position_y)) {
        (Greater, Less) => PlaneAxis::X,    //x
        (Less, Greater) => PlaneAxis::NegX, //-x
        (Greater, Greater) => PlaneAxis::Y, //y
        (Less, Less) => PlaneAxis::NegY,    //-y
        _ => {
            let x = position_x + orientation_x;
            let y = position_y + orientation_y;

            match (x.cmp(&y), x.cmp(&-y)) {
                (Greater, Less) => PlaneAxis::X,    //x
                (Less, Greater) => PlaneAxis::NegX, //-x
                (Greater, Greater) => PlaneAxis::Y, //y
                (Less, Less) => PlaneAxis::NegY,
                _ => panic!("impossible position and/or orientation!"),
            }
        }
    }
}

pub fn calculate_rotation_info(
    cubelet_position: &CubeletPosition,
    normal_orientation: &NormalOrientation,
    tangent_orientation: &TangentOrientation,
) -> RotationInfo {
    let (x_axis, y_axis) = match (
        normal_orientation.0.x,
        normal_orientation.0.y,
        normal_orientation.0.z,
    ) {
        //right
        (1, 0, 0) => (Vector3::new(0, 1, 0), Vector3::new(0, 0, 1)),
        //up
        (0, 1, 0) => (Vector3::new(0, 0, 1), Vector3::new(1, 0, 0)),
        //front
        (0, 0, 1) => (Vector3::new(1, 0, 0), Vector3::new(0, 1, 0)),
        //left
        (-1, 0, 0) => (Vector3::new(0, 0, 1), Vector3::new(0, 1, 0)),
        //bottom
        (0, -1, 0) => (Vector3::new(1, 0, 0), Vector3::new(0, 0, 1)),
        //back
        (0, 0, -1) => (Vector3::new(0, 1, 0), Vector3::new(1, 0, 0)),
        _ => panic!("wrong orientation!"),
    };

    let neg_x_axis = -x_axis;
    let neg_y_axis = -y_axis;

    let (x, y, tx, ty) = (
        cubelet_position.0.dot(x_axis),
        cubelet_position.0.dot(y_axis),
        tangent_orientation.0.dot(x_axis),
        tangent_orientation.0.dot(y_axis),
    );

    let plane_axis = coordinates_to_axis(x, y, tx, ty);
    use PlaneAxis::*;

    let (axis, layer) = match plane_axis {
        X => (x_axis, x),
        NegX => (neg_x_axis, x),
        Y => (y_axis, y),
        NegY => (neg_y_axis, y),
    };

    RotationInfo { axis, layer }
}
