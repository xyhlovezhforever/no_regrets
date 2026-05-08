/**
 * AI 聊天相关 API
 */

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{dto::{CreateChatSessionRequest, CreateCustomAIRequest, PageRequest, SendChatMessageRequest}, ApiResponse},
    service::ai_service,
    utils::error::{AppError, AppResult},
};

/// 配置 AI 相关路由
pub fn routes() -> Router<AppState> {
    Router::new()
        // 聊天会话
        .route("/sessions", get(get_chat_sessions).post(create_chat_session))
        .route("/sessions/:id", get(get_chat_session).delete(delete_chat_session))
        .route("/sessions/:id/messages", get(get_chat_messages).post(send_chat_message))
        // 专属AI
        .route("/custom-ais", get(get_custom_ais).post(create_custom_ai))
        .route("/custom-ais/:id", get(get_custom_ai).put(update_custom_ai).delete(delete_custom_ai))
}

/// 获取聊天会话列表
#[utoipa::path(
    get,
    path = "/api/v1/chat/sessions",
    params(
        ("page" = Option<i64>, Query, description = "页码"),
        ("page_size" = Option<i64>, Query, description = "每页数量"),
    ),
    responses(
        (status = 200, description = "成功", body = Value),
        (status = 401, description = "未授权"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn get_chat_sessions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let sessions = ai_service::get_chat_sessions(&state.db_pool, auth_user.user_id, &params).await?;
    
    Ok(Json(ApiResponse::success(json!(sessions))))
}

/// 创建聊天会话
#[utoipa::path(
    post,
    path = "/api/v1/chat/sessions",
    request_body = CreateChatSessionRequest,
    responses(
        (status = 200, description = "成功", body = Value),
        (status = 401, description = "未授权"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn create_chat_session(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateChatSessionRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let session = ai_service::create_chat_session(&state.db_pool, auth_user.user_id, &req).await?;
    
    Ok(Json(ApiResponse::success(json!(session))))
}

/// 获取聊天会话详情
#[utoipa::path(
    get,
    path = "/api/v1/chat/sessions/{id}",
    params(
        ("id" = String, Path, description = "会话ID"),
    ),
    responses(
        (status = 200, description = "成功", body = Value),
        (status = 404, description = "未找到"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn get_chat_session(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let session_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid session ID".to_string()))?;
    
    let session = ai_service::get_chat_session(&state.db_pool, auth_user.user_id, session_id).await?;
    
    Ok(Json(ApiResponse::success(json!(session))))
}

/// 删除聊天会话
#[utoipa::path(
    delete,
    path = "/api/v1/chat/sessions/{id}",
    params(
        ("id" = String, Path, description = "会话ID"),
    ),
    responses(
        (status = 200, description = "成功"),
        (status = 404, description = "未找到"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn delete_chat_session(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let session_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid session ID".to_string()))?;
    
    ai_service::delete_chat_session(&state.db_pool, auth_user.user_id, session_id).await?;
    
    Ok(Json(ApiResponse::success(())))
}

/// 获取聊天消息列表
#[utoipa::path(
    get,
    path = "/api/v1/chat/sessions/{id}/messages",
    params(
        ("id" = String, Path, description = "会话ID"),
        ("page" = Option<i64>, Query, description = "页码"),
        ("page_size" = Option<i64>, Query, description = "每页数量"),
    ),
    responses(
        (status = 200, description = "成功", body = Value),
        (status = 404, description = "未找到"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn get_chat_messages(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Query(params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let session_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid session ID".to_string()))?;
    
    let messages = ai_service::get_chat_messages(&state.db_pool, auth_user.user_id, session_id, &params).await?;
    
    Ok(Json(ApiResponse::success(json!(messages))))
}

/// 发送聊天消息
#[utoipa::path(
    post,
    path = "/api/v1/chat/sessions/{id}/messages",
    params(
        ("id" = String, Path, description = "会话ID"),
    ),
    request_body = SendChatMessageRequest,
    responses(
        (status = 200, description = "成功", body = Value),
        (status = 404, description = "未找到"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn send_chat_message(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<SendChatMessageRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let session_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid session ID".to_string()))?;
    
    let mut redis_conn = crate::repository::get_redis_conn(&state.redis_pool).await
        .map_err(|e| AppError::InternalServerError(format!("Redis连接失败: {}", e)))?;
    let message = ai_service::send_chat_message(
        &state.db_pool,
        &mut redis_conn,
        &state.config,
        auth_user.user_id,
        session_id,
        &req,
    ).await?;
    
    Ok(Json(ApiResponse::success(json!(message))))
}

/// 获取专属AI列表
#[utoipa::path(
    get,
    path = "/api/v1/chat/custom-ais",
    responses(
        (status = 200, description = "成功", body = Value),
        (status = 401, description = "未授权"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn get_custom_ais(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Value>>> {
    let custom_ais = ai_service::get_custom_ais(&state.db_pool, auth_user.user_id).await?;
    
    Ok(Json(ApiResponse::success(json!(custom_ais))))
}

/// 创建专属AI
#[utoipa::path(
    post,
    path = "/api/v1/chat/custom-ais",
    request_body = CreateCustomAIRequest,
    responses(
        (status = 200, description = "成功", body = Value),
        (status = 401, description = "未授权"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn create_custom_ai(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateCustomAIRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let custom_ai = ai_service::create_custom_ai(&state.db_pool, auth_user.user_id, &req).await?;
    
    Ok(Json(ApiResponse::success(json!(custom_ai))))
}

/// 获取专属AI详情
#[utoipa::path(
    get,
    path = "/api/v1/chat/custom-ais/{id}",
    params(
        ("id" = String, Path, description = "专属AI ID"),
    ),
    responses(
        (status = 200, description = "成功", body = Value),
        (status = 404, description = "未找到"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn get_custom_ai(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let custom_ai_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid custom AI ID".to_string()))?;
    
    let custom_ai = ai_service::get_custom_ai(&state.db_pool, auth_user.user_id, custom_ai_id).await?;
    
    Ok(Json(ApiResponse::success(json!(custom_ai))))
}

/// 更新专属AI
#[utoipa::path(
    put,
    path = "/api/v1/chat/custom-ais/{id}",
    params(
        ("id" = String, Path, description = "专属AI ID"),
    ),
    request_body = CreateCustomAIRequest,
    responses(
        (status = 200, description = "成功", body = Value),
        (status = 404, description = "未找到"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn update_custom_ai(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<CreateCustomAIRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let custom_ai_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid custom AI ID".to_string()))?;
    
    let custom_ai = ai_service::update_custom_ai(&state.db_pool, auth_user.user_id, custom_ai_id, &req).await?;
    
    Ok(Json(ApiResponse::success(json!(custom_ai))))
}

/// 删除专属AI
#[utoipa::path(
    delete,
    path = "/api/v1/chat/custom-ais/{id}",
    params(
        ("id" = String, Path, description = "专属AI ID"),
    ),
    responses(
        (status = 200, description = "成功"),
        (status = 404, description = "未找到"),
    ),
    tag = "ai",
    security(("bearer" = []))
)]
async fn delete_custom_ai(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let custom_ai_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid custom AI ID".to_string()))?;
    
    ai_service::delete_custom_ai(&state.db_pool, auth_user.user_id, custom_ai_id).await?;
    
    Ok(Json(ApiResponse::success(())))
}

