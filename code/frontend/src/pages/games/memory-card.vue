<template>
  <view class="memory-card-game">
    <!-- 游戏信息栏 -->
    <view class="game-header">
      <view class="info-item">
        <text class="label">关卡</text>
        <text class="value">{{ level }}</text>
      </view>
      <view class="info-item">
        <text class="label">翻牌</text>
        <text class="value">{{ moves }}</text>
      </view>
      <view class="info-item">
        <text class="label">时间</text>
        <text class="value">{{ formatTime(timeElapsed) }}</text>
      </view>
      <view class="info-item">
        <text class="label">进度</text>
        <text class="value">{{ matchedPairs }}/{{ totalPairs }}</text>
      </view>
    </view>

    <!-- 游戏区域 -->
    <view v-if="gameStatus === 'playing'" class="game-area">
      <view class="cards-grid" :style="{ gridTemplateColumns: `repeat(${gridSize}, 1fr)` }">
        <view 
          v-for="card in cards" 
          :key="card.id"
          class="card"
          :class="{ flipped: card.isFlipped, matched: card.isMatched }"
          @click="flipCard(card)"
        >
          <view class="card-front">?</view>
          <view class="card-back">{{ card.emoji }}</view>
        </view>
      </view>
    </view>

    <!-- 开始界面 -->
    <view v-if="gameStatus === 'ready'" class="game-overlay">
      <view class="overlay-content">
        <text class="overlay-title">🃏 记忆翻牌</text>
        <text class="overlay-desc">关卡 {{ level }}</text>
        <text class="overlay-desc">{{ totalPairs }} 对卡片</text>
        <text class="overlay-desc">挑战你的记忆力！</text>
        <button class="start-btn" @click="startGame">开始游戏</button>
      </view>
    </view>

    <!-- 游戏结束 -->
    <view v-if="gameStatus === 'finished'" class="game-overlay">
      <view class="overlay-content">
        <text class="overlay-title">🎉 完成!</text>
        <text class="overlay-score">用时: {{ formatTime(timeElapsed) }}</text>
        <text class="overlay-desc">翻牌次数: {{ moves }}</text>
        <text class="overlay-desc">得分: {{ finalScore }}</text>
        <view class="overlay-actions">
          <button v-if="level < maxLevel" class="action-btn next-btn" @click="nextLevel">下一关</button>
          <button v-if="level >= maxLevel" class="action-btn complete-btn" @click="goBack">已通关!</button>
          <button class="action-btn retry-btn" @click="restartGame">再玩一次</button>
          <button class="action-btn back-btn" @click="goBack">返回</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { navigateBack } from '@/utils'
import { getStorage, setStorage } from '@/utils/storage'
import { submitScoreApi } from '@/api/game'

interface Card {
  id: number
  emoji: string
  isFlipped: boolean
  isMatched: boolean
}

const level = ref(1)
const gameType = ref('memory_card')
const moves = ref(0)
const timeElapsed = ref(0)
const gameStatus = ref<'ready' | 'playing' | 'finished'>('ready')
const matchedPairs = ref(0)
const maxLevel = 10

const cards = ref<Card[]>([])
const flippedCards = ref<Card[]>([])
let gameTimer: any = null
let lockBoard = false

const emojis = ['🎈', '🎁', '⭐', '💎', '🎨', '🎵', '🎮', '🏆', '🌈', '🌸', 
              '🍎', '🍕', '🚗', '✈️', '🏠', '📱', '💝', '🎯', '🔥', '⚡']

const gridSize = computed(() => {
  if (level.value <= 2) return 4  // 4x4 = 8对
  if (level.value <= 5) return 5  // 5x5 = 12对 (去掉1个)
  if (level.value <= 8) return 6  // 6x6 = 18对
  return 6  // 最大6x6
})

const totalPairs = computed(() => {
  const size = gridSize.value
  return Math.floor((size * size) / 2)
})

const finalScore = computed(() => {
  // 分数计算：基础分 - 翻牌惩罚 - 时间惩罚
  const baseScore = totalPairs.value * 100
  const movePenalty = Math.max(0, (moves.value - totalPairs.value * 2) * 5)
  const timePenalty = Math.floor(timeElapsed.value / 2)
  return Math.max(50, baseScore - movePenalty - timePenalty)
})

onLoad((options: any) => {
  level.value = parseInt(options.level) || 1
  gameType.value = options.gameType || 'memory_card'
})

const startGame = () => {
  gameStatus.value = 'playing'
  moves.value = 0
  timeElapsed.value = 0
  matchedPairs.value = 0
  flippedCards.value = []
  
  initCards()
  
  // 开始计时
  gameTimer = setInterval(() => {
    timeElapsed.value++
  }, 1000)
}

const initCards = () => {
  const pairCount = totalPairs.value
  const selectedEmojis = emojis.slice(0, pairCount)
  
  // 创建卡片对
  const cardPairs: Card[] = []
  selectedEmojis.forEach((emoji, index) => {
    cardPairs.push({
      id: index * 2,
      emoji,
      isFlipped: false,
      isMatched: false
    })
    cardPairs.push({
      id: index * 2 + 1,
      emoji,
      isFlipped: false,
      isMatched: false
    })
  })
  
  // 洗牌
  cards.value = shuffleArray(cardPairs)
}

const shuffleArray = <T,>(array: T[]): T[] => {
  const newArray = [...array]
  for (let i = newArray.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [newArray[i], newArray[j]] = [newArray[j], newArray[i]]
  }
  return newArray
}

const flipCard = (card: Card) => {
  if (lockBoard) return
  if (card.isFlipped || card.isMatched) return
  if (flippedCards.value.length >= 2) return
  
  card.isFlipped = true
  flippedCards.value.push(card)
  
  if (flippedCards.value.length === 2) {
    moves.value++
    checkMatch()
  }
}

const checkMatch = () => {
  const [card1, card2] = flippedCards.value
  
  if (card1.emoji === card2.emoji) {
    // 匹配成功
    card1.isMatched = true
    card2.isMatched = true
    matchedPairs.value++
    
    uni.vibrateShort({})
    flippedCards.value = []
    
    // 检查是否完成
    if (matchedPairs.value === totalPairs.value) {
      setTimeout(() => {
        endGame()
      }, 500)
    }
  } else {
    // 不匹配，翻回去
    lockBoard = true
    setTimeout(() => {
      card1.isFlipped = false
      card2.isFlipped = false
      flippedCards.value = []
      lockBoard = false
    }, 1000)
  }
}

const endGame = () => {
  gameStatus.value = 'finished'
  clearInterval(gameTimer)
  submitScore()
}

const submitScore = async () => {
  try {
    await submitScoreApi({
      game_type: 'memory_card',
      level: level.value,
      score: finalScore.value,
      time_spent: timeElapsed.value
    })
    saveProgress()
  } catch (error) {
    console.error('提交成绩失败:', error)
  }
}

const saveProgress = () => {
  const gameLevels = getStorage('game_levels', {})
  const currentLevel = gameLevels[gameType.value] || 1
  if (level.value >= currentLevel) {
    gameLevels[gameType.value] = Math.min(level.value + 1, maxLevel)
    setStorage('game_levels', gameLevels)
  }
}

const nextLevel = () => {
  if (level.value < maxLevel) {
    level.value++
    gameStatus.value = 'ready'
  }
}

const restartGame = () => {
  gameStatus.value = 'ready'
}

const goBack = () => {
  navigateBack()
}

const formatTime = (seconds: number) => {
  const mins = Math.floor(seconds / 60)
  const secs = seconds % 60
  return mins > 0 ? `${mins}:${secs.toString().padStart(2, '0')}` : `${secs}s`
}

onUnmounted(() => {
  clearInterval(gameTimer)
})
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.memory-card-game {
  @include cyber-page-bg;
  min-height: 100vh;
  padding: 20rpx;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 30% 25%, rgba(0, 217, 255, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 70% 75%, rgba(255, 0, 214, 0.15) 0%, transparent 50%);
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
  border-radius: 30rpx;
  margin-bottom: 25rpx;
  border: 3rpx solid rgba(0, 217, 255, 0.5);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(0, 217, 255, 0.4),
    inset 0 2rpx 0 rgba(0, 217, 255, 0.3);
  position: relative;
  z-index: 1;

  .info-item {
    flex: 1;
    text-align: center;

    .label {
      display: block;
      font-size: 24rpx;
      @include neon-text(#8B5CF6);
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
  padding: 20rpx;
  position: relative;
  z-index: 1;
}

.cards-grid {
  display: grid;
  gap: 18rpx;
  max-width: 700rpx;
  margin: 0 auto;
}

.card {
  aspect-ratio: 1;
  perspective: 1000rpx;
  cursor: pointer;
  animation: cardFloat 3s ease-in-out infinite;
  
  &:nth-child(2n) {
    animation-delay: 0.2s;
  }
  
  &:nth-child(3n) {
    animation-delay: 0.4s;
  }
}

.card-front,
.card-back {
  position: absolute;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  backface-visibility: hidden;
  border-radius: 20rpx;
  font-size: 60rpx;
  transition: transform 0.6s cubic-bezier(0.4, 0, 0.2, 1);
}

.card-front {
  background: linear-gradient(135deg, rgba(138, 92, 246, 0.9) 0%, rgba(0, 217, 255, 0.9) 100%);
  @include neon-text(#ffffff);
  font-weight: bold;
  font-size: 90rpx;
  border: 3rpx solid rgba(138, 92, 246, 0.7);
  box-shadow: 
    0 10rpx 40rpx rgba(138, 92, 246, 0.5),
    0 0 60rpx rgba(138, 92, 246, 0.4),
    inset 0 0 30rpx rgba(255, 255, 255, 0.2);
  text-shadow: 
    0 0 20rpx rgba(255, 255, 255, 1),
    0 0 40rpx rgba(255, 255, 255, 0.6),
    0 2rpx 5rpx rgba(0, 0, 0, 0.3);
  animation: cardGlow 2s ease-in-out infinite;
}

.card-back {
  background: linear-gradient(135deg, rgba(20, 26, 56, 0.95) 0%, rgba(15, 20, 45, 0.98) 100%);
  backdrop-filter: blur(20rpx);
  border: 3rpx solid rgba(0, 217, 255, 0.6);
  transform: rotateY(180deg);
  box-shadow: 
    0 8rpx 24rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(0, 217, 255, 0.3),
    inset 0 0 30rpx rgba(0, 217, 255, 0.1);
  filter: drop-shadow(0 0 15rpx rgba(0, 217, 255, 0.5));
}

.card.flipped .card-front {
  transform: rotateY(180deg);
}

.card.flipped .card-back {
  transform: rotateY(0deg);
}

.card.matched {
  animation: matchedPulse 1s ease-in-out;
  pointer-events: none;
  
  .card-back {
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    border-color: rgba(255, 0, 214, 0.8);
    box-shadow: 
      0 10rpx 40rpx rgba(255, 0, 214, 0.6),
      0 0 80rpx rgba(255, 0, 214, 0.5),
      inset 0 0 40rpx rgba(255, 255, 255, 0.3);
    filter: drop-shadow(0 0 25rpx rgba(255, 0, 214, 0.8));
  }
}

@keyframes cardFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-5rpx);
  }
}

@keyframes cardGlow {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(138, 92, 246, 0.5),
      0 0 60rpx rgba(138, 92, 246, 0.4),
      inset 0 0 30rpx rgba(255, 255, 255, 0.2);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(138, 92, 246, 0.7),
      0 0 80rpx rgba(138, 92, 246, 0.6),
      inset 0 0 40rpx rgba(255, 255, 255, 0.3);
  }
}

@keyframes matchedPulse {
  0% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
  100% {
    transform: scale(1);
  }
}

.game-overlay {
  position: fixed;
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
  border: 4rpx solid rgba(0, 217, 255, 0.6);
  padding: 65rpx 55rpx;
  text-align: center;
  min-width: 540rpx;
  box-shadow: 
    0 25rpx 80rpx rgba(0, 0, 0, 0.7),
    0 0 100rpx rgba(0, 217, 255, 0.5),
    inset 0 0 80rpx rgba(0, 217, 255, 0.12);
  animation: scaleIn 0.5s cubic-bezier(0.4, 0, 0.2, 1);

  .overlay-title {
    display: block;
    font-size: 52rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
    margin-bottom: 35rpx;
    filter: brightness(1.2);
    animation: titlePulse 2s ease-in-out infinite;
  }

  .overlay-score {
    display: block;
    font-size: 64rpx;
    font-weight: bold;
    @include neon-title(#00D9FF);
    margin-bottom: 25rpx;
    filter: brightness(1.3);
    text-shadow: 
      0 0 30rpx rgba(0, 217, 255, 1),
      0 0 60rpx rgba(0, 217, 255, 0.8),
      0 0 90rpx rgba(0, 217, 255, 0.5),
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
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(255, 0, 214, 0.7);
    font-size: 36rpx;
    font-weight: bold;
    border-radius: 50rpx;
    box-shadow: 
      0 10rpx 40rpx rgba(255, 0, 214, 0.6),
      0 0 60rpx rgba(255, 0, 214, 0.5);
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
        0 12rpx 50rpx rgba(255, 0, 214, 0.8),
        0 0 80rpx rgba(255, 0, 214, 0.7);
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
      0 0 20rpx rgba(255, 214, 0, 1),
      0 0 40rpx rgba(255, 214, 0, 0.6),
      0 2rpx 5rpx rgba(0, 0, 0, 0.3);
  }
  50% {
    text-shadow: 
      0 0 30rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(255, 214, 0, 0.8),
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
      0 10rpx 40rpx rgba(255, 0, 214, 0.6),
      0 0 60rpx rgba(255, 0, 214, 0.5);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(255, 0, 214, 0.8),
      0 0 80rpx rgba(255, 0, 214, 0.7);
  }
}
</style>
