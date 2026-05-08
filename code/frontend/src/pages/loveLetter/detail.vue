<template>
  <view class="letter-detail-page">
    <!-- 信封动画 -->
    <view v-if="!isOpened" class="envelope-container" @click="openEnvelope">
      <view class="envelope">
        <view class="envelope-flap"></view>
        <view class="envelope-body">
          <view class="envelope-seal">
            <text class="seal-icon">💌</text>
          </view>
          <view class="envelope-text">
            <text class="from-label">来自</text>
            <text class="from-name">{{ letter?.from_name }}</text>
          </view>
        </view>
      </view>
      <view class="tap-hint">
        <text class="hint-icon">👆</text>
        <text class="hint-text">点击打开信封</text>
      </view>
    </view>

    <!-- 信件内容 -->
    <view v-else class="letter-content" :class="{ show: isOpened }">
      <view class="letter-paper">
        <view class="paper-header">
          <text class="letter-title">{{ letter?.title }}</text>
          <view class="letter-meta">
            <text class="meta-label">📮 收件人：</text>
            <text class="meta-value">{{ letter?.to_name }}</text>
          </view>
          <view class="letter-meta">
            <text class="meta-label">✍️ 寄件人：</text>
            <text class="meta-value">{{ letter?.from_name }}</text>
          </view>
          <view class="letter-meta">
            <text class="meta-label">📅 日期：</text>
            <text class="meta-value">{{ formatDate(letter?.created_at) }}</text>
          </view>
        </view>

        <view class="paper-divider"></view>

        <view class="paper-content">
          <text class="content-text">{{ letter?.content }}</text>
        </view>

        <view class="paper-footer">
          <text class="signature">—— {{ letter?.from_name }}</text>
        </view>
      </view>

      <view class="letter-actions">
        <button class="action-btn reply-btn" @click="replyLetter">
          <text class="btn-icon">💬</text>
          <text>回复</text>
        </button>
        <button class="action-btn close-btn" @click="closeLetter">
          <text class="btn-icon">📋</text>
          <text>收起</text>
        </button>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { navigateTo } from '@/utils'
import type { LoveLetter } from '@/types'
import { getLetterDetailApi, markLetterAsReadApi } from '@/api/loveLetter'

const letterId = ref('')
const letter = ref<LoveLetter | null>(null)
const isOpened = ref(false)

onLoad((options: any) => {
  if (options.id) {
    letterId.value = options.id
    loadLetter()
  }
})

const loadLetter = async () => {
  try {
    const data = await getLetterDetailApi(letterId.value)
    letter.value = data
    
    // 如果是未读，标记为已读
    if (!data.is_read) {
      await markLetterAsReadApi(letterId.value)
    }
  } catch (error) {
    console.error('加载情书失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  }
}

const openEnvelope = async () => {
  isOpened.value = true
  
  // 标记为已读
  if (letterId.value && letter.value && !letter.value.is_read) {
    try {
      await markLetterAsReadApi(letterId.value)
      // 更新本地状态
      if (letter.value) {
        letter.value.is_read = true
      }
    } catch (error) {
      console.error('标记已读失败:', error)
    }
  }
}

const closeLetter = () => {
  isOpened.value = false
}

const replyLetter = () => {
  if (!letter.value) return
  
  // 跳转到写情书页面，并传递收信人信息
  navigateTo('/pages/warmth/love-letter', {
    replyTo: letter.value.from_user_id,
    replyName: letter.value.from_name
  })
}

const formatDate = (dateStr?: string) => {
  if (!dateStr) return ''
  const date = new Date(dateStr)
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}年${month}月${day}日`
}
</script>

<style lang="scss" scoped>
.letter-detail-page {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  padding: 60rpx 30rpx;
  display: flex;
  align-items: center;
  justify-content: center;
}

.envelope-container {
  width: 100%;
  max-width: 600rpx;
  text-align: center;

  .envelope {
    position: relative;
    width: 100%;
    height: 400rpx;
    margin: 0 auto 40rpx;
    animation: float 3s ease-in-out infinite;

    .envelope-flap {
      position: absolute;
      top: 0;
      left: 50%;
      transform: translateX(-50%);
      width: 0;
      height: 0;
      border-left: 300rpx solid transparent;
      border-right: 300rpx solid transparent;
      border-top: 200rpx solid #ff6b9d;
      transform-origin: center bottom;
      z-index: 2;
    }

    .envelope-body {
      position: absolute;
      top: 180rpx;
      left: 0;
      right: 0;
      width: 600rpx;
      height: 300rpx;
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      border-radius: 0 0 20rpx 20rpx;
      box-shadow: 0 10rpx 30rpx rgba(255, 107, 157, 0.3);
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      gap: 20rpx;

      .envelope-seal {
        width: 100rpx;
        height: 100rpx;
        background: var(--theme-surface);
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.1);

        .seal-icon {
          font-size: 60rpx;
        }
      }

      .envelope-text {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 10rpx;

        .from-label {
          font-size: 24rpx;
          color: rgba(255, 255, 255, 0.9);
        }

        .from-name {
          font-size: 32rpx;
          color: #ffffff;
          font-weight: bold;
        }
      }
    }
  }

  .tap-hint {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10rpx;
    animation: bounce 2s ease-in-out infinite;

    .hint-icon {
      font-size: 48rpx;
    }

    .hint-text {
      font-size: 28rpx;
      color: #ff6b9d;
      font-weight: bold;
    }
  }
}

.letter-content {
  width: 100%;
  opacity: 0;
  transform: scale(0.9);
  transition: all 0.5s ease-out;

  &.show {
    opacity: 1;
    transform: scale(1);
  }

  .letter-paper {
    background: var(--theme-surface);
    border-radius: 20rpx;
    padding: 50rpx;
    box-shadow: 0 10rpx 40rpx rgba(255, 107, 157, 0.15);
    margin-bottom: 30rpx;
    position: relative;
    
    // 信纸纹理
    background-image: 
      linear-gradient(0deg, transparent 24%, rgba(255, 107, 157, 0.03) 25%, rgba(255, 107, 157, 0.03) 26%, transparent 27%, transparent 74%, rgba(255, 107, 157, 0.03) 75%, rgba(255, 107, 157, 0.03) 76%, transparent 77%, transparent);

    .paper-header {
      margin-bottom: 30rpx;

      .letter-title {
        display: block;
        font-size: 40rpx;
        font-weight: bold;
        color: var(--theme-text);
        text-align: center;
        margin-bottom: 30rpx;
        line-height: 1.5;
      }

      .letter-meta {
        display: flex;
        align-items: center;
        gap: 10rpx;
        margin-bottom: 15rpx;
        font-size: 24rpx;

        .meta-label {
          color: var(--theme-text-secondary);
        }

        .meta-value {
          color: var(--theme-text-secondary);
          font-weight: 500;
        }
      }
    }

    .paper-divider {
      height: 2rpx;
      background: linear-gradient(90deg, transparent 0%, var(--theme-primary) 50%, transparent 100%);
      margin-bottom: 30rpx;
    }

    .paper-content {
      margin-bottom: 40rpx;

      .content-text {
        font-size: 30rpx;
        color: var(--theme-text);
        line-height: 2;
        text-indent: 2em;
        display: block;
        white-space: pre-wrap;
        word-break: break-all;
      }
    }

    .paper-footer {
      text-align: right;

      .signature {
        font-size: 28rpx;
        color: var(--theme-text-secondary);
        font-style: italic;
      }
    }
  }

  .letter-actions {
    display: flex;
    gap: 20rpx;

    .action-btn {
      flex: 1;
      height: 80rpx;
      border-radius: 50rpx;
      font-size: 28rpx;
      font-weight: bold;
      border: none;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 10rpx;

      &::after {
        border: none;
      }

      .btn-icon {
        font-size: 32rpx;
      }

      &.reply-btn {
        background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
        color: #ffffff;
        box-shadow: 0 8rpx 20rpx rgba(255, 107, 157, 0.3);
      }

      &.close-btn {
        background: #f0f0f0;
        color: var(--theme-text-secondary);
      }

      &:active {
        transform: scale(0.98);
      }
    }
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-20rpx);
  }
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10rpx);
  }
}
</style>
