use bevy::prelude::*;
use crate::AppState;
use crate::game::enemy::systems::{despawn_enemies, spawn_enemies};
use crate::game::SimulationState;
use crate::game::star::resources::StarSpawnTimer;
use crate::game::star::systems::{spawn_stars, spawn_stars_over_time, tick_star_spawn_timer};
use crate::systems::is_game_running;

pub mod components;
mod resources;
pub mod systems;

pub const STAR_SIZE: f32 = 30.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SPAWN_TIME:f32 = 1.0;


pub struct StarPlugin;
impl Plugin for StarPlugin{

    fn build(&self, app: &mut App){
        app
            .init_resource::<StarSpawnTimer>()
            //.add_systems(Update, tick_star_spawn_timer)
            //.add_systems(Update, spawn_stars_over_time)
            //On Enter State
            .add_systems(OnEnter(AppState::Game), spawn_stars) // Runs when entering AppState::Game

            .add_systems(Update,
                         (
                             tick_star_spawn_timer,
                             spawn_stars_over_time,
                         ).run_if(is_game_running))
                             //TODO: Why cant chain run_if like this?
                         //    .run_if(in_state(AppState::Game)
                          //       .run_if(in_state(SimulationState::Running))))
            //.add_systems(Startup, spawn_stars);
        //OnExit
         .add_systems(OnExit(AppState::Game), despawn_enemies);

    }
}