-- 为 friendships 表添加 message 字段
-- 用于保存好友申请时的消息

ALTER TABLE friendships ADD COLUMN IF NOT EXISTS message TEXT;

