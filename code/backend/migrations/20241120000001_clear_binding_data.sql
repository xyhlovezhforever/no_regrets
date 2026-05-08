-- 清除账号绑定相关数据
-- 执行时间: 2024-11-20
-- 说明: 清空 account_bindings 和 app_permissions 表的所有数据

-- 1. 清空应用权限表
TRUNCATE TABLE app_permissions CASCADE;

-- 2. 清空账号绑定表
TRUNCATE TABLE account_bindings CASCADE;

-- 3. 验证清理结果
SELECT 
    (SELECT COUNT(*) FROM account_bindings) as bindings_count,
    (SELECT COUNT(*) FROM app_permissions) as permissions_count;

-- 预期结果: bindings_count = 0, permissions_count = 0
