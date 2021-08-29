use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_tile(
    commands: &mut Commands,
    game_coordinates: GameCoordinates,
    texture_atlas: Handle<TextureAtlas>,
) {
    let transform = Transform::identity();

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        })
        .insert(game_coordinates)
        .insert(Tile);
}

pub fn create_highlight(
    commands: &mut Commands,
    game_coordinates: GameCoordinates,
    texture_atlas: Handle<TextureAtlas>,
) {
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 2.0));
    let mut sprite_sheet_components = SpriteSheetBundle {
        texture_atlas,
        transform,
        ..Default::default()
    };
    sprite_sheet_components.sprite.color.set_a(0.0);

    commands
        .spawn_bundle(sprite_sheet_components)
        .insert(game_coordinates)
        .insert(Highlight);
}

pub fn create_fog(
    commands: &mut Commands,
    game_coordinates: GameCoordinates,
    texture_atlas: Handle<TextureAtlas>,
) {
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 1.9));
    let sprite_sheet_components = SpriteSheetBundle {
        texture_atlas,
        transform,
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite_sheet_components)
        .insert(game_coordinates)
        .insert(Fog);
}

pub fn create_planet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    use UnitVector::*;

    let tile_texture_handle = asset_server.get_handle("images/tile.png");
    let tile_texture_atlas =
        TextureAtlas::from_grid(tile_texture_handle, Vec2::new(TILE_SIZE, TILE_SIZE), 1, 1);
    let tile_texture_atlas = texture_atlases.add(tile_texture_atlas);

    let highlight_texture_handle = asset_server.get_handle("images/highlight.png");
    let highlight_texture_atlas = TextureAtlas::from_grid(
        highlight_texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        1,
        1,
    );
    let highlight_texture_atlas = texture_atlases.add(highlight_texture_atlas);

    let fog_texture_handle = asset_server.get_handle("images/fog.png");
    let fog_texture_atlas =
        TextureAtlas::from_grid(fog_texture_handle, Vec2::new(TILE_SIZE, TILE_SIZE), 1, 1);
    let fog_texture_atlas = texture_atlases.add(fog_texture_atlas);

    for &normal in &[Right, Up, Front, Left, Down, Back] {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            for y in -PLANET_RADIUS..=PLANET_RADIUS {
                let (abscissa, ordinate) = normal.abscissa_and_ordinate();
                let cubelet = PLANET_RADIUS * normal + x * abscissa + y * ordinate;
                let game_coordinates = Position { cubelet, normal }.into();

                create_tile(commands, game_coordinates, tile_texture_atlas.clone());
                create_highlight(commands, game_coordinates, highlight_texture_atlas.clone());
                create_fog(commands, game_coordinates, fog_texture_atlas.clone());
            }
        }
    }
}
