use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct GameResponseDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub developer: String,
    pub price: String,
    pub genre_id: Uuid,
    pub release_date: Option<NaiveDate>,
    pub game_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
