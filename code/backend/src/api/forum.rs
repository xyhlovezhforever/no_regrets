/**
 * 论坛 API
 * 统一管理灵感、话题、创作等内容
 */

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use uuid::Uuid;

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{
        dto::{CreateForumCommentRequest, CreateForumPostRequest, PageRequest},
        ApiResponse,
    },
    service::forum_service,
    utils::error::AppResult,
};

/// 配置论坛路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/posts", get(get_posts).post(create_post))
        .route("/posts/:id", get(get_post).delete(delete_post))
        .route("/posts/:id/like", post(like_post))
        .route("/posts/:id/comments", get(get_comments).post(create_comment))
        .route("/comments/:id/like", post(like_comment))
}

/// 获取帖子列表
async fn get_posts(
    State(state): State<AppState>,
    Query(params): Query<serde_json::Value>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let post_type = params
        .get("post_type")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let page = PageRequest {
        page: params.get("page").and_then(|v| v.as_i64()).unwrap_or(1),
        page_size: params.get("page_size").and_then(|v| v.as_i64()).unwrap_or(10),
    };

    let posts = forum_service::get_posts(&state.db_pool, post_type.as_deref(), &page).await?;

    // 获取用户信息（简化版，实际应该关联查询）
    let posts_with_user: Vec<serde_json::Value> = posts
        .into_iter()
        .map(|post| {
            serde_json::json!({
                "id": post.id,
                "user_id": post.user_id,
                "post_type": post.post_type,
                "title": post.title,
                "content": post.content,
                "card_category": post.card_category,
                "author": post.author,
                "topic_question": post.topic_question,
                "topic_options": post.topic_options,
                "topic_answer": post.topic_answer,
                "creation_category": post.creation_category,
                "creation_tags": post.creation_tags,
                "image_url": post.image_url,
                "likes": post.likes,
                "views": post.views,
                "comments_count": post.comments_count,
                "is_pinned": post.is_pinned,
                "created_at": post.created_at,
                "updated_at": post.updated_at,
            })
        })
        .collect();

    Ok(Json(ApiResponse::success(serde_json::Value::Array(posts_with_user))))
}

/// 创建帖子
async fn create_post(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateForumPostRequest>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let post = forum_service::create_post(&state.db_pool, auth_user.user_id, &req).await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "id": post.id,
        "post_type": post.post_type,
        "title": post.title,
        "created_at": post.created_at,
    }))))
}

/// 获取帖子详情
async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let post_id = Uuid::parse_str(&id)
        .map_err(|_| crate::utils::error::AppError::BadRequest("Invalid post id".to_string()))?;

    let post = forum_service::get_post(&state.db_pool, post_id).await?;

    Ok(Json(ApiResponse::success(serde_json::json!({
        "id": post.id,
        "user_id": post.user_id,
        "post_type": post.post_type,
        "title": post.title,
        "content": post.content,
        "card_category": post.card_category,
        "author": post.author,
        "topic_question": post.topic_question,
        "topic_options": post.topic_options,
        "topic_answer": post.topic_answer,
        "creation_category": post.creation_category,
        "creation_tags": post.creation_tags,
        "image_url": post.image_url,
        "likes": post.likes,
        "views": post.views,
        "comments_count": post.comments_count,
        "is_pinned": post.is_pinned,
        "created_at": post.created_at,
        "updated_at": post.updated_at,
    }))))
}

/// 删除帖子
async fn delete_post(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let post_id = Uuid::parse_str(&id)
        .map_err(|_| crate::utils::error::AppError::BadRequest("Invalid post id".to_string()))?;

    let deleted = forum_service::delete_post(&state.db_pool, post_id, auth_user.user_id).await?;

    if !deleted {
        return Err(crate::utils::error::AppError::Unauthorized(
            "You can only delete your own posts".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(())))
}

/// 点赞帖子
async fn like_post(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let post_id = Uuid::parse_str(&id)
        .map_err(|_| crate::utils::error::AppError::BadRequest("Invalid post id".to_string()))?;

    forum_service::like_post(&state.db_pool, post_id).await?;

    Ok(Json(ApiResponse::success(())))
}

/// 获取评论列表
async fn get_comments(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let post_id = Uuid::parse_str(&id)
        .map_err(|_| crate::utils::error::AppError::BadRequest("Invalid post id".to_string()))?;

    let comments = forum_service::get_comments(&state.db_pool, post_id, &params).await?;

    // 计算每个评论的回复数量
    use std::collections::HashMap;
    let mut replies_count_map: HashMap<Uuid, i64> = HashMap::new();
    for comment in &comments {
        if let Some(parent_id) = comment.parent_id {
            *replies_count_map.entry(parent_id).or_insert(0) += 1;
        }
    }

    // 获取用户信息
    use crate::repository::database::UserRepository;
    let mut comments_json: Vec<serde_json::Value> = Vec::new();
    
    for comment in comments {
        let replies_count = replies_count_map.get(&comment.id).copied().unwrap_or(0);
        
        let mut comment_json = serde_json::json!({
                "id": comment.id,
                "post_id": comment.post_id,
                "user_id": comment.user_id,
                "parent_id": comment.parent_id,
                "content": comment.content,
                "guest_name": comment.guest_name,
                "likes": comment.likes,
                "created_at": comment.created_at,
            "replies_count": replies_count,
        });

        // 如果有user_id，获取用户信息
        if let Some(user_id) = comment.user_id {
            if let Ok(Some(user)) = UserRepository::find_by_id(&state.db_pool, user_id).await {
                comment_json["username"] = serde_json::Value::String(user.username.clone());
                comment_json["nickname"] = serde_json::Value::String(user.nickname.clone());
                comment_json["avatar"] = user.avatar.as_ref().map(|v| serde_json::Value::String(v.clone())).unwrap_or(serde_json::Value::Null);
            }
        }

        comments_json.push(comment_json);
    }

    Ok(Json(ApiResponse::success(serde_json::Value::Array(comments_json))))
}

/// 创建评论（支持游客）
async fn create_comment(
    State(state): State<AppState>,
    auth_user: Option<AuthUser>,
    Json(req): Json<CreateForumCommentRequest>,
) -> AppResult<Json<ApiResponse<serde_json::Value>>> {
    let user_id = auth_user.map(|u| u.user_id);

    let comment = forum_service::create_comment(&state.db_pool, user_id, &req).await?;

    // 获取用户信息
    let mut comment_json = serde_json::json!({
        "id": comment.id,
        "post_id": comment.post_id,
        "user_id": comment.user_id,
        "content": comment.content,
        "guest_name": comment.guest_name,
        "created_at": comment.created_at,
    });

    if let Some(user_id) = comment.user_id {
        use crate::repository::database::UserRepository;
        if let Ok(Some(user)) = UserRepository::find_by_id(&state.db_pool, user_id).await {
            comment_json["username"] = serde_json::Value::String(user.username.clone());
            comment_json["nickname"] = serde_json::Value::String(user.nickname.clone());
            comment_json["avatar"] = user.avatar.as_ref().map(|v| serde_json::Value::String(v.clone())).unwrap_or(serde_json::Value::Null);
        }
    }

    Ok(Json(ApiResponse::success(comment_json)))
}

/// 点赞评论
async fn like_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let comment_id = Uuid::parse_str(&id)
        .map_err(|_| crate::utils::error::AppError::BadRequest("Invalid comment id".to_string()))?;

    forum_service::like_comment(&state.db_pool, comment_id).await?;

    Ok(Json(ApiResponse::success(())))
}

