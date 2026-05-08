/**
 * 好友管理相关 API
 */

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::{json, Value};
use tracing::{info, error};
use uuid::Uuid;

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{dto::{AddFriendRequest, HandleFriendRequest, PageRequest, SendFriendMessageRequest}, ApiResponse},
    service::friend_service,
    utils::error::{AppError, AppResult},
};

/// 配置好友相关路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_friends))
        .route("/requests", get(get_friend_requests).post(add_friend))
        .route("/requests/:id", put(handle_friend_request))
        .route("/messages", get(get_friend_messages).post(send_friend_message))
        .route("/messages/:friend_id", get(get_chat_history))
        .route("/messages/:friend_id/read", put(mark_messages_as_read))
}

/// 获取好友列表
async fn get_friends(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Value>>> {
    // 获取Redis连接用于检查在线状态（降级方案）
    let mut redis_conn = crate::repository::get_redis_conn(&state.redis_pool).await
        .map_err(|e| AppError::InternalServerError(format!("Redis连接失败: {}", e)))?;
    
    let friends = friend_service::get_friends(
        &state.db_pool, 
        Some(&mut redis_conn), 
        Some(&state.ws_manager),
        auth_user.user_id
    ).await?;
    
    Ok(Json(ApiResponse::success(Value::Array(friends))))
}

/// 获取好友申请列表
async fn get_friend_requests(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Value>>> {
    info!("📝 获取好友申请列表: user_id={}", auth_user.user_id);
    
    let requests = friend_service::get_friend_requests(&state.db_pool, auth_user.user_id).await?;
    
    info!("✅ 找到 {} 个好友申请", requests.len());
    
    Ok(Json(ApiResponse::success(Value::Array(requests))))
}

/// 添加好友
async fn add_friend(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<AddFriendRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    info!("📝 添加好友请求: from={}, to={}, message={:?}", 
        auth_user.user_id, req.friend_id, req.message);
    
    let friend_id = Uuid::parse_str(&req.friend_id)
        .map_err(|_| AppError::BadRequest("Invalid friend ID".to_string()))?;
    
    friend_service::add_friend(&state.db_pool, auth_user.user_id, friend_id, req.message.as_deref()).await?;
    
    info!("✅ 好友申请已发送: from={} to={}", auth_user.user_id, friend_id);
    
    Ok(Json(ApiResponse::success(())))
}

/// 处理好友申请
async fn handle_friend_request(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<HandleFriendRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    info!("📝 处理好友申请: friendship_id={}, user_id={}, action={}", 
        id, auth_user.user_id, req.action);
    
    let friendship_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid friendship ID".to_string()))?;
    
    friend_service::handle_friend_request(&state.db_pool, auth_user.user_id, friendship_id, &req.action).await?;
    
    info!("✅ 好友申请已处理: friendship_id={}, action={}", friendship_id, req.action);
    
    Ok(Json(ApiResponse::success(())))
}

/// 获取好友消息列表
async fn get_friend_messages(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let messages = friend_service::get_friend_messages(&state.db_pool, auth_user.user_id, &params).await?;
    
    Ok(Json(ApiResponse::success(Value::Array(messages))))
}

/// 发送好友消息
async fn send_friend_message(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<SendFriendMessageRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let to_user_id = Uuid::parse_str(&req.to_user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;
    
    // 保存消息到数据库
    let message = friend_service::send_friend_message(
        &state.db_pool,
        auth_user.user_id,
        to_user_id,
        &req.content,
        req.message_type.as_deref().unwrap_or("text"),
    ).await?;
    
    // 通过WebSocket推送消息
    use crate::websocket::message::WsMessage;
    let ws_message = WsMessage::Message {
        id: message.id.to_string(),
        from_user_id: message.from_user_id.to_string(),
        to_user_id: message.to_user_id.to_string(),
        content: message.content.clone(),
        message_type: message.message_type.clone(),
        created_at: message.created_at.to_rfc3339(),
    };
    
    // 发送给接收者
    if let Err(e) = state.ws_manager.send_to_user(&to_user_id, ws_message.clone()).await {
        info!("用户 {} 不在线，消息已保存到数据库: {}", to_user_id, e);
    }
    
    // 也发送给发送者（确认消息已发送）
    let _ = state.ws_manager.send_to_user(&auth_user.user_id, ws_message).await;
    
    Ok(Json(ApiResponse::success(json!(message))))
}

/// 获取与指定好友的聊天记录
async fn get_chat_history(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(friend_id): Path<String>,
    Query(params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let friend_id = Uuid::parse_str(&friend_id)
        .map_err(|_| AppError::BadRequest("Invalid friend ID".to_string()))?;
    
    let messages = friend_service::get_chat_history(&state.db_pool, auth_user.user_id, friend_id, &params).await?;
    
    Ok(Json(ApiResponse::success(json!(messages))))
}

/// 标记与指定好友的所有未读消息为已读
async fn mark_messages_as_read(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(friend_id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let friend_id = Uuid::parse_str(&friend_id)
        .map_err(|_| AppError::BadRequest("Invalid friend ID".to_string()))?;
    
    friend_service::mark_messages_as_read(&state.db_pool, auth_user.user_id, friend_id).await?;
    
    Ok(Json(ApiResponse::success(())))
}

