-- 修复账号绑定系统
-- 执行时间: 2024-11-20
-- 说明: 添加触发器自动更新时间戳，优化索引

-- 1. 为 account_bindings 表添加更新时间戳触发器
CREATE OR REPLACE FUNCTION update_account_bindings_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_update_account_bindings_updated_at ON account_bindings;
CREATE TRIGGER trigger_update_account_bindings_updated_at
    BEFORE UPDATE ON account_bindings
    FOR EACH ROW
    EXECUTE FUNCTION update_account_bindings_updated_at();

-- 2. 为 app_permissions 表添加更新时间戳触发器
CREATE OR REPLACE FUNCTION update_app_permissions_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trigger_update_app_permissions_updated_at ON app_permissions;
CREATE TRIGGER trigger_update_app_permissions_updated_at
    BEFORE UPDATE ON app_permissions
    FOR EACH ROW
    EXECUTE FUNCTION update_app_permissions_updated_at();

-- 3. 添加复合索引以优化查询性能
CREATE INDEX IF NOT EXISTS idx_account_bindings_users_status 
    ON account_bindings(user_id, bound_user_id, status);

CREATE INDEX IF NOT EXISTS idx_account_bindings_bound_status 
    ON account_bindings(bound_user_id, status);

CREATE INDEX IF NOT EXISTS idx_app_permissions_user_bound_app 
    ON app_permissions(user_id, bound_user_id, app_type);

-- 4. 添加检查约束确保状态值有效
ALTER TABLE account_bindings DROP CONSTRAINT IF EXISTS check_valid_status;
ALTER TABLE account_bindings ADD CONSTRAINT check_valid_status 
    CHECK (status IN ('pending', 'accepted', 'rejected', 'cancelled'));

ALTER TABLE app_permissions DROP CONSTRAINT IF EXISTS check_valid_permission_level;
ALTER TABLE app_permissions ADD CONSTRAINT check_valid_permission_level 
    CHECK (permission_level IN ('none', 'read', 'write'));

ALTER TABLE app_permissions DROP CONSTRAINT IF EXISTS check_valid_app_type;
ALTER TABLE app_permissions ADD CONSTRAINT check_valid_app_type 
    CHECK (app_type IN ('note', 'account', 'love_letter', 'writing', 'forum', 'private_space', 'idol', 'chat'));

-- 5. 验证修复结果
SELECT 
    'account_bindings' as table_name,
    COUNT(*) as total_records,
    COUNT(DISTINCT CASE 
        WHEN user_id < bound_user_id THEN user_id || '-' || bound_user_id 
        ELSE bound_user_id || '-' || user_id 
    END) as unique_pairs
FROM account_bindings
WHERE status = 'accepted'
UNION ALL
SELECT 
    'app_permissions' as table_name,
    COUNT(*) as total_records,
    COUNT(DISTINCT (user_id, bound_user_id, app_type)) as unique_combinations
FROM app_permissions;

-- 预期结果: account_bindings 的 total_records 应该等于 unique_pairs
