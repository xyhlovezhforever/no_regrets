/**
 * 账号绑定服务
 * 处理用户账号绑定和应用数据访问权限
 */

use chrono::Utc;
use sqlx::{PgPool, Row};
use tracing::info;
use uuid::Uuid;

use crate::model::{
    entity::{AccountBinding, AppPermission, User},
    vo::{AccountBindingVo, AppPermissionVo, BoundUserDetailVo, UserVo},
};
use crate::utils::error::{AppError, AppResult};

/// 发送账号绑定请求
pub async fn send_binding_request(
    pool: &PgPool,
    user_id: Uuid,
    bound_user_id: Uuid,
    message: Option<&str>,
) -> AppResult<AccountBinding> {
    info!("📝 发送账号绑定请求: from={}, to={}", user_id, bound_user_id);

    // 检查是否是自己
    if user_id == bound_user_id {
        return Err(AppError::BadRequest("不能绑定自己的账号".to_string()));
    }

    // 检查目标用户是否存在
    let target_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(bound_user_id)
        .fetch_optional(pool)
        .await?;

    if target_user.is_none() {
        return Err(AppError::NotFound("目标用户不存在".to_string()));
    }

    // 检查是否已经存在绑定关系（任意方向）
    let existing = sqlx::query_as::<_, AccountBinding>(
        "SELECT * FROM account_bindings 
         WHERE (user_id = $1 AND bound_user_id = $2) 
            OR (user_id = $2 AND bound_user_id = $1)"
    )
    .bind(user_id)
    .bind(bound_user_id)
    .fetch_optional(pool)
    .await?;

    if let Some(binding) = existing {
        // 如果是 pending 或 accepted 状态，不允许重复发送
        if binding.status == "pending" {
            return Err(AppError::BadRequest("已有待处理的绑定请求".to_string()));
        } else if binding.status == "accepted" {
            return Err(AppError::BadRequest("已存在绑定关系".to_string()));
        }
        // 如果是 rejected 状态，删除旧记录，允许重新发送
        else if binding.status == "rejected" {
            info!("删除已拒绝的绑定记录: {}", binding.id);
            sqlx::query("DELETE FROM account_bindings WHERE id = $1")
                .bind(binding.id)
                .execute(pool)
                .await?;
        }
    }

    // 创建绑定请求
    let binding = sqlx::query_as::<_, AccountBinding>(
        "INSERT INTO account_bindings (user_id, bound_user_id, status, initiator_id, message)
         VALUES ($1, $2, 'pending', $1, $3)
         RETURNING *"
    )
    .bind(user_id)
    .bind(bound_user_id)
    .bind(message)
    .fetch_one(pool)
    .await?;

    info!("✅ 绑定请求已发送: binding_id={}", binding.id);

    Ok(binding)
}

/// 处理账号绑定请求
pub async fn handle_binding_request(
    pool: &PgPool,
    user_id: Uuid,
    binding_id: Uuid,
    action: &str,
) -> AppResult<()> {
    info!("📝 处理绑定请求: user_id={}, binding_id={}, action={}", user_id, binding_id, action);

    // 验证操作类型
    if action != "accept" && action != "reject" {
        return Err(AppError::BadRequest("无效的操作类型".to_string()));
    }

    // 获取绑定请求
    let binding = sqlx::query_as::<_, AccountBinding>(
        "SELECT * FROM account_bindings WHERE id = $1"
    )
    .bind(binding_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("绑定请求不存在".to_string()))?;

    // 验证是否是接收方
    if binding.bound_user_id != user_id {
        return Err(AppError::Forbidden("只有接收方可以处理绑定请求".to_string()));
    }

    // 验证状态
    if binding.status != "pending" {
        return Err(AppError::BadRequest("该请求已被处理".to_string()));
    }

    let new_status = if action == "accept" { "accepted" } else { "rejected" };

    // 更新状态
    sqlx::query(
        "UPDATE account_bindings SET status = $1, updated_at = $2 WHERE id = $3"
    )
    .bind(new_status)
    .bind(Utc::now())
    .bind(binding_id)
    .execute(pool)
    .await?;

    // 如果接受，创建默认权限（不再创建反向绑定关系）
    if action == "accept" {
        // 注意：不再创建反向绑定关系
        // 查询时使用 OR 条件来匹配双向关系
        // WHERE (user_id = $1 OR bound_user_id = $1) AND status = 'accepted'

        // 为双方创建默认权限（所有应用默认为 none）
        let app_types = vec![
            "note", "account", "love_letter", "writing", 
            "forum", "private_space", "idol", "chat"
        ];

        for app_type in &app_types {
            // 为发起者创建权限（允许绑定用户访问）
            sqlx::query(
                "INSERT INTO app_permissions (user_id, bound_user_id, app_type, permission_level, created_at, updated_at)
                 VALUES ($1, $2, $3, 'none', NOW(), NOW())
                 ON CONFLICT (user_id, bound_user_id, app_type) DO NOTHING"
            )
            .bind(binding.user_id)
            .bind(binding.bound_user_id)
            .bind(app_type)
            .execute(pool)
            .await?;

            // 为接收者创建权限（允许绑定用户访问）
            sqlx::query(
                "INSERT INTO app_permissions (user_id, bound_user_id, app_type, permission_level, created_at, updated_at)
                 VALUES ($1, $2, $3, 'none', NOW(), NOW())
                 ON CONFLICT (user_id, bound_user_id, app_type) DO NOTHING"
            )
            .bind(binding.bound_user_id)
            .bind(binding.user_id)
            .bind(app_type)
            .execute(pool)
            .await?;
        }

        info!("✅ 绑定请求已接受，创建了默认权限");
    } else {
        info!("✅ 绑定请求已拒绝");
    }

    Ok(())
}

/// 获取我的绑定列表（已接受的绑定）
pub async fn get_my_bindings(pool: &PgPool, user_id: Uuid) -> AppResult<Vec<AccountBindingVo>> {
    info!("📝 获取绑定列表: user_id={}", user_id);

    let bindings = sqlx::query_as::<_, AccountBinding>(
        "SELECT * FROM account_bindings 
         WHERE (user_id = $1 OR bound_user_id = $1) AND status = 'accepted'
         ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for binding in bindings {
        // 获取对方的用户信息
        let other_user_id = if binding.user_id == user_id {
            binding.bound_user_id
        } else {
            binding.user_id
        };

        let other_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(other_user_id)
            .fetch_optional(pool)
            .await?;

        result.push(AccountBindingVo {
            id: binding.id.to_string(),
            user_id: binding.user_id.to_string(),
            bound_user_id: binding.bound_user_id.to_string(),
            status: binding.status,
            initiator_id: binding.initiator_id.to_string(),
            message: binding.message,
            created_at: binding.created_at,
            updated_at: binding.updated_at,
            user_info: None,
            bound_user_info: other_user.map(UserVo::from),
        });
    }

    info!("✅ 找到 {} 个绑定关系", result.len());
    Ok(result)
}

/// 获取待处理的绑定请求
pub async fn get_pending_binding_requests(
    pool: &PgPool,
    user_id: Uuid,
) -> AppResult<Vec<AccountBindingVo>> {
    info!("📝 获取待处理绑定请求: user_id={}", user_id);

    let bindings = sqlx::query_as::<_, AccountBinding>(
        "SELECT * FROM account_bindings 
         WHERE bound_user_id = $1 AND status = 'pending'
         ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for binding in bindings {
        // 获取发起者的用户信息
        let initiator = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(binding.user_id)
            .fetch_optional(pool)
            .await?;

        result.push(AccountBindingVo {
            id: binding.id.to_string(),
            user_id: binding.user_id.to_string(),
            bound_user_id: binding.bound_user_id.to_string(),
            status: binding.status,
            initiator_id: binding.initiator_id.to_string(),
            message: binding.message,
            created_at: binding.created_at,
            updated_at: binding.updated_at,
            user_info: initiator.map(UserVo::from),
            bound_user_info: None,
        });
    }

    info!("✅ 找到 {} 个待处理请求", result.len());
    Ok(result)
}

/// 取消绑定
pub async fn cancel_binding(pool: &PgPool, user_id: Uuid, binding_id: Uuid) -> AppResult<()> {
    info!("📝 取消绑定: user_id={}, binding_id={}", user_id, binding_id);

    // 获取绑定关系
    let binding = sqlx::query_as::<_, AccountBinding>(
        "SELECT * FROM account_bindings WHERE id = $1"
    )
    .bind(binding_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("绑定关系不存在".to_string()))?;

    // 验证权限
    if binding.user_id != user_id && binding.bound_user_id != user_id {
        return Err(AppError::Forbidden("无权操作该绑定".to_string()));
    }

    // 删除绑定关系（只有一条记录，不需要双向删除）
    sqlx::query("DELETE FROM account_bindings WHERE id = $1")
        .bind(binding_id)
        .execute(pool)
        .await?;

    // 删除相关权限
    sqlx::query(
        "DELETE FROM app_permissions 
         WHERE (user_id = $1 AND bound_user_id = $2) 
            OR (user_id = $2 AND bound_user_id = $1)"
    )
    .bind(binding.user_id)
    .bind(binding.bound_user_id)
    .execute(pool)
    .await?;

    info!("✅ 绑定已取消");
    Ok(())
}

/// 更新应用权限
pub async fn update_app_permission(
    pool: &PgPool,
    user_id: Uuid,
    bound_user_id: Uuid,
    app_type: &str,
    permission_level: &str,
) -> AppResult<AppPermissionVo> {
    info!("📝 更新应用权限: user_id={}, bound_user_id={}, app_type={}, level={}", 
        user_id, bound_user_id, app_type, permission_level);

    // 验证权限级别
    if permission_level != "none" && permission_level != "read" && permission_level != "write" {
        return Err(AppError::BadRequest("无效的权限级别".to_string()));
    }

    // 验证应用类型
    let valid_app_types = vec![
        "note", "account", "love_letter", "writing", 
        "forum", "private_space", "idol", "chat"
    ];
    if !valid_app_types.contains(&app_type) {
        return Err(AppError::BadRequest("无效的应用类型".to_string()));
    }

    // 验证绑定关系
    let binding_exists = sqlx::query(
        "SELECT id FROM account_bindings 
         WHERE ((user_id = $1 AND bound_user_id = $2) 
             OR (user_id = $2 AND bound_user_id = $1)) 
           AND status = 'accepted'"
    )
    .bind(user_id)
    .bind(bound_user_id)
    .fetch_optional(pool)
    .await?;

    if binding_exists.is_none() {
        return Err(AppError::BadRequest("不存在已接受的绑定关系".to_string()));
    }

    // 使用事务确保原子性
    let mut tx = pool.begin().await?;
    
    // 先删除可能存在的反向权限记录（修复之前错误设置的权限）
    sqlx::query(
        "DELETE FROM app_permissions 
         WHERE user_id = $1 AND bound_user_id = $2 AND app_type = $3"
    )
    .bind(bound_user_id)  // 注意：这里是反向的
    .bind(user_id)
    .bind(app_type)
    .execute(&mut *tx)
    .await?;

    // 删除当前方向的旧记录（如果存在）
    sqlx::query(
        "DELETE FROM app_permissions 
         WHERE user_id = $1 AND bound_user_id = $2 AND app_type = $3"
    )
    .bind(user_id)
    .bind(bound_user_id)
    .bind(app_type)
    .execute(&mut *tx)
    .await?;

    // 插入新权限记录
    let permission = sqlx::query_as::<_, AppPermission>(
        "INSERT INTO app_permissions (user_id, bound_user_id, app_type, permission_level, created_at, updated_at)
         VALUES ($1, $2, $3, $4, NOW(), NOW())
         RETURNING *"
    )
    .bind(user_id)
    .bind(bound_user_id)
    .bind(app_type)
    .bind(permission_level)
    .fetch_one(&mut *tx)
    .await?;
    
    // 提交事务
    tx.commit().await?;

    info!("✅ 权限已更新");

    Ok(AppPermissionVo {
        id: permission.id.to_string(),
        user_id: permission.user_id.to_string(),
        bound_user_id: permission.bound_user_id.to_string(),
        app_type: permission.app_type,
        permission_level: permission.permission_level,
        created_at: permission.created_at,
        updated_at: permission.updated_at,
    })
}

/// 获取我授予某个绑定用户的权限
pub async fn get_permissions_for_bound_user(
    pool: &PgPool,
    user_id: Uuid,
    bound_user_id: Uuid,
) -> AppResult<Vec<AppPermissionVo>> {
    info!("📝 获取授予权限: user_id={}, bound_user_id={}", user_id, bound_user_id);

    let permissions = sqlx::query_as::<_, AppPermission>(
        "SELECT * FROM app_permissions 
         WHERE user_id = $1 AND bound_user_id = $2
         ORDER BY app_type"
    )
    .bind(user_id)
    .bind(bound_user_id)
    .fetch_all(pool)
    .await?;

    let result = permissions.into_iter().map(|p| AppPermissionVo {
        id: p.id.to_string(),
        user_id: p.user_id.to_string(),
        bound_user_id: p.bound_user_id.to_string(),
        app_type: p.app_type,
        permission_level: p.permission_level,
        created_at: p.created_at,
        updated_at: p.updated_at,
    }).collect();

    Ok(result)
}

/// 检查某个用户对我的某个应用的访问权限
pub async fn check_permission(
    pool: &PgPool,
    data_owner_id: Uuid,
    accessor_id: Uuid,
    app_type: &str,
) -> AppResult<String> {
    let permission = sqlx::query_as::<_, AppPermission>(
        "SELECT * FROM app_permissions 
         WHERE user_id = $1 AND bound_user_id = $2 AND app_type = $3"
    )
    .bind(data_owner_id)
    .bind(accessor_id)
    .bind(app_type)
    .fetch_optional(pool)
    .await?;

    Ok(permission.map(|p| p.permission_level).unwrap_or_else(|| "none".to_string()))
}

/// 获取绑定用户详情（包含权限信息）
pub async fn get_bound_user_detail(
    pool: &PgPool,
    user_id: Uuid,
    bound_user_id: Uuid,
) -> AppResult<BoundUserDetailVo> {
    info!("📝 获取绑定用户详情: user_id={}, bound_user_id={}", user_id, bound_user_id);

    // 获取绑定关系
    let binding = sqlx::query_as::<_, AccountBinding>(
        "SELECT * FROM account_bindings 
         WHERE (user_id = $1 AND bound_user_id = $2) 
            OR (user_id = $2 AND bound_user_id = $1)
         LIMIT 1"
    )
    .bind(user_id)
    .bind(bound_user_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("绑定关系不存在".to_string()))?;

    // 获取对方用户信息
    let bound_user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(bound_user_id)
        .fetch_optional(pool)
        .await?;

    // 获取我授予对方的权限
    let permissions = get_permissions_for_bound_user(pool, user_id, bound_user_id).await?;

    let binding_vo = AccountBindingVo {
        id: binding.id.to_string(),
        user_id: binding.user_id.to_string(),
        bound_user_id: binding.bound_user_id.to_string(),
        status: binding.status,
        initiator_id: binding.initiator_id.to_string(),
        message: binding.message,
        created_at: binding.created_at,
        updated_at: binding.updated_at,
        user_info: None,
        bound_user_info: bound_user.map(UserVo::from),
    };

    Ok(BoundUserDetailVo {
        binding: binding_vo,
        permissions,
    })
}
