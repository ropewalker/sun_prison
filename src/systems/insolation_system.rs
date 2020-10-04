use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn insolation_system(
    sun_path: ResMut<SunPath>,
    mut sky_query: Query<With<Tile, (&mut GameCoordinates, &mut Insolation)>>,
) {
    for (coordinates, mut insolation) in &mut sky_query.iter() {
        let normal = coordinates.position.normal;

        *insolation = match normal {
            sunny_side if sunny_side == sun_path.sunny_side() => Insolation::Day,
            twilight_side
                if twilight_side == sun_path.morning_side()
                    || twilight_side == sun_path.evening_side() =>
            {
                Insolation::Twilight
            }
            _ => Insolation::Night,
        };
    }
}
