use crate::components::*;
use crate::resources::TILE_SIZE;
use bevy::prelude::*;

pub fn create_walls(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    walls_coordinates: Vec<GameCoordinates>,
    movable: bool,
) {
    let texture_handle = asset_server.get_handle(if movable {
        "images/box.png"
    } else {
        "images/wall.png"
    });
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(TILE_SIZE, TILE_SIZE), 1, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 1.0));

    for wall_coordinates in walls_coordinates {
        let commands = commands
            .spawn(SpriteSheetComponents {
                texture_atlas: texture_atlas.clone(),
                transform,
                ..Default::default()
            })
            .with(wall_coordinates)
            .with(Opaque)
            .with(Obstacle);

        if movable {
            commands.with(Movable);
        } else {
            commands.with(Immovable);
        }
    }
}
