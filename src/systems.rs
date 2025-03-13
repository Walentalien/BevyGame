use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::input::keyboard::Key::Soft1;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::{Camera2d, Commands, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Sprite, Time, Transform, Window, With};
use crate::AppState;
use crate::events::GameOver;
use crate::game::SimulationState;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}


pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
){
    if keyboard_input.pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit::Success);
    }
}


pub fn handle_game_over(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Game over! Your final score is {}", event.score.to_string());
        state.set(AppState::GameOver);
    }
}

pub fn transition_to_game_state(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut next_simulation_state: ResMut<NextState<AppState>>,

){
    if keyboard_input.pressed(KeyCode::KeyG) {
        if *app_state.get() != AppState::Game {
            next_simulation_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,

){
    if keyboard_input.pressed(KeyCode::KeyM) {
        if *app_state.get()!= AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
//            next_simulation_state.set(SimulationState::Paused);
            println!("Entered AppState::MainMenu");
        }
    }
}

pub fn is_game_running(
    app_state: Res<State<AppState>>,
    sim_state: Res<State<SimulationState>>,
) -> bool {
    *app_state == AppState::Game && *sim_state == SimulationState::Running
}

