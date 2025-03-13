use bevy::asset::AssetServer;
use bevy::audio::{AudioPlayer, PlaybackSettings};
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Entity, EventWriter, KeyCode, Query, Res, ResMut, Sprite, Time, Transform, Window, With};
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;


use crate::game::player::components::*;
use crate::game::score::resources::*;
use crate::game::enemy::components::*;
use crate::game::enemy::*;
use crate::game::star::components::*;
use crate::game::star::STAR_SIZE;
use crate::events::GameOver;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE:f32=64.0;


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server:Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            Sprite {
                image: asset_server.load("sprites/ball_blue_large.png"),
                texture_atlas: None,
                color: Default::default(),
                flip_x: false,
                flip_y: false,
                custom_size: None,
                rect: None,
                anchor: Anchor::Center,
                image_mode: Default::default(),

            },
            Player {},
        )
    );

}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
){
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
        println!("Player despawned");
    }
}
pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    if let Ok(mut transforms) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA){
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD){
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW){
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS){
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        transforms.translation += direction * PLAYER_SPEED * time.delta_secs();

    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        if let Ok(window) = window_query.get_single() {
            let half_player_size = PLAYER_SIZE / 2.0;
            let x_min = -window.width() /2. +  half_player_size;
            let x_max = window.width()/ 2. - half_player_size;
            let y_min = -window.height()/2.+ half_player_size;
            let y_max = window.height()/2. - half_player_size;

            let mut translation = player_transform.translation;

            translation.x = translation.x.clamp(x_min, x_max);
            translation.y = translation.y.clamp(y_min, y_max);

            player_transform.translation = translation;
        }
    }
}


pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
){
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter(){
            let distance = player_transform.translation.distance(enemy_transform.translation);
            let player_radius  = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Enemy hit player!");
                commands.spawn((
                    AudioPlayer::new(asset_server.load("audio/explosionCrunch_000.ogg")),
                    PlaybackSettings::DESPAWN,
                ));
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver{ score: score.value});
            }
        }
    }
}


pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity,&Transform),With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
){
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter(){
            let distance = player_transform.translation.distance(star_transform.translation);
            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE/2.0 {
                println!("Player hit star!");
                score.value+=1;
                commands.spawn((
                    AudioPlayer::new(asset_server.load("audio/impactWood_light_001.ogg")),
                    PlaybackSettings::DESPAWN,
                ));
                commands.entity(star_entity).despawn();
            }
        }
    }
}

