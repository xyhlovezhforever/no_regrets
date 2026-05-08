/**
 * 论坛服务
 * 处理论坛相关的业务逻辑
 */

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    model::{
        dto::{CreateForumCommentRequest, CreateForumPostRequest, PageRequest},
        entity::{ForumComment, ForumPost},
    },
    repository::database::{ForumCommentRepository, ForumPostRepository},
    utils::error::AppError,
};

/// 创建论坛帖子
pub async fn create_post(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateForumPostRequest,
) -> Result<ForumPost, AppError> {
    let post = ForumPostRepository::create(
        pool,
        user_id,
        &req.post_type,
        &req.title,
        &req.content,
        req.card_category.as_deref(),
        req.author.as_deref(),
        req.topic_question.as_deref(),
        req.topic_options.as_ref().map(|v| v.as_slice()),
        req.topic_answer.as_deref(),
        req.creation_category.as_deref(),
        req.creation_tags.as_ref().map(|v| v.as_slice()),
        req.image_url.as_deref(),
    )
    .await
    .map_err(|e| AppError::InternalServerError(format!("Failed to create post: {}", e)))?;

    Ok(post)
}

/// 获取帖子列表
pub async fn get_posts(
    pool: &PgPool,
    post_type: Option<&str>,
    page: &PageRequest,
) -> Result<Vec<ForumPost>, AppError> {
    let posts = ForumPostRepository::list(pool, post_type, page)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to get posts: {}", e)))?;

    Ok(posts)
}

/// 获取帖子详情
pub async fn get_post(pool: &PgPool, post_id: Uuid) -> Result<ForumPost, AppError> {
    let post = ForumPostRepository::find_by_id(pool, post_id)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to get post: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    // 增加浏览量
    ForumPostRepository::increment_views(pool, post_id)
        .await
        .ok(); // 忽略错误，不影响主流程

    Ok(post)
}

/// 点赞帖子
pub async fn like_post(pool: &PgPool, post_id: Uuid) -> Result<(), AppError> {
    ForumPostRepository::increment_likes(pool, post_id)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to like post: {}", e)))?;

    Ok(())
}

/// 删除帖子
pub async fn delete_post(pool: &PgPool, post_id: Uuid, user_id: Uuid) -> Result<bool, AppError> {
    let deleted = ForumPostRepository::delete(pool, post_id, user_id)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to delete post: {}", e)))?;

    Ok(deleted)
}

/// 创建评论
pub async fn create_comment(
    pool: &PgPool,
    user_id: Option<Uuid>,
    req: &CreateForumCommentRequest,
) -> Result<ForumComment, AppError> {
    let post_id = Uuid::parse_str(&req.post_id)
        .map_err(|_| AppError::BadRequest("Invalid post_id".to_string()))?;

    let parent_id = req
        .parent_id
        .as_ref()
        .map(|id| Uuid::parse_str(id))
        .transpose()
        .map_err(|_| AppError::BadRequest("Invalid parent_id".to_string()))?;

    let comment = ForumCommentRepository::create(
        pool,
        post_id,
        user_id,
        parent_id,
        &req.content,
        req.guest_name.as_deref(),
    )
    .await
    .map_err(|e| AppError::InternalServerError(format!("Failed to create comment: {}", e)))?;

    Ok(comment)
}

/// 获取评论列表
pub async fn get_comments(
    pool: &PgPool,
    post_id: Uuid,
    page: &PageRequest,
) -> Result<Vec<ForumComment>, AppError> {
    let comments = ForumCommentRepository::list_by_post(pool, post_id, page)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to get comments: {}", e)))?;

    Ok(comments)
}

/// 点赞评论
pub async fn like_comment(pool: &PgPool, comment_id: Uuid) -> Result<(), AppError> {
    ForumCommentRepository::increment_likes(pool, comment_id)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Failed to like comment: {}", e)))?;

    Ok(())
}

