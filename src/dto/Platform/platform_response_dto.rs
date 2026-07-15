use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]

pub struct PlatformResponseDto {
    pub id: Uuid,
    pub name:String,
    pub description:String,
}