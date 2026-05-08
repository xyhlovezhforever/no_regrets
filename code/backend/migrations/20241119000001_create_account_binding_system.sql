-- 创建账号绑定系统
-- 包含用户账号绑定表和应用数据访问权限表

-- 用户账号绑定表
CREATE TABLE IF NOT EXISTS account_bindings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    bound_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- pending, accepted, rejected, cancelled
    initiator_id UUID NOT NULL REFERENCES users(id), -- 发起绑定的用户
    message TEXT, -- 绑定请求消息
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_binding UNIQUE(user_id, bound_user_id),
    CONSTRAINT no_self_binding CHECK(user_id != bound_user_id)
);

-- 为账号绑定表创建索引
CREATE INDEX idx_account_bindings_user_id ON account_bindings(user_id);
CREATE INDEX idx_account_bindings_bound_user_id ON account_bindings(bound_user_id);
CREATE INDEX idx_account_bindings_status ON account_bindings(status);

-- 应用数据访问权限表
CREATE TABLE IF NOT EXISTS app_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- 数据拥有者
    bound_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- 被授权的绑定用户
    app_type VARCHAR(50) NOT NULL, -- 应用类型：note(便签), account(记账), love_letter(情书), writing(创作), forum(论坛), private_space(私人空间), idol(偶像), chat(AI聊天)
    permission_level VARCHAR(20) NOT NULL DEFAULT 'none', -- none(无权限), read(只读), write(读写)
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_app_permission UNIQUE(user_id, bound_user_id, app_type)
);

-- 为权限表创建索引
CREATE INDEX idx_app_permissions_user_id ON app_permissions(user_id);
CREATE INDEX idx_app_permissions_bound_user_id ON app_permissions(bound_user_id);
CREATE INDEX idx_app_permissions_app_type ON app_permissions(app_type);

-- 添加注释
COMMENT ON TABLE account_bindings IS '用户账号绑定表 - 用于两个用户互相绑定账号';
COMMENT ON TABLE app_permissions IS '应用数据访问权限表 - 用于控制绑定用户对应用数据的访问权限';

COMMENT ON COLUMN account_bindings.status IS '绑定状态：pending(待接受)、accepted(已接受)、rejected(已拒绝)、cancelled(已取消)';
COMMENT ON COLUMN app_permissions.permission_level IS '权限级别：none(无权限)、read(只读)、write(读写)';
COMMENT ON COLUMN app_permissions.app_type IS '应用类型：note, account, love_letter, writing, forum, private_space, idol, chat';
