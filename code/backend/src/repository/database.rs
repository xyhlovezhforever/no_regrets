/**
 * 数据库操作
 * 封装所有数据库 CRUD 操作
 */

use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::model::entity::*;
use crate::model::dto::PageRequest;

/// 用户 Repository
pub struct UserRepository;

impl UserRepository {
    /// 根据 ID 查找用户
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 AND is_active = true"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /// 根据用户名查找用户
    pub async fn find_by_username(pool: &PgPool, username: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = $1 AND is_active = true"
        )
        .bind(username)
        .fetch_optional(pool)
        .await
    }

    /// 创建用户
    pub async fn create(
        pool: &PgPool,
        username: &str,
        password_hash: &str,
        nickname: &str,
    ) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, username, password_hash, nickname)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(username)
        .bind(password_hash)
        .bind(nickname)
        .fetch_one(pool)
        .await
    }

    /// 更新用户信息
    pub async fn update(pool: &PgPool, id: Uuid, user: &User) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET nickname = $2, avatar = $3, email = $4, phone = $5,
                gender = $6, birthday = $7, bio = $8, updated_at = NOW()
            WHERE id = $1 AND is_active = true
            RETURNING *
            "#
        )
        .bind(id)
        .bind(&user.nickname)
        .bind(&user.avatar)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.gender)
        .bind(&user.birthday)
        .bind(&user.bio)
        .fetch_one(pool)
        .await
    }
}

/// 聊天会话 Repository
pub struct ChatSessionRepository;

impl ChatSessionRepository {
    /// 创建聊天会话
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        title: &str,
    ) -> sqlx::Result<ChatSession> {
        sqlx::query_as::<_, ChatSession>(
            "INSERT INTO chat_sessions (id, user_id, title) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(title)
        .fetch_one(pool)
        .await
    }

    /// 获取用户的聊天会话列表
    pub async fn list_by_user(
        pool: &PgPool,
        user_id: Uuid,
        page: &PageRequest,
    ) -> sqlx::Result<Vec<ChatSession>> {
        sqlx::query_as::<_, ChatSession>(
            "SELECT * FROM chat_sessions WHERE user_id = $1 ORDER BY updated_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(user_id)
        .bind(page.page_size)
        .bind(page.offset())
        .fetch_all(pool)
        .await
    }
}

/// 聊天消息 Repository
pub struct ChatMessageRepository;

impl ChatMessageRepository {
    /// 创建聊天消息
    pub async fn create(
        pool: &PgPool,
        session_id: Uuid,
        role: &str,
        content: &str,
    ) -> sqlx::Result<ChatMessage> {
        sqlx::query_as::<_, ChatMessage>(
            "INSERT INTO chat_messages (id, session_id, role, content) VALUES ($1, $2, $3, $4) RETURNING *"
        )
        .bind(Uuid::new_v4())
        .bind(session_id)
        .bind(role)
        .bind(content)
        .fetch_one(pool)
        .await
    }

    /// 获取会话的消息列表
    pub async fn list_by_session(
        pool: &PgPool,
        session_id: Uuid,
        page: &PageRequest,
    ) -> sqlx::Result<Vec<ChatMessage>> {
        sqlx::query_as::<_, ChatMessage>(
            "SELECT * FROM chat_messages WHERE session_id = $1 ORDER BY created_at ASC LIMIT $2 OFFSET $3"
        )
        .bind(session_id)
        .bind(page.page_size)
        .bind(page.offset())
        .fetch_all(pool)
        .await
    }
}

/// 鼓励卡片 Repository
pub struct EncourageCardRepository;

impl EncourageCardRepository {
    /// 获取随机鼓励卡片
    pub async fn get_random(pool: &PgPool) -> sqlx::Result<Option<EncourageCard>> {
        sqlx::query_as::<_, EncourageCard>(
            "SELECT * FROM encourage_cards ORDER BY RANDOM() LIMIT 1"
        )
        .fetch_optional(pool)
        .await
    }

    /// 获取卡片列表
    pub async fn list(pool: &PgPool, page: &PageRequest) -> sqlx::Result<Vec<EncourageCard>> {
        sqlx::query_as::<_, EncourageCard>(
            "SELECT * FROM encourage_cards ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(page.page_size)
        .bind(page.offset())
        .fetch_all(pool)
        .await
    }
}

/// 哲理命题 Repository
pub struct PhilosophyCardRepository;

impl PhilosophyCardRepository {
    /// 获取今日哲理
    pub async fn get_today(pool: &PgPool) -> sqlx::Result<Option<PhilosophyCard>> {
        sqlx::query_as::<_, PhilosophyCard>(
            "SELECT * FROM philosophy_cards ORDER BY RANDOM() LIMIT 1"
        )
        .fetch_optional(pool)
        .await
    }

    /// 获取卡片列表
    pub async fn list(pool: &PgPool, page: &PageRequest) -> sqlx::Result<Vec<PhilosophyCard>> {
        sqlx::query_as::<_, PhilosophyCard>(
            "SELECT * FROM philosophy_cards ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(page.page_size)
        .bind(page.offset())
        .fetch_all(pool)
        .await
    }
}

/// 写作作品 Repository
pub struct WritingWorkRepository;

impl WritingWorkRepository {
    /// 创建作品
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        title: &str,
        content: &str,
        category: &str,
        tags: &[String],
    ) -> sqlx::Result<WritingWork> {
        sqlx::query_as::<_, WritingWork>(
            r#"
            INSERT INTO writing_works (id, user_id, title, content, category, tags)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(title)
        .bind(content)
        .bind(category)
        .bind(tags)
        .fetch_one(pool)
        .await
    }

    /// 获取作品列表
    pub async fn list(pool: &PgPool, page: &PageRequest) -> sqlx::Result<Vec<WritingWork>> {
        sqlx::query_as::<_, WritingWork>(
            "SELECT * FROM writing_works ORDER BY created_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(page.page_size)
        .bind(page.offset())
        .fetch_all(pool)
        .await
    }

    /// 增加浏览量
    pub async fn increment_views(pool: &PgPool, id: Uuid) -> sqlx::Result<()> {
        sqlx::query("UPDATE writing_works SET views = views + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 增加点赞数
    pub async fn increment_likes(pool: &PgPool, id: Uuid) -> sqlx::Result<()> {
        sqlx::query("UPDATE writing_works SET likes = likes + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

/// 论坛帖子 Repository
pub struct ForumPostRepository;

impl ForumPostRepository {
    /// 创建帖子
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        post_type: &str,
        title: &str,
        content: &str,
        card_category: Option<&str>,
        author: Option<&str>,
        topic_question: Option<&str>,
        topic_options: Option<&[String]>,
        topic_answer: Option<&str>,
        creation_category: Option<&str>,
        creation_tags: Option<&[String]>,
        image_url: Option<&str>,
    ) -> sqlx::Result<ForumPost> {
        sqlx::query_as::<_, ForumPost>(
            r#"
            INSERT INTO forum_posts (
                id, user_id, post_type, title, content,
                card_category, author, topic_question, topic_options, topic_answer,
                creation_category, creation_tags, image_url
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(post_type)
        .bind(title)
        .bind(content)
        .bind(card_category)
        .bind(author)
        .bind(topic_question)
        .bind(topic_options.map(|v| v.to_vec()))
        .bind(topic_answer)
        .bind(creation_category)
        .bind(creation_tags.map(|v| v.to_vec()))
        .bind(image_url)
        .fetch_one(pool)
        .await
    }

    /// 获取帖子列表
    pub async fn list(
        pool: &PgPool,
        post_type: Option<&str>,
        page: &PageRequest,
    ) -> sqlx::Result<Vec<ForumPost>> {
        let query = if let Some(pt) = post_type {
            sqlx::query_as::<_, ForumPost>(
                "SELECT * FROM forum_posts WHERE post_type = $1 ORDER BY is_pinned DESC, created_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(pt)
            .bind(page.page_size)
            .bind(page.offset())
        } else {
            sqlx::query_as::<_, ForumPost>(
                "SELECT * FROM forum_posts ORDER BY is_pinned DESC, created_at DESC LIMIT $1 OFFSET $2"
            )
            .bind(page.page_size)
            .bind(page.offset())
        };
        
        query.fetch_all(pool).await
    }

    /// 根据ID获取帖子
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<ForumPost>> {
        sqlx::query_as::<_, ForumPost>("SELECT * FROM forum_posts WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// 增加浏览量
    pub async fn increment_views(pool: &PgPool, id: Uuid) -> sqlx::Result<()> {
        sqlx::query("UPDATE forum_posts SET views = views + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 增加点赞数
    pub async fn increment_likes(pool: &PgPool, id: Uuid) -> sqlx::Result<()> {
        sqlx::query("UPDATE forum_posts SET likes = likes + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 删除帖子
    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> sqlx::Result<bool> {
        let result = sqlx::query("DELETE FROM forum_posts WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

/// 论坛评论 Repository
pub struct ForumCommentRepository;

impl ForumCommentRepository {
    /// 创建评论
    pub async fn create(
        pool: &PgPool,
        post_id: Uuid,
        user_id: Option<Uuid>,
        parent_id: Option<Uuid>,
        content: &str,
        guest_name: Option<&str>,
    ) -> sqlx::Result<ForumComment> {
        sqlx::query_as::<_, ForumComment>(
            r#"
            INSERT INTO forum_comments (id, post_id, user_id, parent_id, content, guest_name)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(post_id)
        .bind(user_id)
        .bind(parent_id)
        .bind(content)
        .bind(guest_name)
        .fetch_one(pool)
        .await
    }

    /// 获取帖子的评论列表
    pub async fn list_by_post(
        pool: &PgPool,
        post_id: Uuid,
        page: &PageRequest,
    ) -> sqlx::Result<Vec<ForumComment>> {
        sqlx::query_as::<_, ForumComment>(
            "SELECT * FROM forum_comments WHERE post_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(post_id)
        .bind(page.page_size)
        .bind(page.offset())
        .fetch_all(pool)
        .await
    }

    /// 增加评论点赞数
    pub async fn increment_likes(pool: &PgPool, id: Uuid) -> sqlx::Result<()> {
        sqlx::query("UPDATE forum_comments SET likes = likes + 1 WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

/// 记账记录 Repository
pub struct AccountRecordRepository;

impl AccountRecordRepository {
    /// 创建记账记录
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        record_type: &str,
        amount: i64,
        category: &str,
        description: Option<&str>,
        date: &str,
    ) -> sqlx::Result<AccountRecord> {
        sqlx::query_as::<_, AccountRecord>(
            r#"
            INSERT INTO account_records (id, user_id, record_type, amount, category, description, date)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(record_type)
        .bind(amount)
        .bind(category)
        .bind(description)
        .bind(date)
        .fetch_one(pool)
        .await
    }

    /// 获取用户的记账记录列表
    pub async fn list_by_user(
        pool: &PgPool,
        user_id: Uuid,
        page: &PageRequest,
    ) -> sqlx::Result<Vec<AccountRecord>> {
        sqlx::query_as::<_, AccountRecord>(
            "SELECT * FROM account_records WHERE user_id = $1 ORDER BY date DESC, created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(user_id)
        .bind(page.page_size)
        .bind(page.offset())
        .fetch_all(pool)
        .await
    }

    /// 根据ID获取记账记录
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> sqlx::Result<Option<AccountRecord>> {
        sqlx::query_as::<_, AccountRecord>(
            "SELECT * FROM account_records WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    /// 更新记账记录
    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        record_type: &str,
        amount: i64,
        category: &str,
        description: Option<&str>,
        date: &str,
    ) -> sqlx::Result<AccountRecord> {
        sqlx::query_as::<_, AccountRecord>(
            r#"
            UPDATE account_records
            SET record_type = $3, amount = $4, category = $5, description = $6, date = $7
            WHERE id = $1 AND user_id = $2
            RETURNING *
            "#
        )
        .bind(id)
        .bind(user_id)
        .bind(record_type)
        .bind(amount)
        .bind(category)
        .bind(description)
        .bind(date)
        .fetch_one(pool)
        .await
    }

    /// 删除记账记录
    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> sqlx::Result<bool> {
        let result = sqlx::query(
            "DELETE FROM account_records WHERE id = $1 AND user_id = $2"
        )
        .bind(id)
        .bind(user_id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }

    /// 获取用户的收支统计
    pub async fn get_summary(pool: &PgPool, user_id: Uuid) -> sqlx::Result<(i64, i64)> {
        // 使用 CAST 确保返回 BIGINT 类型
        let result = sqlx::query(
            r#"
            SELECT 
                CAST(COALESCE(SUM(CASE WHEN record_type = 'income' THEN amount ELSE 0 END), 0) AS BIGINT) as total_income,
                CAST(COALESCE(SUM(CASE WHEN record_type = 'expense' THEN amount ELSE 0 END), 0) AS BIGINT) as total_expense
            FROM account_records
            WHERE user_id = $1
            "#
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        let total_income: i64 = result.try_get("total_income")?;
        let total_expense: i64 = result.try_get("total_expense")?;

        Ok((total_income, total_expense))
    }

    /// 获取月账单（包含每日和分类统计）
    pub async fn get_monthly_bill(pool: &PgPool, user_id: Uuid, year: i32, month: i32) -> sqlx::Result<serde_json::Value> {
        use serde_json::json;

        // 构造月份的开始和结束日期
        let start_date = format!("{:04}-{:02}-01", year, month);
        let end_date = if month == 12 {
            format!("{:04}-01-01", year + 1)
        } else {
            format!("{:04}-{:02}-01", year, month + 1)
        };

        // 获取月度总计
        let total = sqlx::query(
            r#"
            SELECT 
                CAST(COALESCE(SUM(CASE WHEN record_type = 'income' THEN amount ELSE 0 END), 0) AS BIGINT) as total_income,
                CAST(COALESCE(SUM(CASE WHEN record_type = 'expense' THEN amount ELSE 0 END), 0) AS BIGINT) as total_expense
            FROM account_records
            WHERE user_id = $1 AND date >= $2 AND date < $3
            "#
        )
        .bind(user_id)
        .bind(&start_date)
        .bind(&end_date)
        .fetch_one(pool)
        .await?;

        let total_income: i64 = total.try_get("total_income")?;
        let total_expense: i64 = total.try_get("total_expense")?;

        // 获取每日统计
        let daily = sqlx::query(
            r#"
            SELECT 
                date,
                CAST(SUM(CASE WHEN record_type = 'income' THEN amount ELSE 0 END) AS BIGINT) as income,
                CAST(SUM(CASE WHEN record_type = 'expense' THEN amount ELSE 0 END) AS BIGINT) as expense
            FROM account_records
            WHERE user_id = $1 AND date >= $2 AND date < $3
            GROUP BY date
            ORDER BY date
            "#
        )
        .bind(user_id)
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(pool)
        .await?;

        let daily_stats: Vec<serde_json::Value> = daily.iter().map(|row| {
            json!({
                "date": row.try_get::<String, _>("date").unwrap_or_default(),
                "income": row.try_get::<i64, _>("income").unwrap_or(0),
                "expense": row.try_get::<i64, _>("expense").unwrap_or(0)
            })
        }).collect();

        // 获取分类统计
        let by_category = sqlx::query(
            r#"
            SELECT 
                record_type,
                category,
                CAST(SUM(amount) AS BIGINT) as total
            FROM account_records
            WHERE user_id = $1 AND date >= $2 AND date < $3
            GROUP BY record_type, category
            ORDER BY total DESC
            "#
        )
        .bind(user_id)
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(pool)
        .await?;

        let category_stats: Vec<serde_json::Value> = by_category.iter().map(|row| {
            json!({
                "record_type": row.try_get::<String, _>("record_type").unwrap_or_default(),
                "category": row.try_get::<String, _>("category").unwrap_or_default(),
                "total": row.try_get::<i64, _>("total").unwrap_or(0)
            })
        }).collect();

        Ok(json!({
            "year": year,
            "month": month,
            "total_income": total_income,
            "total_expense": total_expense,
            "balance": total_income - total_expense,
            "daily": daily_stats,
            "by_category": category_stats
        }))
    }

    /// 获取年账单（包含每月统计和分类统计）
    pub async fn get_yearly_bill(pool: &PgPool, user_id: Uuid, year: i32) -> sqlx::Result<serde_json::Value> {
        use serde_json::json;

        // 构造年份的开始和结束日期
        let start_date = format!("{:04}-01-01", year);
        let end_date = format!("{:04}-01-01", year + 1);

        // 获取年度总计
        let total = sqlx::query(
            r#"
            SELECT 
                CAST(COALESCE(SUM(CASE WHEN record_type = 'income' THEN amount ELSE 0 END), 0) AS BIGINT) as total_income,
                CAST(COALESCE(SUM(CASE WHEN record_type = 'expense' THEN amount ELSE 0 END), 0) AS BIGINT) as total_expense
            FROM account_records
            WHERE user_id = $1 AND date >= $2 AND date < $3
            "#
        )
        .bind(user_id)
        .bind(&start_date)
        .bind(&end_date)
        .fetch_one(pool)
        .await?;

        let total_income: i64 = total.try_get("total_income")?;
        let total_expense: i64 = total.try_get("total_expense")?;

        // 获取每月统计
        let monthly = sqlx::query(
            r#"
            SELECT 
                SUBSTRING(date FROM 1 FOR 7) as month,
                CAST(SUM(CASE WHEN record_type = 'income' THEN amount ELSE 0 END) AS BIGINT) as income,
                CAST(SUM(CASE WHEN record_type = 'expense' THEN amount ELSE 0 END) AS BIGINT) as expense
            FROM account_records
            WHERE user_id = $1 AND date >= $2 AND date < $3
            GROUP BY SUBSTRING(date FROM 1 FOR 7)
            ORDER BY month
            "#
        )
        .bind(user_id)
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(pool)
        .await?;

        let monthly_stats: Vec<serde_json::Value> = monthly.iter().map(|row| {
            json!({
                "month": row.try_get::<String, _>("month").unwrap_or_default(),
                "income": row.try_get::<i64, _>("income").unwrap_or(0),
                "expense": row.try_get::<i64, _>("expense").unwrap_or(0)
            })
        }).collect();

        // 获取分类统计
        let by_category = sqlx::query(
            r#"
            SELECT 
                record_type,
                category,
                CAST(SUM(amount) AS BIGINT) as total
            FROM account_records
            WHERE user_id = $1 AND date >= $2 AND date < $3
            GROUP BY record_type, category
            ORDER BY total DESC
            "#
        )
        .bind(user_id)
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(pool)
        .await?;

        let category_stats: Vec<serde_json::Value> = by_category.iter().map(|row| {
            json!({
                "record_type": row.try_get::<String, _>("record_type").unwrap_or_default(),
                "category": row.try_get::<String, _>("category").unwrap_or_default(),
                "total": row.try_get::<i64, _>("total").unwrap_or(0)
            })
        }).collect();

        Ok(json!({
            "year": year,
            "total_income": total_income,
            "total_expense": total_expense,
            "balance": total_income - total_expense,
            "monthly": monthly_stats,
            "by_category": category_stats
        }))
    }
}

