/**
 * 认证 API
 */

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tracing::{info, error};
use validator::Validate;

use super::AppState;
use crate::model::{ApiResponse, LoginRequest, LoginResponse, RegisterRequest, UserVo};
use crate::repository;
use crate::service::{AuthService, UserService};
use crate::utils::error::AppError;

/// 配置认证路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/logout", post(logout))
        .route("/user-info", get(get_user_info))
}

/// 用户登录
async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    info!("📝 收到登录请求: username={}", request.username);
    
    // 验证请求数据
    request.validate()?;

    // 获取Redis连接
    let mut redis_conn = repository::get_redis_conn(&state.redis_pool).await
        .map_err(|e| AppError::InternalServerError(format!("Redis连接失败: {}", e)))?;

    // 执行登录
    let (token, refresh_token, user_info) =
        AuthService::login(&state.db_pool, &mut redis_conn, &state.config, &request).await?;

    info!("✅ 用户登录成功: username={}, user_id={}", request.username, user_info.id);

    let response = LoginResponse {
        token,
        refresh_token,
        user_info,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// 用户注册
async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    info!("📝 收到注册请求: username={}, email={:?}", request.username, request.email);
    
    // 验证请求数据
    request.validate()?;

    // 获取Redis连接
    let mut redis_conn = repository::get_redis_conn(&state.redis_pool).await
        .map_err(|e| AppError::InternalServerError(format!("Redis连接失败: {}", e)))?;

    // 执行注册
    let (token, refresh_token, user_info) =
        AuthService::register(&state.db_pool, &mut redis_conn, &state.config, &request).await?;

    info!("✅ 用户注册成功: username={}, user_id={}", request.username, user_info.id);

    let response = LoginResponse {
        token,
        refresh_token,
        user_info,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// 用户退出登录
async fn logout(
    State(state): State<AppState>,
    auth_user: crate::middleware::auth::AuthUser,
) -> Result<Json<ApiResponse<()>>, AppError> {
    info!("📝 收到退出登录请求: user_id={}", auth_user.user_id);
    
    // 获取Redis连接
    let mut redis_conn = repository::get_redis_conn(&state.redis_pool).await
        .map_err(|e| AppError::InternalServerError(format!("Redis连接失败: {}", e)))?;

    // 使token失效
    AuthService::logout(&mut redis_conn, &auth_user.user_id.to_string()).await
        .map_err(|e| AppError::InternalServerError(format!("退出登录失败: {}", e)))?;
    
    info!("✅ 用户退出登录成功: user_id={}", auth_user.user_id);
    Ok(Json(ApiResponse::success(())))
}

/// 获取当前用户信息
async fn get_user_info(
    State(state): State<AppState>,
    auth_user: crate::middleware::auth::AuthUser,
) -> Result<Json<ApiResponse<UserVo>>, AppError> {
    info!("📝 获取用户信息: user_id={}", auth_user.user_id);
    
    let user_info = UserService::get_user_info(&state.db_pool, auth_user.user_id).await?;
    
    info!("✅ 获取用户信息成功: user_id={}", auth_user.user_id);
    Ok(Json(ApiResponse::success(user_info)))
}

