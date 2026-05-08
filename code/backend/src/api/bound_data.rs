/**
 * 绑定用户数据访问 API
 * 查看绑定用户授权的应用数据
 */

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::AppState;
use crate::middleware::auth::AuthUser;
use crate::model::ApiResponse;
use crate::service::bound_data_service;
use crate::utils::error::AppError;

/// 配置绑定数据路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/:bound_user_id/notes", get(get_bound_user_notes))
        .route("/:bound_user_id/accounts", get(get_bound_user_accounts))
        .route("/:bound_user_id/accounts/statistics", get(get_account_statistics))
}

/// 查询参数：日期范围
#[derive(Debug, Deserialize)]
pub struct DateRangeQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

/// 获取绑定用户的便签数据
async fn get_bound_user_notes(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(bound_user_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<bound_data_service::BoundNoteVo>>>, AppError> {
    info!("📝 API: 获取绑定用户便签");

    let data_owner_id = Uuid::parse_str(&bound_user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let notes = bound_data_service::get_bound_user_notes(
        &state.db_pool,
        auth_user.user_id,
        data_owner_id,
    )
    .await?;

    Ok(Json(ApiResponse::success(notes)))
}

/// 获取绑定用户的记账数据
async fn get_bound_user_accounts(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(bound_user_id): Path<String>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<Vec<bound_data_service::BoundAccountVo>>>, AppError> {
    info!("📝 API: 获取绑定用户记账数据");

    let data_owner_id = Uuid::parse_str(&bound_user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // 解析日期
    let start_date = query.start_date
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    
    let end_date = query.end_date
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let accounts = bound_data_service::get_bound_user_accounts(
        &state.db_pool,
        auth_user.user_id,
        data_owner_id,
        start_date,
        end_date,
    )
    .await?;

    Ok(Json(ApiResponse::success(accounts)))
}

/// 获取绑定用户的记账统计
async fn get_account_statistics(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(bound_user_id): Path<String>,
    Query(query): Query<DateRangeQuery>,
) -> Result<Json<ApiResponse<bound_data_service::AccountStatistics>>, AppError> {
    info!("📊 API: 获取绑定用户记账统计");

    let data_owner_id = Uuid::parse_str(&bound_user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    // 解析日期
    let start_date = query.start_date
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));
    
    let end_date = query.end_date
        .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let statistics = bound_data_service::get_bound_user_account_statistics(
        &state.db_pool,
        auth_user.user_id,
        data_owner_id,
        start_date,
        end_date,
    )
    .await?;

    Ok(Json(ApiResponse::success(statistics)))
}
