use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn create_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    turn_queue: &mut ResMut<TurnQueue>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    player_coordinates: GameCoordinates,
) {
    let texture_handle = asset_server.get_handle("images/player_spritesheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(TILE_SIZE, TILE_SIZE), 4, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 1.0));

    let player_entity = commands
        .spawn(SpriteSheetBundle {
            texture_atlas,
            transform,
            ..Default::default()
        })
        .with(Player)
        .with(calculate_rotation_info(&player_coordinates))
        .with(player_coordinates)
        .with(Movable)
        .with(Obstacle)
        .with(Viewshed {
            visible_positions: HashSet::new(),
            shape: ViewshedShape::Circle,
        })
        .with(Health(3))
        .current_entity()
        .unwrap();

    (*turn_queue).0.push(TurnQueueElement {
        entity: player_entity,
        priority: 0,
    });
}
