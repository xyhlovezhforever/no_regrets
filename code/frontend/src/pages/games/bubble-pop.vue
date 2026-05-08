<template>
  <view class="bubble-pop-game">
    <!-- 游戏信息栏 -->
    <view class="game-header">
      <view class="info-item">
        <text class="label">关卡</text>
        <text class="value">{{ level }}</text>
      </view>
      <view class="info-item">
        <text class="label">分数</text>
        <text class="value">{{ score }}</text>
      </view>
      <view class="info-item">
        <text class="label">时间</text>
        <text class="value">{{ formatTime(timeLeft) }}</text>
      </view>
      <view class="info-item">
        <text class="label">目标</text>
        <text class="value">{{ targetScore }}</text>
      </view>
    </view>

    <!-- 游戏区域 -->
    <view v-if="gameStatus === 'playing'" class="game-area">
      <view 
        v-for="bubble in bubbles" 
        :key="bubble.id"
        class="bubble"
        :class="'bubble-' + bubble.type"
        :style="{
          left: bubble.x + 'px',
          bottom: '0px',
          width: bubble.size + 'px',
          height: bubble.size + 'px',
          animation: `bubble-rise ${bubble.duration}s linear forwards`
        }"
        @click="popBubble(bubble.id)"
      >
        {{ bubble.emoji }}
      </view>
    </view>

    <!-- 开始界面 -->
    <view v-if="gameStatus === 'ready'" class="game-overlay">
      <view class="overlay-content">
        <text class="overlay-title">🎈 扎气球</text>
        <text class="overlay-desc">关卡 {{ level }}</text>
        <text class="overlay-desc">目标分数: {{ targetScore }}</text>
        <text class="overlay-desc">时间: {{ gameTime }}秒</text>
        <button class="start-btn" @click="startGame">开始游戏</button>
      </view>
    </view>

    <!-- 游戏结束 -->
    <view v-if="gameStatus === 'finished'" class="game-overlay">
      <view class="overlay-content">
        <text class="overlay-title">{{ isWin ? '🎉 挑战成功!' : '😢 挑战失败' }}</text>
        <text class="overlay-score">得分: {{ score }}</text>
        <text class="overlay-desc">目标: {{ targetScore }}</text>
        <view class="overlay-actions">
          <button v-if="isWin && level < maxLevel" class="action-btn next-btn" @click="nextLevel">下一关</button>
          <button v-if="isWin && level >= maxLevel" class="action-btn complete-btn" @click="goBack">已通关!</button>
          <button class="action-btn retry-btn" @click="restartGame">重新挑战</button>
          <button class="action-btn back-btn" @click="goBack">返回</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { navigateBack } from '@/utils'
import { getStorage, setStorage } from '@/utils/storage'
import { submitScoreApi } from '@/api/game'

interface Bubble {
  id: number
  x: number
  size: number
  type: number
  emoji: string
  duration: number
  points: number
  timer?: any
}

const level = ref(1)
const gameType = ref('bubble_pop')
const score = ref(0)
const timeLeft = ref(60)
const targetScore = ref(100)
const gameTime = ref(60)
const gameStatus = ref<'ready' | 'playing' | 'finished'>('ready')
const isWin = ref(false)
const maxLevel = 10

const bubbles = ref<Bubble[]>([])
let nextBubbleId = 0
let gameTimer: any = null
let spawnTimer: any = null

const bubbleTypes = [
  { emoji: '🎈', points: 10, speed: 2, size: 60 },
  { emoji: '🎁', points: 20, speed: 3, size: 50 },
  { emoji: '⭐', points: 30, speed: 4, size: 40 },
  { emoji: '💎', points: 50, speed: 5, size: 35 },
  { emoji: '💣', points: -30, speed: 2, size: 55 }
]

onLoad((options: any) => {
  level.value = parseInt(options.level) || 1
  gameType.value = options.gameType || 'bubble_pop'
  initLevel()
})

const initLevel = () => {
  // 根据关卡调整难度
  targetScore.value = 100 + (level.value - 1) * 50
  gameTime.value = Math.max(40, 60 - (level.value - 1) * 2)
  timeLeft.value = gameTime.value
}

const startGame = () => {
  gameStatus.value = 'playing'
  score.value = 0
  bubbles.value = []
  timeLeft.value = gameTime.value
  
  // 开始计时
  gameTimer = setInterval(() => {
    timeLeft.value--
    if (timeLeft.value <= 0) {
      endGame()
    }
  }, 1000)
  
  // 开始生成气球
  spawnBubbles()
}

const spawnBubbles = () => {
  const spawnInterval = Math.max(500, 1000 - level.value * 50)
  
  spawnTimer = setInterval(() => {
    if (bubbles.value.length < 15) {
      createBubble()
    }
  }, spawnInterval)
}

const createBubble = () => {
  const type = Math.floor(Math.random() * bubbleTypes.length)
  const config = bubbleTypes[type]
  
  // 获取游戏区域尺寸
  const gameWidth = uni.getSystemInfoSync().windowWidth
  
  // 计算上升时间（根据速度和关卡）
  const baseDuration = 8 - level.value * 0.3
  const duration = Math.max(3, baseDuration - config.speed * 0.5)
  
  const bubble: Bubble = {
    id: nextBubbleId++,
    x: Math.random() * (gameWidth - config.size),
    size: config.size,
    type,
    emoji: config.emoji,
    duration,
    points: config.points
  }
  
  bubbles.value.push(bubble)
  
  // 设置定时器，在动画结束后移除气球
  bubble.timer = setTimeout(() => {
    const index = bubbles.value.findIndex(b => b.id === bubble.id)
    if (index !== -1) {
      bubbles.value.splice(index, 1)
    }
  }, duration * 1000)
}

const popBubble = (id: number) => {
  const index = bubbles.value.findIndex(b => b.id === id)
  if (index === -1) return
  
  const bubble = bubbles.value[index]
  score.value += bubble.points
  
  // 清除定时器
  if (bubble.timer) {
    clearTimeout(bubble.timer)
  }
  
  // 震动反馈
  uni.vibrateShort({})
  
  // 移除气球
  bubbles.value.splice(index, 1)
  
  // 检查是否达到目标
  if (score.value >= targetScore.value) {
    endGame(true)
  }
}

const endGame = (win = false) => {
  gameStatus.value = 'finished'
  isWin.value = win || score.value >= targetScore.value
  
  clearInterval(gameTimer)
  clearInterval(spawnTimer)
  
  // 清除所有气球的定时器
  bubbles.value.forEach(bubble => {
    if (bubble.timer) {
      clearTimeout(bubble.timer)
    }
  })
  
  // 提交成绩
  if (isWin.value) {
    submitScore()
  }
}

const submitScore = async () => {
  try {
    await submitScoreApi({
      game_type: 'bubble_pop',
      level: level.value,
      score: score.value,
      time_spent: gameTime.value - timeLeft.value
    })
    
    // 如果通关，保存进度
    if (isWin.value) {
      saveProgress()
    }
  } catch (error) {
    console.error('提交成绩失败:', error)
  }
}

const saveProgress = () => {
  const gameLevels = getStorage('game_levels', {})
  const currentLevel = gameLevels[gameType.value] || 1
  
  // 只有当前关卡等于或大于已保存的关卡时才更新
  if (level.value >= currentLevel) {
    gameLevels[gameType.value] = Math.min(level.value + 1, maxLevel)
    setStorage('game_levels', gameLevels)
  }
}

const nextLevel = () => {
  if (level.value < maxLevel) {
    level.value++
    bubbles.value = []
    gameStatus.value = 'ready'
    initLevel()
  }
}

const restartGame = () => {
  bubbles.value = []
  gameStatus.value = 'ready'
  initLevel()
}

const goBack = () => {
  navigateBack()
}

const formatTime = (seconds: number) => {
  return seconds + 's'
}

onUnmounted(() => {
  clearInterval(gameTimer)
  clearInterval(spawnTimer)
  // 清除所有气球的定时器
  bubbles.value.forEach(bubble => {
    if (bubble.timer) {
      clearTimeout(bubble.timer)
    }
  })
})
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.bubble-pop-game {
  @include cyber-page-bg;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  overflow: hidden;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 25% 35%, rgba(255, 0, 214, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 75% 65%, rgba(0, 217, 255, 0.15) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.game-header {
  display: flex;
  padding: 25rpx 30rpx;
  @include neon-card;
  background: linear-gradient(180deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 0.95) 100%);
  backdrop-filter: blur(30rpx);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(138, 92, 246, 0.4),
    inset 0 2rpx 0 rgba(138, 92, 246, 0.3);
  border-bottom: 3rpx solid rgba(138, 92, 246, 0.5);
  position: relative;
  z-index: 10;

  .info-item {
    flex: 1;
    text-align: center;

    .label {
      display: block;
      font-size: 24rpx;
      @include neon-text(#00D9FF);
      margin-bottom: 8rpx;
      font-weight: 600;
    }

    .value {
      display: block;
      font-size: 36rpx;
      font-weight: bold;
      @include neon-title(#FFD600);
      filter: brightness(1.2);
      text-shadow: 
        0 0 20rpx rgba(255, 214, 0, 1),
        0 0 40rpx rgba(255, 214, 0, 0.6),
        0 2rpx 5rpx rgba(0, 0, 0, 0.3);
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

.game-area {
  position: relative;
  flex: 1;
  height: calc(100vh - 200rpx);
  z-index: 1;
}

.bubble {
  position: absolute;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  font-size: 34rpx;
  transition: transform 0.15s;
  cursor: pointer;
  animation: bubbleFloat 2s ease-in-out infinite;

  &:active {
    transform: scale(0.7);
    animation: bubblePop 0.3s ease-out;
  }
}

.bubble-0 {
  background: radial-gradient(circle at 30% 30%, rgba(255, 0, 214, 0.9), rgba(255, 0, 214, 0.6));
  border: 3rpx solid rgba(255, 0, 214, 0.8);
  box-shadow: 
    0 0 30rpx rgba(255, 0, 214, 0.8),
    0 0 60rpx rgba(255, 0, 214, 0.5),
    inset 0 0 20rpx rgba(255, 255, 255, 0.3);
}

.bubble-1 {
  background: radial-gradient(circle at 30% 30%, rgba(0, 217, 255, 0.9), rgba(0, 217, 255, 0.6));
  border: 3rpx solid rgba(0, 217, 255, 0.8);
  box-shadow: 
    0 0 30rpx rgba(0, 217, 255, 0.8),
    0 0 60rpx rgba(0, 217, 255, 0.5),
    inset 0 0 20rpx rgba(255, 255, 255, 0.3);
}

.bubble-2 {
  background: radial-gradient(circle at 30% 30%, rgba(255, 214, 0, 0.9), rgba(255, 214, 0, 0.6));
  border: 3rpx solid rgba(255, 214, 0, 0.8);
  box-shadow: 
    0 0 30rpx rgba(255, 214, 0, 0.8),
    0 0 60rpx rgba(255, 214, 0, 0.5),
    inset 0 0 20rpx rgba(255, 255, 255, 0.3);
}

.bubble-3 {
  background: radial-gradient(circle at 30% 30%, rgba(138, 92, 246, 0.9), rgba(138, 92, 246, 0.6));
  border: 3rpx solid rgba(138, 92, 246, 0.8);
  box-shadow: 
    0 0 30rpx rgba(138, 92, 246, 0.8),
    0 0 60rpx rgba(138, 92, 246, 0.5),
    inset 0 0 20rpx rgba(255, 255, 255, 0.3);
}

.bubble-4 {
  background: radial-gradient(circle at 30% 30%, rgba(255, 0, 79, 0.9), rgba(0, 0, 0, 0.8));
  border: 3rpx solid rgba(255, 0, 79, 0.8);
  box-shadow: 
    0 0 40rpx rgba(255, 0, 79, 1),
    0 0 80rpx rgba(255, 0, 79, 0.7),
    inset 0 0 20rpx rgba(255, 0, 0, 0.5);
  animation: bombPulse 1s ease-in-out infinite;
}

@keyframes bubble-rise {
  from {
    bottom: 0;
    opacity: 1;
  }
  to {
    bottom: 100vh;
    opacity: 0.7;
  }
}

@keyframes bubbleFloat {
  0%, 100% {
    transform: scale(1) rotate(0deg);
  }
  50% {
    transform: scale(1.05) rotate(5deg);
  }
}

@keyframes bubblePop {
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.3);
    opacity: 0.5;
  }
  100% {
    transform: scale(0);
    opacity: 0;
  }
}

@keyframes bombPulse {
  0%, 100% {
    box-shadow: 
      0 0 40rpx rgba(255, 0, 79, 1),
      0 0 80rpx rgba(255, 0, 79, 0.7),
      inset 0 0 20rpx rgba(255, 0, 0, 0.5);
  }
  50% {
    box-shadow: 
      0 0 60rpx rgba(255, 0, 79, 1),
      0 0 120rpx rgba(255, 0, 79, 1),
      inset 0 0 30rpx rgba(255, 0, 0, 0.8);
  }
}

.game-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(15rpx);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
  animation: fadeIn 0.4s ease-out;
}

.overlay-content {
  @include neon-card;
  background: linear-gradient(135deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 1) 100%);
  backdrop-filter: blur(30rpx);
  border-radius: 35rpx;
  border: 4rpx solid rgba(138, 92, 246, 0.6);
  padding: 65rpx 55rpx;
  text-align: center;
  min-width: 540rpx;
  box-shadow: 
    0 25rpx 80rpx rgba(0, 0, 0, 0.7),
    0 0 100rpx rgba(138, 92, 246, 0.5),
    inset 0 0 80rpx rgba(138, 92, 246, 0.12);
  animation: scaleIn 0.5s cubic-bezier(0.4, 0, 0.2, 1);

  .overlay-title {
    display: block;
    font-size: 52rpx;
    font-weight: bold;
    @include neon-title(#00D9FF);
    margin-bottom: 35rpx;
    filter: brightness(1.2);
    animation: titlePulse 2s ease-in-out infinite;
  }

  .overlay-score {
    display: block;
    font-size: 64rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
    margin-bottom: 25rpx;
    filter: brightness(1.3);
    text-shadow: 
      0 0 30rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(255, 214, 0, 0.8),
      0 0 90rpx rgba(255, 214, 0, 0.5),
      0 4rpx 8rpx rgba(0, 0, 0, 0.3);
    animation: scorePulse 1.5s ease-in-out infinite;
  }

  .overlay-desc {
    display: block;
    font-size: 30rpx;
    @include neon-text(#ffffff);
    margin-bottom: 18rpx;
  }

  .start-btn {
    margin-top: 45rpx;
    padding: 28rpx 90rpx;
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(0, 217, 255, 0.7);
    font-size: 36rpx;
    font-weight: bold;
    border-radius: 50rpx;
    box-shadow: 
      0 10rpx 40rpx rgba(0, 217, 255, 0.6),
      0 0 60rpx rgba(0, 217, 255, 0.5);
    text-shadow: 
      0 0 15rpx rgba(255, 255, 255, 0.8),
      0 2rpx 5rpx rgba(0, 0, 0, 0.3);
    transition: all 0.3s;
    animation: btnPulse 3s ease-in-out infinite;

    &::after {
      border: none;
    }

    &:active {
      transform: scale(0.95);
      box-shadow: 
        0 12rpx 50rpx rgba(0, 217, 255, 0.8),
        0 0 80rpx rgba(0, 217, 255, 0.7);
    }
  }

  .overlay-actions {
    display: flex;
    gap: 25rpx;
    margin-top: 45rpx;

    .action-btn {
      flex: 1;
      padding: 28rpx;
      border-radius: 50rpx;
      font-size: 30rpx;
      font-weight: bold;
      border: none;
      transition: all 0.3s;

      &::after {
        border: none;
      }

      &:active {
        transform: scale(0.95);
      }
    }

    .next-btn,
    .complete-btn {
      background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border: 2rpx solid rgba(0, 217, 255, 0.7);
      box-shadow: 
        0 8rpx 24rpx rgba(0, 217, 255, 0.5),
        0 0 40rpx rgba(0, 217, 255, 0.4);
      text-shadow: 
        0 0 12rpx rgba(255, 255, 255, 0.8),
        0 2rpx 5rpx rgba(0, 0, 0, 0.3);

      &:active {
        box-shadow: 
          0 10rpx 30rpx rgba(0, 217, 255, 0.7),
          0 0 60rpx rgba(0, 217, 255, 0.6);
      }
    }

    .retry-btn {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border: 2rpx solid rgba(255, 0, 214, 0.7);
      box-shadow: 
        0 8rpx 24rpx rgba(255, 0, 214, 0.5),
        0 0 40rpx rgba(255, 0, 214, 0.4);
      text-shadow: 
        0 0 12rpx rgba(255, 255, 255, 0.8),
        0 2rpx 5rpx rgba(0, 0, 0, 0.3);

      &:active {
        box-shadow: 
          0 10rpx 30rpx rgba(255, 0, 214, 0.7),
          0 0 60rpx rgba(255, 0, 214, 0.6);
      }
    }

    .back-btn {
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
      backdrop-filter: blur(10rpx);
      @include neon-text(#b8c5d6);
      border: 2rpx solid rgba(138, 92, 246, 0.5);
      box-shadow: 0 6rpx 18rpx rgba(0, 0, 0, 0.4);
    }
  }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes scaleIn {
  from {
    transform: scale(0.85);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 20rpx rgba(0, 217, 255, 1),
      0 0 40rpx rgba(0, 217, 255, 0.6),
      0 2rpx 5rpx rgba(0, 0, 0, 0.3);
  }
  50% {
    text-shadow: 
      0 0 30rpx rgba(0, 217, 255, 1),
      0 0 60rpx rgba(0, 217, 255, 0.8),
      0 2rpx 5rpx rgba(0, 0, 0, 0.3);
  }
}

@keyframes scorePulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(0, 217, 255, 0.6),
      0 0 60rpx rgba(0, 217, 255, 0.5);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(0, 217, 255, 0.8),
      0 0 80rpx rgba(0, 217, 255, 0.7);
  }
}
</style>
