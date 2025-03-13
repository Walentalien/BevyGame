use std::any::Any;
use bevy::prelude::*;
use crate::AppState;

pub mod components;
pub mod resources;
use crate::game::enemy::resources::EnemySpawnTimer;

pub mod systems;
use crate::game::enemy::systems::*;
use crate::game::SimulationState;
use crate::systems::is_game_running;

pub const ENEMY_SIZE:f32=64.0;

    pub const NUMBER_OF_ENEMIES:usize=5;
    pub const ENEMY_SPEED: f32 = 250.0;
    pub const ENEMY_SPAWN_TIME:f32 = 5.0;


pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App){
        app
            .init_resource::<EnemySpawnTimer>()
            //.add_systems(Startup, spawn_enemies)
            //.add_systems(Startup, spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            .add_systems(OnEnter(AppState::Game), spawn_enemies) // Runs when entering AppState::Game
            .add_systems(Update, (
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,

            ).run_if(is_game_running))
            .add_systems(OnExit(AppState::Game), despawn_enemies);

    }
}

