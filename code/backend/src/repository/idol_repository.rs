use crate::model::idol::*;
use crate::utils::error::AppResult;
use sqlx::PgPool;
use uuid::Uuid;

pub struct IdolRepository;

impl IdolRepository {
    pub async fn create(pool: &PgPool, user_id: Uuid, req: &CreateIdolRequest) -> AppResult<Idol> {
        // 如果tags存在且不为空，则包含tags字段；否则不包含
        let has_tags = req.tags.as_ref().map_or(false, |t| !t.is_empty());
        
        let idol = if has_tags {
            sqlx::query_as::<_, Idol>(
                r#"INSERT INTO idols (user_id, name, description, avatar_url, birth_date, 
                                 nationality, profession, tags, is_public)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"#,
            )
            .bind(user_id)
            .bind(&req.name)
            .bind(&req.description)
            .bind(&req.avatar_url)
            .bind(&req.birth_date)
            .bind(&req.nationality)
            .bind(&req.profession)
            .bind(&req.tags)
            .bind(req.is_public.unwrap_or(true))
            .fetch_one(pool).await?
        } else {
            sqlx::query_as::<_, Idol>(
                r#"INSERT INTO idols (user_id, name, description, avatar_url, birth_date, 
                                 nationality, profession, is_public)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *"#,
            )
            .bind(user_id)
            .bind(&req.name)
            .bind(&req.description)
            .bind(&req.avatar_url)
            .bind(&req.birth_date)
            .bind(&req.nationality)
            .bind(&req.profession)
            .bind(req.is_public.unwrap_or(true))
            .fetch_one(pool).await?
        };
        Ok(idol)
    }

    pub async fn find_by_id(pool: &PgPool, idol_id: Uuid) -> AppResult<Option<Idol>> {
        let idol = sqlx::query_as::<_, Idol>("SELECT * FROM idols WHERE id = $1")
            .bind(idol_id).fetch_optional(pool).await?;
        Ok(idol)
    }

    pub async fn update(pool: &PgPool, idol_id: Uuid, req: &UpdateIdolRequest) -> AppResult<Idol> {
        let idol = sqlx::query_as::<_, Idol>(
            r#"UPDATE idols SET name = COALESCE($2, name), description = COALESCE($3, description),
                avatar_url = COALESCE($4, avatar_url), birth_date = COALESCE($5, birth_date),
                nationality = COALESCE($6, nationality), profession = COALESCE($7, profession),
                tags = COALESCE($8, tags), is_public = COALESCE($9, is_public)
            WHERE id = $1 RETURNING *"#,
        )
        .bind(idol_id)
        .bind(req.name.as_deref())
        .bind(req.description.as_deref())
        .bind(req.avatar_url.as_deref())
        .bind(req.birth_date)
        .bind(req.nationality.as_deref())
        .bind(req.profession.as_deref())
        .bind(req.tags.as_ref().filter(|v| !v.is_empty()))
        .bind(req.is_public)
        .fetch_one(pool).await?;
        Ok(idol)
    }

    pub async fn delete(pool: &PgPool, idol_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM idols WHERE id = $1").bind(idol_id).execute(pool).await?;
        Ok(())
    }

    pub async fn find_paginated(pool: &PgPool, user_id: Option<Uuid>, params: &IdolQueryParams) -> AppResult<PaginatedIdols> {
        let page = params.page.unwrap_or(1).max(1);
        let page_size = params.page_size.unwrap_or(10).min(100);
        let offset = (page - 1) * page_size;

        let mut query = String::from("SELECT * FROM idols WHERE 1=1");
        let mut count_query = String::from("SELECT COUNT(*) FROM idols WHERE 1=1");

        if let Some(uid) = user_id {
            query.push_str(&format!(" AND user_id = '{}'", uid));
            count_query.push_str(&format!(" AND user_id = '{}'", uid));
        }

        if params.only_public.unwrap_or(false) {
            query.push_str(" AND is_public = true");
            count_query.push_str(" AND is_public = true");
        }

        if let Some(search) = &params.search {
            let search_pattern = format!("%{}%", search);
            query.push_str(&format!(" AND (name ILIKE '{}' OR description ILIKE '{}')", search_pattern, search_pattern));
            count_query.push_str(&format!(" AND (name ILIKE '{}' OR description ILIKE '{}')", search_pattern, search_pattern));
        }

        if let Some(tags) = &params.tags {
            if !tags.is_empty() {
                query.push_str(&format!(" AND tags && ARRAY{:?}::text[]", tags));
                count_query.push_str(&format!(" AND tags && ARRAY{:?}::text[]", tags));
            }
        }

        query.push_str(" ORDER BY created_at DESC");
        query.push_str(&format!(" LIMIT {} OFFSET {}", page_size, offset));

        let idols = sqlx::query_as::<_, Idol>(&query).fetch_all(pool).await?;
        let total: (i64,) = sqlx::query_as(&count_query).fetch_one(pool).await?;
        let total_pages = (total.0 + page_size - 1) / page_size;

        Ok(PaginatedIdols { items: idols, total: total.0, page, page_size, total_pages })
    }

    pub async fn check_ownership(pool: &PgPool, idol_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result: Option<(bool,)> = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM idols WHERE id = $1 AND user_id = $2)",
        ).bind(idol_id).bind(user_id).fetch_optional(pool).await?;
        Ok(result.map(|r| r.0).unwrap_or(false))
    }

    pub async fn create_quote(pool: &PgPool, idol_id: Uuid, req: &CreateQuoteRequest) -> AppResult<IdolQuote> {
        let quote = sqlx::query_as::<_, IdolQuote>(
            r#"INSERT INTO idol_quotes (idol_id, content, source, quote_date)
            VALUES ($1, $2, $3, $4) RETURNING *"#,
        ).bind(idol_id).bind(&req.content).bind(&req.source).bind(&req.quote_date)
        .fetch_one(pool).await?;
        Ok(quote)
    }

    pub async fn find_quotes_by_idol(pool: &PgPool, idol_id: Uuid) -> AppResult<Vec<IdolQuote>> {
        let quotes = sqlx::query_as::<_, IdolQuote>(
            "SELECT * FROM idol_quotes WHERE idol_id = $1 ORDER BY created_at DESC",
        ).bind(idol_id).fetch_all(pool).await?;
        Ok(quotes)
    }

    pub async fn update_quote(pool: &PgPool, quote_id: Uuid, req: &UpdateQuoteRequest) -> AppResult<IdolQuote> {
        let quote = sqlx::query_as::<_, IdolQuote>(
            r#"UPDATE idol_quotes SET content = COALESCE($2, content),
                source = COALESCE($3, source), quote_date = COALESCE($4, quote_date)
            WHERE id = $1 RETURNING *"#,
        ).bind(quote_id).bind(&req.content).bind(&req.source).bind(&req.quote_date)
        .fetch_one(pool).await?;
        Ok(quote)
    }

    pub async fn delete_quote(pool: &PgPool, quote_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM idol_quotes WHERE id = $1").bind(quote_id).execute(pool).await?;
        Ok(())
    }

    pub async fn create_work(pool: &PgPool, idol_id: Uuid, req: &CreateWorkRequest) -> AppResult<IdolWork> {
        let work = sqlx::query_as::<_, IdolWork>(
            r#"INSERT INTO idol_works (idol_id, title, description, work_type, file_url, 
                                   thumbnail_url, file_size, duration, release_date, tags)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *"#,
        ).bind(idol_id).bind(&req.title).bind(&req.description).bind(&req.work_type)
        .bind(&req.file_url).bind(&req.thumbnail_url).bind(&req.file_size)
        .bind(&req.duration).bind(&req.release_date).bind(&req.tags)
        .fetch_one(pool).await?;
        Ok(work)
    }

    pub async fn find_works_by_idol(pool: &PgPool, idol_id: Uuid) -> AppResult<Vec<IdolWork>> {
        let works = sqlx::query_as::<_, IdolWork>(
            "SELECT * FROM idol_works WHERE idol_id = $1 ORDER BY created_at DESC",
        ).bind(idol_id).fetch_all(pool).await?;
        Ok(works)
    }

    pub async fn find_work_by_id(pool: &PgPool, work_id: Uuid) -> AppResult<Option<IdolWork>> {
        let work = sqlx::query_as::<_, IdolWork>("SELECT * FROM idol_works WHERE id = $1")
            .bind(work_id).fetch_optional(pool).await?;
        Ok(work)
    }

    pub async fn update_work(pool: &PgPool, work_id: Uuid, req: &UpdateWorkRequest) -> AppResult<IdolWork> {
        let work = sqlx::query_as::<_, IdolWork>(
            r#"UPDATE idol_works SET title = COALESCE($2, title), description = COALESCE($3, description),
                work_type = COALESCE($4, work_type), file_url = COALESCE($5, file_url),
                thumbnail_url = COALESCE($6, thumbnail_url), file_size = COALESCE($7, file_size),
                duration = COALESCE($8, duration), release_date = COALESCE($9, release_date),
                tags = COALESCE($10, tags) WHERE id = $1 RETURNING *"#,
        ).bind(work_id).bind(&req.title).bind(&req.description).bind(&req.work_type)
        .bind(&req.file_url).bind(&req.thumbnail_url).bind(&req.file_size)
        .bind(&req.duration).bind(&req.release_date).bind(&req.tags)
        .fetch_one(pool).await?;
        Ok(work)
    }

    pub async fn delete_work(pool: &PgPool, work_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM idol_works WHERE id = $1").bind(work_id).execute(pool).await?;
        Ok(())
    }

    pub async fn increment_view_count(pool: &PgPool, work_id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE idol_works SET view_count = view_count + 1 WHERE id = $1")
            .bind(work_id).execute(pool).await?;
        Ok(())
    }

    pub async fn like_work(pool: &PgPool, work_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query(
            r#"INSERT INTO idol_work_likes (work_id, user_id) VALUES ($1, $2)
            ON CONFLICT (work_id, user_id) DO NOTHING"#,
        ).bind(work_id).bind(user_id).execute(pool).await?;

        if result.rows_affected() > 0 {
            sqlx::query("UPDATE idol_works SET like_count = like_count + 1 WHERE id = $1")
                .bind(work_id).execute(pool).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn unlike_work(pool: &PgPool, work_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result = sqlx::query("DELETE FROM idol_work_likes WHERE work_id = $1 AND user_id = $2")
            .bind(work_id).bind(user_id).execute(pool).await?;

        if result.rows_affected() > 0 {
            sqlx::query("UPDATE idol_works SET like_count = GREATEST(like_count - 1, 0) WHERE id = $1")
                .bind(work_id).execute(pool).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn check_work_liked(pool: &PgPool, work_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        let result: Option<(bool,)> = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM idol_work_likes WHERE work_id = $1 AND user_id = $2)",
        ).bind(work_id).bind(user_id).fetch_optional(pool).await?;
        Ok(result.map(|r| r.0).unwrap_or(false))
    }
}
