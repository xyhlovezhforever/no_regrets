<template>
  <view class="game-page">
    <view class="game-header">
      <view class="score-board">
        <text class="score-label">得分</text>
        <text class="score-value">{{ score }}</text>
      </view>
      <view class="time-board">
        <text class="time-label">时间</text>
        <text class="time-value">{{ timeLeft }}s</text>
      </view>
    </view>

    <view v-if="!gameStarted" class="game-intro">
      <text class="intro-title">🎈 泡泡大作战</text>
      <text class="intro-desc">点击泡泡获得分数</text>
      <text class="intro-desc">越快点击分数越高</text>
      <text class="intro-desc">挑战60秒，看看你能得多少分！</text>
      <button class="start-btn" @click="startGame">开始游戏</button>
    </view>

    <view v-else-if="gameOver" class="game-over">
      <text class="game-over-title">游戏结束</text>
      <text class="game-over-score">最终得分: {{ score }}</text>
      <text class="game-over-rank">{{ getRank() }}</text>
      <button class="restart-btn" @click="startGame">再玩一次</button>
      <button class="back-btn" @click="goBack">返回</button>
    </view>

    <view v-else class="game-canvas">
      <view
        v-for="bubble in bubbles"
        :key="bubble.id"
        class="bubble"
        :class="{ popping: bubble.popping }"
        :style="{
          left: bubble.x + 'px',
          top: bubble.y + 'px',
          width: bubble.size + 'px',
          height: bubble.size + 'px',
          background: bubble.color,
          animationDuration: bubble.duration + 's'
        }"
        @click="popBubble(bubble)"
      >
        <text class="bubble-emoji">{{ bubble.emoji }}</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onUnmounted } from 'vue'
import { getStorage, setStorage } from '@/utils/storage'

interface Bubble {
  id: number
  x: number
  y: number
  size: number
  color: string
  emoji: string
  duration: number
  popping: boolean
}

const gameStarted = ref(false)
const gameOver = ref(false)
const score = ref(0)
const timeLeft = ref(60)
const bubbles = ref<Bubble[]>([])

let bubbleIdCounter = 0
let gameInterval: any = null
let spawnInterval: any = null
let countdownInterval: any = null

const colors = [
  'linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%)',
  'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)',
  'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
  'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)',
  'linear-gradient(135deg, #fa709a 0%, #fee140 100%)',
]

const emojis = ['😊', '🎉', '⭐', '💖', '🌈', '🎈', '✨', '💫', '🌟', '💝']

const startGame = () => {
  gameStarted.value = true
  gameOver.value = false
  score.value = 0
  timeLeft.value = 60
  bubbles.value = []
  bubbleIdCounter = 0

  // 倒计时
  countdownInterval = setInterval(() => {
    timeLeft.value--
    if (timeLeft.value <= 0) {
      endGame()
    }
  }, 1000)

  // 生成泡泡
  spawnInterval = setInterval(() => {
    spawnBubble()
  }, 800)

  // 初始生成几个泡泡
  for (let i = 0; i < 3; i++) {
    setTimeout(() => spawnBubble(), i * 200)
  }
}

const spawnBubble = () => {
  if (!gameStarted.value || gameOver.value) return

  const size = Math.random() * 40 + 60 // 60-100px
  const maxX = uni.getSystemInfoSync().windowWidth - size
  const maxY = uni.getSystemInfoSync().windowHeight - size - 200 // 留出顶部和底部空间

  const bubble: Bubble = {
    id: bubbleIdCounter++,
    x: Math.random() * maxX,
    y: Math.random() * maxY + 150, // 从顶部150px开始
    size,
    color: colors[Math.floor(Math.random() * colors.length)],
    emoji: emojis[Math.floor(Math.random() * emojis.length)],
    duration: Math.random() * 2 + 3, // 3-5秒
    popping: false
  }

  bubbles.value.push(bubble)

  // 泡泡自动消失
  setTimeout(() => {
    removeBubble(bubble.id)
  }, bubble.duration * 1000)
}

const popBubble = (bubble: Bubble) => {
  if (bubble.popping) return

  bubble.popping = true
  score.value += Math.ceil(10 / bubble.size * 100) // 越小的泡泡分数越高

  // 播放戳破动画后移除
  setTimeout(() => {
    removeBubble(bubble.id)
  }, 300)
}

const removeBubble = (id: number) => {
  const index = bubbles.value.findIndex(b => b.id === id)
  if (index !== -1) {
    bubbles.value.splice(index, 1)
  }
}

const endGame = () => {
  gameOver.value = true
  gameStarted.value = false

  clearInterval(countdownInterval)
  clearInterval(spawnInterval)
  bubbles.value = []

  // 保存最高分
  const highScore = getStorage<number>('bubbleGameHighScore', 0)
  if (score.value > highScore) {
    setStorage('bubbleGameHighScore', score.value)
    uni.showToast({ title: '新纪录！', icon: 'success' })
  }
}

const getRank = () => {
  if (score.value >= 1000) return '🏆 泡泡大师'
  if (score.value >= 800) return '⭐ 泡泡高手'
  if (score.value >= 600) return '💫 泡泡达人'
  if (score.value >= 400) return '✨ 泡泡新手'
  return '🎈 继续加油'
}

const goBack = () => {
  uni.navigateBack()
}

onUnmounted(() => {
  clearInterval(countdownInterval)
  clearInterval(spawnInterval)
})
</script>

<style lang="scss" scoped>
.game-page {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  overflow: hidden;
}

.game-header {
  display: flex;
  justify-content: space-between;
  padding: 40rpx 30rpx;
  background: rgba(0, 0, 0, 0.2);

  .score-board,
  .time-board {
    text-align: center;

    .score-label,
    .time-label {
      display: block;
      font-size: 24rpx;
      color: rgba(255, 255, 255, 0.8);
      margin-bottom: 5rpx;
    }

    .score-value,
    .time-value {
      display: block;
      font-size: 48rpx;
      font-weight: bold;
      color: #ffffff;
    }
  }
}

.game-intro,
.game-over {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 200rpx 60rpx;

  .intro-title,
  .game-over-title {
    font-size: 60rpx;
    font-weight: bold;
    color: #ffffff;
    margin-bottom: 40rpx;
  }

  .intro-desc {
    font-size: 28rpx;
    color: rgba(255, 255, 255, 0.9);
    margin-bottom: 15rpx;
  }

  .game-over-score {
    font-size: 48rpx;
    color: #ffffff;
    margin-bottom: 20rpx;
  }

  .game-over-rank {
    font-size: 36rpx;
    color: #ffe066;
    margin-bottom: 60rpx;
  }

  .start-btn,
  .restart-btn,
  .back-btn {
    width: 400rpx;
    height: 90rpx;
    background: var(--theme-surface);
    color: var(--theme-primary);
    border-radius: 50rpx;
    border: none;
    font-size: 32rpx;
    font-weight: bold;
    margin-top: 40rpx;
    line-height: 90rpx;
    
    &::after {
      border: none;
    }
  }

  .back-btn {
    background: rgba(255, 255, 255, 0.3);
    color: #ffffff;
  }
}

.game-canvas {
  position: relative;
  height: calc(100vh - 150rpx);
  overflow: hidden;
}

.bubble {
  position: absolute;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 10rpx 30rpx rgba(0, 0, 0, 0.3);
  animation: float linear infinite;
  transition: transform 0.3s, opacity 0.3s;

  &.popping {
    transform: scale(1.5);
    opacity: 0;
  }

  .bubble-emoji {
    font-size: 40rpx;
  }
}

@keyframes float {
  0% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-20rpx) rotate(180deg);
  }
  100% {
    transform: translateY(0) rotate(360deg);
  }
}
</style>

