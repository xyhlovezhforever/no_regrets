<template>
  <view class="welcome-page">
    <!-- 背景粒子效果 -->
    <view class="particles-bg">
      <view 
        v-for="i in 30" 
        :key="i" 
        class="particle"
        :style="getParticleStyle(i)"
      ></view>
    </view>

    <!-- 主内容区 -->
    <view class="welcome-content">
      <!-- Logo/图标区 -->
      <view class="logo-section">
        <view class="logo-circle">
          <text class="logo-icon">✨</text>
        </view>
        <view class="logo-glow"></view>
      </view>

      <!-- 打字机文字区 -->
      <view class="typing-section">
        <view class="greeting-line">
          <text class="typing-text greeting">{{ displayGreeting }}</text>
          <text v-if="showCursor && currentStep === 0" class="cursor">|</text>
        </view>
        
        <view v-if="currentStep >= 1" class="welcome-line">
          <text class="typing-text welcome">{{ displayWelcome }}</text>
          <text v-if="showCursor && currentStep === 1" class="cursor">|</text>
        </view>
        
        <view v-if="currentStep >= 2" class="subtitle-line">
          <text class="typing-text subtitle">{{ displaySubtitle }}</text>
          <text v-if="showCursor && currentStep === 2" class="cursor">|</text>
        </view>
      </view>

      <!-- 进入按钮 -->
      <view v-if="showEnterButton" class="enter-section">
        <button class="enter-btn" @click="enterApp">
          <text class="btn-text">开启旅程</text>
          <text class="btn-icon">→</text>
        </button>
        <text class="skip-text" @click="enterApp">轻触进入</text>
      </view>

      <!-- 加载进度条 -->
      <view v-if="!showEnterButton" class="progress-section">
        <view class="progress-bar">
          <view class="progress-fill" :style="{ width: progress + '%' }"></view>
        </view>
        <text class="progress-text">{{ Math.floor(progress) }}%</text>
      </view>
    </view>

    <!-- 装饰光效 -->
    <view class="deco-lights">
      <view class="light light-1"></view>
      <view class="light light-2"></view>
      <view class="light light-3"></view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useUserStore } from '@/store'

const userStore = useUserStore()

const currentStep = ref(0)
const displayGreeting = ref('')
const displayWelcome = ref('')
const displaySubtitle = ref('')
const showCursor = ref(true)
const showEnterButton = ref(false)
const progress = ref(0)

const greetingText = computed(() => {
  const hour = new Date().getHours()
  if (hour < 6) return '夜深了'
  if (hour < 9) return '早上好'
  if (hour < 12) return '上午好'
  if (hour < 14) return '中午好'
  if (hour < 18) return '下午好'
  if (hour < 22) return '晚上好'
  return '夜深了'
})

const welcomeText = `欢迎来到「你没有遗憾」`
const subtitleText = '在这里，温暖你的每一天 ✨'

// 打字机效果
const typeWriter = async (text: string, target: any, delay = 80) => {
  for (let i = 0; i <= text.length; i++) {
    target.value = text.slice(0, i)
    await new Promise(resolve => setTimeout(resolve, delay))
  }
}

// 光标闪烁
const startCursorBlink = () => {
  setInterval(() => {
    showCursor.value = !showCursor.value
  }, 530)
}

// 粒子样式
const getParticleStyle = (index: number) => {
  const left = Math.random() * 100
  const top = Math.random() * 100
  const size = Math.random() * 4 + 2
  const duration = Math.random() * 3 + 2
  const delay = Math.random() * 2
  
  return {
    left: `${left}%`,
    top: `${top}%`,
    width: `${size}rpx`,
    height: `${size}rpx`,
    animationDuration: `${duration}s`,
    animationDelay: `${delay}s`
  }
}

// 进度条动画
const animateProgress = () => {
  const interval = setInterval(() => {
    if (progress.value < 100) {
      progress.value += 2
    } else {
      clearInterval(interval)
    }
  }, 50)
}

// 检查是否已经显示过欢迎页
const checkWelcomeShown = () => {
  // 开发模式：注释掉这段代码可以每次都显示欢迎页
  // 正式版本：取消注释下面的代码
  /*
  const hasShown = uni.getStorageSync('welcome_shown')
  if (hasShown) {
    // 已经显示过，直接跳转到TabBar页面
    uni.reLaunch({
      url: '/pages/emotion/index'
    })
    return true
  }
  */
  return false
}

// 开始动画
onMounted(async () => {
  // 检查是否需要显示欢迎页
  if (checkWelcomeShown()) {
    return
  }
  
  startCursorBlink()
  animateProgress()
  
  // 第一行：问候语
  await typeWriter(greetingText.value, displayGreeting, 100)
  await new Promise(resolve => setTimeout(resolve, 300))
  
  currentStep.value = 1
  // 第二行：欢迎文字
  await typeWriter(welcomeText, displayWelcome, 80)
  await new Promise(resolve => setTimeout(resolve, 300))
  
  currentStep.value = 2
  // 第三行：副标题
  await typeWriter(subtitleText, displaySubtitle, 60)
  await new Promise(resolve => setTimeout(resolve, 800))
  
  // 显示进入按钮
  showEnterButton.value = true
})

// 进入应用
const enterApp = () => {
  // 每次登录都显示欢迎页，不保存已显示标记
  // 如果想要只显示一次，取消注释下面这行
  // uni.setStorageSync('welcome_shown', true)
  
  // 跳转到第一个TabBar页面
  uni.reLaunch({
    url: '/pages/emotion/index'
  })
}
</script>

<style lang="scss" scoped>
.welcome-page {
  min-height: 100vh;
  background: #0a0e27;
  background-image: 
    radial-gradient(circle at 20% 30%, rgba(0, 217, 255, 0.15) 0%, transparent 50%),
    radial-gradient(circle at 80% 70%, rgba(255, 0, 214, 0.15) 0%, transparent 50%);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  overflow: hidden;
}

/* 粒子背景 */
.particles-bg {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 0;
  
  .particle {
    position: absolute;
    background: radial-gradient(circle, rgba(0, 217, 255, 1), rgba(255, 0, 214, 0.5));
    border-radius: 50%;
    animation: particleFloat 3s ease-in-out infinite;
    box-shadow: 0 0 10rpx rgba(0, 217, 255, 0.8);
  }
}

@keyframes particleFloat {
  0%, 100% {
    transform: translateY(0) scale(1);
    opacity: 0.3;
  }
  50% {
    transform: translateY(-20rpx) scale(1.2);
    opacity: 0.8;
  }
}

/* 装饰光效 */
.deco-lights {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 1;
  
  .light {
    position: absolute;
    border-radius: 50%;
    filter: blur(80rpx);
    animation: lightPulse 4s ease-in-out infinite;
  }
  
  .light-1 {
    top: 10%;
    left: 10%;
    width: 300rpx;
    height: 300rpx;
    background: radial-gradient(circle, rgba(0, 217, 255, 0.3), transparent);
    animation-delay: 0s;
  }
  
  .light-2 {
    bottom: 20%;
    right: 15%;
    width: 400rpx;
    height: 400rpx;
    background: radial-gradient(circle, rgba(255, 0, 214, 0.3), transparent);
    animation-delay: 1.3s;
  }
  
  .light-3 {
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 500rpx;
    height: 500rpx;
    background: radial-gradient(circle, rgba(255, 214, 0, 0.2), transparent);
    animation-delay: 2.6s;
  }
}

@keyframes lightPulse {
  0%, 100% {
    opacity: 0.3;
    transform: scale(1);
  }
  50% {
    opacity: 0.6;
    transform: scale(1.2);
  }
}

/* 主内容 */
.welcome-content {
  position: relative;
  z-index: 2;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 60rpx;
  animation: contentFadeIn 1s ease-out;
}

@keyframes contentFadeIn {
  from {
    opacity: 0;
    transform: translateY(30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Logo区域 */
.logo-section {
  position: relative;
  margin-bottom: 80rpx;
  
  .logo-circle {
    width: 180rpx;
    height: 180rpx;
    border-radius: 50%;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.9), rgba(15, 20, 45, 1));
    backdrop-filter: blur(20rpx);
    border: 4rpx solid rgba(0, 217, 255, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.6),
      0 0 80rpx rgba(0, 217, 255, 0.6),
      0 0 120rpx rgba(255, 0, 214, 0.4),
      inset 0 0 60rpx rgba(0, 217, 255, 0.2);
    animation: logoFloat 3s ease-in-out infinite;
    
    .logo-icon {
      font-size: 100rpx;
      filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 1));
      animation: iconRotate 4s linear infinite;
    }
  }
  
  .logo-glow {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 250rpx;
    height: 250rpx;
    background: radial-gradient(circle, rgba(0, 217, 255, 0.4), transparent);
    border-radius: 50%;
    filter: blur(40rpx);
    animation: glowPulse 3s ease-in-out infinite;
  }
}

@keyframes logoFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-15rpx);
  }
}

@keyframes iconRotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes glowPulse {
  0%, 100% {
    opacity: 0.6;
    transform: translate(-50%, -50%) scale(1);
  }
  50% {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1.2);
  }
}

/* 打字机区域 */
.typing-section {
  min-height: 300rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 30rpx;
}

.greeting-line,
.welcome-line,
.subtitle-line {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 80rpx;
}

.typing-text {
  font-weight: bold;
  letter-spacing: 2rpx;
  
  &.greeting {
    font-size: 48rpx;
    background: linear-gradient(135deg, #00D9FF, #FF00D6);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-shadow: 0 0 40rpx rgba(0, 217, 255, 0.6);
    filter: drop-shadow(0 0 20rpx rgba(0, 217, 255, 0.8));
    animation: textGlow 2s ease-in-out infinite;
  }
  
  &.welcome {
    font-size: 52rpx;
    color: #ffffff;
    text-shadow: 
      0 0 30rpx rgba(255, 255, 255, 0.8),
      0 0 60rpx rgba(255, 0, 214, 0.8),
      0 0 90rpx rgba(0, 217, 255, 0.6);
    animation: textGlow 2s ease-in-out infinite 0.5s;
  }
  
  &.subtitle {
    font-size: 32rpx;
    color: rgba(255, 214, 0, 1);
    text-shadow: 
      0 0 20rpx rgba(255, 214, 0, 1),
      0 0 40rpx rgba(255, 214, 0, 0.6);
    animation: textGlow 2s ease-in-out infinite 1s;
  }
}

.cursor {
  font-size: 48rpx;
  color: #00D9FF;
  margin-left: 8rpx;
  text-shadow: 0 0 20rpx rgba(0, 217, 255, 1);
  animation: cursorBlink 0.8s step-end infinite;
}

@keyframes cursorBlink {
  0%, 50% {
    opacity: 1;
  }
  51%, 100% {
    opacity: 0;
  }
}

@keyframes textGlow {
  0%, 100% {
    filter: brightness(1);
  }
  50% {
    filter: brightness(1.3);
  }
}

/* 进入按钮 */
.enter-section {
  margin-top: 80rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20rpx;
  animation: btnFadeIn 0.8s ease-out;
}

@keyframes btnFadeIn {
  from {
    opacity: 0;
    transform: translateY(20rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.enter-btn {
  position: relative;
  padding: 28rpx 80rpx;
  background: linear-gradient(135deg, rgba(0, 217, 255, 0.9), rgba(255, 0, 214, 0.9));
  border: 3rpx solid rgba(0, 217, 255, 0.8);
  border-radius: 50rpx;
  display: flex;
  align-items: center;
  gap: 15rpx;
  box-shadow: 
    0 15rpx 50rpx rgba(0, 217, 255, 0.6),
    0 0 80rpx rgba(255, 0, 214, 0.5);
  animation: btnPulse 2s ease-in-out infinite;
  overflow: visible;
  
  &::before {
    content: '';
    position: absolute;
    top: -3rpx;
    left: -3rpx;
    right: -3rpx;
    bottom: -3rpx;
    background: linear-gradient(135deg, #00D9FF, #FF00D6, #FFD600, #00D9FF);
    background-size: 400% 400%;
    border-radius: 50rpx;
    z-index: -1;
    animation: gradientFlow 3s linear infinite;
  }
  
  &::after {
    border: none;
  }
  
  .btn-text {
    font-size: 36rpx;
    font-weight: bold;
    color: #ffffff;
    text-shadow: 
      0 0 15rpx rgba(255, 255, 255, 0.8),
      0 2rpx 5rpx rgba(0, 0, 0, 0.3);
  }
  
  .btn-icon {
    font-size: 40rpx;
    color: #ffffff;
    animation: arrowMove 1.5s ease-in-out infinite;
  }
  
  &:active {
    transform: scale(0.95);
    box-shadow: 
      0 20rpx 60rpx rgba(0, 217, 255, 0.8),
      0 0 100rpx rgba(255, 0, 214, 0.7);
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 15rpx 50rpx rgba(0, 217, 255, 0.6),
      0 0 80rpx rgba(255, 0, 214, 0.5);
  }
  50% {
    box-shadow: 
      0 20rpx 60rpx rgba(0, 217, 255, 0.8),
      0 0 100rpx rgba(255, 0, 214, 0.7),
      0 0 120rpx rgba(255, 214, 0, 0.5);
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

@keyframes arrowMove {
  0%, 100% {
    transform: translateX(0);
  }
  50% {
    transform: translateX(10rpx);
  }
}

.skip-text {
  font-size: 26rpx;
  color: rgba(255, 255, 255, 0.6);
  text-shadow: 0 0 10rpx rgba(0, 217, 255, 0.5);
}

/* 进度条 */
.progress-section {
  margin-top: 60rpx;
  width: 400rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 15rpx;
}

.progress-bar {
  width: 100%;
  height: 8rpx;
  background: rgba(30, 36, 66, 0.6);
  border-radius: 4rpx;
  overflow: hidden;
  box-shadow: inset 0 0 10rpx rgba(0, 0, 0, 0.5);
  
  .progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #00D9FF, #FF00D6, #FFD600);
    border-radius: 4rpx;
    box-shadow: 
      0 0 15rpx rgba(0, 217, 255, 1),
      0 0 30rpx rgba(255, 0, 214, 0.8);
    transition: width 0.3s ease;
    animation: progressGlow 1.5s ease-in-out infinite;
  }
}

.progress-text {
  font-size: 24rpx;
  color: #00D9FF;
  text-shadow: 0 0 10rpx rgba(0, 217, 255, 0.8);
  font-weight: 600;
}

@keyframes progressGlow {
  0%, 100% {
    box-shadow: 
      0 0 15rpx rgba(0, 217, 255, 1),
      0 0 30rpx rgba(255, 0, 214, 0.8);
  }
  50% {
    box-shadow: 
      0 0 25rpx rgba(0, 217, 255, 1),
      0 0 50rpx rgba(255, 0, 214, 1),
      0 0 70rpx rgba(255, 214, 0, 0.8);
  }
}
</style>
