use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ReviewResponseDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub game_id: Uuid,
    pub score: i16,
    pub title: String,
    pub comment: String,
    pub created_at: DateTime<Utc>,
}