<template>
  <view class="warmth-page">
    <view class="header">
      <text class="title">温情篇</text>
      <text class="subtitle">用爱与温暖连接彼此</text>
    </view>

    <view class="feature-list">
      <view class="feature-item" @click="handleNavigate('/pages/warmth/love-letter')">
        <text class="feature-icon">💌</text>
        <text class="feature-title">写情书</text>
        <text class="feature-desc">带给亲人与爱人你的知心话</text>
      </view>

      <view class="feature-item" @click="handleNavigate('/pages/warmth/hug')">
        <text class="feature-icon">🤗</text>
        <text class="feature-title">给自己一个拥抱</text>
        <text class="feature-desc">给自己一个拥抱</text>
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

.warmth-page {
  @include cyber-page-bg;
  min-height: 100vh;
  padding: 60rpx 30rpx;
  position: relative;
  display: flex;
  flex-direction: column;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 20% 20%, rgba(255, 0, 214, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 80%, rgba(0, 217, 255, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 50% 50%, rgba(138, 92, 246, 0.1) 0%, transparent 60%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
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
      rgba(255, 0, 214, 0.05) 90deg,
      transparent 180deg,
      rgba(0, 217, 255, 0.05) 270deg,
      transparent 360deg
    );
    pointer-events: none;
    animation: bgRotate 20s linear infinite;
    z-index: 0;
  }
}

.header {
  text-align: center;
  padding: 40rpx 0 60rpx;
  position: relative;
  z-index: 1;
  animation: fadeInDown 0.8s ease-out;

  .title {
    display: block;
    font-size: 56rpx;
    font-weight: bold;
    @include neon-title(#FF00D6);
    margin-bottom: 20rpx;
    text-shadow: 
      0 0 20rpx rgba(255, 0, 214, 1),
      0 0 40rpx rgba(255, 0, 214, 0.8),
      0 0 60rpx rgba(255, 0, 214, 0.6);
    animation: titlePulse 3s ease-in-out infinite;
  }

  .subtitle {
    display: block;
    font-size: 30rpx;
    @include neon-text(#00D9FF);
    animation: subtitleBlink 2s ease-in-out infinite;
  }
}

@keyframes fadeInDown {
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
  gap: 30rpx;
  position: relative;
  z-index: 1;
  flex: 1;
}

.feature-item {
  @include neon-card;
  border-radius: 30rpx;
  padding: 60rpx 40rpx;
  text-align: center;
  animation: fadeInUp 0.6s ease-out backwards, cardFloat 4s ease-in-out infinite;
  position: relative;
  overflow: hidden;
  border: 2rpx solid transparent;
  background-clip: padding-box;
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(255, 0, 214, 0.4),
    inset 0 0 50rpx rgba(255, 0, 214, 0.1);

  &:nth-child(1) {
    animation-delay: 0.2s;
    border-color: rgba(255, 0, 214, 0.6);
    
    &::before {
      content: '';
      position: absolute;
      inset: -3rpx;
      border-radius: 30rpx;
      padding: 3rpx;
      background: linear-gradient(135deg, 
        rgba(255, 0, 214, 0.8) 0%,
        rgba(138, 92, 246, 0.8) 50%,
        rgba(255, 0, 214, 0.8) 100%
      );
      -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
      -webkit-mask-composite: xor;
      mask-composite: exclude;
      animation: borderRotate 4s linear infinite;
      z-index: -1;
    }
    
    &::after {
      content: '';
      position: absolute;
      top: -50%;
      left: -50%;
      width: 200%;
      height: 200%;
      background: radial-gradient(circle, rgba(255, 0, 214, 0.15) 0%, transparent 60%);
      animation: rotate 8s linear infinite;
      z-index: 0;
    }
  }

  &:nth-child(2) {
    animation-delay: 0.4s;
    border-color: rgba(0, 217, 255, 0.6);
    box-shadow: 
      0 15rpx 50rpx rgba(0, 0, 0, 0.5),
      0 0 60rpx rgba(0, 217, 255, 0.4),
      inset 0 0 50rpx rgba(0, 217, 255, 0.1);
    
    &::before {
      content: '';
      position: absolute;
      inset: -3rpx;
      border-radius: 30rpx;
      padding: 3rpx;
      background: linear-gradient(135deg, 
        rgba(0, 217, 255, 0.8) 0%,
        rgba(138, 92, 246, 0.8) 50%,
        rgba(0, 217, 255, 0.8) 100%
      );
      -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
      -webkit-mask-composite: xor;
      mask-composite: exclude;
      animation: borderRotate 4s linear infinite reverse;
      z-index: -1;
    }
    
    &::after {
      content: '';
      position: absolute;
      top: -50%;
      left: -50%;
      width: 200%;
      height: 200%;
      background: radial-gradient(circle, rgba(0, 217, 255, 0.15) 0%, transparent 60%);
      animation: rotate 10s linear infinite reverse;
      z-index: 0;
    }
  }

  &:active {
    transform: scale(0.96) translateY(2rpx);
    animation-play-state: paused;
    
    &:nth-child(1) {
      box-shadow: 
        0 10rpx 30rpx rgba(0, 0, 0, 0.6),
        0 0 80rpx rgba(255, 0, 214, 0.7),
        inset 0 0 60rpx rgba(255, 0, 214, 0.2);
    }
    
    &:nth-child(2) {
      box-shadow: 
        0 10rpx 30rpx rgba(0, 0, 0, 0.6),
        0 0 80rpx rgba(0, 217, 255, 0.7),
        inset 0 0 60rpx rgba(0, 217, 255, 0.2);
    }
  }

  .feature-icon {
    display: block;
    font-size: 100rpx;
    margin-bottom: 30rpx;
    filter: drop-shadow(0 0 30rpx rgba(255, 0, 214, 1));
    animation: iconBounce 3s ease-in-out infinite;
    position: relative;
    z-index: 1;
  }

  .feature-title {
    display: block;
    font-size: 40rpx;
    font-weight: bold;
    @include neon-title(#ffffff);
    margin-bottom: 15rpx;
    position: relative;
    z-index: 1;
  }

  .feature-desc {
    display: block;
    font-size: 28rpx;
    position: relative;
    z-index: 1;
  }
  
  &:nth-child(1) .feature-desc {
    @include neon-text(#FF00D6);
  }
  
  &:nth-child(2) .feature-desc {
    @include neon-text(#00D9FF);
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(40rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes cardFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-8rpx);
  }
}

@keyframes iconBounce {
  0%, 100% {
    transform: translateY(0) scale(1);
  }
  50% {
    transform: translateY(-10rpx) scale(1.1);
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes bgPulse {
  0%, 100% {
    opacity: 0.6;
  }
  50% {
    opacity: 1;
  }
}

@keyframes bgRotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 20rpx rgba(255, 0, 214, 1),
      0 0 40rpx rgba(255, 0, 214, 0.8),
      0 0 60rpx rgba(255, 0, 214, 0.6);
  }
  50% {
    text-shadow: 
      0 0 30rpx rgba(255, 0, 214, 1),
      0 0 60rpx rgba(255, 0, 214, 1),
      0 0 90rpx rgba(255, 0, 214, 0.8);
  }
}

@keyframes subtitleBlink {
  0%, 100% { opacity: 0.7; }
  50% { opacity: 1; }
}

@keyframes borderRotate {
  from {
    filter: hue-rotate(0deg);
  }
  to {
    filter: hue-rotate(360deg);
  }
}
</style>

