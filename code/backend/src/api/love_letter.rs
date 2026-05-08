/**
 * 情书相关 API
 */

use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{dto::CreateLoveLetterRequest, entity::LoveLetter, ApiResponse},
    utils::error::{AppError, AppResult},
};

/// 配置情书路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_love_letter))
        .route("/received", get(get_received_letters))
        .route("/sent", get(get_sent_letters))
        .route("/:id", get(get_letter_detail).delete(delete_letter))
        .route("/:id/read", put(mark_as_read))
}

/// 创建情书
async fn create_love_letter(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<Value>,
) -> AppResult<Json<ApiResponse<LoveLetter>>> {
    // 解析请求参数
    let to_user_id_str = req.get("to_user_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("Missing to_user_id".to_string()))?;
    
    let to_user_id = Uuid::parse_str(to_user_id_str)
        .map_err(|_| AppError::BadRequest("Invalid to_user_id".to_string()))?;
    
    let title = req.get("title")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("Missing title".to_string()))?;
    
    let content = req.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("Missing content".to_string()))?;
    
    // 获取收信人和寄信人姓名
    let to_user = sqlx::query!(
        "SELECT username, nickname FROM users WHERE id = $1",
        to_user_id
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    
    let from_user = sqlx::query!(
        "SELECT username, nickname FROM users WHERE id = $1",
        auth_user.user_id
    )
    .fetch_one(&state.db_pool)
    .await?;
    
    let to_name = if !to_user.nickname.is_empty() {
        &to_user.nickname
    } else {
        &to_user.username
    };
    
    let from_name = if !from_user.nickname.is_empty() {
        &from_user.nickname
    } else {
        &from_user.username
    };
    
    // 插入情书记录
    let letter = sqlx::query_as::<_, LoveLetter>(
        r#"
        INSERT INTO love_letters (user_id, title, content, to_name, from_name, to_user_id, is_sent)
        VALUES ($1, $2, $3, $4, $5, $6, true)
        RETURNING *
        "#
    )
    .bind(auth_user.user_id)
    .bind(title)
    .bind(content)
    .bind(to_name)
    .bind(from_name)
    .bind(to_user_id)
    .fetch_one(&state.db_pool)
    .await?;
    
    Ok(Json(ApiResponse::success(letter)))
}

/// 获取收到的情书列表
async fn get_received_letters(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<Value>>>> {
    // 查询收到的情书
    let letters = sqlx::query!(
        r#"
        SELECT 
            l.id,
            l.title,
            l.content,
            l.to_name,
            l.from_name,
            COALESCE(l.is_read, false) as "is_read!",
            l.created_at,
            l.user_id as from_user_id,
            l.to_user_id
        FROM love_letters l
        WHERE l.to_user_id = $1
        ORDER BY l.created_at DESC
        "#,
        auth_user.user_id
    )
    .fetch_all(&state.db_pool)
    .await?;
    
    let result: Vec<Value> = letters
        .into_iter()
        .map(|l| {
            json!({
                "id": l.id,
                "title": l.title,
                "content": l.content,
                "to_name": l.to_name,
                "from_name": l.from_name,
                "to_user_id": l.to_user_id,
                "from_user_id": l.from_user_id,
                "is_read": l.is_read,
                "created_at": l.created_at
            })
        })
        .collect();
    
    Ok(Json(ApiResponse::success(result)))
}

/// 获取发送的情书列表
async fn get_sent_letters(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<LoveLetter>>>> {
    let letters = sqlx::query_as::<_, LoveLetter>(
        "SELECT * FROM love_letters WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(auth_user.user_id)
    .fetch_all(&state.db_pool)
    .await?;
    
    Ok(Json(ApiResponse::success(letters)))
}

/// 获取情书详情
async fn get_letter_detail(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let letter_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid letter ID".to_string()))?;
    
    let letter = sqlx::query!(
        r#"
        SELECT 
            l.id,
            l.title,
            l.content,
            l.to_name,
            l.from_name,
            COALESCE(l.is_read, false) as "is_read!",
            l.created_at,
            l.user_id as from_user_id,
            l.to_user_id
        FROM love_letters l
        WHERE l.id = $1
        "#,
        letter_id
    )
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Letter not found".to_string()))?;
    
    let result = json!({
        "id": letter.id,
        "title": letter.title,
        "content": letter.content,
        "to_name": letter.to_name,
        "from_name": letter.from_name,
        "to_user_id": letter.to_user_id,
        "from_user_id": letter.from_user_id,
        "is_read": letter.is_read,
        "created_at": letter.created_at
    });
    
    Ok(Json(ApiResponse::success(result)))
}

/// 标记情书为已读
async fn mark_as_read(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let letter_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid letter ID".to_string()))?;
    
    // 只有收信人才能标记为已读
    sqlx::query!(
        "UPDATE love_letters SET is_read = true WHERE id = $1 AND to_user_id = $2",
        letter_id,
        auth_user.user_id
    )
    .execute(&state.db_pool)
    .await?;
    
    Ok(Json(ApiResponse::success(())))
}

/// 删除情书
async fn delete_letter(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let letter_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid letter ID".to_string()))?;
    
    sqlx::query!(
        "DELETE FROM love_letters WHERE id = $1 AND user_id = $2",
        letter_id,
        auth_user.user_id
    )
    .execute(&state.db_pool)
    .await?;
    
    Ok(Json(ApiResponse::success(())))
}
