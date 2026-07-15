use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub error: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(status: u16, error:String , message: String) -> Self {
        Self {
            status,
            error,
            message
        }
    }
}