<template>
  <view class="tools-page">
    <view class="header">
      <text class="title">🛠️ 生活工具</text>
      <text class="subtitle">实用工具，让生活更便捷</text>
    </view>

    <view class="feature-list">
      <view class="feature-item" @click="handleNavigate('/pages/tools/watermark-remover')">
        <text class="feature-icon">💧</text>
        <text class="feature-title">去文档水印</text>
        <text class="feature-desc">自动去除文档中的水印</text>
      </view>

      <view class="feature-item" @click="handleNavigate('/pages/tools/md-editor')">
        <text class="feature-icon">📝</text>
        <text class="feature-title">Markdown编辑器</text>
        <text class="feature-desc">查看和编辑Markdown文档</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { navigateTo } from '@/utils'

const handleNavigate = (url: string) => {
  navigateTo(url)
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.tools-page {
  @include cyber-page-bg;
  padding: 60rpx 30rpx;
  position: relative;
  animation: fadeIn 0.5s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.header {
  text-align: center;
  margin-bottom: 70rpx;
  animation: slideDown 0.6s ease-out;
  position: relative;
  z-index: 1;

  .title {
    display: block;
    font-size: 56rpx;
    font-weight: bold;
    @include neon-title(#8B5CF6);
    margin-bottom: 15rpx;
    animation: neonFlicker 4s ease-in-out infinite;
  }

  .subtitle {
    display: block;
    font-size: 28rpx;
    @include neon-text(#00D9FF);
    font-weight: 500;
  }
}

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

.feature-list {
  display: flex;
  flex-direction: column;
  gap: 25rpx;
  position: relative;
  z-index: 1;
}

.feature-item {
  @include neon-card;
  @include rainbow-border;
  border-radius: 30rpx;
  padding: 60rpx 40rpx;
  text-align: center;
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  animation: scaleIn 0.5s ease-out backwards, cardFloat 3s ease-in-out infinite;

  &:nth-child(1) {
    animation-delay: 0.1s;
    .feature-icon {
      filter: drop-shadow(0 0 20rpx rgba(0, 217, 255, 0.8));
    }
  }

  &:nth-child(2) {
    animation-delay: 0.2s;
    .feature-icon {
      filter: drop-shadow(0 0 20rpx rgba(139, 92, 246, 0.8));
    }
  }

  &:active {
    transform: scale(0.95) translateY(-5rpx);
    box-shadow: 
      0 15rpx 50rpx rgba(0, 0, 0, 0.6),
      0 0 40rpx rgba(0, 217, 255, 0.6),
      inset 0 0 40rpx rgba(0, 217, 255, 0.2);
  }

  .feature-icon {
    display: block;
    font-size: 100rpx;
    margin-bottom: 25rpx;
    transition: all 0.3s ease;
    animation: bounce 2s ease-in-out infinite, glowPulse 2s ease-in-out infinite;
  }

  &:active .feature-icon {
    transform: scale(1.2) rotate(10deg);
  }

  .feature-title {
    display: block;
    font-size: 38rpx;
    font-weight: bold;
    @include neon-title(#ffffff);
    margin-bottom: 12rpx;
  }

  .feature-desc {
    display: block;
    font-size: 26rpx;
    @include neon-text(#8B5CF6);
    font-weight: 500;
  }
}

@keyframes cardFloat {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10rpx); }
}

@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.9);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-8rpx);
  }
}
</style>

