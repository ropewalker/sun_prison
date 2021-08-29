use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn create_enemies(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    turn_queue: &mut ResMut<TurnQueue>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    enemies_coordinates: Vec<GameCoordinates>,
    kind: Enemy,
) {
    let texture_handle = asset_server.get_handle(match kind {
        Enemy::Zombie => "images/zombie_spritesheet.png",
        Enemy::Ghoul => "images/ghoul_spritesheet.png",
        Enemy::Demon => "images/demon_spritesheet.png",
    });
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(TILE_SIZE, TILE_SIZE), 4, 1);
    let texture_atlas = texture_atlases.add(texture_atlas);

    let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 1.0));

    for (number, &enemy_coordinates) in enemies_coordinates.iter().enumerate() {
        let enemy_entity = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                transform,
                ..Default::default()
            })
            .insert(kind)
            .insert(enemy_coordinates)
            .insert(Movable)
            .insert(Obstacle)
            .insert(Viewshed {
                visible_positions: HashSet::new(),
                shape: match kind {
                    Enemy::Zombie => ViewshedShape::Quadrant,
                    Enemy::Ghoul => ViewshedShape::Circle,
                    Enemy::Demon => ViewshedShape::All,
                },
            })
            .insert(LastPlayerPosition(None))
            .insert(RememberedObstacles(HashSet::new()))
            .id();

        (*turn_queue).0.push(TurnQueueElement {
            entity: enemy_entity,
            priority: 100 * (number as u32),
        });
    }
}
