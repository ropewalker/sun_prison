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

    let cubelet_position = CubeletPosition(Vector3 {
        x: PLANET_RADIUS,
        y: 0,
        z: 0,
    });
    let normal_orientation = NormalOrientation(Vector3 { x: 1, y: 0, z: 0 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: 0, z: 1 });

    let translation = game_coordinates_to_translation(&cubelet_position, &normal_orientation, 1.0);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            translation,
            ..Default::default()
        })
        .with(Player)
        .with_bundle(GameCoordinates {
            cubelet_position,
            normal_orientation,
            tangent_orientation,
        });

    //right
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 2);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 1, y: 0, z: 0 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: 1, z: 0 });

    let insolation = Insolation::Day;
    let is_highlighted = IsHighlighted(false);

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = CubeletPosition(Vector3 {
                x: PLANET_RADIUS,
                y,
                z,
            });

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                &mut commands,
                game_coordinates,
                insolation,
                is_highlighted,
                texture_atlas,
            );
        }
    }

    //up
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 2);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 0, y: 1, z: 0 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: 0, z: 1 });

    let insolation = Insolation::Night;
    let is_highlighted = IsHighlighted(false);

    for x in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = CubeletPosition(Vector3 {
                x,
                y: PLANET_RADIUS,
                z,
            });

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                &mut commands,
                game_coordinates,
                insolation,
                is_highlighted,
                texture_atlas,
            );
        }
    }

    //front
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 2);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 0, y: 0, z: 1 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 1, y: 0, z: 0 });

    let insolation = Insolation::Night;
    let is_highlighted = IsHighlighted(false);

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = CubeletPosition(Vector3 {
                x,
                y,
                z: PLANET_RADIUS,
            });

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                &mut commands,
                game_coordinates,
                insolation,
                is_highlighted,
                texture_atlas,
            );
        }
    }

    //left
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 2);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: -1, y: 0, z: 0 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: -1, z: 0 });

    let insolation = Insolation::Night;
    let is_highlighted = IsHighlighted(false);

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = CubeletPosition(Vector3 {
                x: -PLANET_RADIUS,
                y,
                z,
            });

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                &mut commands,
                game_coordinates,
                insolation,
                is_highlighted,
                texture_atlas,
            );
        }
    }

    //down
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 2);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 0, y: -1, z: 0 });
    let tangent_orientation = TangentOrientation(Vector3 { x: 0, y: 0, z: -1 });

    let insolation = Insolation::Twilight;
    let is_highlighted = IsHighlighted(false);

    for x in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = CubeletPosition(Vector3 {
                x,
                y: -PLANET_RADIUS,
                z,
            });

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                &mut commands,
                game_coordinates,
                insolation,
                is_highlighted,
                texture_atlas,
            );
        }
    }

    //back
    let texture_handle = asset_server
        .load_sync(&mut textures, "assets/images/tile_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 2);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let normal_orientation = NormalOrientation(Vector3 { x: 0, y: 0, z: -1 });
    let tangent_orientation = TangentOrientation(Vector3 { x: -1, y: 0, z: 0 });

    let insolation = Insolation::Twilight;
    let is_highlighted = IsHighlighted(false);

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = CubeletPosition(Vector3 {
                x,
                y,
                z: -PLANET_RADIUS,
            });

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                &mut commands,
                game_coordinates,
                insolation,
                is_highlighted,
                texture_atlas,
            );
        }
    }
}
