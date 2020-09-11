use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_walls(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
) {
    let texture_handle = asset_server
        .load_sync(textures, "assets/images/wall.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 1, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let cubelet_position = Vector3 {
        x: PLANET_RADIUS,
        y: 1,
        z: 1,
    };
    let normal_orientation = Vector3 { x: 1, y: 0, z: 0 };
    let tangent_orientation = Vector3 { x: 0, y: 0, z: 1 };

    let mut translation: Translation = Default::default();
    translation.0.set_z(1.0);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            translation,
            ..Default::default()
        })
        .with(Wall)
        .with(GameCoordinates {
            cubelet_position,
            normal_orientation,
            tangent_orientation,
        })
        .with(Immovable);
}

pub fn create_movable_walls(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
) {
    let texture_handle = asset_server
        .load_sync(textures, "assets/images/box.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 1, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let cubelet_position = Vector3 {
        x: PLANET_RADIUS,
        y: -1,
        z: -1,
    };
    let normal_orientation = Vector3 { x: 1, y: 0, z: 0 };
    let tangent_orientation = Vector3 { x: 0, y: 0, z: 1 };

    let mut translation: Translation = Default::default();
    translation.0.set_z(1.0);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            translation,
            ..Default::default()
        })
        .with(Wall)
        .with(GameCoordinates {
            cubelet_position,
            normal_orientation,
            tangent_orientation,
        })
        .with(Movable);
}
