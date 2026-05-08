/**
 * 游戏 API
 */

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    api::AppState,
    middleware::auth::AuthUser,
    model::ApiResponse,
    utils::error::{AppError, AppResult},
};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct GameScore {
    pub id: Uuid,
    pub user_id: Uuid,
    pub game_type: String,
    pub level: i32,
    pub score: i32,
    pub time_spent: Option<i32>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitScoreRequest {
    pub game_type: String,
    pub level: i32,
    pub score: i32,
    pub time_spent: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RankingQuery {
    pub game_type: String,
    pub level: i32,
}

#[derive(Debug, Serialize)]
pub struct RankingItem {
    pub rank: i32,
    pub user_id: String,
    pub username: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub score: i32,
    pub time_spent: Option<i32>,
    pub is_friend: bool,
    pub is_self: bool,
}

/// 配置游戏路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/scores", post(submit_score))
        .route("/scores/my", get(get_my_scores))
        .route("/ranking", get(get_ranking))
        .route("/ranking/friends", get(get_friends_ranking))
}

/// 提交游戏分数
async fn submit_score(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(req): Json<SubmitScoreRequest>,
) -> AppResult<Json<ApiResponse<GameScore>>> {
    // 验证游戏类型
    let valid_types = vec!["bubble_pop", "memory_card", "puzzle", "whack_mole"];
    if !valid_types.contains(&req.game_type.as_str()) {
        return Err(AppError::BadRequest("Invalid game type".to_string()));
    }
    
    // 验证关卡
    if req.level < 1 || req.level > 10 {
        return Err(AppError::BadRequest("Level must be between 1 and 10".to_string()));
    }
    
    // 插入分数记录
    let score = sqlx::query_as::<_, GameScore>(
        r#"
        INSERT INTO game_scores (user_id, game_type, level, score, time_spent)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(auth_user.user_id)
    .bind(&req.game_type)
    .bind(req.level)
    .bind(req.score)
    .bind(req.time_spent)
    .fetch_one(&state.db_pool)
    .await?;
    
    Ok(Json(ApiResponse::success(score)))
}

/// 获取我的游戏记录
async fn get_my_scores(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<RankingQuery>,
) -> AppResult<Json<ApiResponse<Vec<GameScore>>>> {
    let scores = sqlx::query_as::<_, GameScore>(
        r#"
        SELECT * FROM game_scores
        WHERE user_id = $1 AND game_type = $2 AND level = $3
        ORDER BY score DESC, time_spent ASC
        LIMIT 10
        "#
    )
    .bind(auth_user.user_id)
    .bind(&query.game_type)
    .bind(query.level)
    .fetch_all(&state.db_pool)
    .await?;
    
    Ok(Json(ApiResponse::success(scores)))
}

/// 获取好友排行榜
async fn get_friends_ranking(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<RankingQuery>,
) -> AppResult<Json<ApiResponse<Vec<RankingItem>>>> {
    // 获取好友ID列表
    let friend_records = sqlx::query!(
        r#"
        SELECT friend_id FROM friendships
        WHERE user_id = $1 AND status = 'accepted'
        UNION
        SELECT user_id FROM friendships
        WHERE friend_id = $1 AND status = 'accepted'
        "#,
        auth_user.user_id
    )
    .fetch_all(&state.db_pool)
    .await?;
    
    let friend_ids: Vec<Uuid> = friend_records
        .into_iter()
        .filter_map(|r| r.friend_id)
        .collect();
    
    // 查询好友的最佳成绩
    let mut rankings = Vec::new();
    
    // 包含自己
    let mut user_ids = friend_ids.clone();
    user_ids.push(auth_user.user_id);
    
    for user_id in user_ids {
        let best_score = sqlx::query!(
            r#"
            SELECT 
                gs.score,
                gs.time_spent,
                u.id as user_id,
                u.username,
                u.nickname as "nickname?",
                u.avatar as "avatar?"
            FROM game_scores gs
            JOIN users u ON gs.user_id = u.id
            WHERE gs.user_id = $1 AND gs.game_type = $2 AND gs.level = $3
            ORDER BY gs.score DESC, gs.time_spent ASC
            LIMIT 1
            "#,
            user_id,
            &query.game_type,
            query.level
        )
        .fetch_optional(&state.db_pool)
        .await?;
        
        if let Some(record) = best_score {
            rankings.push(RankingItem {
                rank: 0, // 稍后计算
                user_id: record.user_id.to_string(),
                username: record.username,
                nickname: record.nickname,
                avatar: record.avatar,
                score: record.score,
                time_spent: record.time_spent,
                is_friend: friend_ids.contains(&record.user_id),
                is_self: record.user_id == auth_user.user_id,
            });
        }
    }
    
    // 排序并分配排名
    rankings.sort_by(|a, b| {
        b.score.cmp(&a.score).then_with(|| {
            match (a.time_spent, b.time_spent) {
                (Some(t1), Some(t2)) => t1.cmp(&t2),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        })
    });
    
    for (i, item) in rankings.iter_mut().enumerate() {
        item.rank = (i + 1) as i32;
    }
    
    Ok(Json(ApiResponse::success(rankings)))
}

/// 获取全服排行榜
async fn get_ranking(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<RankingQuery>,
) -> AppResult<Json<ApiResponse<Vec<RankingItem>>>> {
    let records = sqlx::query!(
        r#"
        SELECT DISTINCT ON (gs.user_id)
            gs.score,
            gs.time_spent,
            u.id as user_id,
            u.username,
            u.nickname as "nickname?",
            u.avatar as "avatar?"
        FROM game_scores gs
        JOIN users u ON gs.user_id = u.id
        WHERE gs.game_type = $1 AND gs.level = $2
        ORDER BY gs.user_id, gs.score DESC, gs.time_spent ASC
        "#,
        &query.game_type,
        query.level
    )
    .fetch_all(&state.db_pool)
    .await?;
    
    // 获取好友列表
    let friend_records = sqlx::query!(
        r#"
        SELECT friend_id FROM friendships
        WHERE user_id = $1 AND status = 'accepted'
        UNION
        SELECT user_id FROM friendships
        WHERE friend_id = $1 AND status = 'accepted'
        "#,
        auth_user.user_id
    )
    .fetch_all(&state.db_pool)
    .await?;
    
    let friend_ids: Vec<Uuid> = friend_records
        .into_iter()
        .filter_map(|r| r.friend_id)
        .collect();
    
    let mut rankings: Vec<RankingItem> = records
        .into_iter()
        .map(|r| RankingItem {
            rank: 0,
            user_id: r.user_id.to_string(),
            username: r.username,
            nickname: r.nickname,
            avatar: r.avatar,
            score: r.score,
            time_spent: r.time_spent,
            is_friend: friend_ids.contains(&r.user_id),
            is_self: r.user_id == auth_user.user_id,
        })
        .collect();
    
    // 排序并分配排名
    rankings.sort_by(|a, b| {
        b.score.cmp(&a.score).then_with(|| {
            match (a.time_spent, b.time_spent) {
                (Some(t1), Some(t2)) => t1.cmp(&t2),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => std::cmp::Ordering::Equal,
            }
        })
    });
    
    for (i, item) in rankings.iter_mut().enumerate() {
        item.rank = (i + 1) as i32;
    }
    
    // 只返回前100名
    rankings.truncate(100);
    
    Ok(Json(ApiResponse::success(rankings)))
}
