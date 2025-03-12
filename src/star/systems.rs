use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Query, Res, ResMut, Sprite, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
use rand::{random, rng, Rng};

use crate::star::components::*;
use crate::star::resources::*;
use super::NUMBER_OF_STARS;
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
