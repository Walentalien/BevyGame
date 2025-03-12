use bevy::prelude::{Resource, Timer};
use bevy::prelude::TimerMode::Repeating;
use super::STAR_SPAWN_TIME;
#[derive(Resource)]
pub struct StarSpawnTimer{
    pub timer:Timer,
}

impl Default for StarSpawnTimer{
    fn default()->Self{
        Self {
            timer:Timer::from_seconds(STAR_SPAWN_TIME, Repeating),
        }
    }
}