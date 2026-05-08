<template>
  <view class="philosophy-page">
    <view class="card-wrapper">
      <view class="today-card">
        <text class="date">{{ today }}</text>
        <text class="title">{{ card.title }}</text>
        <text class="content">{{ card.content }}</text>
        <text class="author" v-if="card.author">—— {{ card.author }}</text>
      </view>
    </view>

    <view class="actions">
      <button class="btn" @click="shareCard">分享</button>
      <button class="btn btn-primary" @click="nextCard">换一条</button>
      <button class="btn" @click="viewList">查看更多</button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const card = ref({
  title: '人生的意义',
  content: '人生的意义不在于我们拥有什么，而在于我们成为了什么样的人。',
  author: '佚名'
})

const today = computed(() => {
  const date = new Date()
  return `${date.getFullYear()}年${date.getMonth() + 1}月${date.getDate()}日`
})

const nextCard = () => {
  uni.showToast({ title: '加载中...', icon: 'loading' })
}

const shareCard = () => {
  const shareText = `${card.value.title}\n\n${card.value.content}${card.value.author ? `\n\n—— ${card.value.author}` : ''}`
  
  uni.showShareMenu({
    withShareTicket: true,
    success: () => {
      // 分享成功
    }
  })
  
  // 分享内容
  uni.share({
    provider: 'weixin',
    type: 0,
    title: card.value.title,
    summary: card.value.content,
    imageUrl: '/static/logo.png',
    success: () => {
      uni.showToast({ title: '分享成功', icon: 'success' })
    },
    fail: () => {
      // H5 环境下使用替代方案
      uni.showModal({
        title: '分享哲学命题',
        content: shareText,
        confirmText: '复制文本',
        success: (res) => {
          if (res.confirm) {
            uni.setClipboardData({
              data: shareText,
              success: () => {
                uni.showToast({ title: '已复制到剪贴板', icon: 'success' })
              }
            })
          }
        }
      })
    }
  })
}

const viewList = () => {
  uni.navigateTo({ url: '/pages/philosophy/list' })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.philosophy-page {
  min-height: 100vh;
  @include cyber-page-bg;
  padding: 60rpx 30rpx;
  display: flex;
  flex-direction: column;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 600rpx;
    height: 600rpx;
    background: radial-gradient(circle, rgba(138, 92, 246, 0.2) 0%, transparent 70%);
    pointer-events: none;
    animation: breathe 4s ease-in-out infinite;
  }
}

@keyframes breathe {
  0%, 100% {
    transform: translate(-50%, -50%) scale(1);
    opacity: 0.3;
  }
  50% {
    transform: translate(-50%, -50%) scale(1.2);
    opacity: 0.5;
  }
}

.card-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 1;
  animation: fadeIn 1s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.today-card {
  @include neon-card;
  @include rainbow-border;
  border-radius: 30rpx;
  padding: 60rpx 40rpx;
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(138, 92, 246, 0.4),
    inset 0 0 60rpx rgba(138, 92, 246, 0.1);
  text-align: center;
  position: relative;
  animation: cardAppear 1s ease-out, cardFloat 6s ease-in-out infinite 1s;
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  
  &::before {
    content: '';
    position: absolute;
    top: -2rpx;
    left: 50%;
    transform: translateX(-50%);
    width: 80%;
    height: 2rpx;
    background: linear-gradient(90deg, transparent, rgba(138, 92, 246, 0.8), transparent);
    box-shadow: 0 0 20rpx rgba(138, 92, 246, 0.6);
  }
  
  &::after {
    content: '';
    position: absolute;
    bottom: -2rpx;
    left: 50%;
    transform: translateX(-50%);
    width: 80%;
    height: 2rpx;
    background: linear-gradient(90deg, transparent, rgba(138, 92, 246, 0.8), transparent);
    box-shadow: 0 0 20rpx rgba(138, 92, 246, 0.6);
  }
  
  &:active {
    transform: perspective(1000rpx) rotateX(5deg);
  }

  .date {
    display: block;
    font-size: 24rpx;
    @include neon-text(#8B5CF6);
    margin-bottom: 30rpx;
    letter-spacing: 2rpx;
  }

  .title {
    display: block;
    font-size: 40rpx;
    font-weight: bold;
    @include neon-title(#ffffff);
    margin-bottom: 30rpx;
    animation: titleGlow 3s ease-in-out infinite;
  }

  .content {
    display: block;
    font-size: 30rpx;
    color: #b8c5d6;
    line-height: 1.8;
    margin-bottom: 30rpx;
    text-shadow: 0 0 8rpx rgba(184, 197, 214, 0.3);
  }

  .author {
    display: block;
    font-size: 26rpx;
    @include neon-text(#FFD600);
    font-style: italic;
  }
}

@keyframes cardAppear {
  from {
    opacity: 0;
    transform: perspective(1000rpx) rotateX(-20deg) translateY(50rpx);
  }
  to {
    opacity: 1;
    transform: perspective(1000rpx) rotateX(0deg) translateY(0);
  }
}

@keyframes cardFloat {
  0%, 100% {
    transform: perspective(1000rpx) rotateX(0deg) translateY(0);
  }
  50% {
    transform: perspective(1000rpx) rotateX(2deg) translateY(-10rpx);
  }
}

@keyframes titleGlow {
  0%, 100% {
    text-shadow: 
      0 0 15rpx rgba(255, 255, 255, 0.8),
      0 0 30rpx rgba(138, 92, 246, 0.5),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
  50% {
    text-shadow: 
      0 0 20rpx rgba(255, 255, 255, 1),
      0 0 40rpx rgba(138, 92, 246, 0.8),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
}

.actions {
  display: flex;
  justify-content: center;
  align-items: center;
  padding-top: 60rpx;
  position: relative;
  z-index: 1;
  gap: 20rpx;

  .btn {
    flex: 1;
    height: 90rpx;
    background: rgba(30, 36, 66, 0.6);
    backdrop-filter: blur(10rpx);
    @include neon-text(#ffffff);
    border-radius: 50rpx;
    border: 2rpx solid rgba(0, 217, 255, 0.4);
    box-shadow: 0 0 20rpx rgba(0, 217, 255, 0.3);
    font-size: 28rpx;
    padding: 0;
    line-height: 90rpx;
    transition: all 0.3s ease;
    position: relative;
    overflow: hidden;
    
    &::before {
      content: '';
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      width: 0;
      height: 0;
      background: radial-gradient(circle, rgba(0, 217, 255, 0.3) 0%, transparent 70%);
      border-radius: 50%;
      transition: all 0.5s ease;
    }
    
    &:active::before {
      width: 300rpx;
      height: 300rpx;
    }
    
    /* 重置 button 默认样式 */
    &::after {
      border: none;
    }
    
    &:active {
      transform: scale(0.95);
      border-color: rgba(0, 217, 255, 0.8);
      box-shadow: 
        0 0 30rpx rgba(0, 217, 255, 0.6),
        inset 0 0 20rpx rgba(0, 217, 255, 0.2);
    }

    &.btn-primary {
      @include glow-button(#8B5CF6);
      background: linear-gradient(135deg, rgba(138, 92, 246, 0.8) 0%, rgba(102, 126, 234, 0.8) 100%);
      font-weight: bold;
      animation: btnPulse 3s ease-in-out infinite;
      
      &:active {
        animation: none;
      }
    }
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(138, 92, 246, 0.5),
      0 8rpx 24rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(138, 92, 246, 0.2);
  }
  50% {
    box-shadow: 
      0 0 30rpx rgba(138, 92, 246, 0.8),
      0 12rpx 32rpx rgba(0, 0, 0, 0.4),
      inset 0 0 30rpx rgba(138, 92, 246, 0.3);
  }
}
</style>

