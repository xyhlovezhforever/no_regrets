use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use utoipa::ToSchema;

/// 偶像模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Idol {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub nationality: Option<String>,
    pub profession: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建偶像请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateIdolRequest {
    pub name: String,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub nationality: Option<String>,
    pub profession: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
}

/// 更新偶像请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateIdolRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub nationality: Option<String>,
    pub profession: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_public: Option<bool>,
}

/// 偶像语录模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct IdolQuote {
    pub id: Uuid,
    pub idol_id: Uuid,
    pub content: String,
    pub source: Option<String>,
    pub quote_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建语录请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateQuoteRequest {
    pub content: String,
    pub source: Option<String>,
    pub quote_date: Option<NaiveDate>,
}

/// 更新语录请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateQuoteRequest {
    pub content: Option<String>,
    pub source: Option<String>,
    pub quote_date: Option<NaiveDate>,
}

/// 作品类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum WorkType {
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "other")]
    Other,
}

impl std::fmt::Display for WorkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkType::Audio => write!(f, "audio"),
            WorkType::Video => write!(f, "video"),
            WorkType::Image => write!(f, "image"),
            WorkType::Other => write!(f, "other"),
        }
    }
}

impl std::str::FromStr for WorkType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "audio" => Ok(WorkType::Audio),
            "video" => Ok(WorkType::Video),
            "image" => Ok(WorkType::Image),
            "other" => Ok(WorkType::Other),
            _ => Err(format!("Invalid work type: {}", s)),
        }
    }
}

/// 偶像作品模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct IdolWork {
    pub id: Uuid,
    pub idol_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub work_type: String,
    pub file_url: String,
    pub thumbnail_url: Option<String>,
    pub file_size: Option<i64>,
    pub duration: Option<i32>,
    pub release_date: Option<NaiveDate>,
    pub tags: Option<Vec<String>>,
    pub view_count: i32,
    pub like_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 创建作品请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateWorkRequest {
    pub title: String,
    pub description: Option<String>,
    pub work_type: String,
    pub file_url: String,
    pub thumbnail_url: Option<String>,
    pub file_size: Option<i64>,
    pub duration: Option<i32>,
    pub release_date: Option<NaiveDate>,
    pub tags: Option<Vec<String>>,
}

/// 更新作品请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateWorkRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub work_type: Option<String>,
    pub file_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub file_size: Option<i64>,
    pub duration: Option<i32>,
    pub release_date: Option<NaiveDate>,
    pub tags: Option<Vec<String>>,
}

/// 偶像详情（包含语录和作品）
#[derive(Debug, Serialize, ToSchema)]
pub struct IdolDetail {
    #[serde(flatten)]
    pub idol: Idol,
    pub quotes: Vec<IdolQuote>,
    pub works: Vec<IdolWork>,
}

/// 分页查询参数
#[derive(Debug, Deserialize, ToSchema)]
pub struct IdolQueryParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
    pub tags: Option<Vec<String>>,
    pub only_public: Option<bool>,
}

/// 分页响应
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedIdols {
    pub items: Vec<Idol>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}
