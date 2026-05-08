-- 创建论坛系统表结构
-- 统一管理灵感、话题、创作等内容

-- 论坛帖子表
CREATE TABLE IF NOT EXISTS forum_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    post_type VARCHAR(20) NOT NULL, -- inspiration, topic, creation
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    -- 灵感卡片特有字段
    card_category VARCHAR(50), -- encourage, philosophy
    author VARCHAR(100), -- 哲理卡片作者
    -- 话题特有字段
    topic_question TEXT, -- 话题问题
    topic_options TEXT[], -- 话题选项（如果有）
    topic_answer TEXT, -- 用户的回答
    -- 创作特有字段
    creation_category VARCHAR(50), -- 随笔、日记、诗歌等
    creation_tags TEXT[], -- 创作标签
    -- 通用字段
    image_url VARCHAR(500), -- 图片URL
    likes INTEGER NOT NULL DEFAULT 0,
    views INTEGER NOT NULL DEFAULT 0,
    comments_count INTEGER NOT NULL DEFAULT 0,
    is_pinned BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_forum_posts_user_id ON forum_posts(user_id);
CREATE INDEX idx_forum_posts_type ON forum_posts(post_type);
CREATE INDEX idx_forum_posts_created_at ON forum_posts(created_at DESC);
CREATE INDEX idx_forum_posts_is_pinned ON forum_posts(is_pinned);

-- 论坛评论表
CREATE TABLE IF NOT EXISTS forum_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id UUID NOT NULL REFERENCES forum_posts(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL, -- 允许游客评论（NULL）
    parent_id UUID REFERENCES forum_comments(id) ON DELETE CASCADE, -- 支持回复评论
    content TEXT NOT NULL,
    guest_name VARCHAR(50), -- 游客名称（如果未登录）
    likes INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_forum_comments_post_id ON forum_comments(post_id);
CREATE INDEX idx_forum_comments_user_id ON forum_comments(user_id);
CREATE INDEX idx_forum_comments_parent_id ON forum_comments(parent_id);
CREATE INDEX idx_forum_comments_created_at ON forum_comments(created_at DESC);

-- 论坛点赞表（复用现有的likes表，但添加target_type支持）
-- 如果likes表已存在，则不需要创建
-- 确保likes表支持forum_post和forum_comment类型

-- 更新评论计数触发器
CREATE OR REPLACE FUNCTION update_forum_post_comments_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE forum_posts SET comments_count = comments_count + 1 WHERE id = NEW.post_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE forum_posts SET comments_count = GREATEST(comments_count - 1, 0) WHERE id = OLD.post_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_forum_post_comments_count
    AFTER INSERT OR DELETE ON forum_comments
    FOR EACH ROW
    EXECUTE FUNCTION update_forum_post_comments_count();

