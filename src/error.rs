use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("Invalid key format: {0}")]
    KeyFormatError(String),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),
}

#[derive(Serialize)]
struct ApiErrorResponse {
    success: bool,
    error: String,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status = match self {
            ApiError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::SignatureError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::KeyFormatError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::AuthorizationError(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::ServerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status).json(ApiErrorResponse {
            success: false,
            error: self.to_string(),
        })
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::ValidationError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::SignatureError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::KeyFormatError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            ApiError::AuthorizationError(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            ApiError::ServerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError::ServerError(err.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::ValidationError(err.to_string())
    }
}

// Helper methods for creating errors
impl ApiError {
    pub fn validation(msg: impl Into<String>) -> Self {
        ApiError::ValidationError(msg.into())
    }

    pub fn signature(msg: impl Into<String>) -> Self {
        ApiError::SignatureError(msg.into())
    }

    pub fn key_format(msg: impl Into<String>) -> Self {
        ApiError::KeyFormatError(msg.into())
    }

    pub fn server(msg: impl Into<String>) -> Self {
        ApiError::ServerError(msg.into())
    }

    pub fn authorization(msg: impl Into<String>) -> Self {
        ApiError::AuthorizationError(msg.into())
    }
}

