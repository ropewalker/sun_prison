use crate::algebra::*;
use crate::components::*;
use crate::entities::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    commands.spawn(Camera2dComponents::default());

    //player
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/player_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 4, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let cubelet_position = Vector3 {
        x: PLANET_RADIUS,
        y: 0,
        z: 0,
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
        .with(Player)
        .with(GameCoordinates {
            cubelet_position,
            normal_orientation,
            tangent_orientation,
        })
        .with(Movable);

    //wall
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/wall.png")
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

    //object
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/box.png")
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
        .with(Object)
        .with(GameCoordinates {
            cubelet_position,
            normal_orientation,
            tangent_orientation,
        })
        .with(Movable);

    //tiles
    create_tiles(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
    );
}
