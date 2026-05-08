/**
 * 配置管理模块
 * 加载和管理应用配置
 */

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // 应用配置
    pub app_name: String,
    pub app_env: String,
    pub app_host: String,
    pub app_port: u16,

    // 数据库配置
    pub database_url: String,
    pub database_max_connections: u32,

    // Redis 配置
    pub redis_url: String,
    pub redis_max_connections: usize,

    // JWT 配置
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub jwt_refresh_expiration: i64,

    // CORS 配置
    pub cors_origins: Vec<String>,

    // 文件上传配置
    pub upload_dir: String,
    pub max_file_size: usize,

    // AI API 配置
    pub ai_api_key: String,
    pub ai_api_url: String,
    pub ai_model: String,
}

impl Config {
    /// 加载配置
    pub fn load() -> anyhow::Result<Self> {
        // 加载 .env 文件
        dotenvy::dotenv().ok();

        let config = Config {
            app_name: std::env::var("APP_NAME")
                .unwrap_or_else(|_| "you-have-no-regrets".to_string()),
            app_env: std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
            app_host: std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            app_port: std::env::var("APP_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .unwrap_or(8000),

            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            database_max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),

            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            redis_max_connections: std::env::var("REDIS_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),

            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiration: std::env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "86400".to_string())
                .parse()
                .unwrap_or(86400),
            jwt_refresh_expiration: std::env::var("JWT_REFRESH_EXPIRATION")
                .unwrap_or_else(|_| "604800".to_string())
                .parse()
                .unwrap_or(604800),

            cors_origins: std::env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),

            upload_dir: std::env::var("UPLOAD_DIR")
                .unwrap_or_else(|_| "./uploads".to_string()),
            max_file_size: std::env::var("MAX_FILE_SIZE")
                .unwrap_or_else(|_| "10485760".to_string())
                .parse()
                .unwrap_or(10485760),

            ai_api_key: std::env::var("AI_API_KEY")
                .unwrap_or_default(),
            ai_api_url: std::env::var("AI_API_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            ai_model: std::env::var("AI_MODEL")
                .unwrap_or_else(|_| "gpt-3.5-turbo".to_string()),
        };

        Ok(config)
    }

    /// 是否为开发环境
    pub fn is_development(&self) -> bool {
        self.app_env == "development"
    }

    /// 是否为生产环境
    pub fn is_production(&self) -> bool {
        self.app_env == "production"
    }
}

