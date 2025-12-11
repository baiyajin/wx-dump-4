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
    
    #[error("Validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
    
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),
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
            AppError::ValidationFailed(msg) => (StatusCode::BAD_REQUEST, format!("Validation failed: {}", msg)),
            AppError::PermissionDenied(msg) => (StatusCode::FORBIDDEN, format!("Permission denied: {}", msg)),
            AppError::Timeout(msg) => (StatusCode::REQUEST_TIMEOUT, format!("Timeout: {}", msg)),
            AppError::ResourceExhausted(msg) => (StatusCode::INSUFFICIENT_STORAGE, format!("Resource exhausted: {}", msg)),
            AppError::Internal(err) => {
                tracing::error!("Internal error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let error_code = self.error_code();
        let user_message = self.user_message();
        
        let body = Json(json!({
            "error": error_message,
            "error_code": error_code,
            "user_message": user_message,
            "code": status.as_u16(),
            "recoverable": self.is_recoverable(),
        }));

        (status, body).into_response()
    }
}

impl AppError {
    /// 判断错误是否可恢复
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AppError::Timeout(_) | AppError::Database(_) | AppError::Internal(_)
        )
    }
    
    /// 获取用户友好的错误消息
    pub fn user_message(&self) -> String {
        match self {
            AppError::WeChatNotFound => "未找到微信进程，请确保微信已启动并登录".to_string(),
            AppError::VersionNotSupported(version) => format!("微信版本 {} 暂不支持，请使用支持的版本", version),
            AppError::DecryptionFailed(_) => "数据库解密失败，请检查密钥是否正确".to_string(),
            AppError::ValidationFailed(msg) => format!("输入验证失败: {}", msg),
            AppError::PermissionDenied(_) => "权限不足，请以管理员权限运行".to_string(),
            AppError::Database(msg) => format!("数据库操作失败: {}", msg),
            AppError::NotFound(msg) => format!("未找到资源: {}", msg),
            AppError::BadRequest(msg) => format!("请求错误: {}", msg),
            AppError::Timeout(msg) => format!("操作超时: {}", msg),
            AppError::ResourceExhausted(msg) => format!("资源不足: {}", msg),
            AppError::Internal(_) => "服务器内部错误，请稍后重试".to_string(),
        }
    }
    
    /// 获取错误代码
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::WeChatNotFound => "WECHAT_NOT_FOUND",
            AppError::VersionNotSupported(_) => "VERSION_NOT_SUPPORTED",
            AppError::DecryptionFailed(_) => "DECRYPTION_FAILED",
            AppError::ValidationFailed(_) => "VALIDATION_FAILED",
            AppError::PermissionDenied(_) => "PERMISSION_DENIED",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Timeout(_) => "TIMEOUT",
            AppError::ResourceExhausted(_) => "RESOURCE_EXHAUSTED",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }
}

