<template>
  <view class="encourage-page">
    <!-- 卡片容器 -->
    <view class="card-container">
      <view class="card" :class="{ flipped: isFlipped }" @click="flipCard">
        <!-- 卡片正面 -->
        <view class="card-front">
          <view class="card-content">
            <text class="card-icon">💪</text>
            <text class="card-title">{{ currentCard.title }}</text>
            <text class="card-text">{{ currentCard.content }}</text>
            <text class="card-hint">点击翻转查看更多</text>
          </view>
        </view>

        <!-- 卡片背面 -->
        <view class="card-back">
          <view class="card-content">
            <text class="card-icon">✨</text>
            <text class="card-text">记住：{{ currentCard.title }}</text>
            <text class="card-subtitle">每一次努力都不会白费</text>
          </view>
        </view>
      </view>
    </view>

    <!-- 操作按钮 -->
    <view class="actions">
      <button class="btn btn-secondary" @click="shareCard">
        <text>📤</text>
        <text>分享</text>
      </button>
      <button class="btn btn-primary" @click="nextCard">
        <text>🔄</text>
        <text>换一张</text>
      </button>
      <button class="btn btn-secondary" @click="viewList">
        <text>📚</text>
        <text>全部</text>
      </button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const currentCard = ref({
  title: '你是最棒的',
  content: '每一次努力都不会白费，相信自己，你一定可以的！'
})

const isFlipped = ref(false)

const cards = [
  { title: '你是最棒的', content: '每一次努力都不会白费，相信自己，你一定可以的！' },
  { title: '坚持就是胜利', content: '成功的道路充满挑战，但只要坚持，终会迎来曙光。' },
  { title: '拥抱每一天', content: '生活总会有起伏，但每一天都值得我们用心去感受。' },
]

const flipCard = () => {
  isFlipped.value = !isFlipped.value
}

const nextCard = () => {
  const currentIndex = cards.findIndex(c => c.title === currentCard.value.title)
  const nextIndex = (currentIndex + 1) % cards.length
  currentCard.value = cards[nextIndex]
  isFlipped.value = false
}

const shareCard = () => {
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
    title: currentCard.value.title,
    summary: currentCard.value.content,
    imageUrl: '/static/logo.png',
    success: () => {
      uni.showToast({ title: '分享成功', icon: 'success' })
    },
    fail: () => {
      // H5 环境下使用替代方案
      uni.showModal({
        title: '分享',
        content: `${currentCard.value.title}\n\n${currentCard.value.content}`,
        confirmText: '复制文本',
        success: (res) => {
          if (res.confirm) {
            uni.setClipboardData({
              data: `${currentCard.value.title}\n\n${currentCard.value.content}`,
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
  uni.navigateTo({ url: '/pages/encourage/list' })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.encourage-page {
  min-height: 100vh;
  @include cyber-page-bg;
  padding: 60rpx 30rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 50% 30%, rgba(245, 87, 108, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 50% 70%, rgba(240, 147, 251, 0.15) 0%, transparent 50%);
    pointer-events: none;
    animation: pulseGlow 6s ease-in-out infinite;
  }
}

@keyframes pulseGlow {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 0.8; }
}

.card-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  perspective: 2000rpx;
  position: relative;
  z-index: 1;
  
  &::before {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 500rpx;
    height: 500rpx;
    background: radial-gradient(circle, rgba(245, 87, 108, 0.2) 0%, transparent 70%);
    border-radius: 50%;
    animation: breathe 4s ease-in-out infinite;
    pointer-events: none;
  }
}

@keyframes breathe {
  0%, 100% {
    transform: translate(-50%, -50%) scale(1);
    opacity: 0.3;
  }
  50% {
    transform: translate(-50%, -50%) scale(1.3);
    opacity: 0.6;
  }
}

.card {
  width: 100%;
  max-width: 600rpx;
  height: 600rpx;
  position: relative;
  transform-style: preserve-3d;
  transition: transform 0.8s cubic-bezier(0.4, 0, 0.2, 1);
  animation: cardEnter 1s ease-out, cardFloat 6s ease-in-out infinite 1s;

  &.flipped {
    transform: rotateY(180deg);
  }

  .card-front,
  .card-back {
    position: absolute;
    width: 100%;
    height: 100%;
    backface-visibility: hidden;
    border-radius: 30rpx;
    overflow: hidden;
  }

  .card-front {
    @include neon-card;
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.5),
      0 0 60rpx rgba(245, 87, 108, 0.4),
      inset 0 0 80rpx rgba(245, 87, 108, 0.1);
    border: 2rpx solid rgba(245, 87, 108, 0.6);
    
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
        rgba(245, 87, 108, 0.2) 90deg,
        transparent 180deg,
        rgba(240, 147, 251, 0.2) 270deg,
        transparent 360deg
      );
      animation: rotate 8s linear infinite;
    }
  }

  .card-back {
    background: linear-gradient(135deg, rgba(245, 87, 108, 0.9) 0%, rgba(240, 147, 251, 0.9) 100%);
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.5),
      0 0 60rpx rgba(240, 147, 251, 0.6),
      inset 0 0 80rpx rgba(240, 147, 251, 0.2);
    border: 2rpx solid rgba(240, 147, 251, 0.8);
    transform: rotateY(180deg);
    
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
        rgba(255, 255, 255, 0.1) 90deg,
        transparent 180deg,
        rgba(255, 255, 255, 0.1) 270deg,
        transparent 360deg
      );
      animation: rotate 8s linear infinite reverse;
    }

    .card-content {
      .card-text,
      .card-subtitle {
        @include neon-text(#ffffff);
      }
    }
  }

  .card-content {
    height: 100%;
    padding: 60rpx;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    position: relative;
    z-index: 1;

    .card-icon {
      font-size: 100rpx;
      margin-bottom: 40rpx;
      filter: drop-shadow(0 0 30rpx rgba(245, 87, 108, 0.8));
      animation: iconBounce 3s ease-in-out infinite;
    }

    .card-title {
      font-size: 40rpx;
      font-weight: bold;
      @include neon-title(#F5576C);
      margin-bottom: 30rpx;
      animation: titlePulse 3s ease-in-out infinite;
    }

    .card-text {
      font-size: 30rpx;
      color: #b8c5d6;
      line-height: 1.8;
      margin-bottom: 20rpx;
      text-shadow: 0 0 10rpx rgba(184, 197, 214, 0.3);
    }

    .card-subtitle {
      font-size: 26rpx;
      @include neon-text(#ffffff);
      font-weight: 500;
    }

    .card-hint {
      font-size: 24rpx;
      @include neon-text(#F5576C);
      margin-top: 40rpx;
      animation: blink 2s ease-in-out infinite;
    }
  }
}

@keyframes cardEnter {
  from {
    opacity: 0;
    transform: perspective(2000rpx) rotateY(-30deg) translateY(100rpx);
  }
  to {
    opacity: 1;
    transform: perspective(2000rpx) rotateY(0deg) translateY(0);
  }
}

@keyframes cardFloat {
  0%, 100% {
    transform: perspective(2000rpx) rotateY(0deg) translateY(0) rotateX(0deg);
  }
  25% {
    transform: perspective(2000rpx) rotateY(2deg) translateY(-10rpx) rotateX(2deg);
  }
  75% {
    transform: perspective(2000rpx) rotateY(-2deg) translateY(-10rpx) rotateX(-2deg);
  }
}

@keyframes iconBounce {
  0%, 100% {
    transform: translateY(0) scale(1) rotate(0deg);
  }
  50% {
    transform: translateY(-15rpx) scale(1.1) rotate(5deg);
  }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 15rpx rgba(245, 87, 108, 0.8),
      0 0 30rpx rgba(245, 87, 108, 0.5),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
  50% {
    text-shadow: 
      0 0 25rpx rgba(245, 87, 108, 1),
      0 0 50rpx rgba(245, 87, 108, 0.8),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
}

@keyframes blink {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

.actions {
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  padding-top: 60rpx;
  position: relative;
  z-index: 1;
  gap: 20rpx;

  .btn {
    flex: 1;
    max-width: 200rpx;
    height: 100rpx;
    border-radius: 50rpx;
    border: none;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    font-size: 24rpx;
    padding: 0;
    line-height: 1;
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
      background: radial-gradient(circle, rgba(255, 255, 255, 0.3) 0%, transparent 70%);
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
    }

    text {
      display: block;
      line-height: 1;
      position: relative;
      z-index: 1;
      
      &:first-child {
        font-size: 36rpx;
        margin-bottom: 8rpx;
      }
    }
  }

  .btn-primary {
    @include glow-button(#F5576C);
    background: linear-gradient(135deg, rgba(245, 87, 108, 0.8) 0%, rgba(240, 147, 251, 0.8) 100%);
    font-weight: bold;
    animation: btnPulse 3s ease-in-out infinite;
    
    &:active {
      animation: none;
    }
  }

  .btn-secondary {
    background: rgba(30, 36, 66, 0.6);
    backdrop-filter: blur(10rpx);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(245, 87, 108, 0.4);
    box-shadow: 0 0 20rpx rgba(245, 87, 108, 0.3);
    
    &:active {
      border-color: rgba(245, 87, 108, 0.8);
      box-shadow: 0 0 30rpx rgba(245, 87, 108, 0.6);
    }
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(245, 87, 108, 0.5),
      0 8rpx 24rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(245, 87, 108, 0.2);
  }
  50% {
    box-shadow: 
      0 0 35rpx rgba(245, 87, 108, 0.8),
      0 12rpx 32rpx rgba(0, 0, 0, 0.4),
      inset 0 0 30rpx rgba(245, 87, 108, 0.3);
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes iconBounce {
  0%, 100% {
    transform: translateY(0) scale(1);
  }
  50% {
    transform: translateY(-10rpx) scale(1.05);
  }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 15rpx rgba(245, 87, 108, 0.8),
      0 0 30rpx rgba(245, 87, 108, 0.5);
  }
  50% {
    text-shadow: 
      0 0 25rpx rgba(245, 87, 108, 1),
      0 0 50rpx rgba(245, 87, 108, 0.8);
  }
}
</style>

