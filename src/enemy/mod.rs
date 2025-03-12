use std::any::Any;
use bevy::prelude::*;

pub mod components;
pub mod resources;
use crate::enemy::resources::EnemySpawnTimer;

pub mod systems;
use crate::enemy::systems::*;


    pub const ENEMY_SIZE:f32=64.0;

    pub const NUMBER_OF_ENEMIES:usize=5;
    pub const ENEMY_SPEED: f32 = 250.0;
    pub const ENEMY_SPAWN_TIME:f32 = 5.0;



pub struct EnemyPlugin;

impl Plugin for EnemyPlugin{
    fn build(&self, app: &mut App){
        app
            .init_resource::<EnemySpawnTimer>()
            .add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, update_enemy_direction)
            .add_systems(Update, confine_enemy_movement);
    }
}
