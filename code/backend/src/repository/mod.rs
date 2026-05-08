/**
 * Repository 层（数据访问层）
 * 负责与数据库、缓存等数据源交互
 */

pub mod database;
pub mod redis;
pub mod idol_repository;

use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::{info, error};

/// 初始化数据库连接池
pub async fn init_db_pool(database_url: &str, max_connections: u32) -> Result<PgPool> {
    // 隐藏密码的数据库URL（用于日志）
    let safe_url = database_url
        .split('@')
        .nth(1)
        .map(|s| format!("***@{}", s))
        .unwrap_or_else(|| "***".to_string());
    
    info!("Initializing database connection pool to {} (max_connections: {})", safe_url, max_connections);
    
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(60)) // 等待从池中获取连接的超时时间（60秒）
        .idle_timeout(Some(Duration::from_secs(600))) // 空闲连接超时时间（10分钟）
        .max_lifetime(Some(Duration::from_secs(1800))) // 连接最大生存时间（30分钟）
        .test_before_acquire(true) // 在获取连接前测试连接是否有效
        .connect(database_url)
        .await
        .map_err(|e| {
            error!("Failed to connect to database: {}", e);
            error!("Database URL: {}", safe_url);
            error!("Please check:");
            error!("  1. Database server is running");
            error!("  2. DATABASE_URL is correct");
            error!("  3. Network connectivity");
            error!("  4. Firewall settings");
            anyhow::anyhow!("Database connection failed: {}. Please check your DATABASE_URL and ensure PostgreSQL is running.", e)
        })?;
    
    info!("Database connection pool initialized successfully");
    Ok(pool)
}

use std::sync::Arc;

// Redis 连接池类型（使用Arc共享客户端）
pub type RedisPool = Arc<::redis::Client>;

/// 初始化 Redis 连接池
pub async fn init_redis_pool(redis_url: &str) -> Result<RedisPool> {
    // 创建 Redis 客户端
    let client = ::redis::Client::open(redis_url)?;
    
    Ok(Arc::new(client))
}

/// 从RedisPool获取连接
pub async fn get_redis_conn(pool: &RedisPool) -> Result<::redis::aio::ConnectionManager> {
    let manager = ::redis::aio::ConnectionManager::new((**pool).clone()).await?;
    Ok(manager)
}

// 重新导出
pub use database::*;
pub use redis::*;
pub use idol_repository::*;

