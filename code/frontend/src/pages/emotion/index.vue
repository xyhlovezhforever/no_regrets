<template>
  <view class="emotion-page">
    <!-- 动态粒子背景 -->
    <view class="particles-container">
      <view 
        v-for="i in 20" 
        :key="i" 
        class="particle"
        :style="getParticleStyle(i)"
      ></view>
    </view>

    <!-- 浮动心形装饰 -->
    <view class="floating-hearts">
      <view v-for="i in 8" :key="i" class="heart" :style="getHeartStyle(i)">💝</view>
    </view>

    <!-- 顶部横幅 -->
    <view class="hero-section">
      <view class="hero-glow"></view>
      <view class="hero-content">
        <view class="greeting-wrapper">
          <text class="greeting">{{ greeting }}</text>
          <text class="username">{{ userStore.nickname || '朋友' }}</text>
          <view class="user-level">
            <text class="level-icon">⭐</text>
            <text class="level-text">温暖使者</text>
          </view>
        </view>
        <view class="daily-quote-mini">
          <text class="quote-icon">✨</text>
          <text class="quote-text">温暖你的每一天</text>
          <view class="quote-shine"></view>
        </view>
      </view>
      <view class="hero-decoration">
        <view class="decoration-circle circle-1"></view>
        <view class="decoration-circle circle-2"></view>
        <view class="decoration-circle circle-3"></view>
      </view>
      <!-- 心跳波纹 -->
      <view class="heartbeat-ripple"></view>
    </view>

    <!-- 功能卡片 -->
    <scroll-view class="content-scroll" scroll-y>
      <view class="section">
        <view class="section-header">
          <view class="section-icon">💝</view>
          <text class="section-title">情感陪伴</text>
          <text class="section-subtitle">温暖你的每一天</text>
        </view>
        <view class="section-cards">
          <view class="feature-card card-ai" @click="handleNavigate('/pages/ai/chat-list')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">🤖</text>
              <view class="card-text">
                <text class="card-title">AI 智能体</text>
                <text class="card-desc">全天候陪伴·深度倾听</text>
              </view>
            </view>
            <view class="card-arrow">⚡</view>
          </view>
          <view class="feature-card card-friends" @click="handleNavigate('/pages/friends/list')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">💬</text>
              <view class="card-text">
                <text class="card-title">好友圈</text>
                <text class="card-desc">连接·分享·共鸣</text>
              </view>
            </view>
            <view class="card-arrow">⚡</view>
          </view>
          <view class="feature-card card-warmth" @click="handleNavigate('/pages/warmth/index')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">💝</text>
              <view class="card-text">
                <text class="card-title">温柔时光</text>
                <text class="card-desc">情书·拥抱·治愈</text>
              </view>
            </view>
            <view class="card-arrow">⚡</view>
          </view>
          <view class="feature-card card-idol" @click="handleNavigate('/pages/idol/index')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">✨</text>
              <view class="card-text">
                <text class="card-title">星光榜样</text>
                <text class="card-desc">追光者·梦想力量</text>
              </view>
            </view>
            <view class="card-arrow">⚡</view>
          </view>
        </view>
      </view>
      <view class="bottom-space"></view>
    </scroll-view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { useUserStore } from '@/store'
import { navigateTo } from '@/utils'

const userStore = useUserStore()

const greeting = computed(() => {
  const hour = new Date().getHours()
  if (hour < 6) return '夜深了'
  if (hour < 9) return '早上好'
  if (hour < 12) return '上午好'
  if (hour < 14) return '中午好'
  if (hour < 18) return '下午好'
  if (hour < 22) return '晚上好'
  return '夜深了'
})

// 粒子样式
const getParticleStyle = (index: number) => {
  const left = Math.random() * 100
  const animationDuration = 3 + Math.random() * 4
  const animationDelay = Math.random() * 3
  const size = 4 + Math.random() * 6
  
  return {
    left: `${left}%`,
    animationDuration: `${animationDuration}s`,
    animationDelay: `${animationDelay}s`,
    width: `${size}rpx`,
    height: `${size}rpx`
  }
}

// 心形样式
const getHeartStyle = (index: number) => {
  const left = 10 + Math.random() * 80
  const animationDuration = 5 + Math.random() * 5
  const animationDelay = Math.random() * 5
  const size = 30 + Math.random() * 30
  
  return {
    left: `${left}%`,
    fontSize: `${size}rpx`,
    animationDuration: `${animationDuration}s`,
    animationDelay: `${animationDelay}s`
  }
}

const handleNavigate = (url: string) => {
  navigateTo(url)
}

onShow(() => {
  // TabBar组件会自动检测当前页面路由
})
</script>

<style lang="scss" scoped>
@import '@/styles/tabbar-page.scss';

/* ============= 情感陪伴页面超炫特效 ============= */

/* 动态粒子背景 */
.particles-container {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 0;
  overflow: hidden;
  
  .particle {
    position: absolute;
    bottom: -10rpx;
    background: radial-gradient(circle, rgba(255, 0, 214, 0.8), rgba(0, 217, 255, 0.6));
    border-radius: 50%;
    animation: particleRise 7s infinite ease-in;
    box-shadow: 
      0 0 20rpx rgba(255, 0, 214, 0.8),
      0 0 40rpx rgba(0, 217, 255, 0.6);
  }
}

@keyframes particleRise {
  0% {
    bottom: -10rpx;
    opacity: 0;
    transform: translateY(0) scale(0);
  }
  10% {
    opacity: 1;
  }
  90% {
    opacity: 1;
  }
  100% {
    bottom: 110%;
    opacity: 0;
    transform: translateY(-100rpx) scale(1.5);
  }
}

/* 浮动心形装饰 */
.floating-hearts {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 1;
  overflow: hidden;
  
  .heart {
    position: absolute;
    bottom: -60rpx;
    animation: heartFloat 10s infinite ease-in-out;
    filter: drop-shadow(0 0 15rpx rgba(255, 0, 214, 0.8));
    opacity: 0;
  }
}

@keyframes heartFloat {
  0% {
    bottom: -60rpx;
    opacity: 0;
    transform: translateX(0) rotate(0deg) scale(0.5);
  }
  10% {
    opacity: 0.8;
  }
  50% {
    transform: translateX(20rpx) rotate(180deg) scale(1);
  }
  90% {
    opacity: 0.8;
  }
  100% {
    bottom: 110%;
    opacity: 0;
    transform: translateX(-20rpx) rotate(360deg) scale(0.5);
  }
}

/* Hero区域增强 */
.emotion-page .hero-section {
  overflow: visible !important;
  
  .hero-glow {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 150%;
    height: 150%;
    background: radial-gradient(circle, rgba(255, 0, 214, 0.3), transparent 70%);
    animation: heroGlowPulse 4s ease-in-out infinite;
    pointer-events: none;
  }
  
  .greeting-wrapper {
    .greeting {
      animation: textShine 3s ease-in-out infinite;
      font-size: 32rpx !important;
      text-shadow: 
        0 0 20rpx rgba(0, 217, 255, 1),
        0 0 40rpx rgba(0, 217, 255, 0.6);
    }
    
    .username {
      animation: usernamePulse 2s ease-in-out infinite;
      font-size: 64rpx !important;
    }
    
    .user-level {
      display: flex;
      align-items: center;
      gap: 8rpx;
      margin-top: 15rpx;
      padding: 8rpx 20rpx;
      background: linear-gradient(135deg, rgba(255, 214, 0, 0.2), rgba(255, 0, 214, 0.2));
      border-radius: 20rpx;
      border: 2rpx solid rgba(255, 214, 0, 0.5);
      box-shadow: 
        0 0 20rpx rgba(255, 214, 0, 0.5),
        inset 0 0 10rpx rgba(255, 214, 0, 0.3);
      animation: levelBadgePulse 2s ease-in-out infinite;
      
      .level-icon {
        font-size: 24rpx;
        animation: starRotate 3s linear infinite;
      }
      
      .level-text {
        font-size: 22rpx;
        color: #FFD600;
        font-weight: 600;
        text-shadow: 0 0 10rpx rgba(255, 214, 0, 0.8);
      }
    }
  }
  
  .daily-quote-mini {
    position: relative;
    overflow: visible;
    animation: quoteBounce 3s ease-in-out infinite;
    
    .quote-shine {
      position: absolute;
      top: -5rpx;
      right: -5rpx;
      width: 15rpx;
      height: 15rpx;
      background: radial-gradient(circle, rgba(255, 255, 255, 1), transparent);
      border-radius: 50%;
      animation: shineTwinkle 2s ease-in-out infinite;
    }
  }
  
  .heartbeat-ripple {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 100rpx;
    height: 100rpx;
    border: 3rpx solid rgba(255, 0, 214, 0.6);
    border-radius: 50%;
    animation: heartbeatRipple 2s ease-out infinite;
  }
}

@keyframes heroGlowPulse {
  0%, 100% {
    opacity: 0.3;
    transform: translate(-50%, -50%) scale(1);
  }
  50% {
    opacity: 0.6;
    transform: translate(-50%, -50%) scale(1.2);
  }
}

@keyframes textShine {
  0%, 100% {
    filter: brightness(1);
  }
  50% {
    filter: brightness(1.3);
  }
}

@keyframes usernamePulse {
  0%, 100% {
    transform: scale(1);
    filter: drop-shadow(0 0 20rpx rgba(0, 217, 255, 0.8));
  }
  50% {
    transform: scale(1.02);
    filter: drop-shadow(0 0 30rpx rgba(255, 0, 214, 1));
  }
}

@keyframes levelBadgePulse {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(255, 214, 0, 0.5),
      inset 0 0 10rpx rgba(255, 214, 0, 0.3);
  }
  50% {
    box-shadow: 
      0 0 30rpx rgba(255, 214, 0, 0.8),
      0 0 50rpx rgba(255, 0, 214, 0.5),
      inset 0 0 20rpx rgba(255, 214, 0, 0.5);
  }
}

@keyframes starRotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes quoteBounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-5rpx);
  }
}

@keyframes shineTwinkle {
  0%, 100% {
    opacity: 0;
    transform: scale(0);
  }
  50% {
    opacity: 1;
    transform: scale(1.5);
  }
}

@keyframes heartbeatRipple {
  0% {
    width: 100rpx;
    height: 100rpx;
    opacity: 1;
  }
  100% {
    width: 400rpx;
    height: 400rpx;
    opacity: 0;
  }
}

/* 功能卡片增强 */
.emotion-page .section-cards {
  .feature-card {
    position: relative;
    animation: cardSlideIn 0.6s ease-out backwards, cardFloat 4s ease-in-out infinite;
    
    &:nth-child(1) { 
      animation-delay: 0.1s, 0s;
      background: linear-gradient(135deg, rgba(40, 26, 56, 0.9), rgba(30, 20, 45, 0.95));
    }
    &:nth-child(2) { 
      animation-delay: 0.2s, 1s;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.9), rgba(25, 31, 56, 0.95));
    }
    &:nth-child(3) { 
      animation-delay: 0.3s, 2s;
      background: linear-gradient(135deg, rgba(35, 26, 56, 0.9), rgba(25, 20, 50, 0.95));
    }
    &:nth-child(4) { 
      animation-delay: 0.4s, 3s;
      background: linear-gradient(135deg, rgba(30, 30, 60, 0.9), rgba(20, 25, 50, 0.95));
    }
    
    /* 星光粒子 */
    &::before {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.6), rgba(0, 217, 255, 0.6), rgba(255, 214, 0, 0.6));
      animation: borderGlow 3s ease-in-out infinite;
    }
    
    .card-icon {
      font-size: 80rpx !important;
      margin-right: 25rpx !important;
      animation: cardIconPulse 2s ease-in-out infinite;
      filter: drop-shadow(0 0 20rpx rgba(255, 0, 214, 0.8)) drop-shadow(0 0 40rpx rgba(0, 217, 255, 0.6));
      text-shadow: 
        0 0 30rpx rgba(255, 0, 214, 1),
        0 0 50rpx rgba(0, 217, 255, 0.8);
    }
    
    .card-title {
      font-size: 42rpx !important;
      font-weight: 800 !important;
      background: linear-gradient(135deg, #FF00D6, #00D9FF, #FFD600);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
      filter: drop-shadow(0 0 20rpx rgba(255, 0, 214, 0.6));
      letter-spacing: 2rpx;
      animation: titleShine 3s ease-in-out infinite;
    }
    
    .card-desc {
      font-size: 28rpx !important;
      color: #00D9FF !important;
      font-weight: 600;
      text-shadow: 
        0 0 15rpx rgba(0, 217, 255, 1),
        0 0 30rpx rgba(0, 217, 255, 0.6),
        0 2rpx 8rpx rgba(0, 0, 0, 0.5);
      letter-spacing: 1rpx;
    }
    
    .card-arrow {
      font-size: 60rpx !important;
      filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 1));
      text-shadow: 
        0 0 30rpx rgba(255, 214, 0, 1),
        0 0 50rpx rgba(255, 0, 214, 0.8);
      animation: arrowBounce 1.5s ease-in-out infinite;
    }
    
    &:active .card-icon {
      animation: iconExplode 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55);
      filter: 
        drop-shadow(0 0 40rpx rgba(255, 0, 214, 1)) 
        drop-shadow(0 0 60rpx rgba(0, 217, 255, 1))
        drop-shadow(0 0 80rpx rgba(255, 214, 0, 0.8));
    }
  }
}

@keyframes cardSlideIn {
  from {
    opacity: 0;
    transform: translateX(-50rpx) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateX(0) scale(1);
  }
}

@keyframes cardIconPulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
}

@keyframes iconExplode {
  0% {
    transform: scale(1) rotate(0deg);
  }
  50% {
    transform: scale(1.5) rotate(180deg);
  }
  100% {
    transform: scale(1.3) rotate(360deg);
  }
}

@keyframes titleShine {
  0%, 100% {
    filter: drop-shadow(0 0 20rpx rgba(255, 0, 214, 0.6));
  }
  50% {
    filter: drop-shadow(0 0 40rpx rgba(0, 217, 255, 1)) drop-shadow(0 0 60rpx rgba(255, 0, 214, 0.8));
  }
}

@keyframes arrowBounce {
  0%, 100% {
    transform: translateX(0);
  }
  50% {
    transform: translateX(10rpx);
  }
}

@keyframes cardFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-15rpx);
  }
}

@keyframes borderGlow {
  0%, 100% {
    opacity: 0;
  }
  50% {
    opacity: 1;
  }
}

/* Section标题增强 */
.emotion-page .section-header {
  .section-icon {
    font-size: 56rpx !important;
    filter: drop-shadow(0 0 25rpx rgba(255, 0, 214, 1)) drop-shadow(0 0 50rpx rgba(0, 217, 255, 0.8));
    text-shadow: 
      0 0 30rpx rgba(255, 0, 214, 1),
      0 0 60rpx rgba(0, 217, 255, 0.8);
    animation: iconBounce 2s ease-in-out infinite, iconRotateSlow 10s linear infinite;
  }
  
  .section-title {
    font-size: 46rpx !important;
    background: linear-gradient(135deg, #FF00D6, #00D9FF, #FFD600, #FF00D6);
    background-size: 300% 300%;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    animation: gradientFlow 5s ease infinite;
    filter: drop-shadow(0 0 30rpx rgba(255, 0, 214, 0.8));
  }
  
  .section-subtitle {
    font-size: 28rpx !important;
    color: #FFD600 !important;
    text-shadow: 
      0 0 20rpx rgba(255, 214, 0, 1),
      0 0 40rpx rgba(255, 214, 0, 0.6);
  }
}

@keyframes iconRotateSlow {
  from {
    transform: translateY(0) rotate(0deg);
  }
  to {
    transform: translateY(0) rotate(360deg);
  }
}

@keyframes gradientFlow {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}
</style>
