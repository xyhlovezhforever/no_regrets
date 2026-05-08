# 快速打包App - 执行步骤

本文档提供快速打包App的执行步骤。

## ✅ 已完成的准备工作

1. ✅ 已更新 `manifest.json` 基本配置
2. ✅ 已添加 App 打包脚本到 `package.json`
3. ✅ 已创建打包前检查工具
4. ✅ 环境检查已通过

## 🚀 立即开始打包

### 方式一：使用 HBuilderX（推荐，最简单）

#### 步骤 1: 下载 HBuilderX
```bash
# 访问官网下载 App 开发版
https://www.dcloud.io/hbuilderx.html
```

#### 步骤 2: 导入项目
1. 打开 HBuilderX
2. 菜单：文件 -> 导入 -> 从本地目录导入
3. 选择 `frontend` 目录

#### 步骤 3: 获取 AppID
- HBuilderX 会自动为项目生成 AppID
- 或在 manifest.json 可视化界面中点击"重新获取"

#### 步骤 4: 配置应用信息（可选）
- 在 HBuilderX 中打开 `manifest.json`
- 使用可视化界面配置：
  - 应用图标
  - 启动页
  - 权限设置
  - SDK配置

#### 步骤 5: 云端打包
1. 菜单：发行 -> 原生App-云打包
2. 选择平台：
   - ☑️ Android
   - ☑️ iOS（需要Mac和证书）
3. 证书配置：
   - 测试阶段：选择"使用DCloud公用证书"
   - 正式发布：上传自己的证书
4. 点击"打包"
5. 等待打包完成（约3-10分钟）
6. 下载 apk/ipa 文件

### 方式二：命令行构建（仅生成资源）

#### 步骤 1: 检查环境
```bash
cd frontend
npm run check:app
```

#### 步骤 2: 构建 App 资源
```bash
# 构建通用App资源
npm run build:app

# 或指定平台
npm run build:app-android  # Android
npm run build:app-ios      # iOS
```

#### 步骤 3: 后续打包
命令行构建只生成资源文件，最终打包仍需：
- 使用 HBuilderX 云打包，或
- 配置本地原生开发环境（Android Studio / Xcode）

## 📋 打包检查清单

在打包前，请确认：

- [ ] 已安装依赖：`npm install`
- [ ] manifest.json 配置完整
- [ ] 应用名称、描述已填写
- [ ] 版本号已设置
- [ ] （可选）应用图标已配置
- [ ] （可选）启动页已配置
- [ ] 网络请求地址已配置正确

## 🔧 常用命令

```bash
# 检查打包环境
npm run check:app

# 开发模式（App预览）
npm run dev:app

# 构建App资源
npm run build:app
npm run build:app-android
npm run build:app-ios

# 代码检查
npm run lint

# 类型检查
npm run type-check
```

## 📱 测试打包好的App

### Android
1. 将 apk 文件传输到 Android 手机
2. 允许安装未知来源应用
3. 点击安装
4. 测试功能

### iOS
1. 使用 TestFlight 分发（需要开发者账号）
2. 或使用企业签名
3. 或使用 Xcode 直接安装到测试设备

## ⚠️ 注意事项

1. **AppID 问题**
   - 当前使用默认 AppID `__UNI__XXXXXX`
   - 在 HBuilderX 中打开项目会自动生成真实 AppID
   - 云打包时会使用真实 AppID

2. **证书问题**
   - 测试阶段可使用 DCloud 公用证书
   - 正式发布需要自己的证书：
     - Android: 需要 keystore 文件
     - iOS: 需要开发者证书和描述文件

3. **网络请求**
   - 确保 API 地址配置正确
   - App 中需要使用 https（iOS强制要求）
   - 配置域名白名单

4. **权限配置**
   - Android 权限已在 manifest.json 中配置
   - iOS 权限需要在打包时说明用途

## 📚 相关文档

- 详细打包指南：`打包App指南.md`
- uni-app 官方文档：https://uniapp.dcloud.net.cn/
- HBuilderX 文档：https://hx.dcloud.net.cn/

## 🎯 推荐流程（首次打包）

1. ✅ 运行 `npm run check:app` 检查环境
2. ✅ 下载并安装 HBuilderX
3. ✅ 导入项目到 HBuilderX
4. ✅ 使用云打包（公用证书）
5. ✅ 下载并测试 App
6. ✅ 如果测试通过，配置正式证书重新打包

---

**当前状态：** 环境已准备就绪，可以开始打包！

**下一步：** 下载 HBuilderX 并按照上述步骤操作
