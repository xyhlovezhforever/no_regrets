/**
 * 认证服务
 * 处理用户认证相关业务逻辑
 */

use anyhow::{anyhow, Result};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::PgPool;
use redis::aio::ConnectionManager;
use uuid::Uuid;

use crate::config::Config;
use crate::model::{Claims, LoginRequest, RegisterRequest, TokenPair, User, UserVo};
use crate::repository::{database::UserRepository, redis::{RedisRepository, cache_keys}};

pub struct AuthService;

impl AuthService {
    /// 用户登录
    pub async fn login(
        pool: &PgPool,
        redis_conn: &mut ConnectionManager,
        config: &Config,
        request: &LoginRequest,
    ) -> Result<(String, String, UserVo)> {
        // 查找用户
        let user = UserRepository::find_by_username(pool, &request.username)
            .await?
            .ok_or_else(|| anyhow!("用户名或密码错误"))?;

        // 验证密码
        let password_valid = bcrypt::verify(&request.password, &user.password_hash)?;
        if !password_valid {
            return Err(anyhow!("用户名或密码错误"));
        }

        // 使旧token失效（单点登录）
        Self::invalidate_user_tokens(redis_conn, &user.id.to_string()).await?;

        // 生成新 token
        let token_pair = Self::generate_tokens(&user.id.to_string(), config)?;

        // 将新token存储到Redis
        let token_key = format!("{}:{}", cache_keys::USER_TOKEN, user.id);
        RedisRepository::set(
            redis_conn,
            &token_key,
            &token_pair.access_token,
            config.jwt_expiration as usize,
        ).await?;

        Ok((
            token_pair.access_token,
            token_pair.refresh_token,
            user.into(),
        ))
    }

    /// 用户注册
    pub async fn register(
        pool: &PgPool,
        redis_conn: &mut ConnectionManager,
        config: &Config,
        request: &RegisterRequest,
    ) -> Result<(String, String, UserVo)> {
        // 检查用户名是否已存在
        if let Some(_) = UserRepository::find_by_username(pool, &request.username).await? {
            return Err(anyhow!("用户名已存在"));
        }

        // 加密密码
        let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)?;

        // 创建用户
        let user = UserRepository::create(
            pool,
            &request.username,
            &password_hash,
            &request.username, // 默认昵称为用户名
        )
        .await?;

        // 生成 token
        let token_pair = Self::generate_tokens(&user.id.to_string(), config)?;

        // 将token存储到Redis
        let token_key = format!("{}:{}", cache_keys::USER_TOKEN, user.id);
        RedisRepository::set(
            redis_conn,
            &token_key,
            &token_pair.access_token,
            config.jwt_expiration as usize,
        ).await?;

        Ok((
            token_pair.access_token,
            token_pair.refresh_token,
            user.into(),
        ))
    }

    /// 生成 token 对
    pub fn generate_tokens(user_id: &str, config: &Config) -> Result<TokenPair> {
        let now = Utc::now().timestamp();

        // 生成访问令牌
        let access_claims = Claims {
            sub: user_id.to_string(),
            exp: now + config.jwt_expiration,
            iat: now,
        };
        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )?;

        // 生成刷新令牌
        let refresh_claims = Claims {
            sub: user_id.to_string(),
            exp: now + config.jwt_refresh_expiration,
            iat: now,
        };
        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }

    /// 验证 token
    pub fn verify_token(token: &str, config: &Config) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    /// 刷新 token
    pub fn refresh_token(refresh_token: &str, config: &Config) -> Result<TokenPair> {
        let claims = Self::verify_token(refresh_token, config)?;
        Self::generate_tokens(&claims.sub, config)
    }

    /// 从 token 中提取用户 ID
    pub fn extract_user_id(token: &str, config: &Config) -> Result<Uuid> {
        let claims = Self::verify_token(token, config)?;
        Uuid::parse_str(&claims.sub).map_err(|e| anyhow!("Invalid user ID: {}", e))
    }

    /// 验证token是否在Redis中存在（用于单点登录检查）
    pub async fn verify_token_in_redis(
        redis_conn: &mut ConnectionManager,
        user_id: &str,
        token: &str,
    ) -> Result<bool> {
        let token_key = format!("{}:{}", cache_keys::USER_TOKEN, user_id);
        let stored_token = RedisRepository::get(redis_conn, &token_key).await?;
        
        Ok(stored_token.as_ref().map(|s| s.as_str()) == Some(token))
    }

    /// 使指定用户的所有token失效（单点登录）
    pub async fn invalidate_user_tokens(
        redis_conn: &mut ConnectionManager,
        user_id: &str,
    ) -> Result<()> {
        let token_key = format!("{}:{}", cache_keys::USER_TOKEN, user_id);
        RedisRepository::del(redis_conn, &token_key).await?;
        Ok(())
    }

    /// 退出登录，使token失效
    pub async fn logout(
        redis_conn: &mut ConnectionManager,
        user_id: &str,
    ) -> Result<()> {
        Self::invalidate_user_tokens(redis_conn, user_id).await
    }
}

