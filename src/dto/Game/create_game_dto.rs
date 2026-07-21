use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateGameDto {
    pub title: String,
    pub description: Option<String>,
    pub developer: String,
    pub price: String,
    pub release_date: Option<NaiveDate>,
    pub created_at: Option<NaiveDate>,
    pub game_url: Option<String>,
    pub genres: Vec<Uuid>,
}
