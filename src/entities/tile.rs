use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn create_tile(
    commands: &mut Commands,
    tile_size: &Res<TileSize>,
    game_coordinates: GameCoordinates,
    insolation: Insolation,
    is_highlighted: IsHighlighted,
    texture_atlas: Handle<TextureAtlas>,
) {
    let translation = game_coordinates_to_translation(
        &tile_size,
        &game_coordinates.cubelet_position,
        &game_coordinates.normal_orientation,
        0.0,
    );

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            translation,
            sprite: TextureAtlasSprite {
                index: insolation_to_sprite_index(&insolation, &is_highlighted),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_bundle(game_coordinates)
        .with(Tile)
        .with(insolation)
        .with(is_highlighted);
}
