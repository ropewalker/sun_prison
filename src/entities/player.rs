use crate::components::*;
use bevy::prelude::*;

pub fn create_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
    players_coordinates: Vec<GameCoordinates>,
) {
    let texture_handle = asset_server
        .load_sync(textures, "assets/images/player_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 4, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let mut translation: Translation = Default::default();
    translation.0.set_z(1.0);

    for player_coordinates in players_coordinates {
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas,
                translation,
                ..Default::default()
            })
            .with(Player)
            .with(player_coordinates)
            .with(Movable);
    }
}
