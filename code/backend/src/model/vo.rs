/**
 * 视图对象 (VO)
 * 用于返回给客户端的数据
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::entity::User;

/// API 响应包装
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

/// 分页响应
#[derive(Debug, Serialize, ToSchema)]
pub struct PageResponse<T> {
    pub list: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

/// 用户信息 VO
#[derive(Debug, Serialize, ToSchema)]
pub struct UserVo {
    pub id: String,
    pub username: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<i16>,
    pub birthday: Option<String>,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserVo {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.username,
            nickname: user.nickname,
            avatar: user.avatar,
            email: user.email,
            phone: user.phone,
            gender: user.gender,
            birthday: user.birthday,
            bio: user.bio,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// 登录响应
#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub user_info: UserVo,
}

/// Token 对
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

/// JWT 声明
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: i64,    // expiration
    pub iat: i64,    // issued at
}

/// 账号绑定 VO
#[derive(Debug, Serialize, ToSchema)]
pub struct AccountBindingVo {
    pub id: String,
    pub user_id: String,
    pub bound_user_id: String,
    pub status: String,
    pub initiator_id: String,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // 关联的用户信息
    pub user_info: Option<UserVo>,
    pub bound_user_info: Option<UserVo>,
}

/// 应用权限 VO
#[derive(Debug, Serialize, ToSchema)]
pub struct AppPermissionVo {
    pub id: String,
    pub user_id: String,
    pub bound_user_id: String,
    pub app_type: String,
    pub permission_level: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 绑定用户详情 VO（包含权限信息）
#[derive(Debug, Serialize, ToSchema)]
pub struct BoundUserDetailVo {
    pub binding: AccountBindingVo,
    pub permissions: Vec<AppPermissionVo>,
}

