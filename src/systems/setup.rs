use crate::algebra::*;
use crate::components::{GameCoordinates, Position};
use crate::entities::*;
use crate::resources::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_layout() -> String {
    let path = Path::new("assets/config/map.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();

    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why)
    }

    String::from(s.trim())
}

fn init_map() -> HashMap<(isize, isize), String> {
    let layout = read_layout();

    let mut map = HashMap::new();

    for (y, line) in layout.lines().enumerate() {
        for (x, c) in line.trim().split(' ').enumerate() {
            map.insert((x as isize, y as isize), c.to_string());
        }
    }

    map
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
) {
    commands.spawn(Camera2dComponents::default());

    let mut player_coordinates = None;
    let mut wall_coordinates = Vec::new();
    let mut mov_wall_coordinates = Vec::new();
    let mut dumb_enemies_coordinates = Vec::new();
    let mut smart_enemies_coordinates = Vec::new();

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
                        "z" => dumb_enemies_coordinates.push(GameCoordinates {
                            tangent: Some(ordinate),
                            ..game_coordinates
                        }),
                        "x" => smart_enemies_coordinates.push(GameCoordinates {
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
            &mut texture_atlases,
            &mut textures,
            player_coordinates,
        );
    } else {
        panic!("No player on this map!")
    }

    //enemies
    create_enemies(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
        dumb_enemies_coordinates,
        false,
    );

    create_enemies(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
        smart_enemies_coordinates,
        true,
    );

    //walls
    create_walls(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
        wall_coordinates,
        false,
    );

    //movable_walls
    create_walls(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
        mov_wall_coordinates,
        true,
    );

    //tiles
    create_tiles(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
    );
}
