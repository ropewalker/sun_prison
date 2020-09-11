use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn translation_system(mut translation: Mut<Translation>, coordinates: &GameCoordinates) {
    *translation = game_coordinates_to_translation(coordinates, translation.0.z());
}

fn game_coordinates_to_translation(coordinates: &GameCoordinates, z: f32) -> Translation {
    let edge_len = (PLANET_RADIUS * 2 + 1) as f32;

    let columns = [
        -edge_len * 1.5 - 1.5,
        -edge_len / 2.0 - 0.5,
        edge_len / 2.0 + 0.5,
        edge_len * 1.5 + 1.5,
    ];
    let rows = [edge_len + 1.0, 0.0, -edge_len - 1.0];

    let (x, y) = match (
        coordinates.normal_orientation.x,
        coordinates.normal_orientation.y,
        coordinates.normal_orientation.z,
    ) {
        //right
        (1, 0, 0) => (
            (coordinates.cubelet_position.y as f32 + columns[3]) * TILE_SIZE,
            (coordinates.cubelet_position.z as f32 + rows[2]) * TILE_SIZE,
        ),
        //up
        (0, 1, 0) => (
            (coordinates.cubelet_position.z as f32 + columns[1]) * TILE_SIZE,
            (coordinates.cubelet_position.x as f32 + rows[0]) * TILE_SIZE,
        ),
        //front
        (0, 0, 1) => (
            (coordinates.cubelet_position.x as f32 + columns[2]) * TILE_SIZE,
            (coordinates.cubelet_position.y as f32 + rows[1]) * TILE_SIZE,
        ),
        //left
        (-1, 0, 0) => (
            (coordinates.cubelet_position.z as f32 + columns[1]) * TILE_SIZE,
            (coordinates.cubelet_position.y as f32 + rows[1]) * TILE_SIZE,
        ),
        //bottom
        (0, -1, 0) => (
            (coordinates.cubelet_position.x as f32 + columns[2]) * TILE_SIZE,
            (coordinates.cubelet_position.z as f32 + rows[2]) * TILE_SIZE,
        ),
        //back
        (0, 0, -1) => (
            (coordinates.cubelet_position.y as f32 + columns[0]) * TILE_SIZE,
            (coordinates.cubelet_position.x as f32 + rows[0]) * TILE_SIZE,
        ),
        _ => panic!("wrong orientation!"),
    };

    Translation(Vec3::new(x, y, z))
}
