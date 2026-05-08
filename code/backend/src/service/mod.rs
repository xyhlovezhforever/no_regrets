/**
 * Service 层（业务逻辑层）
 * 负责处理业务逻辑
 */

pub mod ai_service;
pub mod auth_service;
pub mod binding_service;
pub mod bound_data_service;
pub mod content_service;
pub mod forum_service;
pub mod friend_service;
pub mod note_service;
pub mod user_service;
pub mod idol_service;

// 重新导出服务结构体
pub use ai_service::AiService;
pub use auth_service::AuthService;
pub use user_service::UserService;
pub use idol_service::IdolService;

// 其他服务模块已通过 pub mod 导出，可直接使用
// content_service, friend_service, note_service 和 binding_service 导出的是函数，通过 service::xxx 访问

