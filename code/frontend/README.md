# 情绪价值小程序 - 前端

基于 uni-app + Vue 3 + TypeScript + Vite 构建的跨端应用。

## 技术栈

- **框架**: uni-app + Vue 3 + TypeScript
- **构建工具**: Vite
- **状态管理**: Pinia
- **HTTP 请求**: luch-request
- **CSS**: UnoCSS + SCSS
- **代码规范**: ESLint + Prettier

## 支持平台

- ✅ H5 (移动端 + PC 端)
- ✅ 微信小程序
- ✅ 支付宝小程序
- ✅ 百度小程序
- ✅ 字节跳动小程序
- ✅ QQ 小程序
- ✅ App (iOS + Android)

## 项目结构

```
src/
├── api/               # API 接口封装
│   ├── auth.ts       # 认证相关
│   └── mock.ts       # Mock 数据
├── components/        # 公共组件
├── config/           # 配置文件
│   └── index.ts      # 应用配置
├── hooks/            # Vue 3 Composables
├── pages/            # 页面
│   └── index/        # 首页
├── static/           # 静态资源
├── store/            # Pinia 状态管理
│   ├── index.ts      # Store 入口
│   ├── app.ts        # 应用状态
│   └── user.ts       # 用户状态
├── styles/           # 全局样式
│   ├── index.scss    # 主样式
│   └── variables.scss # 变量
├── types/            # TypeScript 类型
│   └── index.ts      # 全局类型
├── utils/            # 工具函数
│   ├── index.ts      # 工具函数入口
│   ├── request.ts    # 请求封装
│   └── storage.ts    # 存储工具
├── App.vue           # 应用入口
├── main.ts           # 主入口
├── manifest.json     # 应用配置
├── pages.json        # 页面路由配置
└── uni.scss          # uni-app 样式变量
```

## 开发命令

### 安装依赖

\`\`\`bash
npm install
# 或
pnpm install
\`\`\`

### 运行开发环境

\`\`\`bash
# H5
npm run dev:h5

# 微信小程序
npm run dev:mp-weixin

# 支付宝小程序
npm run dev:mp-alipay

# 字节跳动小程序
npm run dev:mp-toutiao

# 百度小程序
npm run dev:mp-baidu

# QQ 小程序
npm run dev:mp-qq
\`\`\`

### 构建生产环境

\`\`\`bash
# H5
npm run build:h5

# 微信小程序
npm run build:mp-weixin

# 其他平台同理...
\`\`\`

### 代码检查和格式化

\`\`\`bash
# 代码检查
npm run lint

# 代码格式化
npm run format

# 类型检查
npm run type-check
\`\`\`

## 开发规范

### 命名规范

- **文件名**: 使用 kebab-case (小写短横线)
- **组件名**: 使用 PascalCase
- **变量名**: 使用 camelCase
- **常量名**: 使用 UPPER_SNAKE_CASE

### 代码风格

- 使用 TypeScript 编写代码
- 使用 Composition API
- 每个模块代码不超过 500 行
- 遵循单一职责原则
- 保持高内聚、低耦合

### 提交规范

遵循 Conventional Commits 规范：

- `feat`: 新功能
- `fix`: 修复 Bug
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `perf`: 性能优化
- `test`: 测试相关
- `chore`: 构建/工具链相关

## 环境变量

在项目根目录创建 `.env.development` 和 `.env.production` 文件：

\`\`\`bash
# 开发环境
VITE_APP_ENV=development
VITE_APP_BASE_API=http://localhost:8000/api/v1
VITE_APP_TITLE=情绪价值小程序

# 生产环境
VITE_APP_ENV=production
VITE_APP_BASE_API=https://api.yourdomain.com/api/v1
VITE_APP_TITLE=情绪价值小程序
\`\`\`

## API 使用

```typescript
import { get, post } from '@/utils/request'

// GET 请求
const data = await get('/user/info')

// POST 请求
const result = await post('/auth/login', { username, password })

// 文件上传
import { upload } from '@/utils/request'
const fileUrl = await upload('/upload', filePath)
```

## 状态管理

```typescript
import { useUserStore } from '@/store'

const userStore = useUserStore()

// 登录
await userStore.login(username, password)

// 获取用户信息
console.log(userStore.userInfo)

// 退出登录
await userStore.logout()
```

## 注意事项

1. 开发时先使用 Mock 数据，后端接口完成后再联调
2. 注意各平台的兼容性问题，使用条件编译处理
3. 图片等静态资源放在 `static` 目录
4. API 请求统一通过 `utils/request.ts` 处理
5. 使用 UnoCSS 原子化 CSS，避免样式冲突

## 小程序配置

微信小程序需要在 `manifest.json` 中配置 `appid` 等信息。

## 常见问题

### 1. 如何调试小程序？

在对应平台的开发者工具中导入 `dist/dev/mp-xxx` 目录。

### 2. 如何处理跨域问题？

H5 端在 `vite.config.ts` 中配置代理，小程序端无跨域问题。

### 3. 如何添加新页面？

在 `pages.json` 中添加页面配置，然后在 `src/pages` 目录创建页面文件。

## License

MIT

