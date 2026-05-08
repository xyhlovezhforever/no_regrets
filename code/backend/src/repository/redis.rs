/**
 * Redis 操作
 * 封装所有缓存操作
 */

use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use anyhow::Result;

/// Redis 工具类
pub struct RedisRepository;

impl RedisRepository {
    /// 设置键值
    pub async fn set(
        conn: &mut ConnectionManager,
        key: &str,
        value: &str,
        expiration: usize,
    ) -> Result<()> {
        conn.set_ex(key, value, expiration as u64).await?;
        Ok(())
    }

    /// 获取值
    pub async fn get(conn: &mut ConnectionManager, key: &str) -> Result<Option<String>> {
        Ok(conn.get(key).await?)
    }

    /// 删除键
    pub async fn del(conn: &mut ConnectionManager, key: &str) -> Result<()> {
        conn.del(key).await?;
        Ok(())
    }

    /// 检查键是否存在
    pub async fn exists(conn: &mut ConnectionManager, key: &str) -> Result<bool> {
        Ok(conn.exists(key).await?)
    }

    /// 设置过期时间
    pub async fn expire(conn: &mut ConnectionManager, key: &str, seconds: usize) -> Result<()> {
        conn.expire(key, seconds as i64).await?;
        Ok(())
    }

    /// 增加计数
    pub async fn incr(conn: &mut ConnectionManager, key: &str) -> Result<i64> {
        Ok(conn.incr(key, 1).await?)
    }

    /// 减少计数
    pub async fn decr(conn: &mut ConnectionManager, key: &str) -> Result<i64> {
        Ok(conn.decr(key, 1).await?)
    }

    /// 获取哈希表字段
    pub async fn hget(
        conn: &mut ConnectionManager,
        key: &str,
        field: &str,
    ) -> Result<Option<String>> {
        Ok(conn.hget(key, field).await?)
    }

    /// 设置哈希表字段
    pub async fn hset(
        conn: &mut ConnectionManager,
        key: &str,
        field: &str,
        value: &str,
    ) -> Result<()> {
        conn.hset(key, field, value).await?;
        Ok(())
    }
}

/// 缓存键前缀
pub mod cache_keys {
    pub const USER_INFO: &str = "user:info:";
    pub const USER_TOKEN: &str = "user:token:";
    pub const ENCOURAGE_CARD_TODAY: &str = "encourage:today";
    pub const PHILOSOPHY_CARD_TODAY: &str = "philosophy:today";
    pub const RATE_LIMIT: &str = "rate_limit:";
}

