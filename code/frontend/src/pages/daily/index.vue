<template>
  <view class="daily-page">
    <!-- 浮动心形装饰 -->
    <view class="floating-hearts">
      <view v-for="i in 8" :key="i" class="heart" :style="getHeartStyle(i)">⭐</view>
    </view>

    <!-- 顶部横幅 -->
    <view class="hero-section">
      <view class="hero-glow"></view>
      <view class="hero-content">
        <view class="greeting-wrapper">
          <text class="greeting">{{ greeting }}</text>
          <text class="username">{{ userStore.nickname || '朋友' }}</text>
        </view>
        <view class="daily-quote-mini">
          <text class="quote-icon">⭐</text>
          <text class="quote-text">记录美好生活</text>
        </view>
      </view>
      <view class="hero-decoration">
        <view class="decoration-circle circle-1"></view>
        <view class="decoration-circle circle-2"></view>
        <view class="decoration-circle circle-3"></view>
      </view>
    </view>

    <!-- 功能卡片 -->
    <scroll-view class="content-scroll" scroll-y>
      <view class="section">
        <view class="section-header">
          <view class="section-icon">📝</view>
          <text class="section-title">我的日常</text>
          <text class="section-subtitle">记录美好生活</text>
        </view>
        <view class="section-cards">
          <view class="feature-card card-account" @click="handleNavigate('/pages/self/account')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">💎</text>
              <view class="card-text">
                <text class="card-title">财富账本</text>
                <text class="card-desc">收支·规划·理财</text>
              </view>
            </view>
            <view class="card-arrow">⚡</view>
          </view>
          <view class="feature-card card-note" @click="handleNavigate('/pages/self/note')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">📝</text>
              <view class="card-text">
                <text class="card-title">灵感笔记</text>
                <text class="card-desc">记录·思考·沉淀</text>
              </view>
            </view>
            <view class="card-arrow">⚡</view>
          </view>
          <view class="feature-card card-mirror" @click="handleNavigate('/pages/self/mirror')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">🔮</text>
              <view class="card-text">
                <text class="card-title">心灵镜像</text>
                <text class="card-desc">反思·觉察·成长</text>
              </view>
            </view>
            <view class="card-arrow">⚡</view>
          </view>
          <view class="feature-card card-md" @click="handleNavigate('/pages/tools/md-editor')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">⚙️</text>
              <view class="card-text">
                <text class="card-title">创作工坊</text>
                <text class="card-desc">编辑·排版·输出</text>
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

/* ============= 我的日常页面超炫特效 ============= */

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
    filter: drop-shadow(0 0 15rpx rgba(255, 214, 0, 0.8));
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
.daily-page .hero-section {
  overflow: visible !important;
  
  .hero-glow {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 150%;
    height: 150%;
    background: radial-gradient(circle, rgba(255, 214, 0, 0.3), transparent 70%);
    animation: heroGlowPulse 4s ease-in-out infinite;
    pointer-events: none;
  }
  
  .greeting-wrapper {
    .greeting {
      animation: textShine 3s ease-in-out infinite;
      font-size: 32rpx !important;
      text-shadow: 
        0 0 20rpx rgba(255, 214, 0, 1),
        0 0 40rpx rgba(255, 214, 0, 0.6);
    }
    
    .username {
      animation: usernamePulse 2s ease-in-out infinite;
      font-size: 64rpx !important;
    }
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
    filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 0.8));
  }
  50% {
    transform: scale(1.02);
    filter: drop-shadow(0 0 30rpx rgba(255, 214, 0, 1));
  }
}

/* Section标题增强 */
.daily-page .section-header {
  .section-icon {
    font-size: 56rpx !important;
    filter: drop-shadow(0 0 25rpx rgba(255, 214, 0, 1)) drop-shadow(0 0 50rpx rgba(255, 214, 0, 0.8));
    text-shadow: 
      0 0 30rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(255, 214, 0, 0.8);
    animation: iconBounce 2s ease-in-out infinite, iconRotateSlow 10s linear infinite;
  }
  
  .section-title {
    font-size: 46rpx !important;
    background: linear-gradient(135deg, #FFD600, #FF00D6, #00D9FF, #FFD600);
    background-size: 300% 300%;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    animation: gradientFlow 5s ease infinite;
    filter: drop-shadow(0 0 30rpx rgba(255, 214, 0, 0.8));
  }
  
  .section-subtitle {
    font-size: 28rpx !important;
    color: #00D9FF !important;
    text-shadow: 
      0 0 20rpx rgba(0, 217, 255, 1),
      0 0 40rpx rgba(0, 217, 255, 0.6);
  }
}

@keyframes iconBounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10rpx);
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

/* 功能卡片增强 */
.daily-page .section-cards {
  .feature-card {
    position: relative;
    animation: cardSlideIn 0.6s ease-out backwards, cardFloat 4s ease-in-out infinite;
    
    &:nth-child(1) { 
      animation-delay: 0.1s, 0s;
      background: linear-gradient(135deg, rgba(45, 36, 26, 0.9), rgba(35, 26, 20, 0.95));
    }
    &:nth-child(2) { 
      animation-delay: 0.2s, 1s;
      background: linear-gradient(135deg, rgba(40, 40, 30, 0.9), rgba(30, 30, 25, 0.95));
    }
    &:nth-child(3) { 
      animation-delay: 0.3s, 2s;
      background: linear-gradient(135deg, rgba(42, 35, 28, 0.9), rgba(32, 25, 20, 0.95));
    }
    &:nth-child(4) { 
      animation-delay: 0.4s, 3s;
      background: linear-gradient(135deg, rgba(38, 38, 32, 0.9), rgba(28, 28, 25, 0.95));
    }
    
    /* 星光粒子 */
    &::before {
      background: linear-gradient(135deg, rgba(255, 214, 0, 0.6), rgba(255, 0, 214, 0.6), rgba(0, 217, 255, 0.6));
      animation: borderGlow 3s ease-in-out infinite;
    }
    
    .card-icon {
      font-size: 80rpx !important;
      margin-right: 25rpx !important;
      animation: cardIconPulse 2s ease-in-out infinite;
      filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 0.8)) drop-shadow(0 0 40rpx rgba(255, 214, 0, 0.6));
      text-shadow: 
        0 0 30rpx rgba(255, 214, 0, 1),
        0 0 50rpx rgba(255, 214, 0, 0.8);
    }
    
    .card-title {
      font-size: 42rpx !important;
      font-weight: 800 !important;
      background: linear-gradient(135deg, #FFD600, #FF00D6, #00D9FF);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      background-clip: text;
      filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 0.6));
      letter-spacing: 2rpx;
      animation: titleShine 3s ease-in-out infinite;
    }
    
    .card-desc {
      font-size: 28rpx !important;
      color: #FF00D6 !important;
      font-weight: 600;
      text-shadow: 
        0 0 15rpx rgba(255, 0, 214, 1),
        0 0 30rpx rgba(255, 0, 214, 0.6),
        0 2rpx 8rpx rgba(0, 0, 0, 0.5);
      letter-spacing: 1rpx;
    }
    
    .card-arrow {
      font-size: 60rpx !important;
      filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 1));
      text-shadow: 
        0 0 30rpx rgba(255, 214, 0, 1),
        0 0 50rpx rgba(255, 214, 0, 0.8);
      animation: arrowBounce 1.5s ease-in-out infinite;
    }
    
    &:active .card-icon {
      animation: iconExplode 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55);
      filter: 
        drop-shadow(0 0 40rpx rgba(255, 214, 0, 1)) 
        drop-shadow(0 0 60rpx rgba(255, 0, 214, 1));
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
    filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 0.6));
  }
  50% {
    filter: drop-shadow(0 0 40rpx rgba(255, 214, 0, 1)) drop-shadow(0 0 60rpx rgba(255, 0, 214, 0.8));
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
</style>
