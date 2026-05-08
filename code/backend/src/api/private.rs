/**
 * 私人空间相关 API
 */

use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::{json, Value};

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{dto::{AddPrivateSpaceMusicRequest, UpdatePrivateSpaceSettingsRequest}, ApiResponse},
    utils::error::AppResult,
};

/// 配置私人空间路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/settings", get(get_settings).put(update_settings))
        .route("/music", get(get_music_list).post(add_music))
        .route("/music/:id", delete(delete_music))
}

async fn get_settings(State(_state): State<AppState>, auth_user: AuthUser) -> AppResult<Json<ApiResponse<Value>>> {
    Ok(Json(ApiResponse::success(json!({"light_color": "#667eea", "light_intensity": 5}))))
}

async fn update_settings(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    Json(_req): Json<UpdatePrivateSpaceSettingsRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    Ok(Json(ApiResponse::success(())))
}

async fn get_music_list(State(_state): State<AppState>, auth_user: AuthUser) -> AppResult<Json<ApiResponse<Vec<Value>>>> {
    Ok(Json(ApiResponse::success(vec![])))
}

async fn add_music(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    Json(_req): Json<AddPrivateSpaceMusicRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    Ok(Json(ApiResponse::success(())))
}

async fn delete_music(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    Path(_id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    Ok(Json(ApiResponse::success(())))
}

