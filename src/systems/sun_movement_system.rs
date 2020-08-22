use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn sun_movement_system(
    mut current_turn: ResMut<CurrentTurn>,
    mut sun_path: ResMut<SunPath>,
    mut sky_query: Query<(&Sky, Mut<NormalOrientation>, Mut<TextureAtlasSprite>)>,
) {
    if *current_turn == CurrentTurn(GameSide::Sun) {
        sun_path.current_stage_index = (sun_path.current_stage_index + 1) % sun_path.path.len();

        let index = sun_path.current_stage_index;
        let path = &sun_path.path;

        for (_, normal_orientation, mut sprite) in &mut sky_query.iter() {
            if normal_orientation.0 == *path.get(index).unwrap() {
                sprite.index = 2;
            } else if normal_orientation.0 == *path.get((index + 1) % path.len()).unwrap()
                || normal_orientation.0 == *path.get((index - 1) % path.len()).unwrap()
            {
                sprite.index = 1;
            } else {
                sprite.index = 0;
            }
        }

        *current_turn = CurrentTurn(GameSide::Player);
    }
}
