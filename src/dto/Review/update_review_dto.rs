use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]

pub struct UpdateReviewDto {
    pub score: Option<i16>,
    pub title: Option<String>,
    pub comment: Option<String>,
}