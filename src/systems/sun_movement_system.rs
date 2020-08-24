use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn sun_movement_system(
    mut current_turn: ResMut<CurrentTurn>,
    mut sun_path: ResMut<SunPath>,
    mut sky_query: Query<(&Tile, Mut<NormalOrientation>, Mut<Insolation>)>,
) {
    if current_turn.side == GameSide::Sun {
        if current_turn.turn_number % 5 == 0 {
            sun_path.current_stage_index = (sun_path.current_stage_index + 1) % sun_path.path.len();

            let index = sun_path.current_stage_index;
            let path = &sun_path.path;

            for (_, normal_orientation, mut insolation) in &mut sky_query.iter() {
                if normal_orientation.0 == *path.get(index).unwrap() {
                    *insolation = Insolation::Day;
                } else if normal_orientation.0 == *path.get((index + 1) % path.len()).unwrap()
                    || normal_orientation.0
                        == *path.get((path.len() + index - 1) % path.len()).unwrap()
                {
                    *insolation = Insolation::Twilight;
                } else {
                    *insolation = Insolation::Night;
                }
            }
        }

        *current_turn = CurrentTurn {
            side: GameSide::Player,
            turn_number: current_turn.turn_number + 1,
        };
    }
}
