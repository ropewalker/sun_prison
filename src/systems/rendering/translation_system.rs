use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn translation_system(mut transform: Mut<Transform>, coordinates: Changed<GameCoordinates>) {
    *transform = game_coordinates_to_translation(*coordinates, transform.translation().z());
}

fn game_coordinates_to_translation(coordinates: GameCoordinates, z: f32) -> Transform {
    let edge_len = (PLANET_RADIUS * 2 + 1) as f32;

    let columns = [
        -edge_len * 1.5 - 1.5,
        -edge_len / 2.0 - 0.5,
        edge_len / 2.0 + 0.5,
        edge_len * 1.5 + 1.5,
    ];
    let rows = [edge_len + 1.0, 0.0, -edge_len - 1.0];

    use UnitVector::*;

    let normal = coordinates.position.normal;
    let cubelet = coordinates.position.cubelet;

    let (x, y) = match normal {
        Right => (
            (cubelet.y as f32 + columns[3]) * TILE_SIZE,
            (cubelet.z as f32 + rows[2]) * TILE_SIZE,
        ),
        Up => (
            (cubelet.z as f32 + columns[1]) * TILE_SIZE,
            (cubelet.x as f32 + rows[0]) * TILE_SIZE,
        ),
        Front => (
            (cubelet.x as f32 + columns[2]) * TILE_SIZE,
            (cubelet.y as f32 + rows[1]) * TILE_SIZE,
        ),
        Left => (
            (cubelet.z as f32 + columns[1]) * TILE_SIZE,
            (cubelet.y as f32 + rows[1]) * TILE_SIZE,
        ),
        Down => (
            (cubelet.x as f32 + columns[2]) * TILE_SIZE,
            (cubelet.z as f32 + rows[2]) * TILE_SIZE,
        ),
        Back => (
            (cubelet.y as f32 + columns[0]) * TILE_SIZE,
            (cubelet.x as f32 + rows[0]) * TILE_SIZE,
        ),
    };

    Transform::from_translation(Vec3::new(x, y, z))
}
