use bevy::prelude::*;
use crate::game::score::ScorePlugin;

pub mod enemy;
pub mod player;
pub mod star;
pub mod score;
mod systems;
use systems::*;
use player::PlayerPlugin;
use star::StarPlugin;
use enemy::EnemyPlugin;
use crate::AppState;
use crate::events::GameOver;
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<SimulationState>()
            .add_plugins(ScorePlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(StarPlugin)
            .add_plugins(EnemyPlugin)
            //Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_event::<GameOver>()
        ;
    }
}

#[derive(States, Debug,Clone,Copy,Eq, PartialEq,Default,Hash)]
pub enum SimulationState{
    Running,
    #[default]
    Paused,
}