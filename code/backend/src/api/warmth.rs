/**
 * 温暖板块相关 API
 */

use axum::{extract::State, response::Json, routing::post, Router};
use serde_json::{json, Value};

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{dto::CreateLoveLetterRequest, ApiResponse},
    utils::error::AppResult,
};

/// 配置温暖板块路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/love-letter", post(generate_love_letter))
        .route("/hug", post(record_hug))
}

async fn generate_love_letter(
    State(_state): State<AppState>,
    auth_user: AuthUser,
    Json(_req): Json<CreateLoveLetterRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    Ok(Json(ApiResponse::success(json!({"content": "生成的情书内容"}))))
}

async fn record_hug(
    State(_state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Value>>> {
    Ok(Json(ApiResponse::success(json!({"total_hugs": 1}))))
}

