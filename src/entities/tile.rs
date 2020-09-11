use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_tile(
    commands: &mut Commands,
    game_coordinates: GameCoordinates,
    insolation: Insolation,
    is_highlighted: IsHighlighted,
    texture_atlas: Handle<TextureAtlas>,
) {
    let mut translation: Translation = Default::default();
    translation.0.set_z(0.0);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            translation,
            ..Default::default()
        })
        .with(game_coordinates)
        .with(Tile)
        .with(insolation)
        .with(is_highlighted);
}

pub fn create_tiles(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
) {
    //common
    let texture_handle = asset_server
        .load_sync(textures, "assets/images/tile_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 2);
    let texture_atlas = texture_atlases.add(texture_atlas);

    //right
    let normal_orientation = Vector3 { x: 1, y: 0, z: 0 };
    let tangent_orientation = Vector3 { x: 0, y: 1, z: 0 };

    let insolation = Insolation::Day;

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x: PLANET_RADIUS,
                y,
                z,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                commands,
                game_coordinates,
                insolation,
                IsHighlighted(false),
                texture_atlas,
            );
        }
    }

    //up
    let normal_orientation = Vector3 { x: 0, y: 1, z: 0 };
    let tangent_orientation = Vector3 { x: 0, y: 0, z: 1 };

    let insolation = Insolation::Night;

    for x in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x,
                y: PLANET_RADIUS,
                z,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                commands,
                game_coordinates,
                insolation,
                IsHighlighted(false),
                texture_atlas,
            );
        }
    }

    //front
    let normal_orientation = Vector3 { x: 0, y: 0, z: 1 };
    let tangent_orientation = Vector3 { x: 1, y: 0, z: 0 };

    let insolation = Insolation::Night;

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x,
                y,
                z: PLANET_RADIUS,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                commands,
                game_coordinates,
                insolation,
                IsHighlighted(false),
                texture_atlas,
            );
        }
    }

    //left
    let normal_orientation = Vector3 { x: -1, y: 0, z: 0 };
    let tangent_orientation = Vector3 { x: 0, y: -1, z: 0 };

    let insolation = Insolation::Night;

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x: -PLANET_RADIUS,
                y,
                z,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                commands,
                game_coordinates,
                insolation,
                IsHighlighted(false),
                texture_atlas,
            );
        }
    }

    //down
    let normal_orientation = Vector3 { x: 0, y: -1, z: 0 };
    let tangent_orientation = Vector3 { x: 0, y: 0, z: -1 };

    let insolation = Insolation::Twilight;

    for x in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x,
                y: -PLANET_RADIUS,
                z,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                commands,
                game_coordinates,
                insolation,
                IsHighlighted(false),
                texture_atlas,
            );
        }
    }

    //back
    let normal_orientation = Vector3 { x: 0, y: 0, z: -1 };
    let tangent_orientation = Vector3 { x: -1, y: 0, z: 0 };

    let insolation = Insolation::Twilight;

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x,
                y,
                z: -PLANET_RADIUS,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            create_tile(
                commands,
                game_coordinates,
                insolation,
                IsHighlighted(false),
                texture_atlas,
            );
        }
    }
}
