-- 初始化数据库表结构

-- 用户表
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    nickname VARCHAR(100) NOT NULL,
    avatar VARCHAR(500),
    email VARCHAR(100),
    phone VARCHAR(20),
    gender SMALLINT,
    birthday VARCHAR(20),
    bio TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_phone ON users(phone);

-- AI 聊天会话表
CREATE TABLE IF NOT EXISTS chat_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_chat_sessions_user_id ON chat_sessions(user_id);

-- AI 聊天消息表
CREATE TABLE IF NOT EXISTS chat_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID NOT NULL REFERENCES chat_sessions(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_chat_messages_session_id ON chat_messages(session_id);

-- 情书表
CREATE TABLE IF NOT EXISTS love_letters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    to_name VARCHAR(50) NOT NULL,
    from_name VARCHAR(50) NOT NULL,
    is_sent BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_love_letters_user_id ON love_letters(user_id);

-- 鼓励卡片表
CREATE TABLE IF NOT EXISTS encourage_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    category VARCHAR(50) NOT NULL,
    image_url VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_encourage_cards_category ON encourage_cards(category);

-- 哲理命题表
CREATE TABLE IF NOT EXISTS philosophy_cards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    author VARCHAR(100),
    image_url VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- 偶像信息表
CREATE TABLE IF NOT EXISTS idols (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    avatar VARCHAR(500) NOT NULL,
    description TEXT NOT NULL,
    category VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_idols_category ON idols(category);

-- 偶像语录表
CREATE TABLE IF NOT EXISTS idol_quotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    idol_id UUID NOT NULL REFERENCES idols(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_idol_quotes_idol_id ON idol_quotes(idol_id);

-- 偶像图片表
CREATE TABLE IF NOT EXISTS idol_images (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    idol_id UUID NOT NULL REFERENCES idols(id) ON DELETE CASCADE,
    image_url VARCHAR(500) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_idol_images_idol_id ON idol_images(idol_id);

-- 写作作品表
CREATE TABLE IF NOT EXISTS writing_works (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    category VARCHAR(50) NOT NULL,
    tags TEXT[] NOT NULL DEFAULT '{}',
    likes INTEGER NOT NULL DEFAULT 0,
    views INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_writing_works_user_id ON writing_works(user_id);
CREATE INDEX idx_writing_works_category ON writing_works(category);
CREATE INDEX idx_writing_works_created_at ON writing_works(created_at DESC);

-- 评论表
CREATE TABLE IF NOT EXISTS comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    work_id UUID NOT NULL REFERENCES writing_works(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    likes INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_comments_work_id ON comments(work_id);
CREATE INDEX idx_comments_user_id ON comments(user_id);
CREATE INDEX idx_comments_parent_id ON comments(parent_id);

-- 记账记录表
CREATE TABLE IF NOT EXISTS account_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    record_type VARCHAR(20) NOT NULL,
    amount BIGINT NOT NULL,
    category VARCHAR(50) NOT NULL,
    description TEXT,
    date VARCHAR(20) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_account_records_user_id ON account_records(user_id);
CREATE INDEX idx_account_records_date ON account_records(date);

-- 便签表
CREATE TABLE IF NOT EXISTS notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    color VARCHAR(20) NOT NULL DEFAULT '#ffffff',
    is_pinned BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_notes_user_id ON notes(user_id);
CREATE INDEX idx_notes_is_pinned ON notes(is_pinned);

-- 插入一些示例数据

-- 鼓励卡片
INSERT INTO encourage_cards (title, content, category) VALUES
('你是最棒的', '每一次努力都不会白费，相信自己，你一定可以的！', 'motivation'),
('坚持就是胜利', '成功的道路充满挑战，但只要坚持，终会迎来曙光。', 'perseverance'),
('拥抱每一天', '生活总会有起伏，但每一天都值得我们用心去感受。', 'positivity'),
('勇敢前行', '不要害怕失败，每一次尝试都是成长的机会。', 'courage'),
('珍惜当下', '过去已成为历史，未来还未到来，只有当下才是最真实的。', 'mindfulness');

-- 哲理命题
INSERT INTO philosophy_cards (title, content, author) VALUES
('人生的意义', '人生的意义不在于我们拥有什么，而在于我们成为了什么样的人。', '佚名'),
('时间的价值', '时间是最公平的资源，每个人每天都拥有24小时，关键在于如何使用它。', '佚名'),
('选择与改变', '我们无法改变过去，但可以选择如何面对未来。', '佚名'),
('知行合一', '知道和做到之间，隔着的是行动。真正的智慧在于知行合一。', '王阳明'),
('内心的平静', '真正的强大不是征服别人，而是能够驾驭自己的内心。', '佚名');

-- 偶像信息（示例：张国荣）
INSERT INTO idols (name, avatar, description, category) VALUES
('张国荣', 'https://example.com/avatar/leslie.jpg', '香港著名歌手、演员，华语乐坛和影坛的标杆人物。他的音乐和表演艺术影响了几代人，被誉为"哥哥"。', 'singer');

-- 好友关系表
CREATE TABLE IF NOT EXISTS friendships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    friend_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- pending, accepted, blocked
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, friend_id)
);

CREATE INDEX idx_friendships_user_id ON friendships(user_id);
CREATE INDEX idx_friendships_friend_id ON friendships(friend_id);
CREATE INDEX idx_friendships_status ON friendships(status);

-- 好友聊天消息表
CREATE TABLE IF NOT EXISTS friend_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    to_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    message_type VARCHAR(20) NOT NULL DEFAULT 'text', -- text, image, voice, location
    is_read BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_friend_messages_from_user ON friend_messages(from_user_id);
CREATE INDEX idx_friend_messages_to_user ON friend_messages(to_user_id);
CREATE INDEX idx_friend_messages_created_at ON friend_messages(created_at DESC);

-- 专属AI配置表
CREATE TABLE IF NOT EXISTS custom_ais (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(50) NOT NULL,
    avatar VARCHAR(10) NOT NULL DEFAULT '🤖',
    personality TEXT[] NOT NULL DEFAULT '{}',
    style VARCHAR(20) NOT NULL DEFAULT '温柔',
    background TEXT,
    nickname VARCHAR(20),
    catchphrase VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_custom_ais_user_id ON custom_ais(user_id);

-- 用户偶像关联表（用户收藏的偶像）
CREATE TABLE IF NOT EXISTS user_idols (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    idol_id UUID NOT NULL REFERENCES idols(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, idol_id)
);

CREATE INDEX idx_user_idols_user_id ON user_idols(user_id);
CREATE INDEX idx_user_idols_idol_id ON user_idols(idol_id);

-- 偶像作品表
CREATE TABLE IF NOT EXISTS idol_works (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    idol_id UUID NOT NULL REFERENCES idols(id) ON DELETE CASCADE,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    work_type VARCHAR(50) NOT NULL, -- book, movie, music, etc.
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_idol_works_idol_id ON idol_works(idol_id);

-- 私人空间音乐表
CREATE TABLE IF NOT EXISTS private_space_music (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    artist VARCHAR(100),
    url VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_private_space_music_user_id ON private_space_music(user_id);

-- 私人空间设置表
CREATE TABLE IF NOT EXISTS private_space_settings (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    light_color VARCHAR(20) NOT NULL DEFAULT '#667eea',
    light_intensity INTEGER NOT NULL DEFAULT 5,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- 照镜子照片表
CREATE TABLE IF NOT EXISTS mirror_photos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    photo_url VARCHAR(500) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_mirror_photos_user_id ON mirror_photos(user_id);
CREATE INDEX idx_mirror_photos_created_at ON mirror_photos(created_at DESC);

-- 点赞表
CREATE TABLE IF NOT EXISTS likes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_type VARCHAR(50) NOT NULL, -- work, comment, etc.
    target_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, target_type, target_id)
);

CREATE INDEX idx_likes_user_id ON likes(user_id);
CREATE INDEX idx_likes_target ON likes(target_type, target_id);

-- 获取张国荣的 ID 并插入语录
DO $$
DECLARE
    leslie_id UUID;
BEGIN
    SELECT id INTO leslie_id FROM idols WHERE name = '张国荣' LIMIT 1;
    
    IF leslie_id IS NOT NULL THEN
        INSERT INTO idol_quotes (idol_id, content) VALUES
        (leslie_id, '我就是我，是颜色不一样的烟火。'),
        (leslie_id, '风继续吹，不忍远离。'),
        (leslie_id, '追，寻觅一些自己的东西。');
    END IF;
END $$;

