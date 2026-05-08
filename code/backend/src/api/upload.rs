use axum::{
    extract::{Multipart, State},
    Json, Router,
    routing::post,
};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::api::AppState;
use crate::model::vo::ApiResponse;
use crate::utils::error::{AppResult, AppError};

/// 文件上传路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/image", post(upload_image))
        .route("/video", post(upload_video))
        .route("/audio", post(upload_audio))
}

/// 上传图片响应
#[derive(serde::Serialize)]
pub struct UploadResponse {
    pub url: String,
    pub filename: String,
}

/// 上传图片
async fn upload_image(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<UploadResponse>>> {
    // 确保上传目录存在
    let upload_dir = PathBuf::from("uploads/images");
    fs::create_dir_all(&upload_dir).await
        .map_err(|e| AppError::InternalServerError(format!("创建上传目录失败: {}", e)))?;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| AppError::BadRequest(format!("读取上传文件失败: {}", e)))? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            let filename = field.file_name().unwrap_or("unknown").to_string();
            let data = field.bytes().await
                .map_err(|e| AppError::BadRequest(format!("读取文件数据失败: {}", e)))?;

            // 验证文件类型
            let ext = PathBuf::from(&filename)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            if !matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp") {
                return Err(AppError::BadRequest(
                    "不支持的文件格式，仅支持 jpg, jpeg, png, gif, webp".to_string()
                ));
            }

            // 生成唯一文件名
            let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
            let file_path = upload_dir.join(&new_filename);

            // 保存文件
            let mut file = fs::File::create(&file_path).await
                .map_err(|e| AppError::InternalServerError(format!("创建文件失败: {}", e)))?;
            file.write_all(&data).await
                .map_err(|e| AppError::InternalServerError(format!("写入文件失败: {}", e)))?;

            // 返回文件URL（相对路径）
            let url = format!("/uploads/images/{}", new_filename);

            return Ok(Json(ApiResponse::success(UploadResponse {
                url,
                filename: new_filename,
            })));
        }
    }

    Err(AppError::BadRequest("未找到文件字段".to_string()))
}

/// 上传视频
async fn upload_video(
    State(_state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<UploadResponse>>> {
    use tracing::{info, error};
    
    // 确保上传目录存在
    let upload_dir = PathBuf::from("uploads/videos");
    fs::create_dir_all(&upload_dir).await
        .map_err(|e| AppError::InternalServerError(format!("创建上传目录失败: {}", e)))?;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| AppError::BadRequest(format!("读取上传文件失败: {}", e)))? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            let filename = field.file_name().unwrap_or("unknown").to_string();
            info!("接收视频上传请求: {}", filename);
            
            let data = field.bytes().await
                .map_err(|e| AppError::BadRequest(format!("读取文件数据失败: {}", e)))?;
            
            info!("视频文件大小: {} bytes", data.len());

            // 验证文件类型
            let ext = PathBuf::from(&filename)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            if !matches!(ext.as_str(), "mp4" | "mov" | "avi" | "wmv" | "flv" | "mkv") {
                error!("不支持的视频格式: {}", ext);
                return Err(AppError::BadRequest(
                    "不支持的文件格式，仅支持 mp4, mov, avi, wmv, flv, mkv".to_string()
                ));
            }

            // 生成唯一文件名
            let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
            let file_path = upload_dir.join(&new_filename);
            
            info!("保存视频到: {:?}", file_path);

            // 保存文件
            let mut file = fs::File::create(&file_path).await
                .map_err(|e| {
                    error!("创建视频文件失败: {}", e);
                    AppError::InternalServerError(format!("创建文件失败: {}", e))
                })?;
            file.write_all(&data).await
                .map_err(|e| {
                    error!("写入视频文件失败: {}", e);
                    AppError::InternalServerError(format!("写入文件失败: {}", e))
                })?;
            
            info!("视频文件保存成功: {}", new_filename);

            // 返回文件URL（相对路径）
            let url = format!("/uploads/videos/{}", new_filename);

            return Ok(Json(ApiResponse::success(UploadResponse {
                url,
                filename: new_filename,
            })));
        }
    }

    Err(AppError::BadRequest("未找到文件字段".to_string()))
}

/// 上传音频
async fn upload_audio(
    State(_state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<UploadResponse>>> {
    // 确保上传目录存在
    let upload_dir = PathBuf::from("uploads/audios");
    fs::create_dir_all(&upload_dir).await
        .map_err(|e| AppError::InternalServerError(format!("创建上传目录失败: {}", e)))?;

    while let Some(field) = multipart.next_field().await
        .map_err(|e| AppError::BadRequest(format!("读取上传文件失败: {}", e)))? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            let filename = field.file_name().unwrap_or("unknown").to_string();
            let data = field.bytes().await
                .map_err(|e| AppError::BadRequest(format!("读取文件数据失败: {}", e)))?;

            // 验证文件类型
            let ext = PathBuf::from(&filename)
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_lowercase();

            if !matches!(ext.as_str(), "mp3" | "wav" | "m4a" | "aac" | "flac" | "ogg") {
                return Err(AppError::BadRequest(
                    "不支持的文件格式，仅支持 mp3, wav, m4a, aac, flac, ogg".to_string()
                ));
            }

            // 生成唯一文件名
            let new_filename = format!("{}.{}", Uuid::new_v4(), ext);
            let file_path = upload_dir.join(&new_filename);

            // 保存文件
            let mut file = fs::File::create(&file_path).await
                .map_err(|e| AppError::InternalServerError(format!("创建文件失败: {}", e)))?;
            file.write_all(&data).await
                .map_err(|e| AppError::InternalServerError(format!("写入文件失败: {}", e)))?;

            // 返回文件URL（相对路径）
            let url = format!("/uploads/audios/{}", new_filename);

            return Ok(Json(ApiResponse::success(UploadResponse {
                url,
                filename: new_filename,
            })));
        }
    }

    Err(AppError::BadRequest("未找到文件字段".to_string()))
}
