# TabBar 图标说明

## 需要的图标文件

请在此目录下放置以下图标文件（建议尺寸：81x81 像素）：

### 普通状态图标
- `emotion.png` - 温暖（未选中）
- `leisure.png` - 玩乐（未选中）
- `daily.png` - 生活（未选中）
- `binding.png` - 绑定（未选中）
- `user.png` - 空间（未选中）

### 选中状态图标
- `emotion-active.png` - 温暖（选中）
- `leisure-active.png` - 玩乐（选中）
- `daily-active.png` - 生活（选中）
- `binding-active.png` - 绑定（选中）
- `user-active.png` - 空间（选中）

## 图标设计建议

- **尺寸**：81x81 像素（推荐）
- **格式**：PNG（支持透明背景）
- **颜色**：
  - 普通状态：灰色 (#8F8F94)
  - 选中状态：青色 (#00D9FF)
- **风格**：简洁、扁平化

## 临时方案

如果暂时没有图标，可以：
1. 使用在线图标生成工具（如 iconfont.cn）
2. 使用 emoji 转图片工具
3. 先使用纯色占位图

## 配置位置

图标配置在 `pages.json` 的 `tabBar.list` 中：

```json
{
  "pagePath": "pages/emotion/index",
  "text": "温暖",
  "iconPath": "static/tabbar/emotion.png",
  "selectedIconPath": "static/tabbar/emotion-active.png"
}
```
