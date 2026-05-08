# 情绪价值小程序 - 后端 API

基于 Rust + Axum 构建的高性能后端 API 服务。

## 技术栈

- **框架**: Axum + Tokio
- **数据库**: PostgreSQL + SQLx
- **缓存**: Redis
- **认证**: JWT + bcrypt
- **API 文档**: utoipa (Swagger/OpenAPI)
- **日志**: tracing + tracing-subscriber
- **HTTP 客户端**: reqwest

## 架构设计

### 分层架构

```
API Layer (Routes)
    ↓
Service Layer (Business Logic)
    ↓
Repository Layer (Data Access)
    ↓
Database / Cache
```

### 目录结构

```
src/
├── api/                # API 路由层（处理 HTTP 请求）
│   ├── mod.rs
│   ├── auth.rs        # 认证 API
│   ├── user.rs        # 用户 API
│   └── health.rs      # 健康检查
├── service/           # 业务逻辑层
│   ├── mod.rs
│   ├── auth_service.rs    # 认证服务
│   ├── user_service.rs    # 用户服务
│   ├── ai_service.rs      # AI 服务
│   └── content_service.rs # 内容服务
├── repository/        # 数据访问层
│   ├── mod.rs
│   ├── database.rs    # 数据库操作
│   └── redis.rs       # Redis 操作
├── model/             # 数据模型
│   ├── mod.rs
│   ├── entity.rs      # 数据库实体
│   ├── dto.rs         # 数据传输对象
│   └── vo.rs          # 视图对象
├── middleware/        # 中间件
│   ├── mod.rs
│   └── auth.rs        # 认证中间件
├── utils/             # 工具函数
│   ├── mod.rs
│   └── error.rs       # 错误处理
├── config/            # 配置管理
│   └── mod.rs
└── main.rs            # 应用入口
```

## 环境要求

- Rust 1.75+
- PostgreSQL 14+
- Redis 6+

## 快速开始

### 1. 安装依赖

```bash
# 安装 Rust（如果还没安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 PostgreSQL 和 Redis
# macOS
brew install postgresql redis

# Ubuntu
sudo apt install postgresql postgresql-contrib redis-server

# Windows
# 请从官网下载安装
```

### 2. 配置环境变量

复制 `.env.example` 为 `.env` 并填写配置：

```bash
cp .env.example .env
```

编辑 `.env` 文件：

```env
# 应用配置
APP_NAME=you-have-no-regrets
APP_ENV=development
APP_HOST=0.0.0.0
APP_PORT=8000

# 数据库配置
DATABASE_URL=postgres://username:password@localhost:5432/database_name

# Redis 配置
REDIS_URL=redis://localhost:6379

# JWT 配置
JWT_SECRET=your-secret-key-change-this-in-production
JWT_EXPIRATION=86400
JWT_REFRESH_EXPIRATION=604800

# CORS 配置
CORS_ORIGINS=http://localhost:3000

# AI API 配置
AI_API_KEY=your-api-key
AI_API_URL=https://api.openai.com/v1
AI_MODEL=gpt-3.5-turbo
```

### 3. 创建数据库

```bash
# 登录 PostgreSQL
psql -U postgres

# 创建数据库
CREATE DATABASE your_database_name;

# 创建用户（如果需要）
CREATE USER your_username WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE your_database_name TO your_username;
```

### 4. 运行数据库迁移

```bash
# 安装 sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# 运行迁移
sqlx migrate run
```

### 5. 启动开发服务器

```bash
cargo run
```

服务器将在 `http://localhost:8000` 启动。

### 6. 访问 API 文档

启动服务器后，访问 Swagger UI：

```
http://localhost:8000/swagger-ui
```

## API 端点

### 健康检查

- `GET /api/v1/health` - 健康检查

### 认证

- `POST /api/v1/auth/login` - 用户登录
- `POST /api/v1/auth/register` - 用户注册

### 用户

- `GET /api/v1/user/info` - 获取用户信息（需要认证）
- `PUT /api/v1/user/update` - 更新用户信息（需要认证）
- `PUT /api/v1/user/change-password` - 修改密码（需要认证）

## 开发指南

### 代码规范

- 遵循 Rust 官方代码规范
- 每个模块代码不超过 500 行
- 遵循单一职责原则
- 保持高内聚、低耦合

### 添加新功能

1. 在 `model/entity.rs` 中定义数据库实体
2. 在 `model/dto.rs` 中定义请求 DTO
3. 在 `model/vo.rs` 中定义响应 VO
4. 在 `repository/database.rs` 中实现数据访问
5. 在 `service/` 中实现业务逻辑
6. 在 `api/` 中实现路由处理

### 数据库迁移

```bash
# 创建新的迁移
sqlx migrate add <migration_name>

# 运行迁移
sqlx migrate run

# 回滚迁移
sqlx migrate revert
```

### 运行测试

```bash
cargo test
```

### 构建生产版本

```bash
cargo build --release
```

生产二进制文件位于 `target/release/you-have-no-regrets-backend`。

## 部署

### Docker 部署

```dockerfile
# Dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates
COPY --from=builder /app/target/release/you-have-no-regrets-backend /usr/local/bin/
CMD ["you-have-no-regrets-backend"]
```

```bash
# 构建镜像
docker build -t you-have-no-regrets-backend .

# 运行容器
docker run -p 8000:8000 --env-file .env you-have-no-regrets-backend
```

### 使用 Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  backend:
    build: .
    ports:
      - "8000:8000"
    environment:
      DATABASE_URL: postgres://user:password@postgres:5432/dbname
      REDIS_URL: redis://redis:6379
    depends_on:
      - postgres
      - redis

  postgres:
    image: postgres:14
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: dbname
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:6
    volumes:
      - redis_data:/data

volumes:
  postgres_data:
  redis_data:
```

```bash
docker-compose up -d
```

## 性能优化

### 高并发支持

- 使用 Tokio 异步运行时
- 数据库连接池（默认 10 个连接）
- Redis 连接管理器
- 支持水平扩展

### 缓存策略

- 热点数据缓存到 Redis
- 设置合理的过期时间
- 缓存键前缀管理

### 安全性

- 密码使用 bcrypt 加密
- JWT token 认证
- CORS 配置
- SQL 注入防护（使用 SQLx 参数化查询）

## 故障排除

### 数据库连接失败

检查 `DATABASE_URL` 是否正确，确保 PostgreSQL 服务正在运行。

### Redis 连接失败

检查 `REDIS_URL` 是否正确，确保 Redis 服务正在运行。

### 编译错误

确保 Rust 版本 >= 1.75，运行 `rustup update`。

## License

MIT

