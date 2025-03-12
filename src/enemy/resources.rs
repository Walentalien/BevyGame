use bevy::prelude::{Resource, Timer};
use bevy::prelude::TimerMode::Repeating;
use crate::enemy::*;

#[derive(Resource)]
pub struct EnemySpawnTimer{
    pub timer: Timer,
}

impl Default for EnemySpawnTimer{
    fn default()->Self{
        Self{ timer: Timer::from_seconds(ENEMY_SPAWN_TIME, Repeating) }
    }
}