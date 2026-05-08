-- 添加便签文件夹功能和字体颜色

-- 便签文件夹表
CREATE TABLE IF NOT EXISTS note_folders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(50) NOT NULL,
    color VARCHAR(20) NOT NULL DEFAULT '#666666',
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_note_folders_user_id ON note_folders(user_id);
CREATE INDEX IF NOT EXISTS idx_note_folders_sort_order ON note_folders(sort_order);

-- 给 notes 表添加 folder_id 和 font_color 字段（如果不存在）
DO $$ 
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name='notes' AND column_name='folder_id') THEN
        ALTER TABLE notes ADD COLUMN folder_id UUID REFERENCES note_folders(id) ON DELETE SET NULL;
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns 
                   WHERE table_name='notes' AND column_name='font_color') THEN
        ALTER TABLE notes ADD COLUMN font_color VARCHAR(20) DEFAULT '#333333';
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_notes_folder_id ON notes(folder_id);

