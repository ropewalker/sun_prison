use crate::components::*;
use bevy::prelude::*;

pub fn create_portal(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
    portal_coordinates: GameCoordinates,
) {
    let texture_handle = asset_server
        .load_sync(textures, "assets/images/portal.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 1, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.1));

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas,
            transform,
            ..Default::default()
        })
        .with(Portal)
        .with(portal_coordinates);
}
