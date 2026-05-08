<template>
  <view v-if="visible" class="share-modal-mask" @click="close">
    <view class="share-modal" @click.stop>
      <view class="modal-header">
        <text class="modal-title">分享</text>
        <text class="modal-close" @click="close">✕</text>
      </view>

      <view class="share-options">
        <view class="option-item" @click="handleShare('image')">
          <view class="option-icon">🖼️</view>
          <text class="option-text">生成图片</text>
        </view>
        <view class="option-item" @click="handleShare('text')">
          <view class="option-icon">📋</view>
          <text class="option-text">复制文本</text>
        </view>
        <view class="option-item" @click="handleShare('link')">
          <view class="option-icon">🔗</view>
          <text class="option-text">分享链接</text>
        </view>
      </view>

      <!-- 预览内容 -->
      <view class="preview-content">
        <text class="preview-title">{{ shareData.title }}</text>
        <text class="preview-text">{{ shareData.content }}</text>
      </view>

      <!-- 隐藏的 canvas 用于生成图片 -->
      <canvas
        canvas-id="shareCanvas"
        id="shareCanvas"
        style="position: fixed; left: -9999px; top: -9999px; width: 750px; height: 1200px;"
      ></canvas>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { shareContent, generateShareImage } from '@/utils/share'

interface ShareData {
  title: string
  content: string
  image?: string
  link?: string
}

interface Props {
  visible: boolean
  shareData: ShareData
}

const props = withDefaults(defineProps<Props>(), {
  visible: false,
  shareData: () => ({ title: '', content: '' })
})

const emit = defineEmits<{
  (e: 'update:visible', value: boolean): void
  (e: 'close'): void
}>()

const close = () => {
  emit('update:visible', false)
  emit('close')
}

const handleShare = (type: 'image' | 'text' | 'link') => {
  if (type === 'image') {
    generateShareImage(props.shareData)
  } else if (type === 'text') {
    const shareText = `${props.shareData.title}\n\n${props.shareData.content}\n\n— 来自无悔青春`
    uni.setClipboardData({
      data: shareText,
      success: () => {
        uni.showToast({ title: '已复制到剪贴板', icon: 'success' })
        close()
      }
    })
  } else if (type === 'link') {
    if (props.shareData.link) {
      uni.setClipboardData({
        data: props.shareData.link,
        success: () => {
          uni.showToast({ title: '链接已复制', icon: 'success' })
          close()
        }
      })
    } else {
      uni.showToast({ title: '暂无链接', icon: 'none' })
    }
  }
}
</script>

<style lang="scss" scoped>
.share-modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.6);
  z-index: 9999;
  display: flex;
  align-items: flex-end;
  justify-content: center;
  animation: fade-in 0.3s ease;
}

@keyframes fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.share-modal {
  width: 100%;
  background: var(--theme-surface);
  border-radius: 40rpx 40rpx 0 0;
  padding: 40rpx 30rpx;
  animation: slide-up 0.3s ease;
  max-height: 80vh;
  overflow-y: auto;
}

@keyframes slide-up {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 40rpx;

  .modal-title {
    font-size: 36rpx;
    font-weight: bold;
    color: var(--theme-text);
  }

  .modal-close {
    font-size: 40rpx;
    color: #999999;
    padding: 10rpx;
  }
}

.share-options {
  display: flex;
  justify-content: space-around;
  margin-bottom: 40rpx;

  .option-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 30rpx;
    background: var(--theme-background);
    border-radius: 20rpx;
    min-width: 150rpx;
    transition: all 0.3s;

    &:active {
      background: #e9ecef;
      transform: scale(0.95);
    }

    .option-icon {
      font-size: 60rpx;
      margin-bottom: 15rpx;
    }

    .option-text {
      font-size: 24rpx;
      color: var(--theme-text-secondary);
    }
  }
}

.preview-content {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 20rpx;
  padding: 40rpx;
  color: #ffffff;

  .preview-title {
    display: block;
    font-size: 32rpx;
    font-weight: bold;
    margin-bottom: 20rpx;
  }

  .preview-text {
    display: block;
    font-size: 28rpx;
    line-height: 1.6;
    opacity: 0.9;
  }
}
</style>

