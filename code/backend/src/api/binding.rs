/**
 * 账号绑定 API
 * 处理用户账号绑定和应用权限管理
 */

use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use tracing::info;
use uuid::Uuid;
use validator::Validate;

use super::AppState;
use crate::middleware::auth::AuthUser;
use crate::model::{
    dto::{
        BatchUpdatePermissionsRequest, HandleBindingRequest, SendBindingRequest,
        UpdateAppPermissionRequest,
    },
    vo::{AccountBindingVo, AppPermissionVo, BoundUserDetailVo},
    ApiResponse,
};
use crate::service::binding_service;
use crate::utils::error::AppError;

/// 配置账号绑定路由
pub fn routes() -> Router<AppState> {
    Router::new()
        // 绑定管理
        .route("/", get(get_my_bindings))
        .route("/requests", post(send_binding_request))
        .route("/requests/pending", get(get_pending_requests))
        .route("/requests/:id", put(handle_binding_request))
        // 权限管理（必须在 /:id 之前，因为路由匹配是按顺序的）
        .route("/:bound_user_id/detail", get(get_bound_user_detail))
        .route("/:bound_user_id/permissions", get(get_permissions))
        .route("/:bound_user_id/permissions/:app_type", put(update_permission))
        .route(
            "/:bound_user_id/permissions/batch",
            put(batch_update_permissions),
        )
        // 删除绑定（放在最后，因为 /:id 会匹配所有单段路径）
        // 注意：这里只接受 DELETE 方法，不会与上面的 GET 方法冲突
        .route("/:id", delete(cancel_binding))
}

/// 发送账号绑定请求
async fn send_binding_request(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<SendBindingRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    req.validate()?;

    let bound_user_id = Uuid::parse_str(&req.bound_user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    binding_service::send_binding_request(
        &state.db_pool,
        auth_user.user_id,
        bound_user_id,
        req.message.as_deref(),
    )
    .await?;

    Ok(Json(ApiResponse::success(())))
}

/// 处理账号绑定请求
async fn handle_binding_request(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<HandleBindingRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    req.validate()?;

    let binding_id =
        Uuid::parse_str(&id).map_err(|_| AppError::BadRequest("Invalid binding ID".to_string()))?;

    binding_service::handle_binding_request(
        &state.db_pool,
        auth_user.user_id,
        binding_id,
        &req.action,
    )
    .await?;

    Ok(Json(ApiResponse::success(())))
}

/// 获取我的绑定列表
async fn get_my_bindings(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<Vec<AccountBindingVo>>>, AppError> {
    let bindings = binding_service::get_my_bindings(&state.db_pool, auth_user.user_id).await?;
    Ok(Json(ApiResponse::success(bindings)))
}

/// 获取待处理的绑定请求
async fn get_pending_requests(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<Vec<AccountBindingVo>>>, AppError> {
    let requests =
        binding_service::get_pending_binding_requests(&state.db_pool, auth_user.user_id).await?;
    Ok(Json(ApiResponse::success(requests)))
}

/// 取消绑定
async fn cancel_binding(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let binding_id =
        Uuid::parse_str(&id).map_err(|_| AppError::BadRequest("Invalid binding ID".to_string()))?;

    binding_service::cancel_binding(&state.db_pool, auth_user.user_id, binding_id).await?;

    Ok(Json(ApiResponse::success(())))
}

/// 更新单个应用权限
async fn update_permission(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path((bound_user_id, app_type)): Path<(String, String)>,
    Json(req): Json<UpdateAppPermissionRequest>,
) -> Result<Json<ApiResponse<AppPermissionVo>>, AppError> {
    req.validate()?;

    let bound_user_id = Uuid::parse_str(&bound_user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let permission = binding_service::update_app_permission(
        &state.db_pool,
        auth_user.user_id,
        bound_user_id,
        &app_type,
        &req.permission_level,
    )
    .await?;

    Ok(Json(ApiResponse::success(permission)))
}

/// 批量更新应用权限
async fn batch_update_permissions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(bound_user_id): Path<String>,
    Json(req): Json<BatchUpdatePermissionsRequest>,
) -> Result<Json<ApiResponse<Vec<AppPermissionVo>>>, AppError> {
    req.validate()?;

    let bound_user_id = Uuid::parse_str(&bound_user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let mut updated_permissions = Vec::new();

    // 批量更新，每个更新都有自己的事务保护
    for perm in req.permissions {
        let permission = binding_service::update_app_permission(
            &state.db_pool,
            auth_user.user_id,
            bound_user_id,
            &perm.app_type,
            &perm.permission_level,
        )
        .await?;
        updated_permissions.push(permission);
    }

    Ok(Json(ApiResponse::success(updated_permissions)))
}

/// 获取授予某个绑定用户的权限列表
async fn get_permissions(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(bound_user_id): Path<String>,
) -> Result<Json<ApiResponse<Vec<AppPermissionVo>>>, AppError> {
    let bound_user_id = Uuid::parse_str(&bound_user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let permissions = binding_service::get_permissions_for_bound_user(
        &state.db_pool,
        auth_user.user_id,
        bound_user_id,
    )
    .await?;

    Ok(Json(ApiResponse::success(permissions)))
}

/// 获取绑定用户详情（包含权限信息）
async fn get_bound_user_detail(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(bound_user_id): Path<String>,
) -> Result<Json<ApiResponse<BoundUserDetailVo>>, AppError> {
    let bound_user_id = Uuid::parse_str(&bound_user_id)
        .map_err(|_| AppError::BadRequest("Invalid user ID".to_string()))?;

    let detail = binding_service::get_bound_user_detail(
        &state.db_pool,
        auth_user.user_id,
        bound_user_id,
    )
    .await?;

    Ok(Json(ApiResponse::success(detail)))
}
