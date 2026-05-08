/**
 * 数据传输对象 (DTO)
 * 用于接收客户端请求数据
 */

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// 登录请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(length(min = 6))]
    pub password: String,
}

/// 注册请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(length(min = 6))]
    pub password: String,
    
    #[validate(email)]
    pub email: Option<String>,
    
    #[validate(length(min = 11, max = 11))]
    pub phone: Option<String>,
}

/// 更新用户信息请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<i16>,
    pub birthday: Option<String>,
    pub bio: Option<String>,
}

/// 修改密码请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 6))]
    pub old_password: String,
    
    #[validate(length(min = 6))]
    pub new_password: String,
}

/// 创建聊天会话请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateChatSessionRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
}

/// 发送聊天消息请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendChatMessageRequest {
    pub session_id: String,
    
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
}

/// 创建情书请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateLoveLetterRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    
    #[validate(length(min = 1, max = 5000))]
    pub content: String,
    
    #[validate(length(min = 1, max = 50))]
    pub to_name: String,
    
    #[validate(length(min = 1, max = 50))]
    pub from_name: String,
}

/// 创建写作作品请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateWritingWorkRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    #[validate(length(min = 1, max = 10000))]
    pub content: String,
    
    pub category: String,
    pub tags: Vec<String>,
}

/// 创建评论请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCommentRequest {
    pub work_id: String,
    pub parent_id: Option<String>,
    
    #[validate(length(min = 1, max = 500))]
    pub content: String,
}

/// 创建记账记录请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateAccountRecordRequest {
    pub record_type: String, // income, expense
    pub amount: i64,
    pub category: String,
    pub description: Option<String>,
    pub date: String,
}

/// 创建便签请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateNoteRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    
    #[validate(length(min = 0, max = 5000))]
    pub content: String,
    
    pub color: String,
    pub font_color: Option<String>,
    pub folder_id: Option<String>,
    pub is_pinned: bool,
}

/// 创建便签文件夹请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateNoteFolderRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    
    pub color: String,
}

/// 批量删除便签请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchDeleteNotesRequest {
    pub note_ids: Vec<String>,
}

/// 批量移动便签请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchMoveNotesRequest {
    pub note_ids: Vec<String>,
    pub folder_id: Option<String>,
}

/// 月账单查询参数
#[derive(Debug, Deserialize, ToSchema)]
pub struct MonthlyBillQuery {
    pub year: i32,
    pub month: i32,
}

/// 年账单查询参数
#[derive(Debug, Deserialize, ToSchema)]
pub struct YearlyBillQuery {
    pub year: i32,
}

/// 分页请求
#[derive(Debug, Deserialize, ToSchema)]
pub struct PageRequest {
    #[serde(default = "default_page")]
    pub page: i64,
    
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    10
}

impl PageRequest {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.page_size
    }
}

/// 添加好友请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddFriendRequest {
    pub friend_id: String,
    pub message: Option<String>,
}

/// 处理好友申请请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct HandleFriendRequest {
    pub friendship_id: String,
    pub action: String, // accept, reject
}

/// 发送好友消息请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendFriendMessageRequest {
    pub to_user_id: String,
    
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
    
    pub message_type: Option<String>, // text, image, voice, location
}

/// 创建专属AI请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateCustomAIRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    
    pub avatar: String,
    pub personality: Vec<String>,
    pub style: String,
    pub background: Option<String>,
    pub nickname: Option<String>,
    pub catchphrase: Option<String>,
}

/// 创建偶像请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateIdolRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    pub avatar: String,
    
    #[validate(length(min = 1, max = 1000))]
    pub description: String,
    
    pub category: String,
}

/// 创建偶像语录请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateIdolQuoteRequest {
    pub idol_id: String,
    
    #[validate(length(min = 1, max = 500))]
    pub content: String,
}

/// 创建偶像作品请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateIdolWorkRequest {
    pub idol_id: String,
    
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    pub description: Option<String>,
    pub work_type: String,
}

/// 更新私人空间设置请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePrivateSpaceSettingsRequest {
    pub light_color: Option<String>,
    pub light_intensity: Option<i32>,
}

/// 添加私人空间音乐请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddPrivateSpaceMusicRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    
    pub artist: Option<String>,
    pub url: Option<String>,
}

/// 上传照片请求（用于照镜子功能）
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UploadPhotoRequest {
    pub photo_url: String,
}

/// 创建论坛帖子请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForumPostRequest {
    #[validate(length(min = 1, max = 20))]
    pub post_type: String, // inspiration, topic, creation
    
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    
    #[validate(length(min = 1, max = 10000))]
    pub content: String,
    
    // 灵感卡片特有字段
    pub card_category: Option<String>, // encourage, philosophy
    pub author: Option<String>, // 哲理卡片作者
    
    // 话题特有字段
    pub topic_question: Option<String>,
    pub topic_options: Option<Vec<String>>,
    pub topic_answer: Option<String>,
    
    // 创作特有字段
    pub creation_category: Option<String>,
    pub creation_tags: Option<Vec<String>>,
    
    pub image_url: Option<String>,
}

/// 创建论坛评论请求（支持游客评论）
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForumCommentRequest {
    pub post_id: String,
    pub parent_id: Option<String>, // 回复评论的ID
    
    #[validate(length(min = 1, max = 500))]
    pub content: String,
    
    pub guest_name: Option<String>, // 游客名称（如果未登录）
}

/// 发送账号绑定请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendBindingRequest {
    pub bound_user_id: String,
    pub message: Option<String>,
}

/// 处理账号绑定请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct HandleBindingRequest {
    pub action: String, // accept, reject
}

/// 更新应用权限请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateAppPermissionRequest {
    pub app_type: String, // note, account, love_letter, writing, forum, private_space, idol, chat
    pub permission_level: String, // none, read, write
}

/// 批量更新应用权限请求
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchUpdatePermissionsRequest {
    pub bound_user_id: String,
    pub permissions: Vec<AppPermissionItem>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AppPermissionItem {
    pub app_type: String,
    pub permission_level: String,
}

