mod people_pugin;
mod components;
use components::*;
mod events;
use events::*;
mod resources;
use resources::*;

mod systems;
use systems::*;

use crate::people_pugin::PeoplePlugin;

use std::fmt::Pointer;
use bevy::audio::{AddAudioSource, AudioLoader, PlaybackMode};

use bevy::prelude::*;
use bevy::app::AppExit;
use bevy::sprite::Anchor;
use bevy::time::TimerMode::Repeating;
use bevy::window::PrimaryWindow;
use rand::{random, rng, Rng};
use crate::components::Player;

pub const PLAYER_SIZE:f32=64.0;
pub const ENEMY_SIZE:f32=64.0;
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUMBER_OF_ENEMIES:usize=5;

pub const ENEMY_SPEED: f32 = 250.0;
pub const STAR_SIZE: f32 = 30.0;
pub const NUMBER_OF_STARS: usize = 10;

pub const ENEMY_SPAWN_TIME:f32 = 5.0;
pub const STAR_SPAWN_TIME:f32 = 1.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(PeoplePlugin)
        .add_systems(Startup, spawn_enemies)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<HighScores>()
        .add_event::<GameOver>()
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, confine_player_movement)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Startup, spawn_stars)
        .add_systems(Update, player_hit_star)
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, exit_game)
        .add_systems(Update, update_high_scores)
        .add_systems(Update, high_scores_updated)
        .run();
}
