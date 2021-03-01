use crate::components::*;
use crate::resources::TILE_SIZE;
use bevy::prelude::*;

pub fn create_portal(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    portal_coordinates: GameCoordinates,
) {
    let texture_handle = asset_server.get_handle("images/portal.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(TILE_SIZE, TILE_SIZE), 1, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.1));

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        })
        .with(Exit)
        .with(portal_coordinates);
}
