/**
 * 健康检查 API
 */

use axum::{extract::State, Json};
use serde_json::{json, Value};
use utoipa::path;

use super::AppState;

/// 健康检查
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Service is healthy", body = Value)
    ),
    tag = "health"
)]
pub async fn health_check(State(state): State<AppState>) -> Json<Value> {
    // 检查数据库连接
    let db_status = match sqlx::query("SELECT 1").fetch_one(&state.db_pool).await {
        Ok(_) => "ok",
        Err(_) => "error",
    };

    // 检查 Redis 连接
    let redis_status = {
        match crate::repository::get_redis_conn(&state.redis_pool).await {
            Ok(mut conn) => {
                match redis::cmd("PING").query_async::<_, String>(&mut conn).await {
                    Ok(_) => "ok",
                    Err(_) => "error",
                }
            }
            Err(_) => "error",
        }
    };

    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "services": {
            "database": db_status,
            "redis": redis_status,
        }
    }))
}

