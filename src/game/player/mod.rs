use bevy::app::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin, SystemSet};
use bevy::prelude::IntoSystemSetConfigs;

pub mod components;
mod systems;
use systems::*;
use crate::AppState;
use crate::systems::is_game_running;

#[derive(SystemSet,Debug, Hash, Eq, PartialEq, Clone)]
pub struct MovementSystemSet;
#[derive(SystemSet,Debug, Hash, Eq, PartialEq, Clone)]
pub struct ConfinementSystemSet;
//Example how systemssets can be defined as enums
#[derive(SystemSet,Debug, Hash, Eq, PartialEq, Clone)]
pub enum PlayerSystemSet {
    Movement,
    Confinement,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update, (MovementSystemSet,ConfinementSystemSet).chain())
            //On Enter State
            .add_systems(OnEnter(AppState::Game),spawn_player )
            .add_systems(Update, player_movement.in_set(MovementSystemSet).run_if(is_game_running))
            .add_systems(Update, confine_player_movement.in_set(ConfinementSystemSet).run_if(is_game_running))
            //.add_systems(Startup, spawn_player)
            //alternative ways to chain:
            //.add_systems(Update, confine_player_movement.after(player_movement))
            //.add_systems(Update, (player_movement,confine_player_movement).chain())
            //.add_systems(Update, player_movement.before(confine_player_movement))
            //.add_systems(Update, player_hit_star)
            //.add_systems(Update, enemy_hit_player)
            .add_systems(Update,(enemy_hit_player,player_hit_star).run_if(is_game_running))
            // On exit
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}