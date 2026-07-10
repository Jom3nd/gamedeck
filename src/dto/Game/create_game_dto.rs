use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateGameDto {
    pub title: String,
    pub description: Option<String>,
    pub developer: String,
    pub price: String,
    pub genre_id: Uuid,
    pub release_date: Option<NaiveDate>,
    pub game_url: Option<String>,
}
