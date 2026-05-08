-- 创建用户脑筋急转弯题库表

CREATE TABLE IF NOT EXISTS user_brain_teasers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    is_public BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_user_brain_teasers_user_id ON user_brain_teasers(user_id);
CREATE INDEX IF NOT EXISTS idx_user_brain_teasers_is_public ON user_brain_teasers(is_public);
CREATE INDEX IF NOT EXISTS idx_user_brain_teasers_created_at ON user_brain_teasers(created_at DESC);

-- 添加注释
COMMENT ON TABLE user_brain_teasers IS '用户自定义脑筋急转弯题库';
COMMENT ON COLUMN user_brain_teasers.question IS '问题内容';
COMMENT ON COLUMN user_brain_teasers.answer IS '答案';
COMMENT ON COLUMN user_brain_teasers.is_public IS '是否公开（供其他用户使用）';
