# TabBar 图标设计方案

## 图标规格

- **尺寸**: 81x81 像素
- **格式**: PNG
- **背景**: 透明
- **风格**: 简约、扁平化、赛博朋克

## 需要的图标

### 1. emotion（温暖）
**普通状态 - emotion.png**
- 图标：心形 ❤️
- 颜色：#999999（灰色）
- 描述：简单的心形轮廓，线条粗细适中

**激活状态 - emotion-active.png**
- 图标：心形 ❤️
- 颜色：#00D9FF（青色）
- 描述：同样的心形，但填充青色，带轻微发光效果

### 2. leisure（玩乐）
**普通状态 - leisure.png**
- 图标：闪电 ⚡
- 颜色：#999999（灰色）
- 描述：闪电符号，锐利的线条

**激活状态 - leisure-active.png**
- 图标：闪电 ⚡
- 颜色：#00D9FF（青色）
- 描述：同样的闪电，青色填充，带能量感

### 3. daily（生活）
**普通状态 - daily.png**
- 图标：星星 ⭐
- 颜色：#999999（灰色）
- 描述：五角星轮廓

**激活状态 - daily-active.png**
- 图标：星星 ⭐
- 颜色：#00D9FF（青色）
- 描述：同样的星星，青色填充，带闪烁感

### 4. binding（绑定）
**普通状态 - binding.png**
- 图标：链条 🔗
- 颜色：#999999（灰色）
- 描述：两个环相连的链条

**激活状态 - binding-active.png**
- 图标：链条 🔗
- 颜色：#00D9FF（青色）
- 描述：同样的链条，青色填充

### 5. user（空间）
**普通状态 - user.png**
- 图标：水晶球 🔮
- 颜色：#999999（灰色）
- 描述：圆形带底座的水晶球

**激活状态 - user-active.png**
- 图标：水晶球 🔮
- 颜色：#00D9FF（青色）
- 描述：同样的水晶球，青色填充，带神秘感

## AI 生成提示词

### 使用 DALL-E 或 Midjourney

```
Create a simple, flat design icon for a mobile app tab bar.
Icon: [heart/lightning/star/chain/crystal ball]
Size: 81x81 pixels
Style: minimalist, cyberpunk
Color: [#999999 for normal / #00D9FF for active]
Background: transparent
Format: PNG
```

### 中文提示词

```
创建一个简约扁平风格的移动应用标签栏图标
图标：[心形/闪电/星星/链条/水晶球]
尺寸：81x81像素
风格：极简、赛博朋克
颜色：[灰色#999999 普通状态 / 青色#00D9FF 激活状态]
背景：透明
格式：PNG
```

## 在线工具推荐

### 1. Figma（推荐）
- 网址：https://www.figma.com/
- 免费，专业设计工具
- 可以直接绘制并导出

### 2. Canva
- 网址：https://www.canva.com/
- 简单易用
- 有大量图标模板

### 3. IconScout
- 网址：https://iconscout.com/
- 海量免费图标
- 可自定义颜色

### 4. Flaticon
- 网址：https://www.flaticon.com/
- 免费图标库
- 支持自定义颜色和尺寸

### 5. 阿里巴巴矢量图标库
- 网址：https://www.iconfont.cn/
- 中文界面
- 免费，资源丰富

## 快速制作步骤

### 使用 iconfont.cn（最简单）

1. 访问 https://www.iconfont.cn/
2. 搜索关键词：
   - "heart" 或 "爱心"
   - "lightning" 或 "闪电"
   - "star" 或 "星星"
   - "link" 或 "链接"
   - "user" 或 "用户"
3. 选择喜欢的图标
4. 点击"下载"
5. 选择 PNG 格式，81x81 尺寸
6. 下载后重命名为对应文件名
7. 使用图片编辑工具修改颜色：
   - 普通状态：#999999
   - 激活状态：#00D9FF

## 临时方案

如果暂时无法制作图标，可以：

### 方案 1：使用纯文字（当前方案）
去掉 iconPath 配置，只显示文字

### 方案 2：使用 Emoji
在 text 中添加 emoji：
```json
{
  "text": "❤️ 温暖"
}
```

### 方案 3：使用占位图
创建纯色方块作为临时图标

## 文件放置位置

```
frontend/src/static/tabbar/
├── emotion.png
├── emotion-active.png
├── leisure.png
├── leisure-active.png
├── daily.png
├── daily-active.png
├── binding.png
├── binding-active.png
├── user.png
└── user-active.png
```

## 注意事项

1. **文件名必须完全匹配** pages.json 中的配置
2. **路径是相对于项目根目录**的
3. **图标必须是透明背景**，否则会有白色方块
4. **尺寸建议 81x81**，但也可以是其他尺寸（会自动缩放）
5. **颜色要统一**，普通状态都用灰色，激活状态都用青色

## 完成后

将 10 个图标文件放到 `frontend/src/static/tabbar/` 目录后，刷新页面即可看到效果！
