use bevy::prelude::*;


mod systems;
use systems::*;

mod game;
mod main_menu;
mod events;

use crate::game::GamePlugin;

fn main() {
    App::new()
        //Bevy plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // My plugins
        .add_plugins(GamePlugin)
        // StartUp Systems
        .add_systems(Startup, spawn_camera)
        //Systems
        .add_systems(Update, transition_to_game_state)
        .add_systems(Update, transition_to_main_menu)
        .add_systems(Update, exit_game)
        .add_systems(Update,handle_game_over)
        .run();
}

#[derive(States, Debug,Hash, PartialEq, Clone, Copy,Eq,Default)] // Required for Bevy's state system
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
