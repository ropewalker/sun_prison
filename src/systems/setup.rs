use crate::entities::*;
use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    commands.spawn(Camera2dComponents::default());

    //player
    create_player(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
    );

    //walls
    create_walls(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
    );

    //movable_walls
    create_movable_walls(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
    );

    //tiles
    create_tiles(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
    );
}
