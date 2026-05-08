/**
 * 内容服务
 * 处理鼓励卡片、哲学命题等公共内容
 */

use sqlx::PgPool;

use crate::{
    model::entity::{EncourageCard, PhilosophyCard},
    utils::error::AppError,
};

/// 获取随机鼓励卡片
pub async fn get_random_encourage_card(pool: &PgPool) -> Result<EncourageCard, AppError> {
    let card = sqlx::query_as::<_, EncourageCard>(
        r#"
        SELECT id, title, content, category, image_url, created_at
        FROM encourage_cards
        ORDER BY RANDOM()
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
    .ok_or_else(|| AppError::NotFound("No encourage card found".to_string()))?;

    Ok(card)
}

/// 获取鼓励卡片列表
pub async fn get_encourage_cards(pool: &PgPool) -> Result<Vec<EncourageCard>, AppError> {
    let cards = sqlx::query_as::<_, EncourageCard>(
        r#"
        SELECT id, title, content, category, image_url, created_at
        FROM encourage_cards
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(cards)
}

/// 获取随机哲学命题
pub async fn get_random_philosophy_card(pool: &PgPool) -> Result<PhilosophyCard, AppError> {
    let card = sqlx::query_as::<_, PhilosophyCard>(
        r#"
        SELECT id, title, content, author, image_url, created_at
        FROM philosophy_cards
        ORDER BY RANDOM()
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
    .ok_or_else(|| AppError::NotFound("No philosophy card found".to_string()))?;

    Ok(card)
}

/// 获取哲学命题列表
pub async fn get_philosophy_cards(pool: &PgPool) -> Result<Vec<PhilosophyCard>, AppError> {
    let cards = sqlx::query_as::<_, PhilosophyCard>(
        r#"
        SELECT id, title, content, author, image_url, created_at
        FROM philosophy_cards
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(cards)
}
