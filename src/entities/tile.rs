use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

//TODO: refactor coordinates
#[allow(clippy::too_many_arguments)]
pub fn create_tile(
    commands: &mut Commands,
    tile_size: &Res<TileSize>,
    cubelet_position: &CubeletPosition,
    normal_orientation: &NormalOrientation,
    tangent_orientation: &TangentOrientation,
    insolation: &Insolation,
    is_highlighted: &IsHighlighted,
    texture_atlas: Handle<TextureAtlas>,
) {
    let translation =
        game_coordinates_to_translation(&tile_size, &cubelet_position, &normal_orientation, 0.0);

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            translation,
            sprite: TextureAtlasSprite {
                index: insolation_to_sprite_index(insolation, is_highlighted),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(*cubelet_position)
        .with(*normal_orientation)
        .with(*tangent_orientation)
        .with(Tile)
        .with(*insolation)
        .with(*is_highlighted);
}
