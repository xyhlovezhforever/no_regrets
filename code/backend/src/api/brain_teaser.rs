/**
 * 脑筋急转弯 API
 */

use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::ApiResponse,
    utils::error::{AppError, AppResult},
};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserBrainTeaser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub question: String,
    pub answer: String,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBrainTeaserRequest {
    pub question: String,
    pub answer: String,
    #[serde(default)]
    pub is_public: bool,
}

/// 配置脑筋急转弯路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_brain_teaser).get(get_user_brain_teasers))
        .route("/:id", delete(delete_brain_teaser))
        .route("/public", get(get_public_brain_teasers))
}

/// 创建脑筋急转弯
async fn create_brain_teaser(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateBrainTeaserRequest>,
) -> AppResult<Json<ApiResponse<UserBrainTeaser>>> {
    // 验证输入
    if req.question.trim().is_empty() {
        return Err(AppError::BadRequest("问题不能为空".to_string()));
    }
    
    if req.answer.trim().is_empty() {
        return Err(AppError::BadRequest("答案不能为空".to_string()));
    }
    
    // 插入数据库
    let brain_teaser = sqlx::query_as::<_, UserBrainTeaser>(
        r#"
        INSERT INTO user_brain_teasers (user_id, question, answer, is_public)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#
    )
    .bind(auth_user.user_id)
    .bind(req.question.trim())
    .bind(req.answer.trim())
    .bind(req.is_public)
    .fetch_one(&state.db_pool)
    .await?;
    
    Ok(Json(ApiResponse::success(brain_teaser)))
}

/// 获取用户的脑筋急转弯列表
async fn get_user_brain_teasers(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<UserBrainTeaser>>>> {
    let brain_teasers = sqlx::query_as::<_, UserBrainTeaser>(
        r#"
        SELECT * FROM user_brain_teasers
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(auth_user.user_id)
    .fetch_all(&state.db_pool)
    .await?;
    
    Ok(Json(ApiResponse::success(brain_teasers)))
}

/// 获取公开的脑筋急转弯列表（所有用户分享的）
async fn get_public_brain_teasers(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Vec<UserBrainTeaser>>>> {
    let brain_teasers = sqlx::query_as::<_, UserBrainTeaser>(
        r#"
        SELECT * FROM user_brain_teasers
        WHERE is_public = true
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.db_pool)
    .await?;
    
    Ok(Json(ApiResponse::success(brain_teasers)))
}

/// 删除脑筋急转弯
async fn delete_brain_teaser(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let brain_teaser_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid ID".to_string()))?;
    
    let result = sqlx::query!(
        "DELETE FROM user_brain_teasers WHERE id = $1 AND user_id = $2",
        brain_teaser_id,
        auth_user.user_id
    )
    .execute(&state.db_pool)
    .await?;
    
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("脑筋急转弯不存在或无权删除".to_string()));
    }
    
    Ok(Json(ApiResponse::success(())))
}
