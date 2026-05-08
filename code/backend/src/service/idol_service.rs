use crate::model::idol::*;
use crate::repository::idol_repository::IdolRepository;
use crate::utils::error::{AppError, AppResult};
use sqlx::PgPool;
use uuid::Uuid;

pub struct IdolService;

impl IdolService {
    pub async fn create_idol(pool: &PgPool, user_id: Uuid, req: CreateIdolRequest) -> AppResult<Idol> {
        IdolRepository::create(pool, user_id, &req).await
    }

    pub async fn get_idol_detail(pool: &PgPool, idol_id: Uuid) -> AppResult<IdolDetail> {
        let idol = IdolRepository::find_by_id(pool, idol_id).await?
            .ok_or_else(|| AppError::NotFound("Idol not found".to_string()))?;
        let quotes = IdolRepository::find_quotes_by_idol(pool, idol_id).await?;
        let works = IdolRepository::find_works_by_idol(pool, idol_id).await?;
        Ok(IdolDetail { idol, quotes, works })
    }

    pub async fn update_idol(pool: &PgPool, idol_id: Uuid, user_id: Uuid, req: UpdateIdolRequest) -> AppResult<Idol> {
        if !IdolRepository::check_ownership(pool, idol_id, user_id).await? {
            return Err(AppError::Forbidden("No permission".to_string()));
        }
        IdolRepository::update(pool, idol_id, &req).await
    }

    pub async fn delete_idol(pool: &PgPool, idol_id: Uuid, user_id: Uuid) -> AppResult<()> {
        if !IdolRepository::check_ownership(pool, idol_id, user_id).await? {
            return Err(AppError::Forbidden("No permission".to_string()));
        }
        IdolRepository::delete(pool, idol_id).await
    }

    pub async fn list_idols(pool: &PgPool, user_id: Option<Uuid>, params: IdolQueryParams) -> AppResult<PaginatedIdols> {
        IdolRepository::find_paginated(pool, user_id, &params).await
    }

    pub async fn add_quote(pool: &PgPool, idol_id: Uuid, user_id: Uuid, req: CreateQuoteRequest) -> AppResult<IdolQuote> {
        if !IdolRepository::check_ownership(pool, idol_id, user_id).await? {
            return Err(AppError::Forbidden("No permission".to_string()));
        }
        IdolRepository::create_quote(pool, idol_id, &req).await
    }

    pub async fn update_quote(pool: &PgPool, quote_id: Uuid, user_id: Uuid, req: UpdateQuoteRequest) -> AppResult<IdolQuote> {
        let quote = sqlx::query_as::<_, IdolQuote>("SELECT * FROM idol_quotes WHERE id = $1")
            .bind(quote_id).fetch_optional(pool).await?
            .ok_or_else(|| AppError::NotFound("Quote not found".to_string()))?;
        if !IdolRepository::check_ownership(pool, quote.idol_id, user_id).await? {
            return Err(AppError::Forbidden("No permission".to_string()));
        }
        IdolRepository::update_quote(pool, quote_id, &req).await
    }

    pub async fn delete_quote(pool: &PgPool, quote_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let quote = sqlx::query_as::<_, IdolQuote>("SELECT * FROM idol_quotes WHERE id = $1")
            .bind(quote_id).fetch_optional(pool).await?
            .ok_or_else(|| AppError::NotFound("Quote not found".to_string()))?;
        if !IdolRepository::check_ownership(pool, quote.idol_id, user_id).await? {
            return Err(AppError::Forbidden("No permission".to_string()));
        }
        IdolRepository::delete_quote(pool, quote_id).await
    }

    pub async fn add_work(pool: &PgPool, idol_id: Uuid, user_id: Uuid, req: CreateWorkRequest) -> AppResult<IdolWork> {
        if !IdolRepository::check_ownership(pool, idol_id, user_id).await? {
            return Err(AppError::Forbidden("No permission".to_string()));
        }
        IdolRepository::create_work(pool, idol_id, &req).await
    }

    pub async fn update_work(pool: &PgPool, work_id: Uuid, user_id: Uuid, req: UpdateWorkRequest) -> AppResult<IdolWork> {
        let work = IdolRepository::find_work_by_id(pool, work_id).await?
            .ok_or_else(|| AppError::NotFound("Work not found".to_string()))?;
        if !IdolRepository::check_ownership(pool, work.idol_id, user_id).await? {
            return Err(AppError::Forbidden("No permission".to_string()));
        }
        IdolRepository::update_work(pool, work_id, &req).await
    }

    pub async fn delete_work(pool: &PgPool, work_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let work = IdolRepository::find_work_by_id(pool, work_id).await?
            .ok_or_else(|| AppError::NotFound("Work not found".to_string()))?;
        if !IdolRepository::check_ownership(pool, work.idol_id, user_id).await? {
            return Err(AppError::Forbidden("No permission".to_string()));
        }
        IdolRepository::delete_work(pool, work_id).await
    }

    pub async fn view_work(pool: &PgPool, work_id: Uuid) -> AppResult<IdolWork> {
        IdolRepository::increment_view_count(pool, work_id).await?;
        IdolRepository::find_work_by_id(pool, work_id).await?
            .ok_or_else(|| AppError::NotFound("Work not found".to_string()))
    }

    pub async fn like_work(pool: &PgPool, work_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        IdolRepository::find_work_by_id(pool, work_id).await?
            .ok_or_else(|| AppError::NotFound("Work not found".to_string()))?;
        IdolRepository::like_work(pool, work_id, user_id).await
    }

    pub async fn unlike_work(pool: &PgPool, work_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        IdolRepository::unlike_work(pool, work_id, user_id).await
    }

    pub async fn check_work_liked(pool: &PgPool, work_id: Uuid, user_id: Uuid) -> AppResult<bool> {
        IdolRepository::check_work_liked(pool, work_id, user_id).await
    }
}
