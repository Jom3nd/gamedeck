use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct GenreResponseDto {
    pub id: Uuid,
    pub name: String,
}