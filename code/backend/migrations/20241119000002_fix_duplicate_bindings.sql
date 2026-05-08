-- 修复重复的绑定关系
-- 删除反向创建的重复记录，只保留原始的绑定记录

-- 1. 查看当前的重复记录
-- SELECT 
--   ab1.id as id1,
--   ab1.user_id as user1,
--   ab1.bound_user_id as bound1,
--   ab2.id as id2,
--   ab2.user_id as user2,
--   ab2.bound_user_id as bound2
-- FROM account_bindings ab1
-- JOIN account_bindings ab2 
--   ON ab1.user_id = ab2.bound_user_id 
--   AND ab1.bound_user_id = ab2.user_id
--   AND ab1.id < ab2.id
-- WHERE ab1.status = 'accepted' AND ab2.status = 'accepted';

-- 2. 删除反向创建的重复记录
-- 保留 created_at 较早的记录（原始请求），删除较晚的记录（反向创建的）
DELETE FROM account_bindings
WHERE id IN (
  SELECT ab2.id
  FROM account_bindings ab1
  JOIN account_bindings ab2 
    ON ab1.user_id = ab2.bound_user_id 
    AND ab1.bound_user_id = ab2.user_id
  WHERE ab1.status = 'accepted' 
    AND ab2.status = 'accepted'
    AND ab1.created_at < ab2.created_at
);

-- 3. 添加注释说明新的设计
COMMENT ON TABLE account_bindings IS '用户账号绑定表 - 每个绑定关系只存储一条记录，查询时使用 OR 条件匹配双向关系';

-- 4. 验证清理结果
-- SELECT 
--   COUNT(*) as total_bindings,
--   COUNT(DISTINCT CASE 
--     WHEN user_id < bound_user_id THEN user_id || '-' || bound_user_id 
--     ELSE bound_user_id || '-' || user_id 
--   END) as unique_pairs
-- FROM account_bindings
-- WHERE status = 'accepted';
-- 
-- 如果 total_bindings = unique_pairs，说明没有重复记录了
