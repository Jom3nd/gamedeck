use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct UserLoginDto {
    pub email: String,
    pub password: String,
}