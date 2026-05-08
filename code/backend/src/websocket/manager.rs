/**
 * WebSocket 连接管理器
 * 管理所有活跃的 WebSocket 连接
 */

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    config::Config,
    model::entity::FriendMessage,
    repository::RedisPool,
    service::AuthService,
    websocket::connection::WsConnection,
    websocket::message::WsMessage,
};
use crate::service::friend_service;
use sqlx::PgPool;

/// 连接管理器
#[derive(Clone)]
pub struct ConnectionManager {
    /// 活跃连接：user_id -> connection
    connections: Arc<RwLock<HashMap<Uuid, WsConnection>>>,
    /// 数据库连接池
    db_pool: PgPool,
    /// Redis 连接池
    redis_pool: RedisPool,
    /// 配置
    config: Config,
}

impl ConnectionManager {
    /// 创建新的连接管理器
    pub fn new(db_pool: PgPool, redis_pool: RedisPool, config: Config) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            db_pool,
            redis_pool,
            config,
        }
    }
    
    /// 认证用户
    pub async fn authenticate(&self, token: &str) -> Result<Uuid, String> {
        // 提取 Bearer token
        let token = token.strip_prefix("Bearer ").unwrap_or(token);
        
        // 验证 token
        let user_id = AuthService::extract_user_id(token, &self.config)
            .map_err(|_| "无效的 token".to_string())?;
        
        // 检查 token 是否在 Redis 中存在
        let mut redis_conn = crate::repository::get_redis_conn(&self.redis_pool).await
            .map_err(|e| format!("Redis连接失败: {}", e))?;
        
        let token_valid = AuthService::verify_token_in_redis(&mut redis_conn, &user_id.to_string(), token).await
            .map_err(|e| format!("Token验证失败: {}", e))?;
        
        if !token_valid {
            return Err("Token已失效".to_string());
        }
        
        Ok(user_id)
    }
    
    /// 添加连接
    pub async fn add_connection(&self, user_id: Uuid, connection: WsConnection) {
        let mut connections = self.connections.write().await;
        connections.insert(user_id, connection);
        info!("WebSocket 连接已添加: user_id={}, 当前连接数: {}", user_id, connections.len());
    }
    
    /// 移除连接
    pub async fn remove_connection(&self, user_id: Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(&user_id);
        info!("WebSocket 连接已移除: user_id={}, 当前连接数: {}", user_id, connections.len());
    }
    
    /// 获取连接
    pub async fn get_connection(&self, user_id: &Uuid) -> Option<WsConnection> {
        let connections = self.connections.read().await;
        connections.get(user_id).cloned()
    }
    
    /// 发送消息给指定用户
    pub async fn send_to_user(&self, user_id: &Uuid, message: WsMessage) -> Result<(), String> {
        // 先检查连接是否存在
        let connection = self.get_connection(user_id).await;
        
        if let Some(conn) = connection {
            // 连接存在，尝试发送
            conn.send(message).map_err(|e| {
                error!("发送消息给用户 {} 失败: {}", user_id, e);
                // 发送失败，可能是连接已断开，移除连接
                let manager = conn.manager.clone();
                let uid = *user_id;
                tokio::spawn(async move {
                    manager.remove_connection(uid).await;
                });
                e
            })
        } else {
            // 连接不存在，记录调试信息
            let connections = self.connections.read().await;
            let online_users: Vec<String> = connections.keys().map(|k| k.to_string()).collect();
            info!("🔍 用户 {} 未找到WebSocket连接。当前在线用户: {:?}", user_id, online_users);
            Err(format!("用户 {} 未连接WebSocket", user_id))
        }
    }
    
    /// 广播消息给多个用户
    pub async fn broadcast(&self, user_ids: &[Uuid], message: WsMessage) {
        for user_id in user_ids {
            if let Err(e) = self.send_to_user(user_id, message.clone()).await {
                error!("发送消息给用户 {} 失败: {}", user_id, e);
            }
        }
    }
    
    /// 处理发送消息
    pub async fn handle_send_message(
        &self,
        from_user_id: Uuid,
        to_user_id_str: &str,
        content: &str,
        message_type: &str,
    ) -> Result<(), String> {
        info!("📝 开始处理WebSocket消息: from_user_id={}, to_user_id={}, content={}", 
            from_user_id, to_user_id_str, content);
        
        // 解析目标用户ID
        let to_user_id = Uuid::parse_str(to_user_id_str)
            .map_err(|e| {
                error!("❌ 解析用户ID失败: {} -> {}", to_user_id_str, e);
                format!("无效的用户ID: {}", e)
            })?;
        
        info!("✅ 用户ID解析成功: to_user_id={}", to_user_id);
        
        // 保存消息到数据库
        info!("💾 开始保存消息到数据库...");
        let message = match friend_service::send_friend_message(
            &self.db_pool,
            from_user_id,
            to_user_id,
            content,
            message_type,
        ).await {
            Ok(msg) => {
                info!("✅ 消息已成功保存到数据库: message_id={}", msg.id);
                msg
            }
            Err(e) => {
                error!("❌ 保存消息到数据库失败: {}", e);
                return Err(format!("保存消息失败: {}", e));
            }
        };
        
        // 构建 WebSocket 消息
        let ws_message = WsMessage::Message {
            id: message.id.to_string(),
            from_user_id: message.from_user_id.to_string(),
            to_user_id: message.to_user_id.to_string(),
            content: message.content,
            message_type: message.message_type,
            created_at: message.created_at.to_rfc3339(),
        };
        
        // 发送给接收者
        match self.send_to_user(&to_user_id, ws_message.clone()).await {
            Ok(_) => {
                info!("✅ 消息已通过WebSocket推送给接收者: to_user_id={}", to_user_id);
            }
            Err(e) => {
                // 如果用户不在线，消息已保存到数据库，稍后可以拉取
                // 这是正常情况，不应该作为错误处理
                info!("ℹ️ 接收者 {} 未连接WebSocket，消息已保存到数据库，稍后可拉取", to_user_id);
            }
        }
        
        // 也发送给发送者（确认消息已发送）
        match self.send_to_user(&from_user_id, ws_message).await {
            Ok(_) => {
                info!("✅ 消息确认已通过WebSocket发送给发送者: from_user_id={}", from_user_id);
            }
            Err(e) => {
                // 发送者可能没有连接WebSocket，使用HTTP API发送，这是正常的
                info!("ℹ️ 发送者 {} 未连接WebSocket，消息已保存，可通过HTTP拉取", from_user_id);
            }
        }
        
        Ok(())
    }
    
    /// 获取在线用户数
    pub async fn get_online_count(&self) -> usize {
        let connections = self.connections.read().await;
        connections.len()
    }
}

