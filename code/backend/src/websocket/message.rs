/**
 * WebSocket 消息定义
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// WebSocket 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    /// 心跳消息
    #[serde(rename = "ping")]
    Ping,
    
    /// 心跳响应
    #[serde(rename = "pong")]
    Pong,
    
    /// 认证消息
    #[serde(rename = "auth")]
    Auth {
        token: String,
    },
    
    /// 认证成功
    #[serde(rename = "auth_success")]
    AuthSuccess {
        user_id: String,
    },
    
    /// 认证失败
    #[serde(rename = "auth_error")]
    AuthError {
        message: String,
    },
    
    /// 发送消息
    #[serde(rename = "send_message")]
    SendMessage {
        to_user_id: String,
        content: String,
        message_type: Option<String>,
    },
    
    /// 接收消息
    #[serde(rename = "message")]
    Message {
        id: String,
        from_user_id: String,
        to_user_id: String,
        content: String,
        message_type: String,
        created_at: String,
    },
    
    /// 错误消息
    #[serde(rename = "error")]
    Error {
        message: String,
    },
    
    /// 系统消息
    #[serde(rename = "system")]
    System {
        message: String,
    },
}

impl WsMessage {
    /// 将消息序列化为 JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    /// 从 JSON 反序列化消息
    pub fn from_json(data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }
}

