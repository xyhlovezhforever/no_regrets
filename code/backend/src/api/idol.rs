/**
 * 偶像管理相关 API
 */

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{
        idol::*,
        ApiResponse
    },
    service::IdolService,
    utils::error::{AppError, AppResult},
};

/// 配置偶像路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_idols).post(create_idol))
        .route("/my", get(list_my_idols))
        .route("/:id", get(get_idol_detail).put(update_idol).delete(delete_idol))
        .route("/:id/quotes", post(add_quote))
        .route("/quotes/:quote_id", put(update_quote).delete(delete_quote))
        .route("/:id/works", post(add_work))
        .route("/works/:work_id", put(update_work).delete(delete_work))
        .route("/works/:work_id/view", post(view_work))
        .route("/works/:work_id/like", post(like_work).delete(unlike_work))
}

/// 创建偶像
async fn create_idol(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateIdolRequest>,
) -> AppResult<Json<ApiResponse<Idol>>> {
    let idol = IdolService::create_idol(&state.db_pool, auth_user.user_id, req).await?;
    Ok(Json(ApiResponse::success(idol)))
}

/// 获取偶像详情
async fn get_idol_detail(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ApiResponse<IdolDetail>>> {
    let detail = IdolService::get_idol_detail(&state.db_pool, id).await?;
    Ok(Json(ApiResponse::success(detail)))
}

/// 更新偶像
async fn update_idol(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateIdolRequest>,
) -> AppResult<Json<ApiResponse<Idol>>> {
    let idol = IdolService::update_idol(&state.db_pool, id, auth_user.user_id, req).await?;
    Ok(Json(ApiResponse::success(idol)))
}

/// 删除偶像
async fn delete_idol(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> AppResult<Json<ApiResponse<()>>> {
    IdolService::delete_idol(&state.db_pool, id, auth_user.user_id).await?;
    Ok(Json(ApiResponse::success(())))
}

/// 分页查询偶像列表
async fn list_idols(
    State(state): State<AppState>,
    Query(params): Query<IdolQueryParams>,
) -> AppResult<Json<ApiResponse<PaginatedIdols>>> {
    let result = IdolService::list_idols(&state.db_pool, None, params).await?;
    Ok(Json(ApiResponse::success(result)))
}

/// 获取我的偶像列表
async fn list_my_idols(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<IdolQueryParams>,
) -> AppResult<Json<ApiResponse<PaginatedIdols>>> {
    let result = IdolService::list_idols(&state.db_pool, Some(auth_user.user_id), params).await?;
    Ok(Json(ApiResponse::success(result)))
}

// ==================== 语录相关 ====================

/// 添加语录
async fn add_quote(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(idol_id): Path<Uuid>,
    Json(req): Json<CreateQuoteRequest>,
) -> AppResult<Json<ApiResponse<IdolQuote>>> {
    let quote = IdolService::add_quote(&state.db_pool, idol_id, auth_user.user_id, req).await?;
    Ok(Json(ApiResponse::success(quote)))
}

/// 更新语录
async fn update_quote(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(quote_id): Path<Uuid>,
    Json(req): Json<UpdateQuoteRequest>,
) -> AppResult<Json<ApiResponse<IdolQuote>>> {
    let quote = IdolService::update_quote(&state.db_pool, quote_id, auth_user.user_id, req).await?;
    Ok(Json(ApiResponse::success(quote)))
}

/// 删除语录
async fn delete_quote(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(quote_id): Path<Uuid>,
) -> AppResult<Json<ApiResponse<()>>> {
    IdolService::delete_quote(&state.db_pool, quote_id, auth_user.user_id).await?;
    Ok(Json(ApiResponse::success(())))
}

// ==================== 作品相关 ====================

/// 添加作品
async fn add_work(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(idol_id): Path<Uuid>,
    Json(req): Json<CreateWorkRequest>,
) -> AppResult<Json<ApiResponse<IdolWork>>> {
    let work = IdolService::add_work(&state.db_pool, idol_id, auth_user.user_id, req).await?;
    Ok(Json(ApiResponse::success(work)))
}

/// 更新作品
async fn update_work(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(work_id): Path<Uuid>,
    Json(req): Json<UpdateWorkRequest>,
) -> AppResult<Json<ApiResponse<IdolWork>>> {
    let work = IdolService::update_work(&state.db_pool, work_id, auth_user.user_id, req).await?;
    Ok(Json(ApiResponse::success(work)))
}

/// 删除作品
async fn delete_work(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(work_id): Path<Uuid>,
) -> AppResult<Json<ApiResponse<()>>> {
    IdolService::delete_work(&state.db_pool, work_id, auth_user.user_id).await?;
    Ok(Json(ApiResponse::success(())))
}

/// 浏览作品（增加浏览量）
async fn view_work(
    State(state): State<AppState>,
    Path(work_id): Path<Uuid>,
) -> AppResult<Json<ApiResponse<IdolWork>>> {
    let work = IdolService::view_work(&state.db_pool, work_id).await?;
    Ok(Json(ApiResponse::success(work)))
}

/// 点赞作品
async fn like_work(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(work_id): Path<Uuid>,
) -> AppResult<Json<ApiResponse<bool>>> {
    let liked = IdolService::like_work(&state.db_pool, work_id, auth_user.user_id).await?;
    Ok(Json(ApiResponse::success(liked)))
}

/// 取消点赞作品
async fn unlike_work(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(work_id): Path<Uuid>,
) -> AppResult<Json<ApiResponse<bool>>> {
    let unliked = IdolService::unlike_work(&state.db_pool, work_id, auth_user.user_id).await?;
    Ok(Json(ApiResponse::success(unliked)))
}

