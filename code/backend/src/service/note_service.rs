/**
 * 笔记服务
 */

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    model::{
        dto::{CreateNoteRequest, PageRequest, CreateNoteFolderRequest},
        entity::{Note, NoteFolder},
    },
    utils::error::AppError,
};

/// 获取笔记列表（可选按文件夹筛选）
pub async fn get_notes(
    pool: &PgPool,
    user_id: Uuid,
    page: &PageRequest,
) -> Result<Vec<Note>, AppError> {
    let notes = sqlx::query_as::<_, Note>(
        r#"
        SELECT id, user_id, title, content, color, font_color, folder_id, is_pinned, created_at, updated_at
        FROM notes
        WHERE user_id = $1
        ORDER BY is_pinned DESC, updated_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(user_id)
    .bind(page.page_size)
    .bind(page.offset())
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(notes)
}

/// 按文件夹获取笔记列表
pub async fn get_notes_by_folder(
    pool: &PgPool,
    user_id: Uuid,
    folder_id: Option<Uuid>,
    page: &PageRequest,
) -> Result<Vec<Note>, AppError> {
    let notes = if let Some(fid) = folder_id {
        sqlx::query_as::<_, Note>(
            r#"
            SELECT id, user_id, title, content, color, font_color, folder_id, is_pinned, created_at, updated_at
            FROM notes
            WHERE user_id = $1 AND folder_id = $2
            ORDER BY is_pinned DESC, updated_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(user_id)
        .bind(fid)
        .bind(page.page_size)
        .bind(page.offset())
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, Note>(
            r#"
            SELECT id, user_id, title, content, color, font_color, folder_id, is_pinned, created_at, updated_at
            FROM notes
            WHERE user_id = $1 AND folder_id IS NULL
            ORDER BY is_pinned DESC, updated_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(user_id)
        .bind(page.page_size)
        .bind(page.offset())
        .fetch_all(pool)
        .await
    };

    notes.map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))
}

/// 创建笔记
pub async fn create_note(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateNoteRequest,
) -> Result<Note, AppError> {
    let folder_id = if let Some(ref fid) = req.folder_id {
        Some(Uuid::parse_str(fid).map_err(|_| AppError::BadRequest("Invalid folder ID".to_string()))?)
    } else {
        None
    };

    let note = sqlx::query_as::<_, Note>(
        r#"
        INSERT INTO notes (user_id, title, content, color, font_color, folder_id, is_pinned, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
        RETURNING id, user_id, title, content, color, font_color, folder_id, is_pinned, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(&req.title)
    .bind(&req.content)
    .bind(&req.color)
    .bind(&req.font_color)
    .bind(folder_id)
    .bind(req.is_pinned)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(note)
}

/// 获取笔记详情
pub async fn get_note(
    pool: &PgPool,
    user_id: Uuid,
    note_id: Uuid,
) -> Result<Note, AppError> {
    let note = sqlx::query_as::<_, Note>(
        r#"
        SELECT id, user_id, title, content, color, font_color, folder_id, is_pinned, created_at, updated_at
        FROM notes
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
    .ok_or_else(|| AppError::NotFound("Note not found".to_string()))?;

    Ok(note)
}

/// 更新笔记
pub async fn update_note(
    pool: &PgPool,
    user_id: Uuid,
    note_id: Uuid,
    req: &CreateNoteRequest,
) -> Result<Note, AppError> {
    let folder_id = if let Some(ref fid) = req.folder_id {
        Some(Uuid::parse_str(fid).map_err(|_| AppError::BadRequest("Invalid folder ID".to_string()))?)
    } else {
        None
    };

    let note = sqlx::query_as::<_, Note>(
        r#"
        UPDATE notes
        SET title = $1, content = $2, color = $3, font_color = $4, folder_id = $5, is_pinned = $6, updated_at = NOW()
        WHERE id = $7 AND user_id = $8
        RETURNING id, user_id, title, content, color, font_color, folder_id, is_pinned, created_at, updated_at
        "#,
    )
    .bind(&req.title)
    .bind(&req.content)
    .bind(&req.color)
    .bind(&req.font_color)
    .bind(folder_id)
    .bind(req.is_pinned)
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
    .ok_or_else(|| AppError::NotFound("Note not found".to_string()))?;

    Ok(note)
}

/// 删除笔记
pub async fn delete_note(
    pool: &PgPool,
    user_id: Uuid,
    note_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM notes
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(note_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Note not found".to_string()));
    }

    Ok(())
}

/// 批量删除笔记
pub async fn batch_delete_notes(
    pool: &PgPool,
    user_id: Uuid,
    note_ids: Vec<Uuid>,
) -> Result<u64, AppError> {
    if note_ids.is_empty() {
        return Ok(0);
    }

    let result = sqlx::query(
        r#"
        DELETE FROM notes
        WHERE user_id = $1 AND id = ANY($2)
        "#,
    )
    .bind(user_id)
    .bind(&note_ids)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(result.rows_affected())
}

/// 批量移动笔记到指定文件夹
pub async fn batch_move_notes(
    pool: &PgPool,
    user_id: Uuid,
    note_ids: Vec<Uuid>,
    folder_id: Option<Uuid>,
) -> Result<u64, AppError> {
    if note_ids.is_empty() {
        return Ok(0);
    }

    let result = sqlx::query(
        r#"
        UPDATE notes
        SET folder_id = $1, updated_at = NOW()
        WHERE user_id = $2 AND id = ANY($3)
        "#,
    )
    .bind(folder_id)
    .bind(user_id)
    .bind(&note_ids)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(result.rows_affected())
}

// ========== 便签文件夹服务 ==========

/// 获取用户的文件夹列表
pub async fn get_folders(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<NoteFolder>, AppError> {
    let folders = sqlx::query_as::<_, NoteFolder>(
        r#"
        SELECT id, user_id, name, color, sort_order, created_at, updated_at
        FROM note_folders
        WHERE user_id = $1
        ORDER BY sort_order ASC, created_at ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(folders)
}

/// 创建文件夹
pub async fn create_folder(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateNoteFolderRequest,
) -> Result<NoteFolder, AppError> {
    let folder = sqlx::query_as::<_, NoteFolder>(
        r#"
        INSERT INTO note_folders (user_id, name, color, sort_order, created_at, updated_at)
        VALUES ($1, $2, $3, 0, NOW(), NOW())
        RETURNING id, user_id, name, color, sort_order, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(&req.name)
    .bind(&req.color)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    Ok(folder)
}

/// 更新文件夹
pub async fn update_folder(
    pool: &PgPool,
    user_id: Uuid,
    folder_id: Uuid,
    req: &CreateNoteFolderRequest,
) -> Result<NoteFolder, AppError> {
    let folder = sqlx::query_as::<_, NoteFolder>(
        r#"
        UPDATE note_folders
        SET name = $1, color = $2, updated_at = NOW()
        WHERE id = $3 AND user_id = $4
        RETURNING id, user_id, name, color, sort_order, created_at, updated_at
        "#,
    )
    .bind(&req.name)
    .bind(&req.color)
    .bind(folder_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?
    .ok_or_else(|| AppError::NotFound("Folder not found".to_string()))?;

    Ok(folder)
}

/// 删除文件夹（文件夹中的笔记会被移到未分类）
pub async fn delete_folder(
    pool: &PgPool,
    user_id: Uuid,
    folder_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM note_folders
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(folder_id)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| AppError::InternalServerError(format!("Database error: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Folder not found".to_string()));
    }

    Ok(())
}

