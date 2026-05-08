-- 添加情书已读状态支持

-- 为 love_letters 表添加收信人ID和已读状态字段
ALTER TABLE love_letters ADD COLUMN IF NOT EXISTS to_user_id UUID REFERENCES users(id) ON DELETE CASCADE;
ALTER TABLE love_letters ADD COLUMN IF NOT EXISTS is_read BOOLEAN NOT NULL DEFAULT false;

-- 创建索引以提高查询效率
CREATE INDEX IF NOT EXISTS idx_love_letters_to_user_id ON love_letters(to_user_id);
CREATE INDEX IF NOT EXISTS idx_love_letters_is_read ON love_letters(is_read);

-- 更新现有数据：根据 to_name 找到对应的 to_user_id
UPDATE love_letters l
SET to_user_id = u.id
FROM users u
WHERE l.to_user_id IS NULL 
  AND (u.nickname = l.to_name OR u.username = l.to_name);
