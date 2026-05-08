/**
 * 数据库实体定义
 */

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// 用户实体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub nickname: String,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub gender: Option<i16>,
    pub birthday: Option<String>,
    pub bio: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// AI 聊天会话
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// AI 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: Uuid,
    pub session_id: Uuid,
    pub role: String, // user, assistant, system
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// 情书
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LoveLetter {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub to_name: String,
    pub from_name: String,
    pub is_sent: bool,
    pub created_at: DateTime<Utc>,
}

/// 鼓励卡片
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EncourageCard {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: String,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 哲理命题
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PhilosophyCard {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author: Option<String>,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 偶像信息
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Idol {
    pub id: Uuid,
    pub name: String,
    pub avatar: String,
    pub description: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
}

/// 写作作品
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WritingWork {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub category: String,
    pub tags: Vec<String>,
    pub likes: i32,
    pub views: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 评论
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub work_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub content: String,
    pub likes: i32,
    pub created_at: DateTime<Utc>,
}

/// 记账记录
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub record_type: String, // income, expense
    pub amount: i64,
    pub category: String,
    pub description: Option<String>,
    pub date: String,
    pub created_at: DateTime<Utc>,
}

/// 便签
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Note {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub color: String,
    pub font_color: Option<String>,
    pub folder_id: Option<Uuid>,
    pub is_pinned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 便签文件夹
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NoteFolder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub color: String,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 好友关系
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Friendship {
    pub id: Uuid,
    pub user_id: Uuid,
    pub friend_id: Uuid,
    pub status: String, // pending, accepted, blocked
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 好友消息
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FriendMessage {
    pub id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub content: String,
    pub message_type: String, // text, image, voice, location
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

/// 专属AI配置
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CustomAI {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub avatar: String,
    pub personality: Vec<String>,
    pub style: String,
    pub background: Option<String>,
    pub nickname: Option<String>,
    pub catchphrase: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 用户偶像关联
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserIdol {
    pub id: Uuid,
    pub user_id: Uuid,
    pub idol_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// 偶像语录
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IdolQuote {
    pub id: Uuid,
    pub idol_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

/// 偶像作品
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IdolWork {
    pub id: Uuid,
    pub idol_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub work_type: String,
    pub created_at: DateTime<Utc>,
}

/// 私人空间音乐
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PrivateSpaceMusic {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub artist: Option<String>,
    pub url: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 私人空间设置
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PrivateSpaceSettings {
    pub user_id: Uuid,
    pub light_color: String,
    pub light_intensity: i32,
    pub updated_at: DateTime<Utc>,
}

/// 照镜子照片
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MirrorPhoto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub photo_url: String,
    pub created_at: DateTime<Utc>,
}

/// 点赞
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Like {
    pub id: Uuid,
    pub user_id: Uuid,
    pub target_type: String, // work, comment, forum_post, forum_comment, etc.
    pub target_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// 论坛帖子
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumPost {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_type: String, // inspiration, topic, creation
    pub title: String,
    pub content: String,
    pub card_category: Option<String>, // encourage, philosophy
    pub author: Option<String>, // 哲理卡片作者
    pub topic_question: Option<String>, // 话题问题
    pub topic_options: Option<Vec<String>>, // 话题选项
    pub topic_answer: Option<String>, // 用户的回答
    pub creation_category: Option<String>, // 随笔、日记、诗歌等
    pub creation_tags: Option<Vec<String>>, // 创作标签
    pub image_url: Option<String>,
    pub likes: i32,
    pub views: i32,
    pub comments_count: i32,
    pub is_pinned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 论坛评论
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ForumComment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Option<Uuid>, // 允许游客评论（NULL）
    pub parent_id: Option<Uuid>, // 支持回复评论
    pub content: String,
    pub guest_name: Option<String>, // 游客名称
    pub likes: i32,
    pub created_at: DateTime<Utc>,
}

/// 账号绑定
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountBinding {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bound_user_id: Uuid,
    pub status: String, // pending, accepted, rejected, cancelled
    pub initiator_id: Uuid,
    pub message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 应用数据访问权限
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AppPermission {
    pub id: Uuid,
    pub user_id: Uuid, // 数据拥有者
    pub bound_user_id: Uuid, // 被授权的绑定用户
    pub app_type: String, // note, account, love_letter, writing, forum, private_space, idol, chat
    pub permission_level: String, // none, read, write
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

