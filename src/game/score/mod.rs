use bevy::prelude::*;
use crate::AppState;
use crate::game::score::resources::*;

pub(crate) mod resources;

mod systems;
use crate::game::score::systems::*;
use crate::systems::is_game_running;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App){
        app
            //On Enter State
            .add_systems(Update, insert_score.run_if(in_state(AppState::Game)))
            .init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(Update, update_score.run_if(in_state(AppState::Game)))
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updated)
            //On Exit State
            .add_systems(Update, remove_score.run_if(is_game_running));




    }
}