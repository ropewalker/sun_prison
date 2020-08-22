use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    tile_size: Res<TileSize>,
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

    let normal_orientation = NormalOrientation(Vector3 { x: 1, y: 0, z: 0 });
    let cubelet_position = CubeletPosition(Vector3 { x: 2, y: 0, z: 0 });
    let translation =
        game_coordinates_to_translation(&tile_size, &cubelet_position, &normal_orientation, 1.0);

    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: 0, z: 1 });

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            translation,
            ..Default::default()
        })
        .with(Player)
        .with(tangent_orientation)
        .with(cubelet_position)
        .with(normal_orientation);

    //sky
    let sky_texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/daytime_spritesheet.png")
        .unwrap();
    let sky_texture = textures.get(&sky_texture_handle).unwrap();
    let sky_texture_atlas = TextureAtlas::from_grid(sky_texture_handle, sky_texture.size, 3, 1);
    let sky_texture_atlas = texture_atlases.add(sky_texture_atlas);

    //right
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_white_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 1, y: 0, z: 0 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: 1, z: 0 });

    for y in -2..=2 {
        for z in -2..=2 {
            let cubelet_position = CubeletPosition(Vector3 { x: 2, y, z });
            let translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                0.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas,
                    translation,
                    sprite: TextureAtlasSprite::new(if cubelet_position.0.y == 0 { 1 } else { 0 }),
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(tangent_orientation)
                .with(Tile);

            let sky_translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                2.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: sky_texture_atlas,
                    translation: sky_translation,
                    sprite: TextureAtlasSprite::new(2),
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(Sky);
        }
    }

    //up
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_green_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 0, y: 1, z: 0 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: 0, z: 1 });

    for x in -2..=2 {
        for z in -2..=2 {
            let cubelet_position = CubeletPosition(Vector3 { x, y: 2, z });
            let translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                0.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas,
                    translation,
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(tangent_orientation)
                .with(Tile);

            let sky_translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                2.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: sky_texture_atlas,
                    translation: sky_translation,
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(Sky);
        }
    }

    //front
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_red_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 0, y: 0, z: 1 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 1, y: 0, z: 0 });

    for y in -2..=2 {
        for x in -2..=2 {
            let cubelet_position = CubeletPosition(Vector3 { x, y, z: 2 });
            let translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                0.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas,
                    translation,
                    sprite: TextureAtlasSprite::new(if cubelet_position.0.y == 0 { 1 } else { 0 }),
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(tangent_orientation)
                .with(Tile);

            let sky_translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                2.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: sky_texture_atlas,
                    translation: sky_translation,
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(Sky);
        }
    }

    //left
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_yellow_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: -1, y: 0, z: 0 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: -1, z: 0 });

    for y in -2..=2 {
        for z in -2..=2 {
            let cubelet_position = CubeletPosition(Vector3 { x: -2, y, z });
            let translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                0.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas,
                    translation,
                    sprite: TextureAtlasSprite::new(if cubelet_position.0.y == 0 { 1 } else { 0 }),
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(tangent_orientation)
                .with(Tile);

            let sky_translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                2.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: sky_texture_atlas,
                    translation: sky_translation,
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(Sky);
        }
    }

    //down
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_blue_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 0, y: -1, z: 0 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: 0, z: -1 });

    for x in -2..=2 {
        for z in -2..=2 {
            let cubelet_position = CubeletPosition(Vector3 { x, y: -2, z });
            let translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                0.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas,
                    translation,
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(tangent_orientation)
                .with(Tile);

            let sky_translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                2.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: sky_texture_atlas,
                    translation: sky_translation,
                    sprite: TextureAtlasSprite::new(1),
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(Sky);
        }
    }

    //back
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_orange_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 2, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 0, y: 0, z: -1 });
    let tangent_orientation = TangentOrientation(Vector3 { x: -1, y: 0, z: 0 });

    for y in -2..=2 {
        for x in -2..=2 {
            let cubelet_position = CubeletPosition(Vector3 { x, y, z: -2 });
            let translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                0.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas,
                    translation,
                    sprite: TextureAtlasSprite::new(if cubelet_position.0.y == 0 { 1 } else { 0 }),
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(tangent_orientation)
                .with(Tile);

            let sky_translation = game_coordinates_to_translation(
                &tile_size,
                &cubelet_position,
                &normal_orientation,
                2.0,
            );

            commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: sky_texture_atlas,
                    translation: sky_translation,
                    sprite: TextureAtlasSprite::new(1),
                    ..Default::default()
                })
                .with(cubelet_position)
                .with(normal_orientation)
                .with(Sky);
        }
    }
}
