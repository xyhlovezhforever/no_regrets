<template>
  <view class="puzzle-game">
    <!-- 游戏信息栏 -->
    <view class="game-header">
      <view class="info-item">
        <text class="label">关卡</text>
        <text class="value">{{ level }}</text>
      </view>
      <view class="info-item">
        <text class="label">移动</text>
        <text class="value">{{ moves }}</text>
      </view>
      <view class="info-item">
        <text class="label">时间</text>
        <text class="value">{{ formatTime(timeElapsed) }}</text>
      </view>
    </view>

    <!-- 游戏区域 -->
    <view v-if="gameStatus === 'playing'" class="game-area">
      <view class="puzzle-board" :style="{ gridTemplateColumns: `repeat(${gridSize}, 1fr)` }">
        <view 
          v-for="(tile, index) in tiles" 
          :key="index"
          class="puzzle-tile"
          :class="{ empty: tile === 0, correct: isCorrectPosition(tile, index) }"
          :style="{ background: getTileColor(tile) }"
          @click="moveTile(index)"
        >
          <text v-if="tile !== 0" class="tile-number">{{ tile }}</text>
        </view>
      </view>
      
      <view class="game-actions">
        <button class="action-btn" @click="shuffleBoard">重新打乱</button>
      </view>
    </view>

    <!-- 开始界面 -->
    <view v-if="gameStatus === 'ready'" class="game-overlay">
      <view class="overlay-content">
        <text class="overlay-title">🧩 拼图游戏</text>
        <text class="overlay-desc">关卡 {{ level }}</text>
        <text class="overlay-desc">{{ gridSize }}x{{ gridSize }} 拼图</text>
        <text class="overlay-desc">按数字顺序排列所有方块</text>
        <button class="start-btn" @click="startGame">开始游戏</button>
      </view>
    </view>

    <!-- 游戏结束 -->
    <view v-if="gameStatus === 'finished'" class="game-overlay">
      <view class="overlay-content">
        <text class="overlay-title">🎉 完成!</text>
        <text class="overlay-score">用时: {{ formatTime(timeElapsed) }}</text>
        <text class="overlay-desc">移动次数: {{ moves }}</text>
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

const level = ref(1)
const gameType = ref('puzzle')
const moves = ref(0)
const timeElapsed = ref(0)
const gameStatus = ref<'ready' | 'playing' | 'finished'>('ready')
const tiles = ref<number[]>([])
const maxLevel = 10
let gameTimer: any = null

const gridSize = computed(() => {
  if (level.value <= 3) return 3  // 3x3
  if (level.value <= 6) return 4  // 4x4
  return 5  // 5x5
})

const totalTiles = computed(() => gridSize.value * gridSize.value)

const finalScore = computed(() => {
  const baseScore = gridSize.value * 200
  const movePenalty = Math.max(0, (moves.value - gridSize.value * 20) * 2)
  const timePenalty = Math.floor(timeElapsed.value / 3)
  return Math.max(50, baseScore - movePenalty - timePenalty)
})

onLoad((options: any) => {
  level.value = parseInt(options.level) || 1
  gameType.value = options.gameType || 'puzzle'
})

const startGame = () => {
  gameStatus.value = 'playing'
  moves.value = 0
  timeElapsed.value = 0
  
  initBoard()
  shuffleBoard()
  
  // 开始计时
  gameTimer = setInterval(() => {
    timeElapsed.value++
  }, 1000)
}

const initBoard = () => {
  tiles.value = Array.from({ length: totalTiles.value }, (_, i) => i)
}

const shuffleBoard = () => {
  // 确保拼图可解
  do {
    for (let i = tiles.value.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [tiles.value[i], tiles.value[j]] = [tiles.value[j], tiles.value[i]]
    }
  } while (!isSolvable() || isSolved())
  
  moves.value = 0
}

const isSolvable = () => {
  let inversions = 0
  const size = gridSize.value
  
  for (let i = 0; i < tiles.value.length; i++) {
    for (let j = i + 1; j < tiles.value.length; j++) {
      if (tiles.value[i] !== 0 && tiles.value[j] !== 0 && tiles.value[i] > tiles.value[j]) {
        inversions++
      }
    }
  }
  
  if (size % 2 === 1) {
    return inversions % 2 === 0
  } else {
    const emptyRow = Math.floor(tiles.value.indexOf(0) / size)
    return (inversions + emptyRow) % 2 === 1
  }
}

const moveTile = (index: number) => {
  const size = gridSize.value
  const emptyIndex = tiles.value.indexOf(0)
  
  const row = Math.floor(index / size)
  const col = index % size
  const emptyRow = Math.floor(emptyIndex / size)
  const emptyCol = emptyIndex % size
  
  // 检查是否相邻
  const isAdjacent = 
    (row === emptyRow && Math.abs(col - emptyCol) === 1) ||
    (col === emptyCol && Math.abs(row - emptyRow) === 1)
  
  if (isAdjacent) {
    // 交换
    [tiles.value[index], tiles.value[emptyIndex]] = [tiles.value[emptyIndex], tiles.value[index]]
    moves.value++
    
    uni.vibrateShort({})
    
    // 检查是否完成
    if (isSolved()) {
      setTimeout(() => {
        endGame()
      }, 300)
    }
  }
}

const isSolved = () => {
  for (let i = 0; i < tiles.value.length; i++) {
    if (tiles.value[i] !== i) return false
  }
  return true
}

const isCorrectPosition = (tile: number, index: number) => {
  return tile === index && tile !== 0
}

const getTileColor = (tile: number) => {
  if (tile === 0) return 'transparent'
  const hue = (tile * 30) % 360
  return `hsl(${hue}, 70%, 65%)`
}

const endGame = () => {
  gameStatus.value = 'finished'
  clearInterval(gameTimer)
  submitScore()
}

const submitScore = async () => {
  try {
    await submitScoreApi({
      game_type: 'puzzle',
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

.puzzle-game {
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
      radial-gradient(circle at 25% 30%, rgba(255, 214, 0, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 75% 70%, rgba(138, 92, 246, 0.15) 0%, transparent 50%);
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
  border: 3rpx solid rgba(255, 214, 0, 0.5);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(255, 214, 0, 0.4),
    inset 0 2rpx 0 rgba(255, 214, 0, 0.3);
  position: relative;
  z-index: 1;

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
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 35rpx;
  position: relative;
  z-index: 1;
}

.puzzle-board {
  display: grid;
  gap: 10rpx;
  max-width: 600rpx;
  width: 90vw;
  aspect-ratio: 1;
  @include neon-card;
  background: linear-gradient(135deg, rgba(20, 26, 56, 0.95) 0%, rgba(15, 20, 45, 0.98) 100%);
  backdrop-filter: blur(30rpx);
  padding: 20rpx;
  border-radius: 30rpx;
  border: 4rpx solid rgba(138, 92, 246, 0.6);
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.7),
    0 0 80rpx rgba(138, 92, 246, 0.5),
    inset 0 0 60rpx rgba(138, 92, 246, 0.12);
  animation: boardPulse 3s ease-in-out infinite;
}

.puzzle-tile {
  aspect-ratio: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 15rpx;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  cursor: pointer;
  position: relative;
  overflow: hidden;
  
  &::before {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: conic-gradient(
      from 0deg,
      transparent 0deg,
      rgba(255, 255, 255, 0.2) 90deg,
      transparent 180deg
    );
    animation: tileRotate 4s linear infinite;
    opacity: 0;
  }

  &:hover::before {
    opacity: 1;
  }

  &.empty {
    background: linear-gradient(135deg, rgba(0, 0, 0, 0.3) 0%, rgba(0, 0, 0, 0.2) 100%) !important;
    border: 2rpx dashed rgba(138, 92, 246, 0.4);
    box-shadow: inset 0 4rpx 15rpx rgba(0, 0, 0, 0.4);
    
    &::before {
      display: none;
    }
  }

  &.correct {
    border: 3rpx solid rgba(0, 217, 255, 0.8);
    box-shadow: 
      0 0 30rpx rgba(0, 217, 255, 0.8),
      0 0 60rpx rgba(0, 217, 255, 0.5),
      inset 0 0 20rpx rgba(0, 217, 255, 0.2);
    animation: correctPulse 1s ease-in-out infinite;
  }

  &:active:not(.empty) {
    transform: scale(0.92);
  }

  .tile-number {
    font-size: 52rpx;
    font-weight: bold;
    @include neon-text(#ffffff);
    text-shadow: 
      0 0 15rpx rgba(255, 255, 255, 1),
      0 0 30rpx rgba(255, 255, 255, 0.6),
      0 3rpx 6rpx rgba(0, 0, 0, 0.4);
    position: relative;
    z-index: 1;
  }
}

@keyframes boardPulse {
  0%, 100% {
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.7),
      0 0 80rpx rgba(138, 92, 246, 0.5),
      inset 0 0 60rpx rgba(138, 92, 246, 0.12);
  }
  50% {
    box-shadow: 
      0 25rpx 70rpx rgba(0, 0, 0, 0.8),
      0 0 100rpx rgba(138, 92, 246, 0.7),
      inset 0 0 80rpx rgba(138, 92, 246, 0.18);
  }
}

@keyframes tileRotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes correctPulse {
  0%, 100% {
    box-shadow: 
      0 0 30rpx rgba(0, 217, 255, 0.8),
      0 0 60rpx rgba(0, 217, 255, 0.5),
      inset 0 0 20rpx rgba(0, 217, 255, 0.2);
  }
  50% {
    box-shadow: 
      0 0 40rpx rgba(0, 217, 255, 1),
      0 0 80rpx rgba(0, 217, 255, 0.8),
      inset 0 0 30rpx rgba(0, 217, 255, 0.4);
  }
}

.game-actions {
  .action-btn {
    padding: 25rpx 70rpx;
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(255, 0, 214, 0.7);
    border-radius: 50rpx;
    font-size: 30rpx;
    font-weight: bold;
    box-shadow: 
      0 8rpx 24rpx rgba(255, 0, 214, 0.5),
      0 0 40rpx rgba(255, 0, 214, 0.4);
    text-shadow: 
      0 0 12rpx rgba(255, 255, 255, 0.8),
      0 2rpx 5rpx rgba(0, 0, 0, 0.3);
    transition: all 0.3s;

    &::after {
      border: none;
    }

    &:active {
      transform: scale(0.95);
      box-shadow: 
        0 10rpx 30rpx rgba(255, 0, 214, 0.7),
        0 0 60rpx rgba(255, 0, 214, 0.6);
    }
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
  border: 4rpx solid rgba(255, 214, 0, 0.6);
  padding: 65rpx 55rpx;
  text-align: center;
  min-width: 540rpx;
  box-shadow: 
    0 25rpx 80rpx rgba(0, 0, 0, 0.7),
    0 0 100rpx rgba(255, 214, 0, 0.5),
    inset 0 0 80rpx rgba(255, 214, 0, 0.12);
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
    background: linear-gradient(135deg, rgba(138, 92, 246, 0.9) 0%, rgba(0, 217, 255, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(138, 92, 246, 0.7);
    font-size: 36rpx;
    font-weight: bold;
    border-radius: 50rpx;
    box-shadow: 
      0 10rpx 40rpx rgba(138, 92, 246, 0.6),
      0 0 60rpx rgba(138, 92, 246, 0.5);
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
        0 12rpx 50rpx rgba(138, 92, 246, 0.8),
        0 0 80rpx rgba(138, 92, 246, 0.7);
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
      0 10rpx 40rpx rgba(138, 92, 246, 0.6),
      0 0 60rpx rgba(138, 92, 246, 0.5);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(138, 92, 246, 0.8),
      0 0 80rpx rgba(138, 92, 246, 0.7);
  }
}
</style>
