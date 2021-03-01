use crate::components::*;
use bevy::prelude::*;

pub fn health_display_system(
    player_query: Query<&Health, (Changed<Health>, With<Player>)>,
    mut label_query: Query<(&mut Text, &Label)>,
) {
    if let Some(player_health) = player_query.iter().next() {
        for (mut text, label) in label_query.iter_mut() {
            if label.label_type == LabelType::Health {
                if player_health.0 > 0 {
                    (*text).value = format!("You have {} HP.", player_health.0);
                } else {
                    (*text).value = "You died.".to_string();
                }
            }
        }
    }
}
