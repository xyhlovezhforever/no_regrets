-- 游戏分数和排行榜表

CREATE TABLE IF NOT EXISTS game_scores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    game_type VARCHAR(50) NOT NULL,  -- 游戏类型：bubble_pop, memory_card, puzzle, whack_mole
    level INTEGER NOT NULL,           -- 关卡等级
    score INTEGER NOT NULL,           -- 得分
    time_spent INTEGER,               -- 用时（秒）
    completed BOOLEAN NOT NULL DEFAULT true,  -- 是否完成
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_game_scores_user_id ON game_scores(user_id);
CREATE INDEX IF NOT EXISTS idx_game_scores_game_type ON game_scores(game_type);
CREATE INDEX IF NOT EXISTS idx_game_scores_level ON game_scores(level);
CREATE INDEX IF NOT EXISTS idx_game_scores_score ON game_scores(score DESC);
CREATE INDEX IF NOT EXISTS idx_game_scores_created_at ON game_scores(created_at DESC);

-- 创建复合索引用于排行榜查询
CREATE INDEX IF NOT EXISTS idx_game_scores_ranking ON game_scores(game_type, level, score DESC, time_spent ASC);

-- 添加注释
COMMENT ON TABLE game_scores IS '游戏分数记录表';
COMMENT ON COLUMN game_scores.game_type IS '游戏类型：bubble_pop(扎气球), memory_card(记忆翻牌), puzzle(拼图), whack_mole(打地鼠)';
COMMENT ON COLUMN game_scores.level IS '关卡等级（1-10）';
COMMENT ON COLUMN game_scores.score IS '游戏得分';
COMMENT ON COLUMN game_scores.time_spent IS '完成时间（秒）';
