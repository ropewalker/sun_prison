use crate::components::*;
use bevy::prelude::*;

pub fn create_walls(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
    walls_coordinates: Vec<GameCoordinates>,
    movable: bool,
) {
    let texture_handle = asset_server
        .load_sync(
            textures,
            if movable {
                "assets/images/box.png"
            } else {
                "assets/images/wall.png"
            },
        )
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 1, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let mut translation: Translation = Default::default();
    translation.0.set_z(1.0);

    for wall_coordinates in walls_coordinates {
        let commands = commands
            .spawn(SpriteSheetComponents {
                texture_atlas,
                translation,
                ..Default::default()
            })
            .with(Wall)
            .with(wall_coordinates)
            .with(Opaque);

        if movable {
            commands.with(Movable);
        } else {
            commands.with(Immovable);
        }
    }
}
