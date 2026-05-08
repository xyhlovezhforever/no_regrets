/**
 * 用户 API
 */

use axum::{
    extract::{Path, Query, State},
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;
use tracing::info;
use validator::Validate;

use super::AppState;
use crate::middleware::auth::AuthUser;
use crate::model::{ApiResponse, ChangePasswordRequest, UpdateUserRequest, UserVo};
use crate::service::UserService;
use crate::utils::error::AppError;

#[derive(Debug, Deserialize)]
struct SearchQuery {
    keyword: String,
}

/// 配置用户路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/info", get(get_user_info))
        .route("/:id/public", get(get_public_user_info))
        .route("/update", put(update_user_info))
        .route("/change-password", put(change_password))
        .route("/search", get(search_users))
}

/// 获取用户信息
async fn get_user_info(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<UserVo>>, AppError> {
    let user_info = UserService::get_user_info(&state.db_pool, auth_user.user_id).await?;
    Ok(Json(ApiResponse::success(user_info)))
}

/// 获取其他用户的公开信息
async fn get_public_user_info(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<UserVo>>, AppError> {
    use uuid::Uuid;
    use tracing::info;
    
    info!("📝 获取公开用户信息: user_id={}", id);
    
    let user_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;
    
    let user_info = UserService::get_user_info(&state.db_pool, user_id).await?;
    
    info!("✅ 用户信息: username={}, bio={:?}", user_info.username, user_info.bio);
    
    Ok(Json(ApiResponse::success(user_info)))
}

/// 更新用户信息
async fn update_user_info(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserVo>>, AppError> {
    // 验证请求数据
    request.validate()?;

    let user_info =
        UserService::update_user_info(&state.db_pool, auth_user.user_id, &request).await?;
    Ok(Json(ApiResponse::success(user_info)))
}

/// 修改密码
async fn change_password(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    // 验证请求数据
    request.validate()?;

    UserService::change_password(
        &state.db_pool,
        auth_user.user_id,
        &request.old_password,
        &request.new_password,
    )
    .await?;

    Ok(Json(ApiResponse::success(())))
}

/// 搜索用户
async fn search_users(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<SearchQuery>,
) -> Result<Json<ApiResponse<Vec<UserVo>>>, AppError> {
    info!("📝 搜索用户: keyword={}, user_id={}", params.keyword, auth_user.user_id);
    
    // 搜索用户，排除当前用户
    let users = UserService::search_users(&state.db_pool, &params.keyword, auth_user.user_id).await?;
    
    info!("✅ 搜索到 {} 个用户", users.len());
    
    Ok(Json(ApiResponse::success(users)))
}

