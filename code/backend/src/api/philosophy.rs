/**
 * 哲学命题相关 API
 */

use axum::{extract::State, response::Json, routing::get, Router};
use serde_json::{json, Value};

use crate::{
    api::AppState,
    model::ApiResponse,
    service::content_service,
    utils::error::AppResult,
};

/// 配置哲学命题路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/random", get(get_random_philosophy))
        .route("/list", get(get_philosophy_list))
}

/// 获取随机哲学命题
async fn get_random_philosophy(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let card = content_service::get_random_philosophy_card(&state.db_pool).await?;
    
    Ok(Json(ApiResponse::success(json!(card))))
}

/// 获取哲学命题列表
async fn get_philosophy_list(State(state): State<AppState>) -> AppResult<Json<ApiResponse<Value>>> {
    let cards = content_service::get_philosophy_cards(&state.db_pool).await?;
    
    Ok(Json(ApiResponse::success(json!(cards))))
}

