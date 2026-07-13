use crate::errors::error_response::ErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse,Response},
    Json,
};

pub enum ApiError {
    BadRequest(String),          // 400
    Unauthorized(String),        // 401
    Forbidden(String),           // 403
    NotFound(String),            // 404
    Conflict(String),            // 409
    UnprocessableEntity(String), // 422s
    TooManyRequests(String),     // 429
    InternalServerError(String), // 500
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST,message),
            ApiError::Unauthorized(message) => (StatusCode::UNAUTHORIZED,message),
            ApiError::Forbidden(message) => (StatusCode::FORBIDDEN,message),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND,message),
            ApiError::Conflict(message) => (StatusCode::CONFLICT,message),
            ApiError::UnprocessableEntity(message) => (StatusCode::UNPROCESSABLE_ENTITY,message),
            ApiError::TooManyRequests(message) => (StatusCode::TOO_MANY_REQUESTS,message),
            ApiError::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR,message),
        };
        let body = ErrorResponse::new(
            status.as_u16(),
            status
                .canonical_reason()
                .unwrap_or("Unknown")
                .to_string(),
                message,
        );

        return (status, Json(body)).into_response();
    }
}