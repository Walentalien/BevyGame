use bevy::prelude::*;
use crate::score::resources::*;

pub(crate) mod resources;

mod systems;
use crate::score::systems::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App){
        app
            .init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_systems(Update, update_score)
            .add_systems(Update, update_high_scores)
            .add_systems(Update, high_scores_updated);




    }
}