/**
 * 写作交流相关 API
 */

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::{json, Value};

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{dto::{CreateCommentRequest, CreateWritingWorkRequest, PageRequest}, ApiResponse},
    utils::error::AppResult,
};

/// 配置写作交流路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/works", get(get_works).post(create_work))
        .route("/works/:id", get(get_work).put(update_work).delete(delete_work))
        .route("/works/:id/comments", get(get_comments).post(create_comment))
        .route("/works/:id/like", post(like_work))
}

async fn get_works(
    State(_state): State<AppState>,
    Query(_params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<Vec<Value>>>> {
    Ok(Json(ApiResponse::success(vec![])))
}

async fn create_work(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    Json(_req): Json<CreateWritingWorkRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    Ok(Json(ApiResponse::success(())))
}

async fn get_work(State(_state): State<AppState>, Path(_id): Path<String>) -> AppResult<Json<ApiResponse<()>>> {
    Ok(Json(ApiResponse::success(())))
}

async fn update_work(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    Path(_id): Path<String>,
    Json(_req): Json<CreateWritingWorkRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    Ok(Json(ApiResponse::success(())))
}

async fn delete_work(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    Path(_id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    Ok(Json(ApiResponse::success(())))
}

async fn get_comments(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
    Query(_params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<Vec<Value>>>> {
    Ok(Json(ApiResponse::success(vec![])))
}

async fn create_comment(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    Path(_id): Path<String>,
    Json(_req): Json<CreateCommentRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    Ok(Json(ApiResponse::success(())))
}

async fn like_work(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    Path(_id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    Ok(Json(ApiResponse::success(())))
}

