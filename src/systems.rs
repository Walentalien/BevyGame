use bevy::prelude::*;
use bevy::input::ButtonInput;
use bevy::prelude::{Camera2d, Commands, EventReader, EventWriter, KeyCode, Query, Res, ResMut, Sprite, Time, Transform, Window, With};

use crate::events::GameOver;

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


pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Game over! Your final score is {}", event.score.to_string());
    }
}


