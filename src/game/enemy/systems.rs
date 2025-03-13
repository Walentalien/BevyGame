use bevy::asset::{AssetServer, Handle};
use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Query, Res, Sprite, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
use rand::{random, Rng};
use crate::game::enemy::*;
use crate::game::enemy::components::*;
//TODO: spawn_enemies_ober_time, tick_enemy_spawn_timer




pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();
    let mut rng = rand::rng();
    let half_window_height = window.height() / 2.0;
    let half_window_width = window.width() / 2.0;
    for _ in 0..NUMBER_OF_ENEMIES{
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        let x= rng.random_range(-half_window_width..half_window_width) - ENEMY_SIZE;
        let y = rng.random_range(-half_window_height..half_window_height)- ENEMY_SIZE;

        commands.spawn((Sprite{
            image: asset_server.load("sprites/ball_red_large.png"),
            texture_atlas: None,
            color: Default::default(),
            flip_x: false,
            flip_y: false,
            custom_size: None,
            rect: None,
            anchor: Default::default(),
            image_mode: Default::default(),
        },
                        Transform::from_xyz(x,y,0.,), Enemy{
                direction: Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0))
            }));
    }

}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity,With<Enemy>>
){
    for enemy_entity in enemy_query.iter(){
        commands.entity(enemy_entity).despawn();
    }
}


pub fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Enemy)>,
    time: Res<Time>,
){
    for ( mut transform, enemy) in enemy_query.iter_mut() {
        let direction:Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_secs();
    }
}



pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform,&mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
){
    let window = window_query.get_single().unwrap();

    let half_enemy = ENEMY_SIZE / 2.0;
    let x_min = -window.width() /2. +  half_enemy;
    let x_max = window.width()/ 2. - half_enemy;
    let y_min = -window.height()/2.+ half_enemy;
    let y_max = window.height()/2. - half_enemy;

    for (transform ,  mut enemy ) in enemy_query.iter_mut() {
        let  mut direction_changed = false;
        let translation = transform.translation;
        if translation.x< x_min || translation.x > x_max{
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y< y_min || translation.y > y_max{
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }
        //TODO: Pass sound as argument
        // play SFX
        let sound_effect_1: Handle<AudioSource> = asset_server.load("audio/pluck_001.ogg");
        let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");

        let sound_effect = if random::<f32>() > 0.5 {
            sound_effect_1
        } else {
            sound_effect_2
        };
        if direction_changed == true{
            commands.spawn((
                AudioPlayer::new(asset_server.load("audio/pluck_001.ogg")),
                PlaybackSettings::DESPAWN,
            ));
        }

    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    let window = window_query.get_single().unwrap();

    let half_enemy = ENEMY_SIZE / 2.0;
    let x_min = -window.width() /2. +  half_enemy;
    let x_max = window.width()/ 2. - half_enemy;
    let y_min = -window.height()/2.+ half_enemy;
    let y_max = window.height()/2. - half_enemy;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < x_min {
            translation.y = x_min;
        } else if translation.y > x_max {
            translation.y = x_max;
        }
        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(){
todo!();
}

pub fn spawn_enemies_over_time(){
    todo!()
}
