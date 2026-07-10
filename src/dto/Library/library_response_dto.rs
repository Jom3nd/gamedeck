use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize)]

pub struct LibraryGameDto {
    pub id: Uuid,
    pub title: String,
    pub cover_url: String,
}

#[derive(Debug, Serialize)]
pub struct LibraryResponseDto {
    pub id: Uuid,
    pub game: LibraryGameDto,
    pub created_at: DateTime<Utc>,
}
