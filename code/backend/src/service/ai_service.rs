/**
 * AI 服务
 * 处理 AI 相关业务逻辑
 */

use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    config::Config,
    model::{
        dto::{CreateChatSessionRequest, CreateCustomAIRequest, PageRequest, SendChatMessageRequest},
        entity::{ChatMessage, ChatSession, CustomAI},
    },
    repository,
    utils::error::AppError,
};
use redis::aio::ConnectionManager;

/// 获取聊天会话列表
pub async fn get_chat_sessions(
    pool: &PgPool,
    user_id: Uuid,
    page: &PageRequest,
) -> Result<Vec<ChatSession>, AppError> {
    let sessions = sqlx::query_as::<_, ChatSession>(
        r#"
        SELECT id, user_id, title, created_at, updated_at
        FROM chat_sessions
        WHERE user_id = $1
        ORDER BY updated_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(page.page_size)
    .bind(page.offset())
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(sessions)
}

/// 创建聊天会话
pub async fn create_chat_session(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateChatSessionRequest,
) -> Result<ChatSession, AppError> {
    let session = sqlx::query_as::<_, ChatSession>(
        r#"
        INSERT INTO chat_sessions (user_id, title, created_at, updated_at)
        VALUES ($1, $2, NOW(), NOW())
        RETURNING id, user_id, title, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(&req.title)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(session)
}

/// 获取聊天会话详情
pub async fn get_chat_session(
    pool: &PgPool,
    user_id: Uuid,
    session_id: Uuid,
) -> Result<ChatSession, AppError> {
    let session = sqlx::query_as::<_, ChatSession>(
        r#"
        SELECT id, user_id, title, created_at, updated_at
        FROM chat_sessions
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(session_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
    .ok_or_else(|| AppError::NotFound("Chat session not found".to_string()))?;

    Ok(session)
}

/// 删除聊天会话
pub async fn delete_chat_session(
    pool: &PgPool,
    user_id: Uuid,
    session_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM chat_sessions
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(session_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Chat session not found".to_string()));
    }

    Ok(())
}

/// 获取聊天消息列表
pub async fn get_chat_messages(
    pool: &PgPool,
    user_id: Uuid,
    session_id: Uuid,
    page: &PageRequest,
) -> Result<Vec<ChatMessage>, AppError> {
    // 先验证会话属于该用户
    let _session = get_chat_session(pool, user_id, session_id).await?;

    let messages = sqlx::query_as::<_, ChatMessage>(
        r#"
        SELECT id, session_id, role, content, created_at
        FROM chat_messages
        WHERE session_id = $1
        ORDER BY created_at ASC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(session_id)
    .bind(page.page_size)
    .bind(page.offset())
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(messages)
}

/// 发送聊天消息
pub async fn send_chat_message(
    pool: &PgPool,
    redis_conn: &mut ConnectionManager,
    config: &Config,
    user_id: Uuid,
    session_id: Uuid,
    req: &SendChatMessageRequest,
) -> Result<ChatMessage, AppError> {
    // 验证会话
    let _session = get_chat_session(pool, user_id, session_id).await?;

    // 保存用户消息
    let user_message = sqlx::query_as::<_, ChatMessage>(
        r#"
        INSERT INTO chat_messages (session_id, role, content, created_at)
        VALUES ($1, 'user', $2, NOW())
        RETURNING id, session_id, role, content, created_at
        "#,
    )
    .bind(session_id)
    .bind(&req.content)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    // 获取历史消息（用于上下文）
    let history = sqlx::query_as::<_, ChatMessage>(
        r#"
        SELECT id, session_id, role, content, created_at
        FROM chat_messages
        WHERE session_id = $1
        ORDER BY created_at DESC
        LIMIT 10
        "#,
    )
    .bind(session_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    // 构建消息历史（反转顺序）
    let mut messages = history.into_iter().rev().collect::<Vec<_>>();
    messages.push(user_message.clone());

    // 转换为 JSON 格式
    let messages_json: Vec<serde_json::Value> = messages
        .iter()
        .map(|m| {
            json!({
                "role": m.role,
                "content": m.content
            })
        })
        .collect();

    // 调用 AI 服务生成回复
    let ai_reply = if config.ai_api_key.is_empty() {
        // 如果没有配置 AI API，使用简单的关键词回复
        generate_simple_reply(&req.content)
    } else {
        AiService::send_message(config, messages_json).await
            .unwrap_or_else(|_| generate_simple_reply(&req.content))
    };

    // 保存 AI 回复
    let ai_message = sqlx::query_as::<_, ChatMessage>(
        r#"
        INSERT INTO chat_messages (session_id, role, content, created_at)
        VALUES ($1, 'assistant', $2, NOW())
        RETURNING id, session_id, role, content, created_at
        "#,
    )
    .bind(session_id)
    .bind(&ai_reply)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    // 更新会话更新时间
    sqlx::query(
        r#"
        UPDATE chat_sessions
        SET updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(session_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(ai_message)
}

/// 简单的关键词回复（当没有 AI API 时使用）
fn generate_simple_reply(content: &str) -> String {
    let content_lower = content.to_lowercase();
    
    if content_lower.contains("你好") || content_lower.contains("hello") {
        "你好！很高兴和你聊天，有什么我可以帮助你的吗？".to_string()
    } else if content_lower.contains("谢谢") || content_lower.contains("thank") {
        "不客气！能帮到你我也很开心。".to_string()
    } else if content_lower.contains("再见") || content_lower.contains("bye") {
        "再见！期待下次和你聊天。".to_string()
    } else if content_lower.contains("心情") || content_lower.contains("难过") || content_lower.contains("不开心") {
        "我理解你的感受。每个人都会有低谷期，但请相信，一切都会好起来的。你可以和我分享更多，我会一直在这里陪伴你。".to_string()
    } else if content_lower.contains("开心") || content_lower.contains("高兴") {
        "太好了！看到你开心我也很开心。继续保持这份好心情吧！".to_string()
    } else {
        "我明白了。能告诉我更多细节吗？我会认真倾听的。".to_string()
    }
}

/// 获取专属AI列表
pub async fn get_custom_ais(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<CustomAI>, AppError> {
    let custom_ais = sqlx::query_as::<_, CustomAI>(
        r#"
        SELECT id, user_id, name, avatar, personality, style, background, nickname, catchphrase, created_at, updated_at
        FROM custom_ais
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(custom_ais)
}

/// 创建专属AI
pub async fn create_custom_ai(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateCustomAIRequest,
) -> Result<CustomAI, AppError> {
    let custom_ai = sqlx::query_as::<_, CustomAI>(
        r#"
        INSERT INTO custom_ais (user_id, name, avatar, personality, style, background, nickname, catchphrase, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
        RETURNING id, user_id, name, avatar, personality, style, background, nickname, catchphrase, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(&req.name)
    .bind(&req.avatar)
    .bind(&req.personality)
    .bind(&req.style)
    .bind(&req.background)
    .bind(&req.nickname)
    .bind(&req.catchphrase)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(custom_ai)
}

/// 获取专属AI详情
pub async fn get_custom_ai(
    pool: &PgPool,
    user_id: Uuid,
    custom_ai_id: Uuid,
) -> Result<CustomAI, AppError> {
    let custom_ai = sqlx::query_as::<_, CustomAI>(
        r#"
        SELECT id, user_id, name, avatar, personality, style, background, nickname, catchphrase, created_at, updated_at
        FROM custom_ais
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(custom_ai_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
    .ok_or_else(|| AppError::NotFound("Custom AI not found".to_string()))?;

    Ok(custom_ai)
}

/// 更新专属AI
pub async fn update_custom_ai(
    pool: &PgPool,
    user_id: Uuid,
    custom_ai_id: Uuid,
    req: &CreateCustomAIRequest,
) -> Result<CustomAI, AppError> {
    let custom_ai = sqlx::query_as::<_, CustomAI>(
        r#"
        UPDATE custom_ais
        SET name = $1, avatar = $2, personality = $3, style = $4, background = $5, nickname = $6, catchphrase = $7, updated_at = NOW()
        WHERE id = $8 AND user_id = $9
        RETURNING id, user_id, name, avatar, personality, style, background, nickname, catchphrase, created_at, updated_at
        "#,
    )
    .bind(&req.name)
    .bind(&req.avatar)
    .bind(&req.personality)
    .bind(&req.style)
    .bind(&req.background)
    .bind(&req.nickname)
    .bind(&req.catchphrase)
    .bind(custom_ai_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
    .ok_or_else(|| AppError::NotFound("Custom AI not found".to_string()))?;

    Ok(custom_ai)
}

/// 删除专属AI
pub async fn delete_custom_ai(
    pool: &PgPool,
    user_id: Uuid,
    custom_ai_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM custom_ais
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(custom_ai_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Custom AI not found".to_string()));
    }

    Ok(())
}

/// AI 服务结构体（保留原有方法）
pub struct AiService;

impl AiService {
    /// 发送聊天消息到 AI
    pub async fn send_message(config: &Config, messages: Vec<serde_json::Value>) -> Result<String> {
        let client = reqwest::Client::new();

        let response = client
            .post(format!("{}/chat/completions", config.ai_api_url))
            .header("Authorization", format!("Bearer {}", config.ai_api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": config.ai_model,
                "messages": messages,
                "temperature": 0.7,
                "max_tokens": 1000,
            }))
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;

        // 提取 AI 回复
        let reply = result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("抱歉，我现在无法回复")
            .to_string();

        Ok(reply)
    }

    /// 生成情书
    pub async fn generate_love_letter(
        config: &Config,
        to_name: &str,
        from_name: &str,
        style: Option<&str>,
    ) -> Result<String> {
        let style_text = style.unwrap_or("温馨浪漫");
        let prompt = format!(
            "请为{}写一封给{}的情书，风格：{}，字数在300-500字之间。",
            from_name, to_name, style_text
        );

        let messages = vec![json!({
            "role": "user",
            "content": prompt,
        })];

        Self::send_message(config, messages).await
    }

    /// 生成亲子话语
    pub async fn generate_family_message(
        config: &Config,
        relation: &str, // parent_to_child, child_to_parent
        content: &str,
    ) -> Result<String> {
        let prompt = match relation {
            "parent_to_child" => format!("作为一位父母，请帮我优化和润色这段想对孩子说的话：{}", content),
            "child_to_parent" => format!("作为一个孩子，请帮我优化和润色这段想对父母说的话：{}", content),
            _ => return Err(anyhow::anyhow!("不支持的关系类型")),
        };

        let messages = vec![json!({
            "role": "user",
            "content": prompt,
        })];

        Self::send_message(config, messages).await
    }
}
