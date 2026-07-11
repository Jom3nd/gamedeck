use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct UpdateReviewDto {
    pub score: Option<i16>,
    pub title: Option<String>,
    pub comment: Option<String>,
}