/**
 * 好友服务
 */

use sqlx::{PgPool, Row};
use uuid::Uuid;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use chrono::Utc;

use crate::{
    model::{
        dto::PageRequest,
        entity::{FriendMessage, Friendship},
    },
    utils::error::AppError,
    repository::redis::cache_keys,
};

/// 获取好友列表
pub async fn get_friends(
    pool: &PgPool,
    redis_conn: Option<&mut ConnectionManager>,
    ws_manager: Option<&std::sync::Arc<crate::websocket::manager::ConnectionManager>>,
    user_id: Uuid,
) -> Result<Vec<serde_json::Value>, AppError> {
    // 获取好友信息，使用子查询去重
    // 查询逻辑：找到所有status='accepted'的好友关系，然后获取对方用户信息
    let friends = sqlx::query(
        r#"
        SELECT 
            u.id,
            u.username,
            u.nickname,
            u.avatar,
            COALESCE(
                (SELECT content FROM friend_messages 
                 WHERE (from_user_id = u.id AND to_user_id = $1) 
                    OR (from_user_id = $1 AND to_user_id = u.id)
                 ORDER BY created_at DESC LIMIT 1),
                ''
            ) as last_message,
            (
                SELECT created_at FROM friend_messages 
                WHERE (from_user_id = u.id AND to_user_id = $1) 
                   OR (from_user_id = $1 AND to_user_id = u.id)
                ORDER BY created_at DESC LIMIT 1
            ) as last_time,
            (
                SELECT COUNT(*) FROM friend_messages 
                WHERE from_user_id = u.id AND to_user_id = $1 AND is_read = false
            ) as unread_count
        FROM users u
        WHERE u.id IN (
            SELECT DISTINCT 
                CASE 
                    WHEN f.user_id = $1 THEN f.friend_id 
                    ELSE f.user_id 
                END as friend_id
            FROM friendships f
            WHERE (f.user_id = $1 OR f.friend_id = $1) 
                AND f.status = 'accepted'
        )
        ORDER BY (
            SELECT created_at FROM friend_messages 
            WHERE (from_user_id = u.id AND to_user_id = $1) 
               OR (from_user_id = $1 AND to_user_id = u.id)
            ORDER BY created_at DESC LIMIT 1
        ) DESC NULLS LAST
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    // 先提取所有好友数据
    let mut friend_data: Vec<(Uuid, String, String, Option<String>, String, Option<chrono::DateTime<chrono::Utc>>, i64)> = friends
        .iter()
        .map(|row| {
            let user_id = row.get::<Uuid, _>(0);
            let username = row.get::<String, _>(1);
            let nickname = row.get::<String, _>(2);
            let avatar: Option<String> = row.get(3);
            let last_message: String = row.get(4);
            let last_time: Option<chrono::DateTime<chrono::Utc>> = row.get(5);
            let unread_count: i64 = row.get(6);
            (user_id, username, nickname, avatar, last_message, last_time, unread_count)
        })
        .collect();

    // 批量检查在线状态
    // 优先使用WebSocket连接管理器（更准确），如果没有则使用Redis
    let mut online_status: std::collections::HashMap<Uuid, bool> = std::collections::HashMap::new();
    
    // 优先检查WebSocket连接管理器
    if let Some(ws_mgr) = ws_manager {
        for (friend_user_id, _, _, _, _, _, _) in &friend_data {
            // 检查WebSocket连接管理器中是否有该用户的连接
            let online = ws_mgr.get_connection(friend_user_id).await.is_some();
            online_status.insert(*friend_user_id, online);
        }
    } else if let Some(conn) = redis_conn {
        // 降级到Redis检查（基于最后活跃时间）
        for (friend_user_id, _, _, _, _, _, _) in &friend_data {
            let online_key = format!("{}online:{}", cache_keys::USER_INFO, friend_user_id);
            let online = match conn.get::<_, Option<String>>(&online_key).await {
                Ok(Some(last_active)) => {
                    if let Ok(timestamp) = last_active.parse::<i64>() {
                        if let Some(last_active_time) = chrono::DateTime::<chrono::Utc>::from_timestamp(timestamp, 0) {
                            let now = Utc::now();
                            let diff = now.signed_duration_since(last_active_time);
                            diff.num_minutes() < 5 // 5分钟内活跃认为在线
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            };
            online_status.insert(*friend_user_id, online);
        }
    }

    // 构建结果
    let result: Vec<serde_json::Value> = friend_data
        .into_iter()
        .map(|(user_id, username, nickname, avatar, last_message, last_time, unread_count)| {
            let online = online_status.get(&user_id).copied().unwrap_or(false);
            
            serde_json::json!({
                "user_id": user_id.to_string(),
                "id": user_id.to_string(), // 兼容前端
                "username": username,
                "nickname": nickname,
                "avatar": avatar,
                "last_message": last_message,
                "last_time": last_time.map(|dt| dt.to_rfc3339()),
                "unread_count": unread_count as i32,
                "online": online,
            })
        })
        .collect();

    Ok(result)
}

/// 获取好友申请列表
pub async fn get_friend_requests(pool: &PgPool, user_id: Uuid) -> Result<Vec<serde_json::Value>, AppError> {
    let requests = sqlx::query(
        r#"
        SELECT 
            f.id, 
            f.user_id, 
            u.username,
            u.nickname, 
            u.avatar, 
            f.message,
            f.status,
            f.created_at
        FROM friendships f
        INNER JOIN users u ON f.user_id = u.id
        WHERE f.friend_id = $1 AND f.status = 'pending'
        ORDER BY f.created_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    let result: Vec<serde_json::Value> = requests
        .iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<Uuid, _>(0).to_string(),
                "from_user_id": row.get::<Uuid, _>(1).to_string(),
                "from_username": row.get::<String, _>(2),
                "from_nickname": row.get::<String, _>(3),
                "from_avatar": row.get::<Option<String>, _>(4),
                "message": row.get::<Option<String>, _>(5),
                "status": row.get::<String, _>(6),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(7),
            })
        })
        .collect();

    Ok(result)
}

/// 添加好友
pub async fn add_friend(
    pool: &PgPool,
    user_id: Uuid,
    friend_id: Uuid,
    message: Option<&str>,
) -> Result<(), AppError> {
    if user_id == friend_id {
        return Err(AppError::BadRequest("Cannot add yourself as friend".to_string()));
    }

    // 检查是否已经是好友（accepted状态）
    let existing_friendship = sqlx::query(
        r#"
        SELECT id, status FROM friendships
        WHERE (user_id = $1 AND friend_id = $2) OR (user_id = $2 AND friend_id = $1)
        "#,
    )
    .bind(user_id)
    .bind(friend_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    if let Some(row) = existing_friendship {
        let status: String = row.get(1);
        
        // 如果已经是好友，不允许重复添加
        if status == "accepted" {
            return Err(AppError::BadRequest("已经是好友了".to_string()));
        }
        
        // 如果已经有pending状态的申请（可能是对方发给我，或我之前发过）
        if status == "pending" {
            // 检查申请方向：如果是我发给对方的，可以更新消息
            let my_request = sqlx::query(
                r#"
                SELECT id FROM friendships
                WHERE user_id = $1 AND friend_id = $2 AND status = 'pending'
                "#,
            )
            .bind(user_id)
            .bind(friend_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;
            
            if my_request.is_some() {
                // 更新已存在的申请消息
                sqlx::query(
                    r#"
                    UPDATE friendships
                    SET message = $1, updated_at = NOW()
                    WHERE user_id = $2 AND friend_id = $3 AND status = 'pending'
                    "#,
                )
                .bind(message)
                .bind(user_id)
                .bind(friend_id)
                .execute(pool)
                .await
                .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;
                
                return Ok(());
            } else {
                // 对方已经发给我了，提示可以接受
                return Err(AppError::BadRequest("对方已经向你发送了好友申请，请去好友申请页面处理".to_string()));
            }
        }
    }

    // 创建新的好友申请
    sqlx::query(
        r#"
        INSERT INTO friendships (user_id, friend_id, status, message, created_at, updated_at)
        VALUES ($1, $2, 'pending', $3, NOW(), NOW())
        "#,
    )
    .bind(user_id)
    .bind(friend_id)
    .bind(message)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(())
}

/// 处理好友申请
pub async fn handle_friend_request(
    pool: &PgPool,
    user_id: Uuid,
    friendship_id: Uuid,
    action: &str,
) -> Result<(), AppError> {
    let status = match action {
        "accept" => "accepted",
        "reject" => "rejected",
        _ => return Err(AppError::BadRequest("Invalid action".to_string())),
    };

    let result = sqlx::query(
        r#"
        UPDATE friendships
        SET status = $1, updated_at = NOW()
        WHERE id = $2 AND friend_id = $3 AND status = 'pending'
        "#,
    )
    .bind(status)
    .bind(friendship_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Friend request not found".to_string()));
    }

    // 接受申请时，只需要更新状态为accepted即可
    // 查询好友列表时会考虑双向关系，所以不需要创建反向记录
    Ok(())
}

/// 获取好友消息列表（未读消息）
pub async fn get_friend_messages(
    pool: &PgPool,
    user_id: Uuid,
    page: &PageRequest,
) -> Result<Vec<serde_json::Value>, AppError> {
    let messages = sqlx::query(
        r#"
        SELECT DISTINCT ON (from_user_id) 
            from_user_id, 
            content, 
            created_at,
            (SELECT nickname FROM users WHERE id = from_user_id) as nickname,
            (SELECT avatar FROM users WHERE id = from_user_id) as avatar
        FROM friend_messages
        WHERE to_user_id = $1 AND is_read = false
        ORDER BY from_user_id, created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(page.page_size)
    .bind(page.offset())
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    let result: Vec<serde_json::Value> = messages
        .iter()
        .map(|row| {
            serde_json::json!({
                "from_user_id": row.get::<Uuid, _>(0).to_string(),
                "content": row.get::<String, _>(1),
                "created_at": row.get::<chrono::DateTime<chrono::Utc>, _>(2),
                "nickname": row.get::<Option<String>, _>(3),
                "avatar": row.get::<Option<String>, _>(4),
            })
        })
        .collect();

    Ok(result)
}

/// 发送好友消息
pub async fn send_friend_message(
    pool: &PgPool,
    from_user_id: Uuid,
    to_user_id: Uuid,
    content: &str,
    message_type: &str,
) -> Result<FriendMessage, AppError> {
    use tracing::{info, error, warn};
    
    info!("💾 send_friend_message 开始: from_user_id={}, to_user_id={}, content_len={}", 
        from_user_id, to_user_id, content.len());
    
    // 验证是否为好友
    let friendship = sqlx::query(
        r#"
        SELECT id FROM friendships
        WHERE ((user_id = $1 AND friend_id = $2) OR (user_id = $2 AND friend_id = $1))
        AND status = 'accepted'
        "#,
    )
    .bind(from_user_id)
    .bind(to_user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        error!("❌ 查询好友关系失败: {}", e);
        AppError::InternalServerError(format!("Database error: {}", e))
    })?;

    if friendship.is_none() {
        warn!("⚠️ 用户 {} 和 {} 不是好友关系", from_user_id, to_user_id);
        return Err(AppError::BadRequest("Not friends".to_string()));
    }

    info!("✅ 好友关系验证通过，开始插入消息到数据库");

    let message = sqlx::query_as::<_, FriendMessage>(
        r#"
        INSERT INTO friend_messages (from_user_id, to_user_id, content, message_type, is_read, created_at)
        VALUES ($1, $2, $3, $4, false, NOW())
        RETURNING id, from_user_id, to_user_id, content, message_type, is_read, created_at
        "#,
    )
    .bind(from_user_id)
    .bind(to_user_id)
    .bind(content)
    .bind(message_type)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        error!("❌ 插入消息到数据库失败: {}", e);
        AppError::InternalServerError(format!("Database error: {}", e))
    })?;

    info!("✅ 消息已成功插入数据库: message_id={}, from_user_id={}, to_user_id={}", 
        message.id, message.from_user_id, message.to_user_id);

    Ok(message)
}

/// 获取与指定好友的聊天记录
/// 默认返回最新的50条消息（按时间倒序），支持分页加载更早的消息
pub async fn get_chat_history(
    pool: &PgPool,
    user_id: Uuid,
    friend_id: Uuid,
    page: &PageRequest,
) -> Result<Vec<FriendMessage>, AppError> {
    // 默认返回最新50条，按时间倒序（最新的在前）
    // 如果page > 1，则加载更早的消息
    let page_size = if page.page == 1 { 50 } else { page.page_size };
    
    // 如果加载第一页（最新消息），按时间倒序；否则按时间正序（从旧到新）
    let order_by = if page.page == 1 {
        "ORDER BY created_at DESC"
    } else {
        "ORDER BY created_at ASC"
    };
    
    // 计算偏移量：第一页不需要偏移，后续页面需要偏移
    let offset = if page.page == 1 {
        0
    } else {
        // 对于后续页面，需要跳过第一页的50条 + (page-2) * page_size 条
        50 + (page.page - 2) * page.page_size
    };
    
    let messages = sqlx::query_as::<_, FriendMessage>(
        &format!(
            r#"
            SELECT id, from_user_id, to_user_id, content, message_type, is_read, created_at
            FROM friend_messages
            WHERE (from_user_id = $1 AND to_user_id = $2) OR (from_user_id = $2 AND to_user_id = $1)
            {}
            LIMIT $3 OFFSET $4
            "#,
            order_by
        ),
    )
    .bind(user_id)
    .bind(friend_id)
    .bind(page_size)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    // 标记为已读（只标记最新的未读消息）
    if page.page == 1 {
        sqlx::query(
            r#"
            UPDATE friend_messages
            SET is_read = true
            WHERE from_user_id = $1 AND to_user_id = $2 AND is_read = false
            "#,
        )
        .bind(friend_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;
    }

    // 第一页返回的消息是倒序的，需要反转以便前端按时间正序显示
    let mut result = messages;
    if page.page == 1 {
        result.reverse();
    }

    Ok(result)
}

/// 标记与指定好友的所有未读消息为已读
pub async fn mark_messages_as_read(
    pool: &PgPool,
    user_id: Uuid,
    friend_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE friend_messages
        SET is_read = true
        WHERE from_user_id = $1 AND to_user_id = $2 AND is_read = false
        "#,
    )
    .bind(friend_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(())
}

