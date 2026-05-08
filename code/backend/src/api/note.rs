/**
 * 笔记相关 API
 */

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::{dto::{CreateNoteRequest, PageRequest, CreateNoteFolderRequest, BatchDeleteNotesRequest, BatchMoveNotesRequest}, ApiResponse},
    service::note_service,
    utils::error::{AppError, AppResult},
};

/// 配置笔记路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_notes).post(create_note))
        .route("/:id", get(get_note).put(update_note).delete(delete_note))
        .route("/batch/delete", post(batch_delete_notes))
        .route("/batch/move", post(batch_move_notes))
        .route("/folders", get(get_folders).post(create_folder))
        .route("/folders/:id", put(update_folder).delete(delete_folder))
        .route("/folders/:id/notes", get(get_notes_by_folder))
}

/// 获取笔记列表
async fn get_notes(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let notes = note_service::get_notes(&state.db_pool, auth_user.user_id, &params).await?;
    
    Ok(Json(ApiResponse::success(json!(notes))))
}

/// 创建笔记
async fn create_note(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateNoteRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let note = note_service::create_note(&state.db_pool, auth_user.user_id, &req).await?;
    
    Ok(Json(ApiResponse::success(json!(note))))
}

/// 获取笔记详情
async fn get_note(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let note_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid note ID".to_string()))?;
    
    let note = note_service::get_note(&state.db_pool, auth_user.user_id, note_id).await?;
    
    Ok(Json(ApiResponse::success(json!(note))))
}

/// 更新笔记
async fn update_note(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<CreateNoteRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let note_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid note ID".to_string()))?;
    
    let note = note_service::update_note(&state.db_pool, auth_user.user_id, note_id, &req).await?;
    
    Ok(Json(ApiResponse::success(json!(note))))
}

/// 删除笔记
async fn delete_note(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let note_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid note ID".to_string()))?;
    
    note_service::delete_note(&state.db_pool, auth_user.user_id, note_id).await?;
    
    Ok(Json(ApiResponse::success(())))
}

/// 批量删除笔记
async fn batch_delete_notes(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<BatchDeleteNotesRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let note_ids: Result<Vec<Uuid>, _> = req.note_ids.iter()
        .map(|id| Uuid::parse_str(id))
        .collect();
    
    let note_ids = note_ids.map_err(|_| AppError::BadRequest("Invalid note ID".to_string()))?;
    
    let affected = note_service::batch_delete_notes(&state.db_pool, auth_user.user_id, note_ids).await?;
    
    Ok(Json(ApiResponse::success(json!({ "affected": affected }))))
}

/// 批量移动笔记
async fn batch_move_notes(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<BatchMoveNotesRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let note_ids: Result<Vec<Uuid>, _> = req.note_ids.iter()
        .map(|id| Uuid::parse_str(id))
        .collect();
    
    let note_ids = note_ids.map_err(|_| AppError::BadRequest("Invalid note ID".to_string()))?;
    
    let folder_id = if let Some(ref fid) = req.folder_id {
        Some(Uuid::parse_str(fid).map_err(|_| AppError::BadRequest("Invalid folder ID".to_string()))?)
    } else {
        None
    };
    
    let affected = note_service::batch_move_notes(&state.db_pool, auth_user.user_id, note_ids, folder_id).await?;
    
    Ok(Json(ApiResponse::success(json!({ "affected": affected }))))
}

// ========== 文件夹 API ==========

/// 获取文件夹列表
async fn get_folders(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> AppResult<Json<ApiResponse<Value>>> {
    let folders = note_service::get_folders(&state.db_pool, auth_user.user_id).await?;
    
    Ok(Json(ApiResponse::success(json!(folders))))
}

/// 创建文件夹
async fn create_folder(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<CreateNoteFolderRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let folder = note_service::create_folder(&state.db_pool, auth_user.user_id, &req).await?;
    
    Ok(Json(ApiResponse::success(json!(folder))))
}

/// 更新文件夹
async fn update_folder(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(req): Json<CreateNoteFolderRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let folder_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid folder ID".to_string()))?;
    
    let folder = note_service::update_folder(&state.db_pool, auth_user.user_id, folder_id, &req).await?;
    
    Ok(Json(ApiResponse::success(json!(folder))))
}

/// 删除文件夹
async fn delete_folder(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let folder_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequest("Invalid folder ID".to_string()))?;
    
    note_service::delete_folder(&state.db_pool, auth_user.user_id, folder_id).await?;
    
    Ok(Json(ApiResponse::success(())))
}

/// 获取指定文件夹下的笔记
async fn get_notes_by_folder(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Query(params): Query<PageRequest>,
) -> AppResult<Json<ApiResponse<Value>>> {
    let folder_id = if id == "uncategorized" {
        None
    } else {
        Some(Uuid::parse_str(&id).map_err(|_| AppError::BadRequest("Invalid folder ID".to_string()))?)
    };
    
    let notes = note_service::get_notes_by_folder(&state.db_pool, auth_user.user_id, folder_id, &params).await?;
    
    Ok(Json(ApiResponse::success(json!(notes))))
}

