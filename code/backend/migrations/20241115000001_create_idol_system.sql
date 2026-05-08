-- 创建偶像系统相关表
-- 偶像表
CREATE TABLE idols (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    avatar_url VARCHAR(500),
    birth_date DATE,
    nationality VARCHAR(50),
    profession VARCHAR(100),
    tags TEXT[], -- 标签数组，如 ['歌手', '演员']
    is_public BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 偶像语录表
CREATE TABLE idol_quotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    idol_id UUID NOT NULL REFERENCES idols(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    source VARCHAR(200), -- 语录来源，如 '某某访谈'
    quote_date DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 偶像作品表
CREATE TABLE idol_works (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    idol_id UUID NOT NULL REFERENCES idols(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    work_type VARCHAR(20) NOT NULL, -- 'audio', 'video', 'image', 'other'
    file_url VARCHAR(500) NOT NULL,
    thumbnail_url VARCHAR(500), -- 缩略图
    file_size BIGINT, -- 文件大小（字节）
    duration INTEGER, -- 时长（秒），用于音视频
    release_date DATE,
    tags TEXT[],
    view_count INTEGER DEFAULT 0,
    like_count INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- 偶像作品点赞表
CREATE TABLE idol_work_likes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    work_id UUID NOT NULL REFERENCES idol_works(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(work_id, user_id)
);

-- 创建索引
CREATE INDEX idx_idols_user_id ON idols(user_id);
CREATE INDEX idx_idols_created_at ON idols(created_at DESC);
CREATE INDEX idx_idols_is_public ON idols(is_public);

CREATE INDEX idx_idol_quotes_idol_id ON idol_quotes(idol_id);
CREATE INDEX idx_idol_quotes_created_at ON idol_quotes(created_at DESC);

CREATE INDEX idx_idol_works_idol_id ON idol_works(idol_id);
CREATE INDEX idx_idol_works_work_type ON idol_works(work_type);
CREATE INDEX idx_idol_works_created_at ON idol_works(created_at DESC);

CREATE INDEX idx_idol_work_likes_work_id ON idol_work_likes(work_id);
CREATE INDEX idx_idol_work_likes_user_id ON idol_work_likes(user_id);

-- 创建更新时间触发器函数（如果不存在）
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 为各表添加更新时间触发器
CREATE TRIGGER update_idols_updated_at BEFORE UPDATE ON idols
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_idol_quotes_updated_at BEFORE UPDATE ON idol_quotes
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_idol_works_updated_at BEFORE UPDATE ON idol_works
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
