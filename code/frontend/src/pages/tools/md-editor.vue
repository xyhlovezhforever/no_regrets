<template>
  <view class="md-editor-page">
    <!-- 工具栏 -->
    <view class="toolbar">
      <view class="tab-switch">
        <view 
          class="tab-item" 
          :class="{ active: mode === 'edit' }"
          @click="mode = 'edit'"
        >
          <text>✏️ 编辑</text>
        </view>
        <view 
          class="tab-item" 
          :class="{ active: mode === 'preview' }"
          @click="mode = 'preview'"
        >
          <text>👁️ 预览</text>
        </view>
      </view>
      
      <view class="action-btns">
        <button class="action-btn" @click="uploadFile">
          <text>📤</text>
        </button>
        <button class="action-btn" @click="showFileList = true">
          <text>📁</text>
        </button>
        <button class="action-btn" @click="saveFile">
          <text>💾</text>
        </button>
        <button class="action-btn" @click="createNew">
          <text>➕</text>
        </button>
      </view>
    </view>

    <!-- 编辑/预览区域 -->
    <view class="content-area">
      <!-- 编辑模式 -->
      <view v-if="mode === 'edit'" class="editor-container">
        <textarea
          v-model="content"
          class="editor"
          placeholder="# 开始编写Markdown文档..."
          :show-confirm-bar="false"
          :maxlength="-1"
          @input="handleInput"
        />
        <view class="word-count">{{ wordCount }} 字</view>
      </view>

      <!-- 预览模式 -->
      <scroll-view 
        v-if="mode === 'preview'" 
        class="preview-container" 
        scroll-y
        scroll-x
        :show-scrollbar="true"
        :enable-back-to-top="true"
        :enable-flex="true"
        :style="{ height: scrollViewHeight + 'px' }"
      >
        <view class="preview-wrapper">
          <view class="preview-content-container">
            <rich-text class="preview-content" :nodes="renderedHtml"></rich-text>
          </view>
        </view>
      </scroll-view>
    </view>

    <!-- 文件列表弹窗 -->
    <view v-if="showFileList" class="modal-mask" @click="showFileList = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">📁 我的文档</text>
          <text class="modal-close" @click="showFileList = false">✕</text>
        </view>
        <view class="file-list">
          <view 
            v-for="(file, index) in files" 
            :key="index"
            class="file-item"
            @click="loadFile(file)"
          >
            <text class="file-icon">📄</text>
            <view class="file-info">
              <text class="file-name">{{ file.name }}</text>
              <text class="file-time">{{ formatTime(file.time) }}</text>
            </view>
            <text class="file-delete" @click.stop="deleteFile(index)">🗑️</text>
          </view>
          <view v-if="files.length === 0" class="empty">
            <text>暂无文档</text>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getStorage, setStorage } from '@/utils/storage'

interface MdFile {
  name: string
  content: string
  time: number
}

const mode = ref<'edit' | 'preview'>('edit')
const content = ref(`# 欢迎使用Markdown编辑器

## 📤 上传功能
点击上传按钮(📤)，选择本地的 .md/.markdown/.txt 文件即可导入查看和编辑。

## 🎨 支持的语法
- **粗体文本** 和 *斜体文本*
- [链接文字](https://example.com)
- \`行内代码\`
- > 引用文本
- 无序列表和有序列表

## 💾 保存与管理
- 💾 保存：将当前文档保存到本地
- 📁 文档列表：查看所有已保存的文档
- ➕ 新建：创建新的空白文档

开始编写你的文档吧！`)
const currentFileName = ref('未命名文档')
const showFileList = ref(false)
const files = ref<MdFile[]>([])
const scrollViewHeight = ref(0)

onMounted(() => {
  loadFiles()
  calculateScrollHeight()
})

const calculateScrollHeight = () => {
  const systemInfo = uni.getSystemInfoSync()
  const windowHeight = systemInfo.windowHeight
  // 工具栏高度：padding (20rpx * 2) + 内容高度约 60rpx = 100rpx
  // rpx 转 px: 100rpx / 750 * 屏幕宽度，简化计算约为 50-70px
  const toolbarHeight = 70
  scrollViewHeight.value = windowHeight - toolbarHeight
  console.log('计算的滚动高度:', scrollViewHeight.value, 'px')
}

const wordCount = computed(() => {
  return content.value.replace(/\s/g, '').length
})

const renderedHtml = computed(() => {
  return parseMarkdown(content.value)
})

const handleInput = () => {
  // 实时更新
}

const parseMarkdown = (text: string): string => {
  if (!text) return ''
  
  let html = text
  
  // 转义HTML特殊字符
  html = html.replace(/&/g, '&amp;')
  html = html.replace(/</g, '&lt;')
  html = html.replace(/>/g, '&gt;')
  
  // 代码块（必须先处理）
  html = html.replace(/```[\s\S]*?```/g, (match) => {
    const code = match.slice(3, -3).trim()
    // 使用 table 布局，设置 nowrap 阻止换行，这样会触发 scroll-view 的横向滚动
    return `<table style="width:auto;min-width:100%;margin:10px 0;background: var(--theme-background);border-radius:5px;table-layout:auto;"><tr><td style="white-space:nowrap;"><pre style="white-space:pre;margin:0;padding:10px;font-size:24rpx;line-height:1.5;display:inline-block;">${code}</pre></td></tr></table>`
  })
  
  // 行内代码
  html = html.replace(/`([^`]+)`/g, '<code style="padding:2px 5px;background: var(--theme-background);border-radius:3px;">$1</code>')
  
  // 标题
  html = html.replace(/^### (.+)$/gim, '<h3>$1</h3>')
  html = html.replace(/^## (.+)$/gim, '<h2>$1</h2>')
  html = html.replace(/^# (.+)$/gim, '<h1>$1</h1>')
  
  // 粗体和斜体
  html = html.replace(/\*\*\*(.+?)\*\*\*/g, '<strong><em>$1</em></strong>')
  html = html.replace(/\*\*(.+?)\*\*/g, '<strong>$1</strong>')
  html = html.replace(/\*(.+?)\*/g, '<em>$1</em>')
  
  // 链接（转义后恢复）
  html = html.replace(/\[([^\]]+)\]\(([^)]+)\)/g, (match, text, url) => {
    const decodedUrl = url.replace(/&amp;/g, '&').replace(/&lt;/g, '<').replace(/&gt;/g, '>')
    return `<a href="${decodedUrl}">${text}</a>`
  })
  
  // 图片
  html = html.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, (match, alt, url) => {
    const decodedUrl = url.replace(/&amp;/g, '&').replace(/&lt;/g, '<').replace(/&gt;/g, '>')
    return `<img src="${decodedUrl}" alt="${alt}" style="max-width:100%;" />`
  })
  
  // 引用
  html = html.replace(/^&gt; (.+)$/gim, '<blockquote>$1</blockquote>')
  
  // 列表
  html = html.replace(/^[\-\*] (.+)$/gim, '<li>$1</li>')
  html = html.replace(/^(\d+)\. (.+)$/gim, '<li>$2</li>')
  
  // 分段
  const lines = html.split('\n')
  let result = ''
  let inList = false
  let inPre = false
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i]
    
    if (line.includes('<pre>')) {
      inPre = true
      result += line
      continue
    }
    if (line.includes('</pre>')) {
      inPre = false
      result += line
      continue
    }
    if (inPre) {
      result += line + '\n'
      continue
    }
    
    if (line.includes('<li>')) {
      if (!inList) {
        result += '<ul>'
        inList = true
      }
      result += line
    } else {
      if (inList) {
        result += '</ul>'
        inList = false
      }
      if (line.trim()) {
        if (!line.match(/<h\d>|<blockquote>|<pre>/)) {
          result += '<p>' + line + '</p>'
        } else {
          result += line
        }
      } else {
        result += '<br />'
      }
    }
  }
  
  if (inList) {
    result += '</ul>'
  }
  
  return result
}

const loadFiles = () => {
  files.value = getStorage('md_files', [])
}

const loadFile = (file: MdFile) => {
  content.value = file.content
  currentFileName.value = file.name
  showFileList.value = false
  uni.showToast({ title: '文档已加载', icon: 'success' })
}

const saveFile = () => {
  uni.showModal({
    title: '保存文档',
    editable: true,
    placeholderText: '请输入文档名称',
    content: currentFileName.value,
    success: (res) => {
      if (res.confirm && res.content) {
        const fileName = res.content || '未命名文档'
        const newFile: MdFile = {
          name: fileName,
          content: content.value,
          time: Date.now()
        }
        
        // 检查是否存在同名文件
        const index = files.value.findIndex(f => f.name === fileName)
        if (index !== -1) {
          files.value[index] = newFile
        } else {
          files.value.unshift(newFile)
        }
        
        setStorage('md_files', files.value)
        currentFileName.value = fileName
        uni.showToast({ title: '保存成功', icon: 'success' })
      }
    }
  })
}

const createNew = () => {
  uni.showModal({
    title: '创建新文档',
    content: '当前文档未保存的内容将丢失',
    success: (res) => {
      if (res.confirm) {
        content.value = '# 新文档\n\n开始编写...'
        currentFileName.value = '未命名文档'
        mode.value = 'edit'
        uni.showToast({ title: '已创建', icon: 'success' })
      }
    }
  })
}

const deleteFile = (index: number) => {
  uni.showModal({
    title: '确认删除',
    content: `确定要删除"${files.value[index].name}"吗？`,
    success: (res) => {
      if (res.confirm) {
        files.value.splice(index, 1)
        setStorage('md_files', files.value)
        uni.showToast({ title: '已删除', icon: 'success' })
      }
    }
  })
}

const uploadFile = () => {
  // #ifdef H5
  // H5端使用input标签上传
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = '.md,.markdown,.txt'
  input.onchange = (e: any) => {
    const file = e.target.files[0]
    if (file) {
      const reader = new FileReader()
      reader.onload = (ev: any) => {
        content.value = ev.target.result
        currentFileName.value = file.name.replace(/\.(md|markdown|txt)$/, '')
        mode.value = 'edit'
        uni.showToast({ 
          title: '文件已加载', 
          icon: 'success' 
        })
      }
      reader.onerror = () => {
        uni.showToast({ 
          title: '读取失败', 
          icon: 'none' 
        })
      }
      reader.readAsText(file, 'UTF-8')
    }
  }
  input.click()
  // #endif
  
  // #ifndef H5
  // 小程序端使用chooseMessageFile
  uni.chooseMessageFile({
    count: 1,
    type: 'file',
    extension: ['.md', '.markdown', '.txt'],
    success: (res) => {
      const file = res.tempFiles[0]
      const filePath = file.path
      const fileName = file.name
      
      // 读取文件内容
      const fs = uni.getFileSystemManager()
      fs.readFile({
        filePath: filePath,
        encoding: 'utf-8',
        success: (readRes) => {
          content.value = readRes.data as string
          currentFileName.value = fileName.replace(/\.(md|markdown|txt)$/, '')
          mode.value = 'edit'
          uni.showToast({ 
            title: '文件已加载', 
            icon: 'success' 
          })
        },
        fail: (err) => {
          console.error('读取文件失败:', err)
          uni.showToast({ 
            title: '读取失败', 
            icon: 'none' 
          })
        }
      })
    },
    fail: (err) => {
      console.error('选择文件失败:', err)
      if (err.errMsg !== 'chooseMessageFile:fail cancel') {
        uni.showToast({ 
          title: '选择文件失败', 
          icon: 'none' 
        })
      }
    }
  })
  // #endif
}

const formatTime = (timestamp: number) => {
  const date = new Date(timestamp)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  
  if (diff < 60000) return '刚刚'
  if (diff < 3600000) return `${Math.floor(diff / 60000)}分钟前`
  if (diff < 86400000) return `${Math.floor(diff / 3600000)}小时前`
  
  return `${date.getMonth() + 1}/${date.getDate()}`
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.md-editor-page {
  @include cyber-page-bg;
  height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 20% 30%, rgba(138, 92, 246, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 70%, rgba(0, 217, 255, 0.15) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.toolbar {
  @include neon-card;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 25rpx 30rpx;
  background: linear-gradient(180deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 0.95) 100%);
  backdrop-filter: blur(30rpx);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(138, 92, 246, 0.4),
    inset 0 2rpx 0 rgba(138, 92, 246, 0.3);
  border-bottom: 3rpx solid rgba(138, 92, 246, 0.5);
  z-index: 10;
  position: relative;

  .tab-switch {
    display: flex;
    gap: 12rpx;

    .tab-item {
      padding: 18rpx 35rpx;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
      backdrop-filter: blur(10rpx);
      border: 2rpx solid rgba(138, 92, 246, 0.4);
      border-radius: 20rpx;
      font-size: 26rpx;
      @include neon-text(#b8c5d6);
      transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
      box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.3);

      &.active {
        background: linear-gradient(135deg, rgba(138, 92, 246, 0.9) 0%, rgba(0, 217, 255, 0.9) 100%);
        @include neon-text(#ffffff);
        border-color: rgba(138, 92, 246, 0.8);
        font-weight: bold;
        box-shadow: 
          0 8rpx 24rpx rgba(138, 92, 246, 0.5),
          0 0 40rpx rgba(138, 92, 246, 0.5);
        transform: scale(1.05);
        text-shadow: 
          0 0 15rpx rgba(255, 255, 255, 0.8),
          0 2rpx 5rpx rgba(0, 0, 0, 0.3);
      }
      
      &:active {
        transform: scale(0.95);
      }
    }
  }

  .action-btns {
    display: flex;
    gap: 18rpx;

    .action-btn {
      width: 70rpx;
      height: 70rpx;
      padding: 0;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
      backdrop-filter: blur(10rpx);
      border: 2rpx solid rgba(0, 217, 255, 0.5);
      border-radius: 18rpx;
      font-size: 36rpx;
      display: flex;
      align-items: center;
      justify-content: center;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      box-shadow: 
        0 6rpx 18rpx rgba(0, 0, 0, 0.4),
        0 0 30rpx rgba(0, 217, 255, 0.2);
      position: relative;
      overflow: hidden;
      
      text {
        filter: drop-shadow(0 0 10rpx rgba(0, 217, 255, 0.8));
        position: relative;
        z-index: 1;
      }
      
      &::before {
        content: '';
        position: absolute;
        top: -50%;
        left: -50%;
        width: 200%;
        height: 200%;
        background: conic-gradient(
          from 0deg,
          transparent 0deg,
          rgba(0, 217, 255, 0.3) 90deg,
          transparent 180deg
        );
        animation: rotate 3s linear infinite;
        opacity: 0;
        transition: opacity 0.3s;
      }

      &::after {
        border: none;
      }

      &:active {
        transform: scale(0.88);
        box-shadow: 
          0 4rpx 12rpx rgba(0, 0, 0, 0.5),
          0 0 50rpx rgba(0, 217, 255, 0.6);
          
        &::before {
          opacity: 1;
        }
      }
    }
  }
}

.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
  z-index: 1;
}

.editor-container {
  position: relative;
  height: 100%;
  background: linear-gradient(135deg, rgba(15, 20, 45, 0.95) 0%, rgba(10, 15, 35, 0.98) 100%);
  backdrop-filter: blur(20rpx);
  overflow-y: scroll;
  overflow-x: hidden;
  border-radius: 30rpx;
  margin: 20rpx;
  border: 3rpx solid rgba(138, 92, 246, 0.3);
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(138, 92, 246, 0.3),
    inset 0 0 50rpx rgba(138, 92, 246, 0.08);

  .editor {
    width: 100%;
    min-height: 5000rpx;
    height: auto;
    padding: 35rpx;
    padding-bottom: 120rpx;
    font-size: 30rpx;
    line-height: 1.9;
    box-sizing: border-box;
    @include neon-text(#ffffff);
    background: transparent;
    text-shadow: 
      0 1rpx 3rpx rgba(0, 0, 0, 0.3),
      0 0 10rpx rgba(255, 255, 255, 0.1);
  }

  .word-count {
    position: fixed;
    bottom: 80rpx;
    right: 30rpx;
    font-size: 26rpx;
    @include neon-text(#FFD600);
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.95) 0%, rgba(20, 26, 56, 0.98) 100%);
    backdrop-filter: blur(20rpx);
    padding: 12rpx 20rpx;
    border-radius: 20rpx;
    border: 2rpx solid rgba(255, 214, 0, 0.5);
    z-index: 100;
    box-shadow: 
      0 8rpx 24rpx rgba(0, 0, 0, 0.5),
      0 0 40rpx rgba(255, 214, 0, 0.4);
    font-weight: 600;
  }
}

.preview-container {
  background: linear-gradient(135deg, rgba(15, 20, 45, 0.95) 0%, rgba(10, 15, 35, 0.98) 100%);
  backdrop-filter: blur(20rpx);
  border-radius: 30rpx;
  margin: 20rpx;
  border: 3rpx solid rgba(0, 217, 255, 0.3);
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(0, 217, 255, 0.3),
    inset 0 0 50rpx rgba(0, 217, 255, 0.08);
}

.preview-wrapper {
  padding: 35rpx;
  padding-bottom: 100rpx;
  box-sizing: border-box;
}

.preview-content-container {
  display: inline-block;
  min-width: 100%;
}

.preview-content {
  font-size: 30rpx;
  line-height: 1.9;
  @include neon-text(#ffffff);
  text-shadow: 
    0 1rpx 3rpx rgba(0, 0, 0, 0.3),
    0 0 10rpx rgba(255, 255, 255, 0.1);
}

/* 弹窗样式 */
.modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.75);
  backdrop-filter: blur(10rpx);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
  animation: fadeIn 0.3s ease-out;
}

.modal-content {
  width: 85%;
  max-height: 70vh;
  @include neon-card;
  background: linear-gradient(135deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 1) 100%);
  backdrop-filter: blur(30rpx);
  border-radius: 30rpx;
  border: 3rpx solid rgba(255, 0, 214, 0.6);
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.7),
    0 0 80rpx rgba(255, 0, 214, 0.5),
    inset 0 0 60rpx rgba(255, 0, 214, 0.15);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  animation: scaleIn 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 35rpx;
  border-bottom: 2rpx solid rgba(255, 0, 214, 0.3);

  .modal-title {
    font-size: 36rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
  }

  .modal-close {
    font-size: 40rpx;
    @include neon-text(#FF00D6);
    padding: 8rpx;
    transition: all 0.3s;
    
    &:active {
      transform: scale(0.88) rotate(90deg);
      text-shadow: 
        0 0 30rpx rgba(255, 0, 214, 1),
        0 0 50rpx rgba(255, 0, 214, 0.8);
    }
  }
}

.file-list {
  flex: 1;
  overflow-y: auto;
  padding: 25rpx;

  .file-item {
    display: flex;
    align-items: center;
    padding: 28rpx 22rpx;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.5) 0%, rgba(30, 36, 66, 0.3) 100%);
    backdrop-filter: blur(10rpx);
    border-radius: 20rpx;
    border: 2rpx solid rgba(138, 92, 246, 0.3);
    margin-bottom: 18rpx;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 
      0 6rpx 18rpx rgba(0, 0, 0, 0.3),
      0 0 30rpx rgba(138, 92, 246, 0.2);
    animation: itemSlideIn 0.5s ease-out backwards;

    &:active {
      transform: translateX(10rpx);
      border-color: rgba(138, 92, 246, 0.6);
      background: linear-gradient(135deg, rgba(138, 92, 246, 0.4) 0%, rgba(0, 217, 255, 0.3) 100%);
      box-shadow: 
        0 8rpx 24rpx rgba(0, 0, 0, 0.4),
        0 0 50rpx rgba(138, 92, 246, 0.5);
    }

    .file-icon {
      font-size: 44rpx;
      margin-right: 22rpx;
      filter: drop-shadow(0 0 10rpx rgba(0, 217, 255, 0.6));
    }

    .file-info {
      flex: 1;
      display: flex;
      flex-direction: column;
      gap: 10rpx;

      .file-name {
        font-size: 30rpx;
        @include neon-text(#ffffff);
        font-weight: bold;
      }

      .file-time {
        font-size: 24rpx;
        @include neon-text(#00D9FF);
        opacity: 0.8;
      }
    }

    .file-delete {
      font-size: 36rpx;
      padding: 12rpx;
      filter: drop-shadow(0 0 10rpx rgba(255, 0, 79, 0.8));
      transition: all 0.3s;
      
      &:active {
        transform: scale(1.2);
        filter: drop-shadow(0 0 20rpx rgba(255, 0, 79, 1));
      }
    }
  }

  .empty {
    text-align: center;
    padding: 120rpx 0;
    font-size: 32rpx;
    @include neon-text(#8B5CF6);
    animation: emptyFloat 3s ease-in-out infinite;
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes scaleIn {
  from {
    transform: scale(0.85);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

@keyframes itemSlideIn {
  from {
    opacity: 0;
    transform: translateX(-30rpx);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes emptyFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-20rpx);
  }
}
</style>
