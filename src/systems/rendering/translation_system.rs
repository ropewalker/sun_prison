use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn translation_system(
    mut translation: Mut<Translation>,
    cubelet_position: &CubeletPosition,
    normal_orientation: &NormalOrientation,
) {
    *translation =
        game_coordinates_to_translation(cubelet_position, normal_orientation, translation.0.z());
}

fn game_coordinates_to_translation(
    cubelet_position: &CubeletPosition,
    normal_orientation: &NormalOrientation,
    z: f32,
) -> Translation {
    let edge_len = (PLANET_RADIUS * 2 + 1) as f32;

    let columns = [
        -edge_len * 1.5 - 1.5,
        -edge_len / 2.0 - 0.5,
        edge_len / 2.0 + 0.5,
        edge_len * 1.5 + 1.5,
    ];
    let rows = [edge_len + 1.0, 0.0, -edge_len - 1.0];

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
