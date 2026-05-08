<template>
  <view class="memory-card-page">
    <view class="header">
      <view class="stats">
        <text class="stat">翻牌: {{ flips }}</text>
        <text class="stat">配对: {{ matches }}/{{ totalPairs }}</text>
        <text class="stat">⏱️ {{ formatTime(time) }}</text>
      </view>
    </view>

    <view class="card-grid">
      <view
        v-for="(card, index) in cards"
        :key="index"
        class="card"
        :class="{ flipped: card.flipped, matched: card.matched }"
        @click="flipCard(index)"
      >
        <view class="card-front">?</view>
        <view class="card-back">{{ card.emoji }}</view>
      </view>
    </view>

    <button v-if="gameOver" class="restart-btn" @click="initGame">
      <text>🎉 重新开始</text>
    </button>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface Card {
  emoji: string
  flipped: boolean
  matched: boolean
}

const emojis = ['🍎', '🍌', '🍇', '🍊', '🍓', '🍉', '🍒', '🥝']
const cards = ref<Card[]>([])
const flips = ref(0)
const matches = ref(0)
const totalPairs = ref(8)
const time = ref(0)
const gameOver = ref(false)
let firstCard: number | null = null
let secondCard: number | null = null
let canFlip = true
let timer: any = null

onMounted(() => {
  initGame()
})

onUnmounted(() => {
  if (timer) clearInterval(timer)
})

const initGame = () => {
  const shuffled = [...emojis, ...emojis]
    .sort(() => Math.random() - 0.5)
    .map(emoji => ({ emoji, flipped: false, matched: false }))
  
  cards.value = shuffled
  flips.value = 0
  matches.value = 0
  time.value = 0
  gameOver.value = false
  firstCard = null
  secondCard = null
  canFlip = true

  if (timer) clearInterval(timer)
  timer = setInterval(() => { time.value++ }, 1000)
}

const flipCard = (index: number) => {
  if (!canFlip || cards.value[index].flipped || cards.value[index].matched) return
  
  cards.value[index].flipped = true
  flips.value++

  if (firstCard === null) {
    firstCard = index
  } else if (secondCard === null) {
    secondCard = index
    canFlip = false

    setTimeout(() => {
      checkMatch()
    }, 500)
  }
}

const checkMatch = () => {
  if (firstCard !== null && secondCard !== null) {
    if (cards.value[firstCard].emoji === cards.value[secondCard].emoji) {
      cards.value[firstCard].matched = true
      cards.value[secondCard].matched = true
      matches.value++

      if (matches.value === totalPairs.value) {
        clearInterval(timer)
        gameOver.value = true
        uni.showToast({ title: '🎉 恭喜通关！', icon: 'success' })
      }
    } else {
      cards.value[firstCard].flipped = false
      cards.value[secondCard].flipped = false
    }
  }

  firstCard = null
  secondCard = null
  canFlip = true
}

const formatTime = (seconds: number) => {
  const mins = Math.floor(seconds / 60)
  const secs = seconds % 60
  return `${mins}:${secs.toString().padStart(2, '0')}`
}
</script>

<style lang="scss" scoped>
.memory-card-page {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  padding: 40rpx 30rpx;
}

.header {
  margin-bottom: 40rpx;

  .stats {
    display: flex;
    justify-content: space-around;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 20rpx;
    padding: 20rpx;

    .stat {
      font-size: 26rpx;
      color: #ffffff;
      font-weight: bold;
    }
  }
}

.card-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 15rpx;
  margin-bottom: 40rpx;

  .card {
    aspect-ratio: 1;
    perspective: 1000rpx;
    cursor: pointer;

    .card-front,
    .card-back {
      position: absolute;
      width: 100%;
      height: 100%;
      backface-visibility: hidden;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 15rpx;
      font-size: 50rpx;
      transition: transform 0.6s;
    }

    .card-front {
      background: rgba(255, 255, 255, 0.9);
      color: var(--theme-primary);
      font-weight: bold;
    }

    .card-back {
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      transform: rotateY(180deg);
    }

    &.flipped {
      .card-front { transform: rotateY(180deg); }
      .card-back { transform: rotateY(0); }
    }

    &.matched {
      opacity: 0.5;
      pointer-events: none;
    }
  }
}

.restart-btn {
  width: 100%;
  height: 90rpx;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  color: #ffffff;
  border-radius: 50rpx;
  border: none;
  font-size: 32rpx;
  font-weight: bold;
  line-height: 90rpx;

  &::after { border: none; }
}
</style>

