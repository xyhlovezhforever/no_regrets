<template>
  <view class="count-sheep-page">
    <!-- 天空背景 -->
    <view class="sky">
      <view class="moon">🌙</view>
      <view class="stars">
        <text class="star">⭐</text>
        <text class="star">✨</text>
        <text class="star">⭐</text>
      </view>
    </view>

    <!-- 计数显示 -->
    <view class="counter">
      <text class="count-number">{{ count }}</text>
      <text class="count-label">只羊</text>
    </view>

    <!-- 羊的动画区域 -->
    <view class="sheep-area">
      <view
        v-for="(sheep, index) in visibleSheep"
        :key="index"
        class="sheep"
        :style="{ animationDelay: `${index * 0.8}s` }"
      >
        🐑
      </view>
    </view>

    <!-- 操作按钮 -->
    <view class="actions">
      <button class="action-btn count-btn" @click="countSheep" :disabled="isAutoMode">
        <text class="btn-text">数一只羊</text>
      </button>
      <button class="action-btn auto-btn" @click="toggleAutoMode">
        <text class="btn-text">{{ isAutoMode ? '停止' : '自动数羊' }}</text>
      </button>
      <button class="action-btn reset-btn" @click="reset">
        <text class="btn-text">重新开始</text>
      </button>
    </view>

    <!-- 助眠音乐 -->
    <view class="music-control">
      <text class="music-label">助眠音乐</text>
      <switch :checked="isMusicOn" @change="toggleMusic" color="#667eea" />
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const count = ref(0)
const isAutoMode = ref(false)
const isMusicOn = ref(false)
let autoTimer: any = null

const visibleSheep = computed(() => {
  return Math.min(count.value % 10 || 10, 10)
})

const countSheep = () => {
  count.value++
  
  // 震动反馈
  uni.vibrateShort({
    type: 'light'
  })
  
  // 语音播报（使用系统TTS，如果支持）
  if (count.value % 10 === 0) {
    uni.showToast({
      title: `已数${count.value}只羊`,
      icon: 'none',
      duration: 1000
    })
  }
}

const toggleAutoMode = () => {
  if (isAutoMode.value) {
    // 停止自动模式
    clearInterval(autoTimer)
    autoTimer = null
    isAutoMode.value = false
  } else {
    // 开始自动模式
    isAutoMode.value = true
    autoTimer = setInterval(() => {
      countSheep()
      
      // 数到100只后停止
      if (count.value >= 100) {
        toggleAutoMode()
        uni.showModal({
          title: '提示',
          content: '已经数了100只羊啦，该睡觉了~ 😴',
          showCancel: false
        })
      }
    }, 2000) // 每2秒数一只
  }
}

const reset = () => {
  if (isAutoMode.value) {
    toggleAutoMode()
  }
  count.value = 0
  uni.showToast({ title: '已重置', icon: 'success' })
}

const toggleMusic = (e: any) => {
  isMusicOn.value = e.detail.value
  
  if (isMusicOn.value) {
    // 播放助眠音乐（示例，实际需要音频文件）
    uni.showToast({ title: '助眠音乐功能开发中', icon: 'none' })
  } else {
    // 停止音乐
  }
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.count-sheep-page {
  @include cyber-page-bg;
  min-height: 100vh;
  position: relative;
  overflow: hidden;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 30% 20%, rgba(138, 92, 246, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 70% 80%, rgba(0, 217, 255, 0.15) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.sky {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 1;

  .moon {
    position: absolute;
    top: 100rpx;
    right: 80rpx;
    font-size: 140rpx;
    filter: drop-shadow(0 0 40rpx rgba(255, 214, 0, 0.8));
    animation: float 6s ease-in-out infinite, moonGlow 3s ease-in-out infinite;
  }

  .stars {
    .star {
      position: absolute;
      font-size: 50rpx;
      filter: drop-shadow(0 0 20rpx rgba(0, 217, 255, 0.8));
      animation: twinkle 2s ease-in-out infinite, starGlow 3s ease-in-out infinite;

      &:nth-child(1) {
        top: 150rpx;
        left: 100rpx;
      }

      &:nth-child(2) {
        top: 300rpx;
        right: 150rpx;
        animation-delay: 0.5s;
      }

      &:nth-child(3) {
        top: 450rpx;
        left: 200rpx;
        animation-delay: 1s;
      }
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-25rpx);
  }
}

@keyframes moonGlow {
  0%, 100% {
    filter: drop-shadow(0 0 40rpx rgba(255, 214, 0, 0.8));
  }
  50% {
    filter: drop-shadow(0 0 60rpx rgba(255, 214, 0, 1));
  }
}

@keyframes twinkle {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.3;
  }
}

@keyframes starGlow {
  0%, 100% {
    filter: drop-shadow(0 0 20rpx rgba(0, 217, 255, 0.8));
  }
  50% {
    filter: drop-shadow(0 0 35rpx rgba(0, 217, 255, 1));
  }
}

.counter {
  padding-top: 150rpx;
  text-align: center;
  position: relative;
  z-index: 2;

  .count-number {
    display: block;
    font-size: 140rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
    filter: brightness(1.3);
    text-shadow: 
      0 0 30rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(255, 214, 0, 0.8),
      0 0 90rpx rgba(255, 214, 0, 0.5),
      0 5rpx 10rpx rgba(0, 0, 0, 0.3);
    animation: countPulse 2s ease-in-out infinite;
  }

  .count-label {
    display: block;
    font-size: 36rpx;
    @include neon-text(#00D9FF);
    margin-top: 25rpx;
    font-weight: 600;
  }
}

@keyframes countPulse {
  0%, 100% {
    transform: scale(1);
    text-shadow: 
      0 0 30rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(255, 214, 0, 0.8),
      0 0 90rpx rgba(255, 214, 0, 0.5),
      0 5rpx 10rpx rgba(0, 0, 0, 0.3);
  }
  50% {
    transform: scale(1.05);
    text-shadow: 
      0 0 40rpx rgba(255, 214, 0, 1),
      0 0 80rpx rgba(255, 214, 0, 1),
      0 0 120rpx rgba(255, 214, 0, 0.7),
      0 5rpx 10rpx rgba(0, 0, 0, 0.3);
  }
}

.sheep-area {
  height: 400rpx;
  margin: 80rpx 0;
  position: relative;
  z-index: 2;
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  align-items: center;
  gap: 35rpx;
  padding: 0 50rpx;

  .sheep {
    font-size: 90rpx;
    filter: drop-shadow(0 0 20rpx rgba(255, 255, 255, 0.8));
    animation: jump 1.5s ease-in-out infinite, sheepGlow 3s ease-in-out infinite;
  }
}

@keyframes jump {
  0%, 100% {
    transform: translateY(0) scale(1);
    opacity: 1;
  }
  50% {
    transform: translateY(-35rpx) scale(1.15);
    opacity: 0.85;
  }
}

@keyframes sheepGlow {
  0%, 100% {
    filter: drop-shadow(0 0 20rpx rgba(255, 255, 255, 0.8));
  }
  50% {
    filter: drop-shadow(0 0 35rpx rgba(255, 255, 255, 1));
  }
}

.actions {
  padding: 30rpx;
  display: flex;
  flex-direction: column;
  gap: 25rpx;
  position: relative;
  z-index: 2;

  .action-btn {
    width: 100%;
    height: 95rpx;
    border-radius: 50rpx;
    border: none;
    font-size: 34rpx;
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);

    &::after {
      border: none;
    }

    &:active {
      transform: scale(0.92);
    }

    .btn-text {
      @include neon-text(#ffffff);
      font-weight: bold;
      text-shadow: 
        0 0 10rpx rgba(255, 255, 255, 0.6),
        0 2rpx 5rpx rgba(0, 0, 0, 0.3);
    }
  }

  .count-btn {
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    border: 2rpx solid rgba(0, 217, 255, 0.7);
    box-shadow: 
      0 10rpx 40rpx rgba(0, 217, 255, 0.5),
      0 0 60rpx rgba(0, 217, 255, 0.4);
    animation: btnPulse 3s ease-in-out infinite;

    &:disabled {
      background: linear-gradient(135deg, rgba(100, 100, 100, 0.4) 0%, rgba(80, 80, 80, 0.4) 100%);
      border-color: rgba(100, 100, 100, 0.4);
      box-shadow: none;
      animation: none;
      
      .btn-text {
        @include neon-text(#6b7b93);
        text-shadow: none;
      }
    }

    &:active:not(:disabled) {
      box-shadow: 
        0 12rpx 50rpx rgba(0, 217, 255, 0.7),
        0 0 80rpx rgba(0, 217, 255, 0.6);
    }
  }

  .auto-btn {
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    border: 2rpx solid rgba(255, 0, 214, 0.7);
    box-shadow: 
      0 10rpx 40rpx rgba(255, 0, 214, 0.5),
      0 0 60rpx rgba(255, 0, 214, 0.4);

    &:active {
      box-shadow: 
        0 12rpx 50rpx rgba(255, 0, 214, 0.7),
        0 0 80rpx rgba(255, 0, 214, 0.6);
    }
  }

  .reset-btn {
    background: linear-gradient(135deg, rgba(138, 92, 246, 0.9) 0%, rgba(0, 217, 255, 0.9) 100%);
    border: 2rpx solid rgba(138, 92, 246, 0.7);
    box-shadow: 
      0 10rpx 40rpx rgba(138, 92, 246, 0.5),
      0 0 60rpx rgba(138, 92, 246, 0.4);

    &:active {
      box-shadow: 
        0 12rpx 50rpx rgba(138, 92, 246, 0.7),
        0 0 80rpx rgba(138, 92, 246, 0.6);
    }
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(0, 217, 255, 0.5),
      0 0 60rpx rgba(0, 217, 255, 0.4);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(0, 217, 255, 0.7),
      0 0 80rpx rgba(0, 217, 255, 0.6);
  }
}

.music-control {
  position: fixed;
  bottom: 180rpx;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 25rpx;
  padding: 22rpx 35rpx;
  background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
  backdrop-filter: blur(20rpx);
  border-radius: 50rpx;
  border: 2rpx solid rgba(255, 0, 214, 0.5);
  box-shadow: 
    0 8rpx 24rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(255, 0, 214, 0.3);
  z-index: 2;

  .music-label {
    font-size: 30rpx;
    @include neon-text(#FFD600);
    font-weight: 600;
  }
}
</style>

