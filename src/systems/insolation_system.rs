use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn insolation_system(
    sun_path: ResMut<SunPath>,
    mut sky_query: Query<(&Tile, Mut<NormalOrientation>, Mut<Insolation>)>,
) {
    let index = sun_path.current_stage_index;
    let path = &sun_path.path;

    for (_, normal_orientation, mut insolation) in &mut sky_query.iter() {
        if normal_orientation.0 == *path.get(index).unwrap() {
            *insolation = Insolation::Day;
        } else if normal_orientation.0 == *path.get((index + 1) % path.len()).unwrap()
            || normal_orientation.0 == *path.get((path.len() + index - 1) % path.len()).unwrap()
        {
            *insolation = Insolation::Twilight;
        } else {
            *insolation = Insolation::Night;
        }
    }
}
