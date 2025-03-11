mod people_pugin;
use crate::people_pugin::PeoplePlugin;
use std::fmt::Pointer;
use std::slice::Windows;
use bevy::audio::{AddAudioSource, AudioLoader, PlaybackMode};
use bevy::ecs::system::lifetimeless::SCommands;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::time::TimerMode::Repeating;
use bevy::window::PrimaryWindow;
use rand::{random, rng, Rng};

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
        .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy{
    pub direction:Vec2,
}

#[derive(Component)]
pub struct Star{}

#[derive(Resource)]
pub struct Score{
    pub value:u32,
}


impl Default for Score{
    fn default()->Self{
        Self{value:0}
    }
}

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
            Player  {},
            )
    );

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

pub fn spawn_stars(
mut commands: Commands,
window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
){
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS{
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        let half_window_height = window.height() / 2.5;
        let half_window_width = window.width() / 2.5;
        let x= rng().random_range((-half_window_width)..half_window_width);
        let y = rng().random_range(-half_window_height..half_window_height);

        commands.spawn((
            Sprite{
                image: asset_server.load("sprites/Flower.png"),
                ..Default::default()
            },
            Transform::from_xyz(x,y,0.),
            Star{},
            ));
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

    for (mut transform) in enemy_query.iter_mut() {
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

pub fn enemy_hit_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>
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

pub fn update_score(score: Res<Score>){
    if score.is_changed(){
        println!("Score updated!New score: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(
mut star_spawn_timer: ResMut<StarSpawnTimer>,
time: Res<Time>,
){
    star_spawn_timer.timer.tick(time.delta());

}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
){
    if star_spawn_timer.timer.finished(){
        println!("Spawn stars over time!");
        let window = window_query.get_single().unwrap();
        let random_x= random::<f32>() * window.width();
        let random_y= random::<f32>() * window.height();

        commands.spawn((Sprite{
            image: asset_server.load("sprites/Flower.png"),
            texture_atlas: None,
            color: Default::default(),
            flip_x: false,
            flip_y: false,
            custom_size: None,
            rect: None,
            anchor: Default::default(),
            image_mode: Default::default(),
        },
                        Transform::from_xyz(random_x,random_y,0.,), Star {}));

    }
}