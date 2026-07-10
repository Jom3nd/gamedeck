use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]

pub struct CreateReviewDto {
    pub game_id: Uuid,
    pub score: i16,
    pub title: String,
    pub comment: String,
}