use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct UpdatePlatformDto {
    pub name: Option<String>,
    pub description: Option<String>,
}