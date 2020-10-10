use crate::algebra::*;
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_tile(
    commands: &mut Commands,
    game_coordinates: GameCoordinates,
    insolation: Insolation,
    texture_atlas: Handle<TextureAtlas>,
) {
    let transform = Transform::identity();

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            transform,
            ..Default::default()
        })
        .with(game_coordinates)
        .with(Tile)
        .with(insolation);
}

pub fn create_highlight(
    commands: &mut Commands,
    game_coordinates: GameCoordinates,
    texture_atlas: Handle<TextureAtlas>,
) {
    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 2.0));
    let mut sprite_sheet_components = SpriteSheetComponents {
        texture_atlas,
        transform,
        ..Default::default()
    };
    sprite_sheet_components.sprite.color.a = 0.0;

    commands
        .spawn(sprite_sheet_components)
        .with(game_coordinates)
        .with(Highlight);
}

pub fn create_tiles_and_highlights(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
) {
    use UnitVector::*;

    let tile_texture_handle = asset_server
        .load_sync(textures, "assets/images/tile_spritesheet.png")
        .unwrap();
    let tile_texture = textures.get(&tile_texture_handle).unwrap();
    let tile_texture_atlas = TextureAtlas::from_grid(tile_texture_handle, tile_texture.size, 3, 2);
    let tile_texture_atlas = texture_atlases.add(tile_texture_atlas);

    let highlight_texture_handle = asset_server
        .load_sync(textures, "assets/images/highlight.png")
        .unwrap();
    let highlight_texture = textures.get(&highlight_texture_handle).unwrap();
    let highlight_texture_atlas =
        TextureAtlas::from_grid(highlight_texture_handle, highlight_texture.size, 1, 1);
    let highlight_texture_atlas = texture_atlases.add(highlight_texture_atlas);

    for &normal in &[Right, Up, Front, Left, Down, Back] {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            for y in -PLANET_RADIUS..=PLANET_RADIUS {
                use Insolation::*;

                let insolation = match normal {
                    Right => Day,
                    Down | Back => Twilight,
                    _ => Night,
                };

                let (abscissa, ordinate) = normal.abscissa_and_ordinate();
                let cubelet = PLANET_RADIUS * normal + x * abscissa + y * ordinate;
                let game_coordinates = Position { cubelet, normal }.into();

                create_tile(commands, game_coordinates, insolation, tile_texture_atlas);
                create_highlight(commands, game_coordinates, highlight_texture_atlas);
            }
        }
    }
}
