/**
 * 认证中间件
 * 验证 JWT token
 */

use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use uuid::Uuid;

use crate::api::AppState;
use crate::repository;
use crate::repository::redis::{RedisRepository, cache_keys};
use crate::service::AuthService;
use crate::utils::error::AppError;
use chrono::Utc;

/// 认证用户信息
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 从请求头获取 token
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("缺少 Authorization 头".to_string()))?;

        // 提取 Bearer token
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("无效的 Authorization 头格式".to_string()))?;

        // 验证 token 并提取用户 ID
        let user_id = AuthService::extract_user_id(token, &state.config)
            .map_err(|_| AppError::Unauthorized("无效的 token".to_string()))?;

        // 检查token是否在Redis中存在（单点登录验证）
        let mut redis_conn = crate::repository::get_redis_conn(&state.redis_pool).await
            .map_err(|e| AppError::InternalServerError(format!("Redis连接失败: {}", e)))?;
        
        let token_valid = AuthService::verify_token_in_redis(&mut redis_conn, &user_id.to_string(), token).await
            .map_err(|e| AppError::InternalServerError(format!("Token验证失败: {}", e)))?;

        if !token_valid {
            return Err(AppError::Unauthorized("Token已失效，请重新登录".to_string()));
        }

        // 更新用户最后活跃时间（用于在线状态检测）
        let online_key = format!("{}online:{}", cache_keys::USER_INFO, user_id);
        let now_timestamp = Utc::now().timestamp();
        let _ = RedisRepository::set(&mut redis_conn, &online_key, &now_timestamp.to_string(), 300).await;
        // 忽略错误，在线状态不是关键功能

        Ok(AuthUser { user_id })
    }
}

