/**
 * 错误处理
 * 统一的错误类型和响应格式
 */

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

/// 应用错误类型
#[derive(Debug, Error)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Redis 错误: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("验证错误: {0}")]
    Validation(#[from] validator::ValidationErrors),

    #[error("认证错误: {0}")]
    Unauthorized(String),

    #[error("未找到资源: {0}")]
    NotFound(String),

    #[error("请求错误: {0}")]
    BadRequest(String),

    #[error("内部服务器错误: {0}")]
    InternalServerError(String),

    #[error("业务错误: {0}")]
    Business(String),

    #[error("权限不足: {0}")]
    Forbidden(String),

    #[error("内部错误: {0}")]
    Internal(#[from] anyhow::Error),
}

/// 应用结果类型
pub type AppResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, 5000, "数据库错误".to_string())
            }
            AppError::Redis(e) => {
                tracing::error!("Redis error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, 5001, "缓存错误".to_string())
            }
            AppError::Validation(e) => {
                let message = format!("验证失败: {}", e);
                (StatusCode::BAD_REQUEST, 4000, message)
            }
            AppError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, 4001, msg)
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, 4004, msg)
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, 4000, msg)
            }
            AppError::InternalServerError(msg) => {
                tracing::error!("Internal server error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, 5000, msg)
            }
            AppError::Business(msg) => {
                (StatusCode::BAD_REQUEST, 4000, msg)
            }
            AppError::Forbidden(msg) => {
                (StatusCode::FORBIDDEN, 4003, msg)
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, 5000, format!("内部错误: {}", e))
            }
        };

        let body = Json(json!({
            "code": code,
            "message": message,
            "data": serde_json::Value::Null,
        }));

        (status, body).into_response()
    }
}

/// 转换 anyhow::Error 为 AppError
impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Business(s)
    }
}

