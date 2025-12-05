use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("WeChat process not found")]
    WeChatNotFound,
    
    #[error("WeChat version not supported: {0}")]
    VersionNotSupported(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::WeChatNotFound => (StatusCode::NOT_FOUND, "WeChat process not found".to_string()),
            AppError::VersionNotSupported(version) => {
                (StatusCode::BAD_REQUEST, format!("WeChat version {} not supported", version))
            }
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", msg)),
            AppError::DecryptionFailed(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Decryption failed: {}", msg)),
            AppError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(json!({
            "error": error_message,
            "code": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

