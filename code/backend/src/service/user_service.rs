/**
 * 用户服务
 * 处理用户相关业务逻辑
 */

use anyhow::{anyhow, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{UpdateUserRequest, User, UserVo};
use crate::repository::database::UserRepository;

pub struct UserService;

impl UserService {
    /// 获取用户信息
    pub async fn get_user_info(pool: &PgPool, user_id: Uuid) -> Result<UserVo> {
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        Ok(user.into())
    }

    /// 更新用户信息
    pub async fn update_user_info(
        pool: &PgPool,
        user_id: Uuid,
        request: &UpdateUserRequest,
    ) -> Result<UserVo> {
        // 获取当前用户
        let mut user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 更新字段
        if let Some(nickname) = &request.nickname {
            user.nickname = nickname.clone();
        }
        if let Some(avatar) = &request.avatar {
            user.avatar = Some(avatar.clone());
        }
        if let Some(email) = &request.email {
            user.email = Some(email.clone());
        }
        if let Some(phone) = &request.phone {
            user.phone = Some(phone.clone());
        }
        if let Some(gender) = request.gender {
            user.gender = Some(gender);
        }
        if let Some(birthday) = &request.birthday {
            user.birthday = Some(birthday.clone());
        }
        if let Some(bio) = &request.bio {
            user.bio = Some(bio.clone());
        }

        // 保存更新
        let updated_user = UserRepository::update(pool, user_id, &user).await?;

        Ok(updated_user.into())
    }

    /// 修改密码
    pub async fn change_password(
        pool: &PgPool,
        user_id: Uuid,
        old_password: &str,
        new_password: &str,
    ) -> Result<()> {
        // 获取用户
        let user = UserRepository::find_by_id(pool, user_id)
            .await?
            .ok_or_else(|| anyhow!("用户不存在"))?;

        // 验证旧密码
        let password_valid = bcrypt::verify(old_password, &user.password_hash)?;
        if !password_valid {
            return Err(anyhow!("原密码错误"));
        }

        // 加密新密码
        let new_password_hash = bcrypt::hash(new_password, bcrypt::DEFAULT_COST)?;

        // 更新密码
        sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2")
            .bind(new_password_hash)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// 搜索用户
    /// 根据用户名或昵称进行模糊搜索，排除当前用户
    pub async fn search_users(pool: &PgPool, keyword: &str, exclude_user_id: Uuid) -> Result<Vec<UserVo>> {
        let search_pattern = format!("%{}%", keyword);
        
        let users = sqlx::query_as::<_, User>(
            r#"
            SELECT * FROM users
            WHERE (username ILIKE $1 OR nickname ILIKE $1)
            AND is_active = true
            AND id != $2
            ORDER BY created_at DESC
            LIMIT 20
            "#
        )
        .bind(&search_pattern)
        .bind(exclude_user_id)
        .fetch_all(pool)
        .await?;

        Ok(users.into_iter().map(|u| u.into()).collect())
    }
}

