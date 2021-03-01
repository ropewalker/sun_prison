use super::*;

pub fn turn_system(
    mut game_state: ResMut<GameState>,
    turn_queue: ResMut<TurnQueue>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    if *game_state == GameState::PassingTurn {
        if let Some(TurnQueueElement {
            entity,
            priority: _,
        }) = (*turn_queue).0.peek()
        {
            if player_query.get(*entity).is_ok() {
                *game_state = GameState::PlayerTurn;
            } else if let Ok(enemy) = enemy_query.get(*entity) {
                *game_state = GameState::EnemyTurn(enemy);
            }
        }
    }
}
