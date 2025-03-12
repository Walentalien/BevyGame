use bevy::prelude::{DetectChanges, EventReader, Res, ResMut};
use crate::score::{resources };
use crate::score::resources::*;
use crate::events::GameOver;
pub fn update_score(score: Res<Score>){
    if score.is_changed(){
        println!("Score updated!New score: {}", score.value.to_string());
    }
}


pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
){
    for event in game_over_event_reader.read(){
        high_scores.scores.push(("Player".to_string(), event.score))
    }
}

pub fn high_scores_updated(
    high_scores: Res<HighScores>,
){
    if   high_scores.is_changed(){
        println!("High score updated! High scores: {:?}", high_scores);
    }
}