<template>
  <view class="release-page">
    <view class="header">
      <text class="title">🎉 释放空间</text>
      <text class="subtitle">放松心情，释放压力</text>
    </view>

    <!-- 游戏区 -->
    <view class="category-section">
      <view class="category-header">
        <text class="category-icon">🎮</text>
        <text class="category-title">轻松游戏</text>
      </view>
      <view class="feature-grid">
        <view class="feature-item game-item" @click="handleNavigate('/pages/release/mini-game')">
          <text class="item-icon">🎈</text>
          <text class="item-title">扎气球</text>
        </view>
        <view class="feature-item game-item" @click="handleNavigate('/pages/release/memory-card')">
          <text class="item-icon">🃏</text>
          <text class="item-title">记忆翻牌</text>
        </view>
      </view>
    </view>

    <!-- 脑力区 -->
    <view class="category-section">
      <view class="category-header">
        <text class="category-icon">🧠</text>
        <text class="category-title">脑力挑战</text>
      </view>
      <view class="feature-grid">
        <view class="feature-item brain-item" @click="handleNavigate('/pages/release/brain-teaser')">
          <text class="item-icon">💡</text>
          <text class="item-title">脑筋急转弯</text>
        </view>
        <view class="feature-item brain-item" @click="handleNavigate('/pages/daily/topic')">
          <text class="item-icon">🍽️</text>
          <text class="item-title">今日话题</text>
        </view>
      </view>
    </view>

    <!-- 助眠区 -->
    <view class="category-section">
      <view class="category-header">
        <text class="category-icon">🌙</text>
        <text class="category-title">助眠放松</text>
      </view>
      <view class="feature-grid">
        <view class="feature-item sleep-item" @click="handleNavigate('/pages/release/count-sheep')">
          <text class="item-icon">🐑</text>
          <text class="item-title">数羊助眠</text>
        </view>
        <view class="feature-item sleep-item" @click="uni.showToast({ title: '冥想功能开发中', icon: 'none' })">
          <text class="item-icon">🧘</text>
          <text class="item-title">冥想放松</text>
        </view>
      </view>
    </view>

    <!-- 创作区 -->
    <view class="category-section">
      <view class="category-header">
        <text class="category-icon">✍️</text>
        <text class="category-title">创作交流</text>
      </view>
      <view class="feature-grid">
        <view class="feature-item writing-item" @click="handleNavigate('/pages/release/writing-list')">
          <text class="item-icon">📝</text>
          <text class="item-title">写作广场</text>
        </view>
        <view class="feature-item writing-item" @click="handleNavigate('/pages/release/writing-create')">
          <text class="item-icon">✨</text>
          <text class="item-title">发布作品</text>
        </view>
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

.release-page {
  @include cyber-page-bg;
  padding: 40rpx 30rpx 50rpx;
  position: relative;
  
  &::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 15% 25%, rgba(102, 126, 234, 0.15) 0%, transparent 40%),
      radial-gradient(circle at 85% 75%, rgba(240, 147, 251, 0.15) 0%, transparent 40%);
    pointer-events: none;
    z-index: 0;
  }
}

.header {
  text-align: center;
  margin-bottom: 50rpx;
  position: relative;
  z-index: 1;
  animation: fadeInDown 0.8s ease-out;

  .title {
    display: block;
    font-size: 48rpx;
    font-weight: bold;
    @include neon-title(#FF00D6);
    margin-bottom: 15rpx;
    animation: neonFlicker 4s ease-in-out infinite;
  }

  .subtitle {
    display: block;
    font-size: 26rpx;
    @include neon-text(#FFD600);
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

.category-section {
  margin-bottom: 50rpx;
  position: relative;
  z-index: 1;
  animation: fadeInUp 0.6s ease-out backwards;

  &:nth-child(2) { animation-delay: 0.1s; }
  &:nth-child(3) { animation-delay: 0.2s; }
  &:nth-child(4) { animation-delay: 0.3s; }
  &:nth-child(5) { animation-delay: 0.4s; }

  &:last-child {
    margin-bottom: 0;
  }

  .category-header {
    display: flex;
    align-items: center;
    margin-bottom: 25rpx;
    background: rgba(30, 36, 66, 0.6);
    backdrop-filter: blur(10rpx);
    padding: 20rpx 25rpx;
    border-radius: 20rpx;
    box-shadow: 0 0 20rpx rgba(0, 217, 255, 0.2);
    border: 1rpx solid rgba(0, 217, 255, 0.3);

    .category-icon {
      font-size: 44rpx;
      margin-right: 15rpx;
      filter: drop-shadow(0 0 15rpx rgba(0, 217, 255, 0.8));
      animation: iconPulse 2s ease-in-out infinite;
    }

    .category-title {
      font-size: 34rpx;
      font-weight: bold;
      @include neon-title(#00D9FF);
    }
  }

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes iconPulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

  .feature-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 20rpx;
  }

  .feature-item {
    @include neon-card;
    border-radius: 25rpx;
    padding: 40rpx 30rpx;
    text-align: center;
    transition: all 0.3s;
    position: relative;
    animation: cardFloat 4s ease-in-out infinite;
    
    &:nth-child(odd) {
      animation-delay: 0s;
    }
    
    &:nth-child(even) {
      animation-delay: 0.5s;
    }

    &:active {
      transform: scale(0.95) translateY(-5rpx);
      box-shadow: 
        0 15rpx 50rpx rgba(0, 0, 0, 0.6),
        0 0 40rpx currentColor,
        inset 0 0 40rpx currentColor;
    }

    .item-icon {
      display: block;
      font-size: 60rpx;
      margin-bottom: 15rpx;
      animation: iconBounce 3s ease-in-out infinite;
    }

    .item-title {
      display: block;
      font-size: 28rpx;
      @include neon-title(#ffffff);
      font-weight: 500;
    }
  }

@keyframes cardFloat {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-5rpx); }
}

@keyframes iconBounce {
  0%, 100% { transform: translateY(0) scale(1); }
  50% { transform: translateY(-5rpx) scale(1.05); }
}

  .game-item {
    border-left: 4rpx solid rgba(102, 126, 234, 0.8);
    box-shadow: 
      0 10rpx 40rpx rgba(0, 0, 0, 0.5),
      -4rpx 0 20rpx rgba(102, 126, 234, 0.5),
      inset 0 0 30rpx rgba(102, 126, 234, 0.1);
    
    .item-icon {
      filter: drop-shadow(0 0 15rpx rgba(102, 126, 234, 0.8));
    }
  }

  .brain-item {
    border-left: 4rpx solid rgba(240, 147, 251, 0.8);
    box-shadow: 
      0 10rpx 40rpx rgba(0, 0, 0, 0.5),
      -4rpx 0 20rpx rgba(240, 147, 251, 0.5),
      inset 0 0 30rpx rgba(240, 147, 251, 0.1);
    
    .item-icon {
      filter: drop-shadow(0 0 15rpx rgba(240, 147, 251, 0.8));
    }
  }

  .sleep-item {
    border-left: 4rpx solid rgba(138, 92, 246, 0.8);
    box-shadow: 
      0 10rpx 40rpx rgba(0, 0, 0, 0.5),
      -4rpx 0 20rpx rgba(138, 92, 246, 0.5),
      inset 0 0 30rpx rgba(138, 92, 246, 0.1);
    
    .item-icon {
      filter: drop-shadow(0 0 15rpx rgba(138, 92, 246, 0.8));
    }
  }

  .writing-item {
    border-left: 4rpx solid rgba(67, 233, 123, 0.8);
    box-shadow: 
      0 10rpx 40rpx rgba(0, 0, 0, 0.5),
      -4rpx 0 20rpx rgba(67, 233, 123, 0.5),
      inset 0 0 30rpx rgba(67, 233, 123, 0.1);
    
    .item-icon {
      filter: drop-shadow(0 0 15rpx rgba(67, 233, 123, 0.8));
    }
  }
}
</style>
