-- 确保 account_records 表有 category 字段
-- 执行时间: 2024-11-20
-- 说明: 修复可能缺失的 category 字段

-- 检查并添加 category 字段（如果不存在）
DO $$ 
BEGIN
    -- 检查 category 字段是否存在
    IF NOT EXISTS (
        SELECT 1 
        FROM information_schema.columns 
        WHERE table_name = 'account_records' 
        AND column_name = 'category'
    ) THEN
        -- 添加 category 字段
        ALTER TABLE account_records 
        ADD COLUMN category VARCHAR(50) NOT NULL DEFAULT '其他';
        
        RAISE NOTICE 'Added category column to account_records table';
    ELSE
        RAISE NOTICE 'Category column already exists in account_records table';
    END IF;
END $$;

-- 验证表结构
SELECT 
    column_name, 
    data_type, 
    character_maximum_length,
    is_nullable,
    column_default
FROM information_schema.columns
WHERE table_name = 'account_records'
ORDER BY ordinal_position;
