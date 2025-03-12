use bevy::prelude::*;

mod events;
use events::*;

mod systems;
use systems::*;
mod star;
 mod score;

mod player;
mod enemy;





use std::fmt::Pointer;
use bevy::audio::{AddAudioSource};
use crate::enemy::EnemyPlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;
use crate::star::StarPlugin;

fn main() {
    App::new()
        .add_plugins(ScorePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(DefaultPlugins)
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, exit_game)
        .run();
}

// Added some useless stuff (Hope no one sees it)
