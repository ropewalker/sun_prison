use crate::algebra::*;
use crate::components::{Enemy, GameCoordinates, Position};
use crate::entities::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs;

fn init_map() -> HashMap<(isize, isize), String> {
    match fs::read_to_string("assets/config/map.txt") {
        Ok(layout) => {
            let mut map = HashMap::new();

            for (y, line) in layout.trim().lines().enumerate() {
                for (x, c) in line.trim().split(' ').enumerate() {
                    map.insert((x as isize, y as isize), c.to_string());
                }
            }

            map
        }
        Err(e) => panic!("{}", e),
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut turn_queue: ResMut<TurnQueue>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 10.0));
    commands.spawn_bundle(camera);

    commands.spawn_bundle(UiCameraBundle::default());

    asset_server.load_folder("images").unwrap();
    asset_server.load_folder("fonts").unwrap();

    create_labels(&mut commands, &asset_server);

    let mut player_coordinates = None;
    let mut portal_coordinates = None;
    let mut wall_coordinates = Vec::new();
    let mut mov_wall_coordinates = Vec::new();
    let mut zombies_coordinates = Vec::new();
    let mut ghouls_coordinates = Vec::new();
    let mut demons_coordinates = Vec::new();

    let map = init_map();

    use UnitVector::*;

    for (i, &normal) in [Back, Up, Left, Front, Down, Right].iter().enumerate() {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            for y in -PLANET_RADIUS..=PLANET_RADIUS {
                let (abscissa, ordinate) = normal.abscissa_and_ordinate();

                let cubelet = PLANET_RADIUS * normal + x * abscissa + y * ordinate;

                let game_coordinates = Position { cubelet, normal }.into();

                let i = i as isize;

                let sign = 2 * (i % 2) - 1;

                let index = (
                    sign * x + PLANET_RADIUS,
                    -y * sign + (2 * i + 1) * (PLANET_RADIUS + 1),
                );

                if let Some(c) = map.get(&index) {
                    match &c[..] {
                        "#" => wall_coordinates.push(game_coordinates),
                        "@" => {
                            if player_coordinates.is_some() {
                                panic!("Only one player is supported!")
                            } else {
                                player_coordinates = Some(GameCoordinates {
                                    tangent: Some(abscissa),
                                    ..game_coordinates
                                });
                            }
                        }
                        "Q" => {
                            if portal_coordinates.is_some() {
                                panic!("Only one portal is supported!")
                            } else {
                                portal_coordinates = Some(game_coordinates);
                            }
                        }
                        "z" => zombies_coordinates.push(GameCoordinates {
                            tangent: Some(ordinate),
                            ..game_coordinates
                        }),
                        "x" => ghouls_coordinates.push(GameCoordinates {
                            tangent: Some(ordinate),
                            ..game_coordinates
                        }),
                        "d" => demons_coordinates.push(GameCoordinates {
                            tangent: Some(ordinate),
                            ..game_coordinates
                        }),
                        "o" => mov_wall_coordinates.push(game_coordinates),
                        _ => (),
                    }
                }
            }
        }
    }

    //player
    if let Some(player_coordinates) = player_coordinates {
        create_player(
            &mut commands,
            &asset_server,
            &mut turn_queue,
            &mut texture_atlases,
            player_coordinates,
        );
    } else {
        panic!("No player on this map!")
    }

    //portal
    if let Some(portal_coordinates) = portal_coordinates {
        create_portal(
            &mut commands,
            &asset_server,
            &mut texture_atlases,
            portal_coordinates,
        );
    } else {
        panic!("No portal on this map!")
    }

    //enemies
    create_enemies(
        &mut commands,
        &asset_server,
        &mut turn_queue,
        &mut texture_atlases,
        zombies_coordinates,
        Enemy::Zombie,
    );

    create_enemies(
        &mut commands,
        &asset_server,
        &mut turn_queue,
        &mut texture_atlases,
        ghouls_coordinates,
        Enemy::Ghoul,
    );

    create_enemies(
        &mut commands,
        &asset_server,
        &mut turn_queue,
        &mut texture_atlases,
        demons_coordinates,
        Enemy::Demon,
    );

    //walls
    create_walls(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        wall_coordinates,
        false,
    );

    //movable_walls
    create_walls(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        mov_wall_coordinates,
        true,
    );

    //tiles
    create_planet(&mut commands, &asset_server, &mut texture_atlases);
}
