/**
 * API 路由层
 * 定义所有 HTTP 路由和处理器
 */

pub mod ai;
pub mod auth;
pub mod binding;
pub mod bound_data;
pub mod brain_teaser;
pub mod encourage;
pub mod forum;
pub mod friend;
pub mod game;
pub mod health;
pub mod idol;
pub mod love_letter;
pub mod watermark;
pub mod note;
pub mod philosophy;
pub mod private;
pub mod release;
pub mod self_management;
pub mod upload;
pub mod user;
pub mod warmth;

use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{config::Config, repository::RedisPool, websocket::handler::websocket_handler, websocket::manager::ConnectionManager};

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_pool: RedisPool,
    pub config: Config,
    pub ws_manager: Arc<ConnectionManager>,
}

/// 配置所有路由
pub fn routes(state: AppState) -> Router {
    Router::new()
        // 健康检查
        .route("/health", get(health::health_check))
        // WebSocket 路由
        .route("/ws", get(websocket_handler))
        // 认证路由
        .nest("/auth", auth::routes())
        // 用户路由
        .nest("/user", user::routes())
        // AI 路由
        .nest("/chat", ai::routes())
        // 好友路由
        .nest("/friends", friend::routes())
        // 账号绑定路由
        .nest("/bindings", binding::routes())
        // 绑定用户数据访问路由
        .nest("/bound-data", bound_data::routes())
        // 偶像路由
        .nest("/idols", idol::routes())
        // 笔记路由
        .nest("/notes", note::routes())
        // 情书路由
        .nest("/love-letters", love_letter::routes())
        // 脑筋急转弯路由
        .nest("/brain-teasers", brain_teaser::routes())
        // 游戏路由
        .nest("/games", game::routes())
        // 水印去除路由
        .nest("/watermark", watermark::routes())
        // 上传路由
        .nest("/upload", upload::routes())
        // 记账路由
        .nest("/accounts", self_management::account_routes())
        // 鼓励卡片路由
        .nest("/encourage", encourage::routes())
        // 哲学命题路由
        .nest("/philosophy", philosophy::routes())
        // 温暖板块路由
        .nest("/warmth", warmth::routes())
        // 写作交流路由
        .nest("/writing", release::routes())
        // 论坛路由（统一管理灵感、话题、创作）
        .nest("/forum", forum::routes())
        // 私人空间路由
        .nest("/private", private::routes())
        // 设置应用状态
        .with_state(state)
}

