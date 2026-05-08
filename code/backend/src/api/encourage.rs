/**
 * 鼓励卡片相关 API
 */

use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::{json, Value};

use crate::{
    api::AppState,
    model::ApiResponse,
    service::content_service,
    utils::error::AppResult,
};

/// 配置鼓励卡片路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/random", get(get_random_encourage))
        .route("/list", get(get_encourage_list))
}

/// 获取随机鼓励卡片
async fn get_random_encourage(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let card = content_service::get_random_encourage_card(&state.db_pool).await?;
    
    Ok(Json(ApiResponse::success(json!(card))))
}

/// 获取鼓励卡片列表
async fn get_encourage_list(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let cards = content_service::get_encourage_cards(&state.db_pool).await?;
    
    Ok(Json(ApiResponse::success(json!(cards))))
}

