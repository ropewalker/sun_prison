use crate::components::*;
use bevy::prelude::*;

pub fn create_labels(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font_handle = asset_server.get_handle("fonts/FiraSans-Bold.ttf");
    let font_size = 30.0;

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(font_size),
                    right: Val::Px(60.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "It is dark here.".to_string(),
                TextStyle {
                    font: font_handle.clone(),
                    font_size,
                    color: Color::BLACK,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(Label {
            label_type: LabelType::GameEvents,
        });

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(font_size * 2.0),
                    right: Val::Px(60.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "You have 3 HP.".to_string(),
                TextStyle {
                    font: font_handle,
                    font_size,
                    color: Color::BLACK,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(Label {
            label_type: LabelType::Health,
        });
}
