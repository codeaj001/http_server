use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde_json::Value;

pub type AppJson = Result<Json<Value>, AppError>;

#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub message: Json<Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

use serde_json::json;

pub fn success(data: Value) -> AppJson {
    Ok(Json(json!({
        "success": true,
        "data": data
    })))
}

pub fn bad_request(error: &str) -> AppJson {
    Err(AppError {
        status: StatusCode::BAD_REQUEST,
        message: Json(json!({
            "success": false,
            "error": error
        })),
    })
}
