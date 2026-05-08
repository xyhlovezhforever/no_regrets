/**
 * 绑定用户数据访问服务
 * 根据权限查看绑定用户的应用数据
 */

use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::utils::error::{AppError, AppResult};

/// 检查用户是否有权限访问绑定用户的数据
pub async fn check_permission(
    pool: &PgPool,
    viewer_id: Uuid,      // 查看者ID（被授权者）
    data_owner_id: Uuid,  // 数据拥有者ID（授权者）
    app_type: &str,       // 应用类型
    required_level: &str, // 需要的权限级别：read 或 write
) -> AppResult<bool> {
    info!("🔍 检查权限: viewer={}, owner={}, app={}, level={}", 
        viewer_id, data_owner_id, app_type, required_level);

    // 查询权限
    // 权限表中：user_id = 数据拥有者（授权者），bound_user_id = 查看者（被授权者）
    let permission: Option<(String,)> = sqlx::query_as(
        "SELECT permission_level FROM app_permissions 
         WHERE user_id = $1 AND bound_user_id = $2 AND app_type = $3"
    )
    .bind(data_owner_id)  // 数据拥有者（授权者）
    .bind(viewer_id)      // 查看者（被授权者）
    .bind(app_type)
    .fetch_optional(pool)
    .await?;

    if let Some((level,)) = permission {
        let has_permission = match required_level {
            "read" => level == "read" || level == "write",
            "write" => level == "write",
            _ => false,
        };
        
        info!("✅ 权限检查结果: {} (权限级别: {})", has_permission, level);
        Ok(has_permission)
    } else {
        info!("❌ 未找到权限记录: user_id={}, bound_user_id={}, app_type={}", 
            data_owner_id, viewer_id, app_type);
        Ok(false)
    }
}

/// 便签数据 VO
#[derive(Debug, Serialize, Deserialize)]
pub struct BoundNoteVo {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub is_pinned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 记账数据 VO
#[derive(Debug, Serialize, Deserialize)]
pub struct BoundAccountVo {
    pub id: String,
    pub user_id: String,
    pub amount: f64,
    pub category: String,
    pub description: Option<String>,
    pub date: DateTime<Utc>,
    pub type_: String, // income 或 expense
    pub created_at: DateTime<Utc>,
}

/// 获取绑定用户的便签数据
pub async fn get_bound_user_notes(
    pool: &PgPool,
    viewer_id: Uuid,
    data_owner_id: Uuid,
) -> AppResult<Vec<BoundNoteVo>> {
    info!("📝 获取绑定用户便签: viewer={}, owner={}", viewer_id, data_owner_id);

    // 检查权限
    if !check_permission(pool, viewer_id, data_owner_id, "note", "read").await? {
        return Err(AppError::Forbidden("没有权限查看该用户的便签".to_string()));
    }

    // 查询便签数据
    let notes: Vec<(Uuid, Uuid, String, String, Option<String>, Vec<String>, bool, DateTime<Utc>, DateTime<Utc>)> = sqlx::query_as(
        "SELECT id, user_id, title, content, category, tags, is_pinned, created_at, updated_at 
         FROM notes 
         WHERE user_id = $1 AND deleted_at IS NULL
         ORDER BY is_pinned DESC, updated_at DESC
         LIMIT 100"
    )
    .bind(data_owner_id)
    .fetch_all(pool)
    .await?;

    let result: Vec<BoundNoteVo> = notes.into_iter().map(|(id, user_id, title, content, category, tags, is_pinned, created_at, updated_at)| {
        BoundNoteVo {
            id: id.to_string(),
            user_id: user_id.to_string(),
            title,
            content,
            category,
            tags,
            is_pinned,
            created_at,
            updated_at,
        }
    }).collect();

    info!("✅ 找到 {} 条便签", result.len());
    Ok(result)
}

/// 获取绑定用户的记账数据
pub async fn get_bound_user_accounts(
    pool: &PgPool,
    viewer_id: Uuid,
    data_owner_id: Uuid,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> AppResult<Vec<BoundAccountVo>> {
    info!("📝 获取绑定用户记账: viewer={}, owner={}", viewer_id, data_owner_id);

    // 检查权限
    if !check_permission(pool, viewer_id, data_owner_id, "account", "read").await? {
        return Err(AppError::Forbidden("没有权限查看该用户的记账数据".to_string()));
    }

    // 构建查询
    let mut query = String::from(
        "SELECT id, user_id, amount, category, description, date, record_type, created_at 
         FROM account_records 
         WHERE user_id = $1"
    );

    let mut bind_count = 2;
    if start_date.is_some() {
        query.push_str(&format!(" AND date >= ${}", bind_count));
        bind_count += 1;
    }
    if end_date.is_some() {
        query.push_str(&format!(" AND date <= ${}", bind_count));
    }

    query.push_str(" ORDER BY date DESC LIMIT 100");

    // 执行查询
    let mut query_builder = sqlx::query_as::<_, (Uuid, Uuid, f64, String, Option<String>, DateTime<Utc>, String, DateTime<Utc>)>(&query)
        .bind(data_owner_id);

    if let Some(start) = start_date {
        query_builder = query_builder.bind(start);
    }
    if let Some(end) = end_date {
        query_builder = query_builder.bind(end);
    }

    let accounts = query_builder.fetch_all(pool).await?;

    let result: Vec<BoundAccountVo> = accounts.into_iter().map(|(id, user_id, amount, category, description, date, type_, created_at)| {
        BoundAccountVo {
            id: id.to_string(),
            user_id: user_id.to_string(),
            amount,
            category,
            description,
            date,
            type_,
            created_at,
        }
    }).collect();

    info!("✅ 找到 {} 条记账记录", result.len());
    Ok(result)
}

/// 记账统计数据
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountStatistics {
    pub total_income: f64,
    pub total_expense: f64,
    pub balance: f64,
    pub record_count: i64,
}

/// 获取绑定用户的记账统计
pub async fn get_bound_user_account_statistics(
    pool: &PgPool,
    viewer_id: Uuid,
    data_owner_id: Uuid,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> AppResult<AccountStatistics> {
    info!("📊 获取绑定用户记账统计: viewer={}, owner={}", viewer_id, data_owner_id);

    // 检查权限
    if !check_permission(pool, viewer_id, data_owner_id, "account", "read").await? {
        return Err(AppError::Forbidden("没有权限查看该用户的记账数据".to_string()));
    }

    // 构建查询
    let mut where_clause = String::from("WHERE user_id = $1");
    let mut bind_count = 2;
    
    if start_date.is_some() {
        where_clause.push_str(&format!(" AND date >= ${}", bind_count));
        bind_count += 1;
    }
    if end_date.is_some() {
        where_clause.push_str(&format!(" AND date <= ${}", bind_count));
    }

    let query = format!(
        "SELECT 
            COALESCE(SUM(CASE WHEN record_type = 'income' THEN amount ELSE 0 END), 0) as total_income,
            COALESCE(SUM(CASE WHEN record_type = 'expense' THEN amount ELSE 0 END), 0) as total_expense,
            COUNT(*) as record_count
         FROM account_records 
         {}",
        where_clause
    );

    let mut query_builder = sqlx::query_as::<_, (f64, f64, i64)>(&query)
        .bind(data_owner_id);

    if let Some(start) = start_date {
        query_builder = query_builder.bind(start);
    }
    if let Some(end) = end_date {
        query_builder = query_builder.bind(end);
    }

    let (total_income, total_expense, record_count) = query_builder
        .fetch_one(pool)
        .await?;

    let balance = total_income - total_expense;

    info!("✅ 统计完成: 收入={}, 支出={}, 余额={}", total_income, total_expense, balance);

    Ok(AccountStatistics {
        total_income,
        total_expense,
        balance,
        record_count,
    })
}
