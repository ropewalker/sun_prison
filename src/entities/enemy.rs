use crate::components::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn create_enemies(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    textures: &mut ResMut<Assets<Texture>>,
    enemies_coordinates: Vec<GameCoordinates>,
) {
    let texture_handle = asset_server
        .load_sync(textures, "assets/images/zombie_spritesheet.png")
        .unwrap();
    let texture = textures.get(&texture_handle).unwrap();
    let texture_atlas = TextureAtlas::from_grid(texture_handle, texture.size, 4, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 1.0));

    for enemy_coordinates in enemies_coordinates {
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas,
                transform,
                ..Default::default()
            })
            .with(Enemy)
            .with(enemy_coordinates)
            .with(Movable)
            .with(Viewshed {
                visible_positions: HashSet::new(),
                shape: ViewshedShape::Quadrant,
            })
            .with(LastPlayerPosition(None));
    }
}
