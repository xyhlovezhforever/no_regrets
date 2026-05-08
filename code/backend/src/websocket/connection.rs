/**
 * WebSocket 连接管理
 */

use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::websocket::message::WsMessage;
use crate::websocket::manager::ConnectionManager;

/// WebSocket 连接
#[derive(Clone)]
pub struct WsConnection {
    pub user_id: Uuid,
    pub sender: mpsc::UnboundedSender<Message>,
    pub manager: Arc<ConnectionManager>,
}

impl WsConnection {
    /// 创建新连接
    pub fn new(
        user_id: Uuid,
        sender: mpsc::UnboundedSender<Message>,
        manager: Arc<ConnectionManager>,
    ) -> Self {
        Self {
            user_id,
            sender,
            manager,
        }
    }
    
    /// 发送消息
    pub fn send(&self, message: WsMessage) -> Result<(), String> {
        let json = message.to_json().map_err(|e| format!("序列化失败: {}", e))?;
        self.sender
            .send(Message::Text(json))
            .map_err(|_| "发送失败：连接已关闭".to_string())
    }
    
    /// 发送文本消息
    pub fn send_text(&self, text: String) -> Result<(), String> {
        self.sender
            .send(Message::Text(text))
            .map_err(|_| "发送失败：连接已关闭".to_string())
    }
}

/// 处理 WebSocket 连接
pub async fn handle_socket(
    socket: WebSocket,
    manager: Arc<ConnectionManager>,
) {
    let (mut sender, mut receiver) = socket.split();
    
    // 创建消息通道
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    
    // 启动发送任务
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });
    
    // 用户ID（初始为None，等待认证）
    let mut user_id: Option<Uuid> = None;
    let mut authenticated = false;
    
    // 接收消息循环
    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match WsMessage::from_json(&text) {
                        Ok(ws_msg) => {
                            match ws_msg {
                                WsMessage::Ping => {
                                    // 心跳响应
                                    let pong = WsMessage::Pong;
                                    if let Ok(json) = pong.to_json() {
                                        let _ = tx.send(Message::Text(json));
                                    }
                                }
                                WsMessage::Auth { token } => {
                                    // 认证
                                    if authenticated {
                                        continue;
                                    }
                                    
                                    // 验证 token
                                    match manager.authenticate(&token).await {
                                        Ok(uid) => {
                                            user_id = Some(uid);
                                            authenticated = true;
                                            
                                            // 注册连接（必须在发送消息之前）
                                            let connection = WsConnection::new(uid, tx.clone(), manager.clone());
                                            manager.add_connection(uid, connection.clone()).await;
                                            
                                            info!("✅ WebSocket 认证成功并已注册连接: user_id={}", uid);
                                            
                                            // 发送认证成功消息
                                            let auth_success = WsMessage::AuthSuccess {
                                                user_id: uid.to_string(),
                                            };
                                            if let Err(e) = connection.send(auth_success) {
                                                warn!("发送认证成功消息失败: {}", e);
                                            }
                                        }
                                        Err(e) => {
                                            let auth_error = WsMessage::AuthError {
                                                message: e,
                                            };
                                            if let Ok(json) = auth_error.to_json() {
                                                let _ = tx.send(Message::Text(json));
                                            }
                                            warn!("WebSocket 认证失败");
                                        }
                                    }
                                }
                                WsMessage::SendMessage { to_user_id, content, message_type } => {
                                    if !authenticated {
                                        warn!("⚠️ 收到未认证的消息发送请求");
                                        let error = WsMessage::Error {
                                            message: "未认证".to_string(),
                                        };
                                        if let Ok(json) = error.to_json() {
                                            let _ = tx.send(Message::Text(json));
                                        }
                                        continue;
                                    }
                                    
                                    if let Some(uid) = user_id {
                                        info!("📤 收到WebSocket发送消息请求: from={}, to={}, content_len={}", 
                                            uid, to_user_id, content.len());
                                        
                                        // 处理发送消息
                                        match manager.handle_send_message(
                                            uid,
                                            &to_user_id,
                                            &content,
                                            message_type.as_deref().unwrap_or("text"),
                                        ).await {
                                            Ok(_) => {
                                                info!("✅ WebSocket消息处理成功: from={}, to={}", uid, to_user_id);
                                            }
                                            Err(e) => {
                                                error!("❌ WebSocket消息处理失败: from={}, to={}, error={}", uid, to_user_id, e);
                                                let error = WsMessage::Error {
                                                    message: e,
                                                };
                                                if let Ok(json) = error.to_json() {
                                                    let _ = tx.send(Message::Text(json));
                                                }
                                            }
                                        }
                                    } else {
                                        warn!("⚠️ 收到消息发送请求但user_id为None");
                                    }
                                }
                                _ => {
                                    warn!("收到未知消息类型");
                                }
                            }
                        }
                        Err(e) => {
                            error!("解析消息失败: {}", e);
                            let error = WsMessage::Error {
                                message: format!("消息格式错误: {}", e),
                            };
                            if let Ok(json) = error.to_json() {
                                let _ = tx.send(Message::Text(json));
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    break;
                }
                Ok(Message::Ping(data)) => {
                    // 自动响应 Ping
                    let _ = tx.send(Message::Pong(data));
                }
                Ok(Message::Pong(_)) => {
                    // 忽略 Pong
                }
                Ok(Message::Binary(_)) => {
                    warn!("收到二进制消息，暂不支持");
                }
                Err(e) => {
                    error!("WebSocket 错误: {}", e);
                    break;
                }
            }
        }
        
        // 清理连接
        if let Some(uid) = user_id {
            info!("🔌 WebSocket 连接断开，清理连接: user_id={}", uid);
            manager.remove_connection(uid).await;
        }
    });
    
    // 等待任一任务完成
    tokio::select! {
        _ = (&mut send_task) => {
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            send_task.abort();
        }
    }
}

