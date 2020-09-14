use crate::algebra::*;
use crate::components::GameCoordinates;
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

    let mut player_coordinates = Vec::new();
    let mut wall_coordinates = Vec::new();
    let mut mov_wall_coordinates = Vec::new();

    let map = init_map();

    let tangent_orientation = None;

    use UnitVector::*;

    let normal_orientation = Back;

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x,
                y,
                z: -PLANET_RADIUS,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            let index = (y + PLANET_RADIUS, -x + PLANET_RADIUS + 1);

            if let Some(c) = map.get(&index) {
                match &c[..] {
                    "#" => wall_coordinates.push(game_coordinates),
                    "@" => player_coordinates.push(GameCoordinates {
                        tangent_orientation: Some(Left),
                        ..game_coordinates
                    }),
                    "o" => mov_wall_coordinates.push(game_coordinates),
                    _ => (),
                }
            }
        }
    }

    //up

    let normal_orientation = Up;

    for x in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x,
                y: PLANET_RADIUS,
                z,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            let index = (z + PLANET_RADIUS, -x + 3 * PLANET_RADIUS + 3);

            if let Some(c) = map.get(&index) {
                match &c[..] {
                    "#" => wall_coordinates.push(game_coordinates),
                    "@" => player_coordinates.push(GameCoordinates {
                        tangent_orientation: Some(Front),
                        ..game_coordinates
                    }),
                    "o" => mov_wall_coordinates.push(game_coordinates),
                    _ => (),
                }
            }
        }
    }

    //left

    let normal_orientation = Left;

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x: -PLANET_RADIUS,
                y,
                z,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            let index = (z + PLANET_RADIUS, -y + 5 * PLANET_RADIUS + 5);

            if let Some(c) = map.get(&index) {
                match &c[..] {
                    "#" => wall_coordinates.push(game_coordinates),
                    "@" => player_coordinates.push(GameCoordinates {
                        tangent_orientation: Some(Down),
                        ..game_coordinates
                    }),
                    "o" => mov_wall_coordinates.push(game_coordinates),
                    _ => (),
                }
            }
        }
    }

    //front

    let normal_orientation = Front;

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for x in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x,
                y,
                z: PLANET_RADIUS,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            let index = (x + PLANET_RADIUS, -y + 7 * PLANET_RADIUS + 7);

            if let Some(c) = map.get(&index) {
                match &c[..] {
                    "#" => wall_coordinates.push(game_coordinates),
                    "@" => player_coordinates.push(GameCoordinates {
                        tangent_orientation: Some(Right),
                        ..game_coordinates
                    }),
                    "o" => mov_wall_coordinates.push(game_coordinates),
                    _ => (),
                }
            }
        }
    }

    //down

    let normal_orientation = Down;

    for x in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x,
                y: -PLANET_RADIUS,
                z,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            let index = (x + PLANET_RADIUS, -z + 9 * PLANET_RADIUS + 9);

            if let Some(c) = map.get(&index) {
                match &c[..] {
                    "#" => wall_coordinates.push(game_coordinates),
                    "@" => player_coordinates.push(GameCoordinates {
                        tangent_orientation: Some(Back),
                        ..game_coordinates
                    }),
                    "o" => mov_wall_coordinates.push(game_coordinates),
                    _ => (),
                }
            }
        }
    }

    //right

    let normal_orientation = Right;

    for y in -PLANET_RADIUS..=PLANET_RADIUS {
        for z in -PLANET_RADIUS..=PLANET_RADIUS {
            let cubelet_position = Vector3 {
                x: PLANET_RADIUS,
                y,
                z,
            };

            let game_coordinates = GameCoordinates {
                cubelet_position,
                normal_orientation,
                tangent_orientation,
            };

            let index = (y + PLANET_RADIUS, -z + 11 * PLANET_RADIUS + 11);

            if let Some(c) = map.get(&index) {
                match &c[..] {
                    "#" => wall_coordinates.push(game_coordinates),
                    "@" => player_coordinates.push(GameCoordinates {
                        tangent_orientation: Some(Up),
                        ..game_coordinates
                    }),
                    "o" => mov_wall_coordinates.push(game_coordinates),
                    _ => (),
                }
            }
        }
    }

    //player
    create_player(
        &mut commands,
        &asset_server,
        &mut texture_atlases,
        &mut textures,
        player_coordinates,
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
