mod algebra;
mod components;
mod entities;
mod resources;
mod systems;

use crate::resources::*;
use crate::systems::*;
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::window::WindowMode;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Sun Prison".to_string(),
            width: 1440,
            height: 1080,
            vsync: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgba(1.0, 1.0, 1.0, 1.0)))
        .init_resource::<SunPath>()
        .init_resource::<CurrentTurn>()
        .add_default_plugins()
        .add_startup_system(setup.system())
        .add_system(player_movement_system.system())
        .add_system(viewshed_update_system.system())
        .add_system(cube_rotation_system.system())
        .add_system(highlight_layer_system.system())
        .add_system(translation_system.system())
        .add_system(insolation_system.system())
        .add_system(sun_movement_system.system())
        .add_system(player_sprite_update_system.system())
        .add_system(tile_sprite_update_system.system())
        .add_system(objects_visibility_system.system())
        .run();
}
