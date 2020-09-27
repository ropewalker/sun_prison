use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn insolation_system(
    sun_path: ResMut<SunPath>,
    mut sky_query: Query<With<Tile, (&mut GameCoordinates, &mut Insolation)>>,
) {
    let index = sun_path.current_stage_index;
    let path = &sun_path.path;

    for (coordinates, mut insolation) in &mut sky_query.iter() {
        let normal = coordinates.position.normal;

        *insolation = match normal {
            sunny_side if sunny_side == *path.get(index).unwrap() => Insolation::Day,
            twilight_side
                if twilight_side == *path.get((index + 1) % path.len()).unwrap()
                    || twilight_side
                        == *path.get((path.len() + index - 1) % path.len()).unwrap() =>
            {
                Insolation::Twilight
            }
            _ => Insolation::Night,
        };
    }
}
