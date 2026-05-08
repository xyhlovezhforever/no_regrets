/**
 * 主入口文件
 * 初始化应用、路由、中间件等
 */

mod api;
mod config;
mod middleware;
mod model;
mod repository;
mod service;
mod utils;
mod websocket;

use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Method},
    Router,
};
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, TraceLayer},
    timeout::TimeoutLayer,
};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志 - 更详细的输出
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .with_ansi(true)
        .init();

    // 加载配置
    let config = Config::load()?;
    info!("Configuration loaded successfully");

    // 初始化数据库连接池
    info!("Connecting to database...");
    let db_pool = repository::init_db_pool(&config.database_url, config.database_max_connections)
        .await
        .map_err(|e| {
            tracing::error!("Failed to initialize database connection pool: {}", e);
            e
        })?;
    info!("Database connection pool initialized with {} max connections", config.database_max_connections);

    // 运行数据库迁移
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .map_err(|e| anyhow::anyhow!("Migration failed: {}", e))?;
    info!("Database migrations completed");

    // 初始化 Redis 连接池
    let redis_pool = repository::init_redis_pool(&config.redis_url).await?;
    info!("Redis connection pool initialized");

    // 创建 WebSocket 连接管理器
    let ws_manager = std::sync::Arc::new(websocket::manager::ConnectionManager::new(
        db_pool.clone(),
        redis_pool.clone(),
        config.clone(),
    ));
    info!("WebSocket connection manager initialized");

    // 创建应用状态
    let app_state = api::AppState {
        db_pool,
        redis_pool,
        config: config.clone(),
        ws_manager,
    };

    // 配置 CORS
    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors_origins
                .iter()
                .map(|origin| origin.parse::<HeaderValue>().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE, 
            header::AUTHORIZATION,
            header::RANGE,
            header::ACCEPT_RANGES,
        ])
        .expose_headers([
            header::CONTENT_LENGTH,
            header::CONTENT_RANGE,
            header::ACCEPT_RANGES,
        ])
        .allow_credentials(true);

    // OpenAPI 文档
    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::health::health_check,
        ),
        tags(
            (name = "health", description = "Health check endpoints"),
            (name = "auth", description = "Authentication endpoints"),
            (name = "user", description = "User management endpoints"),
            (name = "ai", description = "AI chat endpoints"),
            (name = "warmth", description = "Warmth content endpoints"),
            (name = "encourage", description = "Encourage card endpoints"),
            (name = "philosophy", description = "Philosophy card endpoints"),
            (name = "idol", description = "Idol information endpoints"),
            (name = "release", description = "Release content endpoints"),
            (name = "self", description = "Self management endpoints"),
        )
    )]
    struct ApiDoc;

    // 确保uploads目录存在
    tokio::fs::create_dir_all("uploads/images").await?;
    tokio::fs::create_dir_all("uploads/videos").await?;
    tokio::fs::create_dir_all("uploads/audios").await?;
    info!("Upload directories created/verified");

    // 构建应用路由
    let app = Router::new()
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // API 路由
        .nest("/api/v1", api::routes(app_state))
        // 静态文件服务（上传的文件）
        .nest_service(
            "/uploads",
            tower_http::services::ServeDir::new("uploads")
                .append_index_html_on_directories(false)
                .precompressed_gzip()
                .precompressed_br()
        )
        // 中间件
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(120))) // 120秒超时
        .layer(DefaultBodyLimit::max(100 * 1024 * 1024)); // 100MB 文件上传限制

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], config.app_port));
    info!("Server starting on http://{}", addr);
    info!("Swagger UI available at http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

