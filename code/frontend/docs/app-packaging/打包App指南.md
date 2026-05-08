# 前端打包成App指南

本项目基于 uni-app 框架，支持打包成 Android 和 iOS App。

## 📋 前置准备

### 1. 开发环境要求
- Node.js 16+ 
- HBuilderX（推荐使用官方IDE）或 使用命令行打包
- Android Studio（打包Android App）
- Xcode（打包iOS App，仅Mac系统）

### 2. 账号准备
- DCloud 开发者账号（用于云打包）
- 或本地打包环境配置

## 🚀 打包方式

### 方式一：使用 HBuilderX 可视化打包（推荐）

#### Android App 打包步骤

1. **安装 HBuilderX**
   - 下载地址：https://www.dcloud.io/hbuilderx.html
   - 下载 App 开发版

2. **导入项目**
   - 打开 HBuilderX
   - 文件 -> 导入 -> 从本地目录导入
   - 选择 `frontend` 目录

3. **配置 manifest.json**
   - 打开 `src/manifest.json`
   - 填写应用基本信息：
     ```json
     {
       "name": "你没有遗憾",
       "appid": "__UNI__XXXXXX",  // HBuilderX会自动生成
       "description": "一个为给人们提供情绪价值的App",
       "versionName": "1.0.0",
       "versionCode": "100"
     }
     ```
   - 配置 App 图标和启动页
   - 配置 App 权限（已在 manifest.json 中配置）

4. **云端打包**
   - 菜单：发行 -> 原生App-云打包
   - 选择 Android 或 iOS
   - 填写证书信息（测试可使用公共测试证书）
   - 点击打包
   - 等待打包完成，下载 apk/ipa 文件

5. **本地打包（可选）**
   - 菜单：发行 -> 原生App-本地打包
   - 需要配置本地 Android SDK 环境
   - 生成本地打包资源
   - 使用 Android Studio 打开并编译

#### iOS App 打包步骤

1. **准备工作**
   - Mac 电脑
   - 安装 Xcode
   - Apple 开发者账号
   - 配置证书和描述文件

2. **使用 HBuilderX 打包**
   - 发行 -> 原生App-云打包
   - 选择 iOS
   - 上传证书和描述文件
   - 打包完成后下载 ipa 文件

3. **发布到 App Store**
   - 使用 Xcode 或 Application Loader 上传 ipa
   - 在 App Store Connect 提交审核

### 方式二：命令行打包

#### 1. 安装依赖
```bash
cd frontend
npm install
```

#### 2. 构建 App 资源
```bash
# 构建 App-plus 资源
npm run build:app-plus
```

注意：命令行只能生成 App 资源包，最终打包成 apk/ipa 仍需要：
- 使用 HBuilderX 的云打包服务
- 或配置本地原生开发环境

#### 3. 本地原生打包（高级）

**Android:**
```bash
# 1. 生成 App 资源
npm run build:app-plus

# 2. 将生成的资源复制到 Android 原生项目
# 3. 使用 Android Studio 打开原生项目
# 4. 配置签名
# 5. Build -> Generate Signed Bundle / APK
```

**iOS:**
```bash
# 1. 生成 App 资源
npm run build:app-plus

# 2. 将生成的资源复制到 iOS 原生项目
# 3. 使用 Xcode 打开原生项目
# 4. 配置证书和描述文件
# 5. Product -> Archive
```

## 📱 打包配置说明

### manifest.json 关键配置

```json
{
  "app-plus": {
    "splashscreen": {
      "alwaysShowBeforeRender": true,
      "autoclose": true,
      "delay": 0
    },
    "distribute": {
      "android": {
        "permissions": [
          // 已配置必要权限
        ]
      },
      "ios": {},
      "sdkConfigs": {
        // 第三方SDK配置（如推送、分享等）
      }
    }
  }
}
```

### 需要配置的内容

1. **应用信息**
   - name: 应用名称
   - appid: 应用ID（HBuilderX自动生成）
   - description: 应用描述
   - versionName/versionCode: 版本信息

2. **图标和启动页**
   - 在 HBuilderX 中通过可视化界面配置
   - 或手动配置图片路径

3. **权限配置**
   - Android 权限已在 manifest.json 中配置
   - iOS 权限需在打包时配置

4. **SDK配置**（如需要）
   - 推送服务
   - 分享功能
   - 支付功能
   - 地图服务等

## 🔧 常见问题

### 1. 打包失败
- 检查 manifest.json 配置是否正确
- 检查证书是否有效
- 查看 HBuilderX 控制台错误信息

### 2. App 无法运行
- 检查网络请求地址是否正确
- 检查权限配置
- 查看设备日志

### 3. 性能优化
- 开启代码压缩
- 优化图片资源
- 使用分包加载

## 📦 打包产物

- **Android**: `.apk` 文件（可直接安装）或 `.aab` 文件（Google Play）
- **iOS**: `.ipa` 文件（需通过 TestFlight 或企业签名分发）

## 🎯 快速开始（推荐流程）

1. 安装 HBuilderX
2. 导入项目
3. 配置 manifest.json 基本信息
4. 使用云打包（测试证书）
5. 下载并测试 App
6. 正式发布时配置正式证书

## 📚 参考文档

- uni-app 官方文档：https://uniapp.dcloud.net.cn/
- App 打包文档：https://uniapp.dcloud.net.cn/tutorial/app-base.html
- HBuilderX 使用教程：https://hx.dcloud.net.cn/

---

**注意事项：**
- 首次打包建议使用 HBuilderX 可视化工具
- 云打包需要 DCloud 账号
- iOS 打包需要 Mac 电脑和开发者账号
- 测试阶段可使用公共测试证书
