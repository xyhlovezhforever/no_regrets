<template>
  <view class="hug-page">
    <!-- 拥抱动画阶段 -->
    <view v-if="showHugAnimation" class="hug-animation-container">
      <!-- 背景光芒 -->
      <view class="light-rays">
        <view v-for="i in 12" :key="i" class="ray" :style="{ transform: `rotate(${i * 30}deg)` }"></view>
      </view>
      
      <!-- 粒子特效 -->
      <view class="particles">
        <view
          v-for="(particle, index) in particles"
          :key="index"
          class="particle"
          :style="{
            left: particle.x + '%',
            top: particle.y + '%',
            animationDelay: particle.delay + 's',
            animationDuration: particle.duration + 's'
          }"
        >
          {{ particle.emoji }}
        </view>
      </view>
      
      <view class="hug-animation">
        <view class="person left-person" :class="{ hugging: isHugging }">
          <text class="person-emoji">🙆</text>
        </view>
        <view class="person right-person" :class="{ hugging: isHugging }">
          <text class="person-emoji">🙆</text>
        </view>
        <view class="hug-effect" :class="{ show: isHugging }">
          <text class="effect-emoji">💝</text>
        </view>
      </view>
      <text class="hug-text animate-text">给自己一个温暖的拥抱</text>
    </view>

    <!-- 文字雨阶段 -->
    <view v-if="showTextRain" class="text-rain-container">
      <!-- 烟花特效 -->
      <view
        v-for="(firework, index) in fireworks"
        :key="'firework-' + index"
        class="firework"
        :style="{
          left: firework.x + '%',
          top: firework.y + '%',
          animationDelay: firework.delay + 's'
        }"
      >
        <view v-for="i in 8" :key="i" class="spark" :style="{ transform: `rotate(${i * 45}deg)` }"></view>
      </view>
      
      <!-- 彩色文字雨 -->
      <view
        v-for="(text, index) in rainTexts"
        :key="index"
        class="rain-text"
        :class="'color-' + text.color"
        :style="{
          left: text.left + '%',
          animationDelay: text.delay + 's',
          animationDuration: text.duration + 's',
          fontSize: text.size + 'rpx'
        }"
      >
        你是最棒的!
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const showHugAnimation = ref(true)
const showTextRain = ref(false)
const isHugging = ref(false)

interface RainText {
  left: number
  delay: number
  duration: number
  color: number
  size: number
}

interface Particle {
  x: number
  y: number
  delay: number
  duration: number
  emoji: string
}

interface Firework {
  x: number
  y: number
  delay: number
}

const rainTexts = ref<RainText[]>([])
const particles = ref<Particle[]>([])
const fireworks = ref<Firework[]>([])

// 生成粒子数据
const generateParticles = () => {
  const emojis = ['✨', '⭐', '💫', '🌟', '💖', '💕', '💗', '🎈', '🎉', '🎊']
  const particleList: Particle[] = []
  for (let i = 0; i < 20; i++) {
    particleList.push({
      x: Math.random() * 100,
      y: Math.random() * 100,
      delay: Math.random() * 2,
      duration: 2 + Math.random() * 2,
      emoji: emojis[Math.floor(Math.random() * emojis.length)]
    })
  }
  particles.value = particleList
}

// 生成烟花数据
const generateFireworks = () => {
  const fireworkList: Firework[] = []
  for (let i = 0; i < 15; i++) {
    fireworkList.push({
      x: 20 + Math.random() * 60,
      y: 20 + Math.random() * 60,
      delay: Math.random() * 5
    })
  }
  fireworks.value = fireworkList
}

// 生成彩色文字雨数据
const generateRainTexts = () => {
  const texts: RainText[] = []
  for (let i = 0; i < 40; i++) {
    texts.push({
      left: Math.random() * 100,
      delay: Math.random() * 5,
      duration: 3 + Math.random() * 3,
      color: Math.floor(Math.random() * 5),
      size: 40 + Math.random() * 30
    })
  }
  rainTexts.value = texts
}

onMounted(() => {
  // 生成粒子
  generateParticles()
  
  // 开始拥抱动画
  setTimeout(() => {
    isHugging.value = true
    uni.vibrateShort({})
  }, 500)

  // 3秒后切换到文字雨
  setTimeout(() => {
    showHugAnimation.value = false
    showTextRain.value = true
    generateRainTexts()
    generateFireworks()
  }, 3000)
})
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.hug-page {
  @include cyber-page-bg;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 30% 30%, rgba(255, 0, 214, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 70% 70%, rgba(250, 112, 154, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 50% 50%, rgba(0, 217, 255, 0.1) 0%, transparent 60%);
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
      rgba(255, 0, 214, 0.08) 90deg,
      transparent 180deg,
      rgba(250, 112, 154, 0.08) 270deg,
      transparent 360deg
    );
    pointer-events: none;
    animation: bgRotate 20s linear infinite;
    z-index: 0;
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes bgRotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 拥抱动画容器 */
.hug-animation-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 60rpx;
  position: relative;
  z-index: 10;
}

.hug-animation {
  position: relative;
  width: 600rpx;
  height: 400rpx;
  display: flex;
  align-items: center;
  justify-content: center;
}

.person {
  position: absolute;
  font-size: 160rpx;
  transition: all 1s cubic-bezier(0.4, 0, 0.2, 1);
  filter: drop-shadow(0 0 40rpx rgba(255, 0, 214, 0.8));
  
  .person-emoji {
    display: block;
    transform-origin: center;
    animation: personGlow 2s ease-in-out infinite;
  }
}

@keyframes personGlow {
  0%, 100% {
    filter: drop-shadow(0 0 20rpx rgba(255, 0, 214, 0.6));
  }
  50% {
    filter: drop-shadow(0 0 40rpx rgba(255, 0, 214, 1));
  }
}

.left-person {
  left: 0;
  
  &.hugging {
    left: 150rpx;
    transform: scaleX(-1);
  }
}

.right-person {
  right: 0;
  
  &.hugging {
    right: 150rpx;
  }
}

.hug-effect {
  position: absolute;
  opacity: 0;
  transform: scale(0);
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  filter: drop-shadow(0 0 60rpx rgba(250, 112, 154, 1));
  
  &.show {
    opacity: 1;
    transform: scale(1);
    animation: heartbeat 1s ease-in-out infinite, pulse 0.5s ease-out, effectGlow 2s ease-in-out infinite;
  }
  
  .effect-emoji {
    font-size: 120rpx;
    animation: rainbow 2s linear infinite;
  }
}

@keyframes effectGlow {
  0%, 100% {
    filter: drop-shadow(0 0 40rpx rgba(250, 112, 154, 0.8));
  }
  50% {
    filter: drop-shadow(0 0 80rpx rgba(250, 112, 154, 1)) drop-shadow(0 0 100rpx rgba(255, 0, 214, 0.8));
  }
}

@keyframes heartbeat {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.3);
  }
}

@keyframes pulse {
  0% {
    transform: scale(0);
    filter: brightness(2);
  }
  50% {
    transform: scale(1.5);
    filter: brightness(3);
  }
  100% {
    transform: scale(1);
    filter: brightness(1);
  }
}

@keyframes rainbow {
  0% {
    filter: hue-rotate(0deg);
  }
  100% {
    filter: hue-rotate(360deg);
  }
}

.hug-text {
  font-size: 42rpx;
  @include neon-title(#FF00D6);
  font-weight: bold;
  position: relative;
  z-index: 10;
}

.animate-text {
  animation: textGlow 2s ease-in-out infinite;
}

@keyframes textGlow {
  0%, 100% {
    text-shadow: 
      0 0 20rpx rgba(255, 0, 214, 1),
      0 0 40rpx rgba(255, 0, 214, 0.8),
      0 0 60rpx rgba(255, 0, 214, 0.6);
    transform: scale(1);
  }
  50% {
    text-shadow: 
      0 0 30rpx rgba(255, 0, 214, 1),
      0 0 60rpx rgba(255, 0, 214, 1),
      0 0 90rpx rgba(255, 0, 214, 0.8),
      0 0 120rpx rgba(250, 112, 154, 0.6);
    transform: scale(1.05);
  }
}

/* 光芒特效 */
.light-rays {
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  overflow: hidden;
}

.ray {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 6rpx;
  height: 50%;
  background: linear-gradient(to bottom, 
    rgba(255, 0, 214, 0.8), 
    rgba(250, 112, 154, 0.5),
    transparent);
  transform-origin: 0 0;
  animation: rotateRay 10s linear infinite;
  filter: blur(2rpx);
}

@keyframes rotateRay {
  from {
    opacity: 0.4;
  }
  to {
    opacity: 1;
    transform: rotate(360deg);
  }
}

/* 粒子特效 */
.particles {
  position: absolute;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
  pointer-events: none;
}

.particle {
  position: absolute;
  font-size: 50rpx;
  animation: float-particle linear infinite;
  filter: drop-shadow(0 0 20rpx rgba(255, 0, 214, 0.8));
}

@keyframes float-particle {
  0% {
    transform: translate(0, 0) rotate(0deg) scale(0);
    opacity: 0;
    filter: drop-shadow(0 0 10rpx rgba(255, 0, 214, 0.5));
  }
  10% {
    opacity: 1;
  }
  50% {
    filter: drop-shadow(0 0 30rpx rgba(255, 0, 214, 1));
  }
  90% {
    opacity: 1;
  }
  100% {
    transform: translate(
      calc((var(--random-x, 50) - 50) * 2rpx),
      -100vh
    ) rotate(360deg) scale(1.2);
    opacity: 0;
    filter: drop-shadow(0 0 10rpx rgba(255, 0, 214, 0.3));
  }
}

/* 文字雨容器 */
.text-rain-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.rain-text {
  position: absolute;
  top: -100rpx;
  font-weight: bold;
  white-space: nowrap;
  animation: fall linear infinite;
}

/* 霓虹彩色文字 */
.rain-text.color-0 {
  color: #ff6b9d;
  text-shadow: 
    0 0 15rpx #ff6b9d,
    0 0 30rpx #ff6b9d,
    0 0 45rpx #ff6b9d;
}

.rain-text.color-1 {
  color: #feca57;
  text-shadow: 
    0 0 15rpx #feca57,
    0 0 30rpx #feca57,
    0 0 45rpx #feca57;
}

.rain-text.color-2 {
  color: #48dbfb;
  text-shadow: 
    0 0 15rpx #48dbfb,
    0 0 30rpx #48dbfb,
    0 0 45rpx #48dbfb;
}

.rain-text.color-3 {
  color: #1dd1a1;
  text-shadow: 
    0 0 15rpx #1dd1a1,
    0 0 30rpx #1dd1a1,
    0 0 45rpx #1dd1a1;
}

.rain-text.color-4 {
  color: #ee5a6f;
  text-shadow: 
    0 0 15rpx #ee5a6f,
    0 0 30rpx #ee5a6f,
    0 0 45rpx #ee5a6f;
}

@keyframes fall {
  0% {
    top: -100rpx;
    opacity: 0;
    transform: rotate(0deg) scale(0.8);
  }
  10% {
    opacity: 1;
    transform: rotate(36deg) scale(1);
  }
  50% {
    text-shadow: 
      0 0 20rpx currentColor,
      0 0 40rpx currentColor,
      0 0 60rpx currentColor;
  }
  90% {
    opacity: 1;
  }
  100% {
    top: 100vh;
    opacity: 0;
    transform: rotate(360deg) scale(0.8);
  }
}

/* 烟花特效 */
.firework {
  position: absolute;
  width: 10rpx;
  height: 10rpx;
  animation: explode 2s ease-out infinite;
}

.spark {
  position: absolute;
  width: 120rpx;
  height: 6rpx;
  background: linear-gradient(to right, 
    rgba(255, 0, 214, 1),
    rgba(250, 112, 154, 0.8),
    transparent);
  transform-origin: 0 50%;
  animation: sparkle 1s ease-out infinite;
  box-shadow: 
    0 0 10rpx rgba(255, 0, 214, 0.8),
    0 0 20rpx rgba(255, 0, 214, 0.6);
}

@keyframes explode {
  0% {
    transform: scale(0);
    opacity: 1;
  }
  50% {
    transform: scale(1);
    opacity: 1;
  }
  100% {
    transform: scale(1.5);
    opacity: 0;
  }
}

@keyframes sparkle {
  0% {
    transform: rotate(var(--rotation, 0deg)) translateX(0);
    opacity: 1;
  }
  100% {
    transform: rotate(var(--rotation, 0deg)) translateX(150rpx);
    opacity: 0;
  }
}
</style>

