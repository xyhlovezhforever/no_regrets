use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
    routing::{get, post},
    Router,
};
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

use crate::api::AppState;
use crate::model::vo::ApiResponse;
use crate::utils::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct WatermarkResponse {
    pub file_url: String,
    pub filename: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/upload", post(upload_and_process))
        .route("/download/:filename", get(download_file))
        .route("/test", get(test_handler))
}

async fn test_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 0,
        "message": "水印API正常工作",
        "data": "OK"
    }))
}

async fn upload_and_process(
    State(_state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<Json<ApiResponse<WatermarkResponse>>> {
    tracing::info!("收到水印去除请求");
    
    let upload_dir = PathBuf::from("uploads");
    let output_dir = PathBuf::from("processed");

    // 创建目录
    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| {
            tracing::error!("创建上传目录失败: {}", e);
            AppError::InternalServerError(format!("创建上传目录失败: {}", e))
        })?;
    fs::create_dir_all(&output_dir)
        .await
        .map_err(|e| {
            tracing::error!("创建输出目录失败: {}", e);
            AppError::InternalServerError(format!("创建输出目录失败: {}", e))
        })?;

    let mut intensity = 5;
    let mut watermark_type = "auto".to_string();
    let mut file_path: Option<PathBuf> = None;

    // 处理multipart数据
    tracing::info!("开始处理multipart数据");
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| {
            tracing::error!("读取multipart字段失败: {}", e);
            AppError::BadRequest(format!("读取上传文件失败: {}", e))
        })?
    {
        let name = field.name().unwrap_or("").to_string();
        tracing::info!("处理字段: {}", name);

        match name.as_str() {
            "file" => {
                tracing::info!("处理文件字段");
                let file_name = field
                    .file_name()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("{}.jpg", Uuid::new_v4()));
                
                tracing::info!("文件名: {}", file_name);
                
                tracing::info!("开始读取文件字节数据...");
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| {
                        tracing::error!("读取文件数据失败: {}", e);
                        AppError::BadRequest(format!("读取文件数据失败: {}", e))
                    })?;
                
                tracing::info!("文件大小: {} 字节", data.len());

                let file_path_buf = upload_dir.join(&file_name);
                tracing::info!("保存文件到: {:?}", file_path_buf);
                
                fs::write(&file_path_buf, &data)
                    .await
                    .map_err(|e| {
                        tracing::error!("保存文件失败: {}", e);
                        AppError::InternalServerError(format!("保存文件失败: {}", e))
                    })?;
                
                tracing::info!("文件保存成功");
                file_path = Some(file_path_buf);
            }
            "intensity" => {
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::BadRequest(e.to_string()))?;
                if let Ok(val) = String::from_utf8(data.to_vec()) {
                    intensity = val.parse().unwrap_or(5);
                }
            }
            "type" => {
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::BadRequest(e.to_string()))?;
                if let Ok(val) = String::from_utf8(data.to_vec()) {
                    watermark_type = val;
                }
            }
            _ => {}
        }
    }

    let input_path = file_path
        .ok_or_else(|| AppError::BadRequest("未上传文件".to_string()))?;

    tracing::info!("准备处理图像: {:?}", input_path);
    
    // 处理图像
    let output_filename = format!("processed_{}.png", Uuid::new_v4());
    let output_path = output_dir.join(&output_filename);

    tracing::info!("输出文件: {:?}, 类型: {}, 强度: {}", output_path, watermark_type, intensity);

    // 在阻塞线程中处理图像，避免阻塞异步运行时
    let input_clone = input_path.clone();
    let output_clone = output_path.clone();
    let type_clone = watermark_type.clone();
    
    tokio::task::spawn_blocking(move || {
        tracing::info!("开始图像处理...");
        remove_watermark_sync(&input_clone, &output_clone, intensity, &type_clone)
    })
    .await
    .map_err(|e| {
        tracing::error!("任务执行失败: {}", e);
        AppError::InternalServerError(format!("任务执行失败: {}", e))
    })?
    .map_err(|e| {
        tracing::error!("图像处理失败: {}", e);
        AppError::InternalServerError(format!("处理失败: {}", e))
    })?;

    tracing::info!("图像处理完成");

    // 删除原文件
    let _ = fs::remove_file(input_path).await;

    Ok(Json(ApiResponse::success(WatermarkResponse {
        file_url: format!("/api/watermark/download/{}", output_filename),
        filename: output_filename,
    })))
}

fn remove_watermark_sync(
    input_path: &PathBuf,
    output_path: &PathBuf,
    intensity: u8,
    watermark_type: &str,
) -> Result<(), String> {
    tracing::info!("正在打开图像...");
    // 读取图像
    let img = image::open(input_path).map_err(|e| e.to_string())?;
    tracing::info!("图像打开成功，尺寸: {}x{}", img.width(), img.height());

    // 根据水印类型选择处理方法
    tracing::info!("应用去水印算法: {}", watermark_type);
    let processed = match watermark_type {
        "text" => remove_text_watermark(&img, intensity),
        "image" => remove_image_watermark(&img, intensity),
        "background" => remove_background_watermark(&img, intensity),
        _ => auto_remove_watermark(&img, intensity),
    };

    tracing::info!("保存处理后的图像...");
    // 保存处理后的图像
    processed.save(output_path).map_err(|e| e.to_string())?;
    tracing::info!("图像保存成功");

    Ok(())
}

/// 自动检测并去除水印
fn auto_remove_watermark(img: &DynamicImage, intensity: u8) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let processed_pixel = process_pixel_auto(pixel, intensity);
            output.put_pixel(x, y, processed_pixel);
        }
    }

    DynamicImage::ImageRgba8(output)
}

/// 去除文字水印
fn remove_text_watermark(img: &DynamicImage, intensity: u8) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            
            // 检测浅色水印（通常文字水印是浅灰色）
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = pixel[3];

            // 如果像素接近灰色且亮度较高，可能是水印
            let is_gray = (r as i16 - g as i16).abs() < 30
                && (g as i16 - b as i16).abs() < 30
                && (b as i16 - r as i16).abs() < 30;
            let is_light = (r as u16 + g as u16 + b as u16) / 3 > 180;

            let processed_pixel = if is_gray && is_light {
                // 水印区域 - 使用周围像素的平均值替换
                inpaint_pixel(img, x, y, intensity)
            } else {
                Rgba([r, g, b, a])
            };

            output.put_pixel(x, y, processed_pixel);
        }
    }

    DynamicImage::ImageRgba8(output)
}

/// 去除图片水印
fn remove_image_watermark(img: &DynamicImage, intensity: u8) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            
            // 检测半透明区域（通常图片水印是半透明的）
            let a = pixel[3];
            
            let processed_pixel = if a < 250 {
                // 半透明区域 - 使用修复算法
                inpaint_pixel(img, x, y, intensity)
            } else {
                pixel
            };

            output.put_pixel(x, y, processed_pixel);
        }
    }

    DynamicImage::ImageRgba8(output)
}

/// 去除背景水印
fn remove_background_watermark(img: &DynamicImage, intensity: u8) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);

    // 使用中值滤波去除背景噪声
    for y in 0..height {
        for x in 0..width {
            let pixel = median_filter(img, x, y, intensity);
            output.put_pixel(x, y, pixel);
        }
    }

    DynamicImage::ImageRgba8(output)
}

/// 处理单个像素（自动模式）
fn process_pixel_auto(pixel: Rgba<u8>, intensity: u8) -> Rgba<u8> {
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];
    let a = pixel[3];

    // 增强对比度
    let factor = 1.0 + (intensity as f32 / 50.0);
    
    let new_r = ((r as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;
    let new_g = ((g as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;
    let new_b = ((b as f32 - 128.0) * factor + 128.0).clamp(0.0, 255.0) as u8;

    Rgba([new_r, new_g, new_b, a])
}

/// 修复像素（使用周围像素的平均值）
fn inpaint_pixel(img: &DynamicImage, x: u32, y: u32, radius: u8) -> Rgba<u8> {
    let (width, height) = img.dimensions();
    let radius = radius as i32;
    
    let mut sum_r = 0u32;
    let mut sum_g = 0u32;
    let mut sum_b = 0u32;
    let mut count = 0u32;

    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let nx = (x as i32 + dx).clamp(0, width as i32 - 1) as u32;
            let ny = (y as i32 + dy).clamp(0, height as i32 - 1) as u32;

            if dx == 0 && dy == 0 {
                continue;
            }

            let pixel = img.get_pixel(nx, ny);
            sum_r += pixel[0] as u32;
            sum_g += pixel[1] as u32;
            sum_b += pixel[2] as u32;
            count += 1;
        }
    }

    if count > 0 {
        Rgba([
            (sum_r / count) as u8,
            (sum_g / count) as u8,
            (sum_b / count) as u8,
            255,
        ])
    } else {
        img.get_pixel(x, y)
    }
}

/// 中值滤波
fn median_filter(img: &DynamicImage, x: u32, y: u32, radius: u8) -> Rgba<u8> {
    let (width, height) = img.dimensions();
    let radius = radius as i32 / 2;
    
    let mut pixels_r = Vec::new();
    let mut pixels_g = Vec::new();
    let mut pixels_b = Vec::new();

    for dy in -radius..=radius {
        for dx in -radius..=radius {
            let nx = (x as i32 + dx).clamp(0, width as i32 - 1) as u32;
            let ny = (y as i32 + dy).clamp(0, height as i32 - 1) as u32;

            let pixel = img.get_pixel(nx, ny);
            pixels_r.push(pixel[0]);
            pixels_g.push(pixel[1]);
            pixels_b.push(pixel[2]);
        }
    }

    pixels_r.sort_unstable();
    pixels_g.sort_unstable();
    pixels_b.sort_unstable();

    let mid = pixels_r.len() / 2;

    Rgba([pixels_r[mid], pixels_g[mid], pixels_b[mid], 255])
}

async fn download_file(
    Path(filename): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let file_path = PathBuf::from("processed").join(&filename);

    if !file_path.exists() {
        return Err((StatusCode::NOT_FOUND, "File not found".to_string()));
    }

    let file_data = fs::read(&file_path)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(Body::from(file_data))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(response)
}
