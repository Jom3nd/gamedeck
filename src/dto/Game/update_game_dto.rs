use chrono::NaiveDate;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UpdateGameDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub developer: Option<String>,
    pub price: Option<String>,
    pub genre_id: Option<Uuid>,
    pub release_date: Option<NaiveDate>,
    pub game_url: Option<String>,
}
