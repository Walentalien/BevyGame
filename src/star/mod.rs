use bevy::prelude::*;
use crate::star::resources::StarSpawnTimer;
use crate::star::systems::{spawn_stars, spawn_stars_over_time, tick_star_spawn_timer};

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
            .add_systems(Update, tick_star_spawn_timer)
            .add_systems(Update, spawn_stars_over_time)
            .add_systems(Startup, spawn_stars);
    }
}