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
    use UnitVector::*;

    let texture_handle = asset_server
        .load_sync(textures, "assets/images/tile_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 3, 2);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let tangent_orientation = None;
    let is_highlighted = IsHighlighted(false);

    for &normal_orientation in &[Right, Up, Front, Left, Down, Back] {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            for y in -PLANET_RADIUS..=PLANET_RADIUS {
                use Insolation::*;

                let insolation = match normal_orientation {
                    Right => Day,
                    Down | Back => Twilight,
                    _ => Night,
                };

                let (abscissa, ordinate) = normal_orientation.abscissa_and_ordinate();

                let cubelet_position =
                    PLANET_RADIUS * normal_orientation + x * abscissa + y * ordinate;

                let game_coordinates = GameCoordinates {
                    cubelet_position,
                    normal_orientation,
                    tangent_orientation,
                };

                create_tile(
                    commands,
                    game_coordinates,
                    insolation,
                    is_highlighted,
                    texture_atlas,
                );
            }
        }
    }
}
