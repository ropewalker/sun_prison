use crate::components::*;
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
        .with_bundle(game_coordinates)
        .with(Tile)
        .with(insolation)
        .with(is_highlighted);
}
