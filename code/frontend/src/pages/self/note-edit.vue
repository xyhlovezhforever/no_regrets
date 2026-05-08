<template>
  <scroll-view class="note-edit-page" scroll-y>
    <view class="note-container" :style="{ background: noteData.color }">
      <textarea
        v-model="noteData.title"
        class="note-title-input"
        :style="{ color: noteData.fontColor }"
        placeholder="标题"
        maxlength="50"
        :show-confirm-bar="false"
        :adjust-position="false"
        :auto-height="true"
      />
      <textarea
        v-model="noteData.content"
        class="note-content-input"
        :style="{ color: noteData.fontColor }"
        placeholder="开始写笔记..."
        :maxlength="5000"
        :adjust-position="false"
        :auto-height="true"
      />
      <view class="char-count">{{ noteData.content.length }} / 5000</view>
    </view>

    <!-- 底部工具栏 -->
    <view 
      class="bottom-toolbar" 
      :class="{ 'color-expanded': showColorPicker || showFontColorPicker }"
    >
      <!-- 颜色选择器 -->
      <view v-if="showColorPicker" class="color-picker-section">
        <view class="color-picker-header">
          <text class="color-picker-title">选择背景颜色</text>
          <button class="color-close-btn" @click="showColorPicker = false">✕</button>
        </view>
        <view class="color-list">
          <view
            v-for="color in bgColors"
            :key="color"
            class="color-item"
            :style="{ background: color }"
            :class="{ active: noteData.color === color }"
            @click="selectBgColor(color)"
          >
            <text v-if="noteData.color === color" class="color-check">✓</text>
          </view>
        </view>
      </view>

      <!-- 字体颜色选择器 -->
      <view v-if="showFontColorPicker" class="color-picker-section">
        <view class="color-picker-header">
          <text class="color-picker-title">选择字体颜色</text>
          <button class="color-close-btn" @click="showFontColorPicker = false">✕</button>
        </view>
        <view class="color-list">
          <view
            v-for="color in fontColors"
            :key="color"
            class="color-item font-color-item"
            :style="{ background: '#ffffff', border: '3rpx solid ' + color }"
            :class="{ active: noteData.fontColor === color }"
            @click="selectFontColor(color)"
          >
            <text :style="{ color: color, fontSize: '50rpx', fontWeight: 'bold' }">A</text>
          </view>
        </view>
      </view>

      <!-- 工具栏按钮 -->
      <view class="toolbar-actions">
        <button class="toolbar-btn" @click="toggleColorPicker">
          <text class="btn-icon">🎨</text>
          <text class="btn-text">背景</text>
        </button>
        <button class="toolbar-btn" @click="toggleFontColorPicker">
          <text class="btn-icon">🖊️</text>
          <text class="btn-text">字体</text>
        </button>
        <button class="toolbar-btn primary" @click="saveNote">
          <text class="btn-icon">✓</text>
          <text class="btn-text">保存</text>
        </button>
      </view>
    </view>
  </scroll-view>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { getStorage, setStorage } from '@/utils/storage'
import { formatDate, generateId } from '@/utils'

// 定义变量
const showColorPicker = ref(false)
const showFontColorPicker = ref(false)

interface Note {
  id: string
  title: string
  content: string
  color: string
  fontColor?: string
  folderId?: string | null
  isPinned: boolean
  createdAt: string
  updatedAt: string
}

const noteId = ref('')
const noteData = ref<Note>({
  id: '',
  title: '',
  content: '',
  color: '#FFE4B5',
  fontColor: '#333333',
  folderId: null,
  isPinned: false,
  createdAt: '',
  updatedAt: ''
})

// 背景颜色
const bgColors = [
  '#FFE4B5', '#E0F7FA', '#F8BBD0', '#DCEDC8',
  '#FFF9C4', '#E1BEE7', '#B3E5FC', '#C8E6C9',
  '#FFCCBC', '#D1C4E9', '#FFE0B2', '#F0F4C3'
]

// 字体颜色
const fontColors = [
  '#000000', '#333333', '#666666', '#999999',
  '#FF5722', '#E91E63', '#9C27B0', '#673AB7',
  '#3F51B5', '#2196F3', '#00BCD4', '#009688',
  '#4CAF50', '#8BC34A', '#FFC107', '#FF9800'
]

onLoad((options: any) => {
  if (options.id) {
    noteId.value = options.id
    loadNote()
  } else {
    // 新建笔记
    noteData.value.id = generateId()
    noteData.value.folderId = options.folderId || null
    noteData.value.createdAt = formatDate(new Date())
    noteData.value.updatedAt = formatDate(new Date())
  }
})

const loadNote = () => {
  const notes = getStorage<Note[]>('notes', []) || []
  const note = notes.find(n => n.id === noteId.value)
  if (note) {
    noteData.value = { ...note }
  }
}

const saveNote = () => {
  if (!noteData.value.title.trim() && !noteData.value.content.trim()) {
    uni.showToast({ title: '请输入内容', icon: 'none' })
    return
  }

  noteData.value.updatedAt = formatDate(new Date())

  const notes = getStorage<Note[]>('notes', []) || []
  const index = notes.findIndex(n => n.id === noteData.value.id)

  if (index >= 0) {
    // 更新
    notes[index] = { ...noteData.value }
  } else {
    // 新增
    notes.unshift(noteData.value)
  }

  setStorage('notes', notes)
  uni.showToast({ title: '保存成功', icon: 'success' })

  setTimeout(() => {
    uni.navigateBack()
  }, 500)
}

const deleteNote = () => {
  uni.showModal({
    title: '确认删除',
    content: '确定要删除这条笔记吗？',
    success: (res) => {
      if (res.confirm) {
        const notes = getStorage<Note[]>('notes', []) || []
        const newNotes = notes.filter(n => n.id !== noteData.value.id)
        setStorage('notes', newNotes)
        uni.showToast({ title: '已删除', icon: 'success' })
        setTimeout(() => {
          uni.navigateBack()
        }, 500)
      }
    }
  })
}

const togglePin = () => {
  noteData.value.isPinned = !noteData.value.isPinned
  uni.showToast({
    title: noteData.value.isPinned ? '已置顶' : '已取消置顶',
    icon: 'none'
  })
}

const toggleColorPicker = () => {
  showColorPicker.value = !showColorPicker.value
  if (showColorPicker.value) {
    showFontColorPicker.value = false
  }
}

const toggleFontColorPicker = () => {
  showFontColorPicker.value = !showFontColorPicker.value
  if (showFontColorPicker.value) {
    showColorPicker.value = false
  }
}

const selectBgColor = (color: string) => {
  noteData.value.color = color
  showColorPicker.value = false
  uni.showToast({
    title: '背景颜色已更改',
    icon: 'none',
    duration: 1000
  })
}

const selectFontColor = (color: string) => {
  noteData.value.fontColor = color
  showFontColorPicker.value = false
  uni.showToast({
    title: '字体颜色已更改',
    icon: 'none',
    duration: 1000
  })
}

// 页面卸载时提示保存
onUnmounted(() => {
  // 无需处理，使用系统导航返回
})
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.note-edit-page {
  @include cyber-page-bg;
  height: 100vh;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 30% 20%, rgba(255, 0, 214, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 70% 80%, rgba(0, 217, 255, 0.15) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.bottom-toolbar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 100;
  @include neon-card;
  background: linear-gradient(180deg, 
    rgba(20, 26, 56, 0.98) 0%, 
    rgba(15, 20, 45, 1) 100%);
  backdrop-filter: blur(40rpx) saturate(180%);
  box-shadow: 
    0 -15rpx 50rpx rgba(0, 0, 0, 0.7),
    0 0 80rpx rgba(255, 0, 214, 0.5),
    inset 0 2rpx 0 rgba(255, 0, 214, 0.3);
  border-top: 3rpx solid rgba(255, 0, 214, 0.6);
  transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
  padding-bottom: env(safe-area-inset-bottom);

  &.color-expanded {
    transform: translateY(0);
  }

  .color-picker-section {
    padding: 40rpx;
    border-bottom: 3rpx solid rgba(255, 0, 214, 0.3);
    background: linear-gradient(180deg, 
      rgba(30, 36, 66, 0.98) 0%, 
      rgba(20, 26, 56, 0.95) 100%);
    backdrop-filter: blur(30rpx);
    box-shadow: 
      inset 0 2rpx 0 rgba(255, 0, 214, 0.2),
      0 15rpx 40rpx rgba(0, 0, 0, 0.5);
    animation: slideDown 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);

    @keyframes slideDown {
      from {
        opacity: 0;
        transform: translateY(-30rpx);
      }
      to {
        opacity: 1;
        transform: translateY(0);
      }
    }

    .color-picker-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 32rpx;

      .color-picker-title {
        font-size: 36rpx;
        font-weight: bold;
        @include neon-title(#FFD600);
      }

      .color-close-btn {
        padding: 12rpx 24rpx;
        font-size: 36rpx;
        @include neon-text(#FF00D6);
        background: linear-gradient(135deg, rgba(255, 0, 214, 0.2) 0%, rgba(138, 92, 246, 0.2) 100%);
        border: 2rpx solid rgba(255, 0, 214, 0.5);
        border-radius: 25rpx;
        box-shadow: 
          0 6rpx 20rpx rgba(0, 0, 0, 0.3),
          0 0 30rpx rgba(255, 0, 214, 0.3);
        transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);

        &::after {
          border: none;
        }

        &:active {
          background: linear-gradient(135deg, rgba(255, 0, 214, 0.4) 0%, rgba(138, 92, 246, 0.4) 100%);
          transform: scale(0.88) rotate(90deg);
          box-shadow: 
            0 4rpx 12rpx rgba(0, 0, 0, 0.4),
            0 0 50rpx rgba(255, 0, 214, 0.6);
        }
      }
    }

    .color-list {
      display: grid;
      grid-template-columns: repeat(6, 1fr);
      gap: 24rpx;

      .color-item {
        aspect-ratio: 1;
        border-radius: 20rpx;
        display: flex;
        align-items: center;
        justify-content: center;
        border: 3rpx solid rgba(255, 255, 255, 0.2);
        box-shadow: 
          0 8rpx 20rpx rgba(0, 0, 0, 0.4),
          0 0 30rpx rgba(255, 255, 255, 0.1),
          inset 0 2rpx 6rpx rgba(255, 255, 255, 0.3);
        transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
        position: relative;
        overflow: hidden;

        &::before {
          content: '';
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          bottom: 0;
          background: linear-gradient(135deg, 
            rgba(255, 255, 255, 0.4) 0%, 
            transparent 100%);
          opacity: 0;
          transition: opacity 0.3s ease;
        }
        
        &::after {
          content: '';
          position: absolute;
          top: -50%;
          left: -50%;
          width: 200%;
          height: 200%;
          background: conic-gradient(
            from 0deg,
            transparent 0deg,
            rgba(255, 255, 255, 0.2) 90deg,
            transparent 180deg
          );
          animation: rotate 3s linear infinite;
          opacity: 0;
        }

        &.active {
          border-color: rgba(255, 0, 214, 0.9);
          box-shadow: 
            0 12rpx 30rpx rgba(0, 0, 0, 0.5),
            0 0 60rpx rgba(255, 0, 214, 0.8),
            0 0 0 5rpx rgba(255, 0, 214, 0.3),
            inset 0 2rpx 12rpx rgba(255, 255, 255, 0.5);
          transform: scale(1.2) translateY(-6rpx);
          animation: colorPulse 1.5s ease-in-out infinite;

          &::before {
            opacity: 1;
          }
          
          &::after {
            opacity: 1;
          }
        }

        &:active {
          transform: scale(0.9);
        }

        .color-check {
          font-size: 60rpx;
          font-weight: bold;
          color: #ffffff;
          text-shadow: 
            0 0 20rpx rgba(255, 0, 214, 1),
            0 0 40rpx rgba(255, 0, 214, 0.8),
            0 3rpx 10rpx rgba(0, 0, 0, 0.5);
          filter: drop-shadow(0 0 15rpx rgba(255, 0, 214, 1));
          animation: checkPop 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
          position: relative;
          z-index: 1;
        }

        @keyframes checkPop {
          0% {
            transform: scale(0) rotate(-180deg);
          }
          100% {
            transform: scale(1) rotate(0deg);
          }
        }
        
        @keyframes colorPulse {
          0%, 100% {
            box-shadow: 
              0 12rpx 30rpx rgba(0, 0, 0, 0.5),
              0 0 60rpx rgba(255, 0, 214, 0.8),
              0 0 0 5rpx rgba(255, 0, 214, 0.3),
              inset 0 2rpx 12rpx rgba(255, 255, 255, 0.5);
          }
          50% {
            box-shadow: 
              0 15rpx 35rpx rgba(0, 0, 0, 0.6),
              0 0 80rpx rgba(255, 0, 214, 1),
              0 0 0 7rpx rgba(255, 0, 214, 0.5),
              inset 0 2rpx 15rpx rgba(255, 255, 255, 0.6);
          }
        }
      }
    }
  }

  .toolbar-actions {
    display: flex;
    justify-content: space-around;
    align-items: center;
    padding: 30rpx;
    gap: 25rpx;
    background: linear-gradient(180deg, 
      rgba(20, 26, 56, 0.95) 0%, 
      rgba(15, 20, 45, 0.98) 100%);

    .toolbar-btn {
      flex: 1;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 12rpx;
      padding: 24rpx 18rpx;
      font-size: 24rpx;
      font-weight: 600;
      @include neon-text(#b8c5d6);
      background: linear-gradient(135deg, 
        rgba(30, 36, 66, 0.6) 0%, 
        rgba(30, 36, 66, 0.4) 100%);
      backdrop-filter: blur(10rpx);
      border: 2rpx solid rgba(138, 92, 246, 0.5);
      border-radius: 25rpx;
      transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
      min-width: 0;
      box-shadow: 
        0 6rpx 18rpx rgba(0, 0, 0, 0.4),
        0 0 30rpx rgba(138, 92, 246, 0.2);
      position: relative;
      overflow: hidden;

      &::before {
        content: '';
        position: absolute;
        top: 50%;
        left: 50%;
        width: 0;
        height: 0;
        border-radius: 50%;
        background: rgba(255, 0, 214, 0.3);
        transform: translate(-50%, -50%);
        transition: width 0.4s, height 0.4s;
      }

      &::after {
        border: none;
      }

      &:active {
        transform: scale(0.9) translateY(2rpx);
        box-shadow: 
          0 3rpx 12rpx rgba(0, 0, 0, 0.5),
          0 0 40rpx rgba(138, 92, 246, 0.4);

        &::before {
          width: 250%;
          height: 250%;
        }
      }

      .btn-icon {
        font-size: 48rpx;
        margin-bottom: 6rpx;
        filter: drop-shadow(0 0 12rpx rgba(255, 255, 255, 0.6));
        transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
      }

      .btn-text {
        font-size: 23rpx;
        line-height: 1.2;
      }

      &:active .btn-icon {
        transform: scale(1.3);
      }

      &.primary {
        background: linear-gradient(135deg, 
          rgba(255, 0, 214, 0.9) 0%, 
          rgba(138, 92, 246, 0.9) 50%, 
          rgba(0, 217, 255, 0.9) 100%);
        background-size: 200% 200%;
        @include neon-text(#ffffff);
        border-color: rgba(255, 0, 214, 0.7);
        box-shadow: 
          0 10rpx 40rpx rgba(255, 0, 214, 0.6),
          0 0 60rpx rgba(255, 0, 214, 0.5),
          inset 0 2rpx 8rpx rgba(255, 255, 255, 0.4);
        animation: gradientShift 3s ease infinite, btnPulse 3s ease-in-out infinite;

        @keyframes gradientShift {
          0%, 100% {
            background-position: 0% 50%;
          }
          50% {
            background-position: 100% 50%;
          }
        }

        .btn-icon {
          color: #ffffff;
          filter: drop-shadow(0 0 20rpx rgba(255, 255, 255, 1));
        }

        .btn-text {
          color: #ffffff;
          font-weight: 700;
          text-shadow: 
            0 0 15rpx rgba(255, 255, 255, 0.8),
            0 2rpx 5rpx rgba(0, 0, 0, 0.3);
        }

        &::before {
          background: rgba(255, 255, 255, 0.4);
        }

        &:active {
          animation: none;
          box-shadow: 
            0 12rpx 45rpx rgba(255, 0, 214, 0.8),
            0 0 80rpx rgba(255, 0, 214, 0.7),
            inset 0 2rpx 12rpx rgba(0, 0, 0, 0.2);
        }
      }

      &.danger {
        color: #ff4d4f;
        background: linear-gradient(135deg, 
          rgba(255, 77, 79, 0.15) 0%, 
          rgba(255, 77, 79, 0.1) 100%);
        border-color: rgba(255, 77, 79, 0.2);

        .btn-icon {
          color: #ff4d4f;
        }

        .btn-text {
          color: #ff4d4f;
        }

        &::before {
          background: rgba(255, 77, 79, 0.3);
        }

        &:active {
          background: linear-gradient(135deg, 
            rgba(255, 77, 79, 0.25) 0%, 
            rgba(255, 77, 79, 0.2) 100%);
        }
      }

      .font-color-item {
        background: var(--theme-surface) !important;
        display: flex;
        align-items: center;
        justify-content: center;
      }
    }
  }
}

.note-container {
  padding: 35rpx;
  padding-bottom: 200rpx;
  transition: background 0.4s ease;
  min-height: calc(100vh - 88rpx);
  position: relative;
  z-index: 1;
  border-radius: 30rpx;
  margin: 20rpx;
  border: 3rpx solid rgba(255, 0, 214, 0.3);
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.4),
    0 0 60rpx rgba(255, 0, 214, 0.3),
    inset 0 0 50rpx rgba(255, 255, 255, 0.08);
  animation: containerFloat 4s ease-in-out infinite;

  .note-title-input {
    width: 100%;
    font-size: 40rpx;
    font-weight: bold;
    margin-bottom: 25rpx;
    padding: 20rpx 0;
    line-height: 1.5;
    border: none;
    background: transparent;
    border-bottom: 3rpx solid rgba(255, 0, 214, 0.3);
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    box-sizing: border-box;
    word-wrap: break-word;
    overflow-wrap: break-word;
    text-shadow: 
      0 2rpx 8rpx rgba(0, 0, 0, 0.3),
      0 0 15rpx rgba(255, 255, 255, 0.2);
    filter: brightness(1.1);

    &:focus {
      border-bottom-color: rgba(255, 0, 214, 0.8);
      box-shadow: 0 5rpx 0 -2rpx rgba(255, 0, 214, 0.6);
      text-shadow: 
        0 2rpx 8rpx rgba(0, 0, 0, 0.4),
        0 0 20rpx rgba(255, 255, 255, 0.3);
    }

    &::placeholder {
      color: rgba(255, 255, 255, 0.3);
      font-size: 40rpx;
      line-height: 1.5;
      font-weight: 600;
    }
  }

  .note-content-input {
    width: 100%;
    font-size: 32rpx;
    line-height: 1.85;
    border: none;
    background: transparent;
    padding: 20rpx 0;
    font-weight: 400;
    text-shadow: 
      0 1rpx 4rpx rgba(0, 0, 0, 0.2),
      0 0 10rpx rgba(255, 255, 255, 0.15);

    &::placeholder {
      color: rgba(255, 255, 255, 0.25);
      font-weight: 400;
    }
  }

  .char-count {
    text-align: right;
    font-size: 26rpx;
    @include neon-text(#00D9FF);
    margin-top: 20rpx;
    padding-top: 20rpx;
    border-top: 2rpx solid rgba(0, 217, 255, 0.2);
    font-weight: 600;
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

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(255, 0, 214, 0.6),
      0 0 60rpx rgba(255, 0, 214, 0.5),
      inset 0 2rpx 8rpx rgba(255, 255, 255, 0.4);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(255, 0, 214, 0.8),
      0 0 80rpx rgba(255, 0, 214, 0.7),
      inset 0 2rpx 12rpx rgba(255, 255, 255, 0.5);
  }
}

@keyframes containerFloat {
  0%, 100% {
    box-shadow: 
      0 15rpx 50rpx rgba(0, 0, 0, 0.4),
      0 0 60rpx rgba(255, 0, 214, 0.3),
      inset 0 0 50rpx rgba(255, 255, 255, 0.08);
  }
  50% {
    box-shadow: 
      0 18rpx 55rpx rgba(0, 0, 0, 0.5),
      0 0 80rpx rgba(255, 0, 214, 0.5),
      inset 0 0 60rpx rgba(255, 255, 255, 0.12);
  }
}

</style>

