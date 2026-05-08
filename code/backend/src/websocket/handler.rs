/**
 * WebSocket 处理器
 */

use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::Response,
};
use tracing::info;

use crate::api::AppState;
use crate::websocket::connection::handle_socket;

/// WebSocket 升级处理器
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    info!("收到 WebSocket 连接请求");
    
    // 使用全局连接管理器
    let manager = state.ws_manager.clone();
    
    ws.on_upgrade(|socket| handle_socket(socket, manager))
}

