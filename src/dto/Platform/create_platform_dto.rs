use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct CreatePlatformDto {
    pub name: String,
    pub description: String,
}