<template>
  <view class="whack-mole-game">
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
        <text class="value">{{ timeLeft }}s</text>
      </view>
      <view class="info-item">
        <text class="label">连击</text>
        <text class="value">{{ combo }}x</text>
      </view>
    </view>

    <!-- 游戏区域 -->
    <view v-if="gameStatus === 'playing'" class="game-area">
      <!-- 背景装饰 -->
      <view class="bg-decoration">
        <view v-for="i in 20" :key="i" class="star" :style="getStarStyle(i)"></view>
      </view>
      
      <!-- 连击特效 -->
      <view v-if="combo > 2" class="combo-display">
        <text class="combo-text">{{ combo }} COMBO!</text>
        <view class="combo-fire">🔥</view>
      </view>
      
      <view class="moles-grid">
        <view 
          v-for="(hole, index) in holes" 
          :key="index"
          class="hole-container"
          :class="{ active: hole.hasMole, hit: hole.isHit }"
        >
          <!-- 洞穴 -->
          <view class="hole" @click="whackMole(index)">
            <view class="hole-shadow"></view>
            <view class="hole-inner"></view>
            <view class="hole-rim"></view>
            
            <!-- 地鼠 -->
            <view v-if="hole.hasMole" class="mole" :class="'mole-type-' + hole.type">
              <view class="mole-emoji">{{ getMoleEmoji(hole.type) }}</view>
              <view v-if="hole.type === 1" class="sparkle"></view>
            </view>
            
            <!-- 打击特效 -->
            <view v-if="hole.isHit" class="hit-effect">
              <view class="hit-burst"></view>
              <view class="hit-stars">
                <text v-for="s in 5" :key="s" class="hit-star">⭐</text>
              </view>
              <view class="score-popup">+{{ moleTypes[hole.type].points }}</view>
            </view>
          </view>
        </view>
      </view>
    </view>

    <!-- 开始界面 -->
    <view v-if="gameStatus === 'ready'" class="game-overlay">
      <view class="overlay-content">
        <text class="overlay-title">🔨 打地鼠</text>
        <text class="overlay-desc">关卡 {{ level }}</text>
        <text class="overlay-desc">时间: {{ gameTime }}秒</text>
        <text class="overlay-desc">击中地鼠得分，小心炸弹！</text>
        <button class="start-btn" @click="startGame">开始游戏</button>
      </view>
    </view>

    <!-- 游戏结束 -->
    <view v-if="gameStatus === 'finished'" class="game-overlay">
      <view class="overlay-content">
        <text class="overlay-title">{{ score >= targetScore ? '🎉 成功!' : '😢 失败' }}</text>
        <text class="overlay-score">得分: {{ score }}</text>
        <text class="overlay-desc">目标: {{ targetScore }}</text>
        <text class="overlay-desc">最高连击: {{ maxCombo }}x</text>
        <view class="overlay-actions">
          <button v-if="score >= targetScore && level < maxLevel" class="action-btn next-btn" @click="nextLevel">下一关</button>
          <button v-if="score >= targetScore && level >= maxLevel" class="action-btn complete-btn" @click="goBack">已通关!</button>
          <button class="action-btn retry-btn" @click="restartGame">再玩一次</button>
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

interface Hole {
  hasMole: boolean
  type: number  // 0: 普通, 1: 金色, 2: 炸弹
  isHit: boolean
  showTime: number
}

const level = ref(1)
const gameType = ref('whack_mole')
const score = ref(0)
const timeLeft = ref(60)
const gameTime = ref(60)
const targetScore = ref(100)
const combo = ref(0)
const maxCombo = ref(0)
const gameStatus = ref<'ready' | 'playing' | 'finished'>('ready')
const maxLevel = 10

const holes = ref<Hole[]>([])
let gameTimer: any = null
let spawnTimer: any = null
let comboTimer: any = null

const moleTypes = [
  { emoji: '🐭', points: 10, probability: 0.7, showTime: 1500 },
  { emoji: '👑', points: 30, probability: 0.2, showTime: 1000 },
  { emoji: '💣', points: -20, probability: 0.1, showTime: 2000 }
]

onLoad((options: any) => {
  level.value = parseInt(options.level) || 1
  gameType.value = options.gameType || 'whack_mole'
  initLevel()
})

const initLevel = () => {
  // 根据关卡调整难度
  targetScore.value = 100 + (level.value - 1) * 50
  gameTime.value = Math.max(40, 60 - (level.value - 1) * 2)
  timeLeft.value = gameTime.value
  
  // 初始化洞穴 (3x3)
  holes.value = Array.from({ length: 9 }, () => ({
    hasMole: false,
    type: 0,
    isHit: false,
    showTime: 0
  }))
}

const startGame = () => {
  gameStatus.value = 'playing'
  score.value = 0
  combo.value = 0
  maxCombo.value = 0
  timeLeft.value = gameTime.value
  
  // 清理所有洞穴
  holes.value.forEach(hole => {
    hole.hasMole = false
    hole.isHit = false
  })
  
  // 开始计时
  gameTimer = setInterval(() => {
    timeLeft.value--
    if (timeLeft.value <= 0) {
      endGame()
    }
  }, 1000)
  
  // 开始生成地鼠
  spawnMoles()
}

const spawnMoles = () => {
  const spawnInterval = Math.max(300, 800 - level.value * 50)
  
  spawnTimer = setInterval(() => {
    // 找出空闲的洞
    const emptyHoles = holes.value
      .map((hole, index) => ({ hole, index }))
      .filter(({ hole }) => !hole.hasMole)
    
    if (emptyHoles.length > 0) {
      // 随机选择一个空洞
      const randomIndex = Math.floor(Math.random() * emptyHoles.length)
      const { hole, index } = emptyHoles[randomIndex]
      
      // 确定地鼠类型
      const rand = Math.random()
      let type = 0
      let cumulativeProbability = 0
      
      for (let i = 0; i < moleTypes.length; i++) {
        cumulativeProbability += moleTypes[i].probability
        if (rand < cumulativeProbability) {
          type = i
          break
        }
      }
      
      // 显示地鼠
      hole.hasMole = true
      hole.type = type
      hole.isHit = false
      hole.showTime = moleTypes[type].showTime
      
      // 一段时间后自动消失
      setTimeout(() => {
        if (hole.hasMole && !hole.isHit) {
          hole.hasMole = false
          resetCombo()
        }
      }, hole.showTime)
    }
  }, spawnInterval)
}

const whackMole = (index: number) => {
  const hole = holes.value[index]
  
  if (!hole.hasMole || hole.isHit) return
  
  hole.isHit = true
  const points = moleTypes[hole.type].points
  
  // 计算得分（连击加成）
  if (points > 0) {
    combo.value++
    maxCombo.value = Math.max(maxCombo.value, combo.value)
    const comboBonus = Math.floor(combo.value / 5)
    score.value += points * (1 + comboBonus * 0.5)
    
    // 重置连击计时器
    clearTimeout(comboTimer)
    comboTimer = setTimeout(() => {
      resetCombo()
    }, 2000)
  } else {
    score.value += points
    resetCombo()
  }
  
  // 震动反馈
  uni.vibrateShort({})
  
  // 显示打击特效后再隐藏地鼠
  setTimeout(() => {
    hole.hasMole = false
  }, 100)
  
  // 延长打击特效显示时间
  setTimeout(() => {
    hole.isHit = false
  }, 800)
}

const resetCombo = () => {
  combo.value = 0
}

const getMoleEmoji = (type: number) => {
  return moleTypes[type].emoji
}

const getStarStyle = (index: number) => {
  const left = Math.random() * 100
  const top = Math.random() * 100
  const size = Math.random() * 3 + 1
  const duration = Math.random() * 3 + 2
  const delay = Math.random() * 2
  return {
    left: `${left}%`,
    top: `${top}%`,
    width: `${size}px`,
    height: `${size}px`,
    animationDuration: `${duration}s`,
    animationDelay: `${delay}s`
  }
}

const endGame = () => {
  gameStatus.value = 'finished'
  clearInterval(gameTimer)
  clearInterval(spawnTimer)
  clearTimeout(comboTimer)
  
  // 提交成绩
  if (score.value >= targetScore.value) {
    submitScore()
  }
}

const submitScore = async () => {
  try {
    await submitScoreApi({
      game_type: 'whack_mole',
      level: level.value,
      score: score.value,
      time_spent: gameTime.value - timeLeft.value
    })
    if (score.value >= targetScore.value) {
      saveProgress()
    }
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
    initLevel()
  }
}

const restartGame = () => {
  gameStatus.value = 'ready'
  initLevel()
}

const goBack = () => {
  navigateBack()
}

onUnmounted(() => {
  clearInterval(gameTimer)
  clearInterval(spawnTimer)
  clearTimeout(comboTimer)
})
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.whack-mole-game {
  @include cyber-page-bg;
  min-height: 100vh;
  padding: 20rpx;
  position: relative;
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
      radial-gradient(circle at 75% 65%, rgba(255, 214, 0, 0.15) 0%, transparent 50%);
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

.game-header {
  display: flex;
  padding: 25rpx 30rpx;
  @include neon-card;
  background: linear-gradient(180deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 0.95) 100%);
  backdrop-filter: blur(30rpx);
  border-radius: 30rpx;
  margin-bottom: 25rpx;
  border: 3rpx solid rgba(255, 0, 214, 0.5);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(255, 0, 214, 0.4),
    inset 0 2rpx 0 rgba(255, 0, 214, 0.3);
  position: relative;
  z-index: 2;

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

.game-area {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: calc(100vh - 200rpx);
  padding: 20rpx;
  z-index: 1;
}

.bg-decoration {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  overflow: hidden;
  z-index: 0;

  .star {
    position: absolute;
    background: radial-gradient(circle, rgba(0, 217, 255, 1), rgba(255, 0, 214, 0.5));
    border-radius: 50%;
    box-shadow: 0 0 15rpx rgba(0, 217, 255, 0.8);
    animation: twinkle 2s ease-in-out infinite;
  }
}

@keyframes twinkle {
  0%, 100% { 
    opacity: 0.4;
    transform: scale(1);
    box-shadow: 0 0 15rpx rgba(0, 217, 255, 0.8);
  }
  50% { 
    opacity: 1;
    transform: scale(1.5);
    box-shadow: 0 0 30rpx rgba(0, 217, 255, 1);
  }
}

.combo-display {
  position: absolute;
  top: 150rpx;
  left: 50%;
  transform: translateX(-50%);
  z-index: 10;
  text-align: center;
  animation: combo-pulse 0.5s ease-in-out infinite;

  .combo-text {
    display: block;
    font-size: 56rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
    filter: brightness(1.3);
    text-shadow: 
      0 0 30rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(255, 214, 0, 1),
      0 0 90rpx rgba(255, 0, 214, 0.8),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
    animation: combo-shake 0.3s ease-in-out infinite;
  }

  .combo-fire {
    font-size: 70rpx;
    filter: drop-shadow(0 0 30rpx rgba(255, 100, 0, 1));
    animation: fire-dance 0.5s ease-in-out infinite;
  }
}

@keyframes combo-pulse {
  0%, 100% { transform: translateX(-50%) scale(1); }
  50% { transform: translateX(-50%) scale(1.15); }
}

@keyframes combo-shake {
  0%, 100% { transform: rotate(-3deg); }
  50% { transform: rotate(3deg); }
}

@keyframes fire-dance {
  0%, 100% { 
    transform: scale(1) rotate(0deg);
    filter: drop-shadow(0 0 30rpx rgba(255, 100, 0, 1));
  }
  25% { 
    transform: scale(1.3) rotate(-15deg);
    filter: drop-shadow(0 0 50rpx rgba(255, 100, 0, 1));
  }
  75% { 
    transform: scale(1.3) rotate(15deg);
    filter: drop-shadow(0 0 50rpx rgba(255, 100, 0, 1));
  }
}

.moles-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 40rpx;
  max-width: 600rpx;
  position: relative;
  z-index: 1;
}

.hole-container {
  position: relative;
  aspect-ratio: 1;
  
  &.active {
    .hole {
      animation: hole-active 0.3s ease-out;
    }
  }
  
  &.hit {
    .hole {
      animation: hole-shake 0.2s ease-out;
    }
  }
}

@keyframes hole-active {
  0% { transform: scale(1); }
  50% { transform: scale(1.05); }
  100% { transform: scale(1); }
}

@keyframes hole-shake {
  0%, 100% { transform: translate(0, 0); }
  25% { transform: translate(-5rpx, 0); }
  75% { transform: translate(5rpx, 0); }
}

.hole {
  position: relative;
  width: 100%;
  height: 100%;
  cursor: pointer;

  .hole-shadow {
    position: absolute;
    bottom: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 90%;
    height: 30rpx;
    background: radial-gradient(ellipse at center, rgba(138, 92, 246, 0.6), transparent);
    border-radius: 50%;
    filter: blur(8rpx);
  }

  .hole-inner {
    position: absolute;
    bottom: 20rpx;
    left: 50%;
    transform: translateX(-50%);
    width: 75%;
    height: 50%;
    background: radial-gradient(ellipse at center, rgba(10, 5, 20, 1) 0%, rgba(0, 0, 0, 1) 100%);
    border-radius: 50%;
    box-shadow: 
      inset 0 -15rpx 30rpx rgba(138, 92, 246, 0.4),
      inset 0 0 20rpx rgba(0, 0, 0, 0.9),
      0 5rpx 15rpx rgba(0, 0, 0, 0.5);
    z-index: 1;
  }

  .hole-rim {
    position: absolute;
    bottom: 20rpx;
    left: 50%;
    transform: translateX(-50%);
    width: 80%;
    height: 55%;
    border: 6rpx solid rgba(138, 92, 246, 0.6);
    border-radius: 50%;
    background: linear-gradient(180deg, rgba(60, 40, 80, 0.9) 0%, rgba(40, 25, 65, 0.95) 50%, rgba(20, 10, 40, 1) 100%);
    box-shadow: 
      0 5rpx 15rpx rgba(0, 0, 0, 0.6),
      0 0 25rpx rgba(138, 92, 246, 0.5),
      inset 0 3rpx 8rpx rgba(138, 92, 246, 0.3);
    z-index: 2;
  }

  .mole {
    position: absolute;
    bottom: 35%;
    left: 50%;
    transform: translateX(-50%);
    z-index: 3;
    animation: pop-up 0.3s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    
    .mole-emoji {
      font-size: 90rpx;
      filter: drop-shadow(0 8rpx 15rpx rgba(0, 0, 0, 0.5));
      animation: mole-bounce 0.5s ease-in-out infinite;
    }
  }

  &:active .mole .mole-emoji {
    transform: scale(0.9);
  }
}

@keyframes mole-bounce {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-8rpx); }
}

@keyframes pop-up {
  0% {
    bottom: -50%;
    opacity: 0;
    transform: translateX(-50%) scale(0.5);
  }
  60% {
    bottom: 40%;
    transform: translateX(-50%) scale(1.1);
  }
  100% {
    bottom: 35%;
    opacity: 1;
    transform: translateX(-50%) scale(1);
  }
}

.mole-type-1 {
  .sparkle {
    position: absolute;
    top: -20rpx;
    left: 50%;
    transform: translateX(-50%);
    width: 140rpx;
    height: 140rpx;
    background: radial-gradient(circle, rgba(255, 214, 0, 0.8) 0%, rgba(255, 0, 214, 0.4) 50%, transparent 70%);
    border-radius: 50%;
    box-shadow: 
      0 0 40rpx rgba(255, 214, 0, 1),
      0 0 80rpx rgba(255, 0, 214, 0.6);
    animation: sparkle-pulse 1s ease-in-out infinite;
  }
}

@keyframes sparkle-pulse {
  0%, 100% {
    transform: translateX(-50%) scale(0.7) rotate(0deg);
    opacity: 0.6;
    box-shadow: 
      0 0 40rpx rgba(255, 214, 0, 1),
      0 0 80rpx rgba(255, 0, 214, 0.6);
  }
  50% {
    transform: translateX(-50%) scale(1.3) rotate(180deg);
    opacity: 1;
    box-shadow: 
      0 0 60rpx rgba(255, 214, 0, 1),
      0 0 120rpx rgba(255, 0, 214, 1);
  }
}

.mole-type-2 .mole-emoji {
  filter: drop-shadow(0 0 20rpx rgba(255, 0, 0, 1));
  animation: bomb-shake 0.15s ease-in-out infinite, mole-bounce 0.5s ease-in-out infinite;
}

@keyframes bomb-shake {
  0%, 100% { 
    transform: rotate(-5deg);
    filter: drop-shadow(0 0 20rpx rgba(255, 0, 0, 1));
  }
  50% { 
    transform: rotate(5deg);
    filter: drop-shadow(0 0 30rpx rgba(255, 0, 0, 1));
  }
}

.hit-effect {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  z-index: 10;
  pointer-events: none;

  .hit-burst {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 180rpx;
    height: 180rpx;
    background: radial-gradient(circle, rgba(0, 217, 255, 1) 0%, rgba(255, 0, 214, 0.8) 40%, transparent 70%);
    border-radius: 50%;
    box-shadow: 
      0 0 60rpx rgba(0, 217, 255, 1),
      0 0 120rpx rgba(255, 0, 214, 0.8);
    animation: burst-expand 0.4s ease-out forwards;
  }

  .hit-stars {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);

    .hit-star {
      position: absolute;
      font-size: 40rpx;
      animation: star-burst 0.6s ease-out forwards;

      &:nth-child(1) {
        animation-delay: 0s;
        left: -60rpx;
        top: -60rpx;
      }
      &:nth-child(2) {
        animation-delay: 0.05s;
        left: 60rpx;
        top: -60rpx;
      }
      &:nth-child(3) {
        animation-delay: 0.1s;
        left: -60rpx;
        top: 60rpx;
      }
      &:nth-child(4) {
        animation-delay: 0.15s;
        left: 60rpx;
        top: 60rpx;
      }
      &:nth-child(5) {
        animation-delay: 0.2s;
        left: 0;
        top: 0;
      }
    }
  }

  .score-popup {
    position: absolute;
    top: -80rpx;
    left: 50%;
    transform: translateX(-50%);
    font-size: 56rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
    filter: brightness(1.3);
    text-shadow: 
      0 0 20rpx rgba(255, 214, 0, 1),
      0 0 40rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(0, 217, 255, 0.8),
      3rpx 3rpx 6rpx rgba(0, 0, 0, 0.6);
    animation: score-float 0.8s ease-out forwards;
  }
}

@keyframes burst-expand {
  0% {
    transform: translate(-50%, -50%) scale(0);
    opacity: 1;
    box-shadow: 
      0 0 60rpx rgba(0, 217, 255, 1),
      0 0 120rpx rgba(255, 0, 214, 0.8);
  }
  100% {
    transform: translate(-50%, -50%) scale(2.5);
    opacity: 0;
    box-shadow: 
      0 0 100rpx rgba(0, 217, 255, 0),
      0 0 200rpx rgba(255, 0, 214, 0);
  }
}

@keyframes star-burst {
  0% {
    transform: scale(0) rotate(0deg);
    opacity: 1;
  }
  50% {
    transform: scale(1.5) rotate(180deg);
    opacity: 1;
  }
  100% {
    transform: scale(0.5) rotate(360deg);
    opacity: 0;
  }
}

@keyframes score-float {
  0% {
    transform: translateX(-50%) translateY(0) scale(0.5);
    opacity: 1;
  }
  50% {
    transform: translateX(-50%) translateY(-30rpx) scale(1.2);
    opacity: 1;
  }
  100% {
    transform: translateX(-50%) translateY(-60rpx) scale(1);
    opacity: 0;
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
  border: 4rpx solid rgba(255, 0, 214, 0.6);
  padding: 65rpx 55rpx;
  text-align: center;
  min-width: 540rpx;
  box-shadow: 
    0 25rpx 80rpx rgba(0, 0, 0, 0.7),
    0 0 100rpx rgba(255, 0, 214, 0.5),
    inset 0 0 80rpx rgba(255, 0, 214, 0.12);
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
    @include neon-title(#FF00D6);
    margin-bottom: 25rpx;
    filter: brightness(1.3);
    text-shadow: 
      0 0 30rpx rgba(255, 0, 214, 1),
      0 0 60rpx rgba(255, 0, 214, 0.8),
      0 0 90rpx rgba(255, 0, 214, 0.5),
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
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(255, 214, 0, 0.9) 100%);
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
