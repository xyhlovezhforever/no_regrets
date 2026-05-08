/**
 * 自我管理相关 API（记账等）
 */

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::{json, Value};
use tracing::info;
use uuid::Uuid;

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{dto::{CreateAccountRecordRequest, PageRequest, MonthlyBillQuery, YearlyBillQuery}, ApiResponse},
    repository::database::AccountRecordRepository,
    utils::error::{AppError, AppResult},
};

/// 配置记账路由
pub fn account_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_account_records).post(create_account_record))
        .route("/:id", get(get_account_record).put(update_account_record).delete(delete_account_record))
        .route("/summary", get(get_account_summary))
        .route("/monthly-bill", get(get_monthly_bill))
        .route("/yearly-bill", get(get_yearly_bill))
}

/// 获取记账记录列表
async fn get_account_records(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<Vec<Value>>>> {
    info!("📝 获取记账记录列表: user_id={}", auth_user.user_id);

    let records = AccountRecordRepository::list_by_user(&state.db_pool, auth_user.user_id, &params)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    let records_json: Vec<Value> = records.into_iter().map(|r| json!({
        "id": r.id,
        "record_type": r.record_type,
        "amount": r.amount,
        "category": r.category,
        "description": r.description,
        "date": r.date,
        "created_at": r.created_at,
    })).collect();

    info!("✅ 获取到 {} 条记账记录", records_json.len());

    Ok(Json(ApiResponse::success(records_json)))
}

/// 创建记账记录
async fn create_account_record(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateAccountRecordRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    info!("📝 创建记账记录: user_id={}, type={}, amount={}", 
        auth_user.user_id, req.record_type, req.amount);

    let record = AccountRecordRepository::create(
        &state.db_pool,
        auth_user.user_id,
        &req.record_type,
        req.amount,
        &req.category,
        req.description.as_deref(),
        &req.date,
    )
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    let record_json = json!({
        "id": record.id,
        "record_type": record.record_type,
        "amount": record.amount,
        "category": record.category,
        "description": record.description,
        "date": record.date,
        "created_at": record.created_at,
    });

    info!("✅ 记账记录创建成功: id={}", record.id);

    Ok(Json(ApiResponse::success(record_json)))
}

/// 获取单个记账记录
async fn get_account_record(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<Value>>> {
    info!("📝 获取记账记录详情: id={}", id);

    let record_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid record ID".to_string()))?;

    let record = AccountRecordRepository::find_by_id(&state.db_pool, record_id)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Record not found".to_string()))?;

    // 验证记录所有权
    if record.user_id != auth_user.user_id {
        return Err(AppError::Unauthorized("You can only view your own records".to_string()));
    }

    let record_json = json!({
        "id": record.id,
        "record_type": record.record_type,
        "amount": record.amount,
        "category": record.category,
        "description": record.description,
        "date": record.date,
        "created_at": record.created_at,
    });

    Ok(Json(ApiResponse::success(record_json)))
}

/// 更新记账记录
async fn update_account_record(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<CreateAccountRecordRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    info!("📝 更新记账记录: id={}", id);

    let record_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid record ID".to_string()))?;

    let record = AccountRecordRepository::update(
        &state.db_pool,
        record_id,
        auth_user.user_id,
        &req.record_type,
        req.amount,
        &req.category,
        req.description.as_deref(),
        &req.date,
    )
    .await
    .map_err(|e| {
        if e.to_string().contains("rows affected") {
            AppError::NotFound("Record not found or you don't have permission".to_string())
        } else {
            AppError::InternalServerError(format!("Database error: {}", e))
        }
    })?;

    let record_json = json!({
        "id": record.id,
        "record_type": record.record_type,
        "amount": record.amount,
        "category": record.category,
        "description": record.description,
        "date": record.date,
        "created_at": record.created_at,
    });

    info!("✅ 记账记录更新成功: id={}", record.id);

    Ok(Json(ApiResponse::success(record_json)))
}

/// 删除记账记录
async fn delete_account_record(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    info!("📝 删除记账记录: id={}", id);

    let record_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid record ID".to_string()))?;

    let deleted = AccountRecordRepository::delete(&state.db_pool, record_id, auth_user.user_id)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    if !deleted {
        return Err(AppError::NotFound("Record not found or you don't have permission".to_string()));
    }

    info!("✅ 记账记录删除成功: id={}", record_id);

    Ok(Json(ApiResponse::success(())))
}

/// 获取收支统计
async fn get_account_summary(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Value>>> {
    info!("📝 获取收支统计: user_id={}", auth_user.user_id);

    let (total_income, total_expense) = AccountRecordRepository::get_summary(&state.db_pool, auth_user.user_id)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    let summary = json!({
        "total_income": total_income,
        "total_expense": total_expense,
        "balance": total_income - total_expense,
    });

    info!("✅ 统计完成: 收入={}, 支出={}", total_income, total_expense);

    Ok(Json(ApiResponse::success(summary)))
}

/// 获取月账单
async fn get_monthly_bill(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<MonthlyBillQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    info!("📝 获取月账单: user_id={}, year={}, month={}", auth_user.user_id, query.year, query.month);

    let bill = AccountRecordRepository::get_monthly_bill(&state.db_pool, auth_user.user_id, query.year, query.month)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    info!("✅ 月账单查询完成");

    Ok(Json(ApiResponse::success(bill)))
}

/// 获取年账单
async fn get_yearly_bill(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<YearlyBillQuery>,
) -> AppResult<Json<ApiResponse<Value>>> {
    info!("📝 获取年账单: user_id={}, year={}", auth_user.user_id, query.year);

    let bill = AccountRecordRepository::get_yearly_bill(&state.db_pool, auth_user.user_id, query.year)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    info!("✅ 年账单查询完成");

    Ok(Json(ApiResponse::success(bill)))
}

