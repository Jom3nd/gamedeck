use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AddGameDto {
    pub game_id: Uuid,
}