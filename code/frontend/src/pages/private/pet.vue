<template>
  <view class="pet-page">
    <!-- 宠物展示区 -->
    <view class="pet-stage">
      <!-- 天空背景 -->
      <view class="sky">
        <view v-for="i in 3" :key="i" class="cloud" :style="{ animationDelay: `${i * 2}s` }">☁️</view>
      </view>

      <!-- 宠物 -->
      <view class="pet-container" @click="interactWithPet">
        <text class="pet-emoji" :class="{ happy: isHappy, sad: isHungry || isSick }">
          {{ currentPet.emoji }}
        </text>
        <view v-if="showLove" class="love-effect">❤️</view>
      </view>

      <!-- 宠物名称 -->
      <view class="pet-name">
        <text>{{ pet.name || '我的宠物' }}</text>
        <text class="pet-level">Lv.{{ pet.level }}</text>
      </view>
    </view>

    <!-- 宠物状态栏 -->
    <view class="status-bar">
      <view class="status-item">
        <text class="status-icon">❤️</text>
        <view class="progress-bar">
          <view class="progress" :style="{ width: pet.health + '%' }"></view>
        </view>
        <text class="status-value">{{ pet.health }}</text>
      </view>
      <view class="status-item">
        <text class="status-icon">🍖</text>
        <view class="progress-bar">
          <view class="progress" :style="{ width: pet.hunger + '%' }"></view>
        </view>
        <text class="status-value">{{ pet.hunger }}</text>
      </view>
      <view class="status-item">
        <text class="status-icon">😊</text>
        <view class="progress-bar">
          <view class="progress" :style="{ width: pet.happiness + '%' }"></view>
        </view>
        <text class="status-value">{{ pet.happiness }}</text>
      </view>
    </view>

    <!-- 操作按钮 -->
    <view class="actions">
      <button class="action-btn feed-btn" @click="feedPet">
        <text class="btn-icon">🍖</text>
        <text class="btn-text">喂食</text>
      </button>
      <button class="action-btn play-btn" @click="playWithPet">
        <text class="btn-icon">🎾</text>
        <text class="btn-text">玩耍</text>
      </button>
      <button class="action-btn clean-btn" @click="cleanPet">
        <text class="btn-icon">🛁</text>
        <text class="btn-text">清洁</text>
      </button>
      <button class="action-btn change-btn" @click="changePet">
        <text class="btn-icon">🔄</text>
        <text class="btn-text">换宠物</text>
      </button>
    </view>

    <!-- 宠物信息 -->
    <view class="pet-info">
      <view class="info-row">
        <text class="info-label">年龄：</text>
        <text class="info-value">{{ pet.age }} 天</text>
      </view>
      <view class="info-row">
        <text class="info-label">经验：</text>
        <text class="info-value">{{ pet.exp }} / {{ expToNextLevel }}</text>
      </view>
      <view class="info-row">
        <text class="info-label">上次互动：</text>
        <text class="info-value">{{ formatLastInteraction() }}</text>
      </view>
    </view>

    <!-- 选择宠物弹窗 -->
    <view v-if="showPetSelector" class="pet-selector-mask" @click="showPetSelector = false">
      <view class="pet-selector" @click.stop>
        <view class="selector-header">
          <text class="selector-title">选择你的宠物</text>
        </view>
        <view class="pet-grid">
          <view
            v-for="(petType, index) in petTypes"
            :key="index"
            class="pet-option"
            :class="{ selected: currentPet.id === petType.id }"
            @click="selectPet(petType)"
          >
            <text class="pet-option-emoji">{{ petType.emoji }}</text>
            <text class="pet-option-name">{{ petType.name }}</text>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { getStorage, setStorage } from '@/utils/storage'

interface Pet {
  id: string
  name: string
  emoji: string
  level: number
  exp: number
  health: number
  hunger: number
  happiness: number
  age: number // 天数
  lastInteraction: number // 时间戳
}

interface PetType {
  id: string
  name: string
  emoji: string
}

const petTypes: PetType[] = [
  { id: 'cat', name: '小猫', emoji: '🐱' },
  { id: 'dog', name: '小狗', emoji: '🐶' },
  { id: 'rabbit', name: '小兔', emoji: '🐰' },
  { id: 'hamster', name: '仓鼠', emoji: '🐹' },
  { id: 'bird', name: '小鸟', emoji: '🐦' },
  { id: 'fish', name: '小鱼', emoji: '🐠' },
  { id: 'panda', name: '熊猫', emoji: '🐼' },
  { id: 'bear', name: '小熊', emoji: '🐻' }
]

const pet = ref<Pet>({
  id: 'cat',
  name: '咪咪',
  emoji: '🐱',
  level: 1,
  exp: 0,
  health: 100,
  hunger: 80,
  happiness: 70,
  age: 0,
  lastInteraction: Date.now()
})

const currentPet = computed(() => {
  return petTypes.find(p => p.id === pet.value.id) || petTypes[0]
})

const expToNextLevel = computed(() => pet.value.level * 100)

const isHappy = ref(false)
const isHungry = computed(() => pet.value.hunger < 30)
const isSick = computed(() => pet.value.health < 50)
const showLove = ref(false)
const showPetSelector = ref(false)

let updateTimer: any = null

onMounted(() => {
  loadPet()
  startAutoUpdate()
})

onUnmounted(() => {
  if (updateTimer) {
    clearInterval(updateTimer)
  }
})

const loadPet = () => {
  const savedPet = getStorage<Pet | null>('myPet', null)
  if (savedPet) {
    pet.value = savedPet
    updateAgeAndStats()
  }
}

const savePet = () => {
  setStorage('myPet', pet.value)
}

const startAutoUpdate = () => {
  updateTimer = setInterval(() => {
    // 每分钟自动减少状态
    pet.value.hunger = Math.max(0, pet.value.hunger - 0.5)
    pet.value.happiness = Math.max(0, pet.value.happiness - 0.3)
    
    if (pet.value.hunger < 20) {
      pet.value.health = Math.max(0, pet.value.health - 0.5)
    }
    
    savePet()
  }, 60000) // 每分钟
}

const updateAgeAndStats = () => {
  const now = Date.now()
  const daysPassed = Math.floor((now - pet.value.lastInteraction) / (1000 * 60 * 60 * 24))
  
  if (daysPassed > 0) {
    pet.value.age += daysPassed
    // 长时间未互动，状态下降
    pet.value.hunger = Math.max(0, pet.value.hunger - daysPassed * 10)
    pet.value.happiness = Math.max(0, pet.value.happiness - daysPassed * 5)
  }
}

const interactWithPet = () => {
  showLove.value = true
  isHappy.value = true
  
  pet.value.happiness = Math.min(100, pet.value.happiness + 3)
  pet.value.exp += 1
  
  checkLevelUp()
  
  uni.vibrateShort({ type: 'light' })
  
  setTimeout(() => {
    showLove.value = false
    isHappy.value = false
  }, 1500)
  
  pet.value.lastInteraction = Date.now()
  savePet()
}

const feedPet = () => {
  if (pet.value.hunger >= 100) {
    uni.showToast({ title: '宠物不饿哦', icon: 'none' })
    return
  }
  
  pet.value.hunger = Math.min(100, pet.value.hunger + 30)
  pet.value.health = Math.min(100, pet.value.health + 10)
  pet.value.happiness = Math.min(100, pet.value.happiness + 10)
  pet.value.exp += 5
  
  checkLevelUp()
  
  uni.showToast({ title: '喂食成功 🍖', icon: 'none' })
  
  pet.value.lastInteraction = Date.now()
  savePet()
}

const playWithPet = () => {
  if (pet.value.hunger < 20) {
    uni.showToast({ title: '宠物太饿了，先喂食吧', icon: 'none' })
    return
  }
  
  pet.value.happiness = Math.min(100, pet.value.happiness + 20)
  pet.value.hunger = Math.max(0, pet.value.hunger - 10)
  pet.value.exp += 10
  
  checkLevelUp()
  
  // 多个爱心特效
  for (let i = 0; i < 3; i++) {
    setTimeout(() => {
      isHappy.value = true
      showLove.value = true
      uni.vibrateShort({ type: 'medium' })
      
      setTimeout(() => {
        showLove.value = false
      }, 500)
    }, i * 400)
  }
  
  setTimeout(() => {
    isHappy.value = false
  }, 2500)
  
  uni.showToast({ title: '玩得真开心 🎾', icon: 'none' })
  
  pet.value.lastInteraction = Date.now()
  savePet()
}

const cleanPet = () => {
  pet.value.health = Math.min(100, pet.value.health + 15)
  pet.value.happiness = Math.min(100, pet.value.happiness + 10)
  pet.value.exp += 5
  
  checkLevelUp()
  
  uni.showToast({ title: '真干净 🛁', icon: 'none' })
  
  pet.value.lastInteraction = Date.now()
  savePet()
}

const checkLevelUp = () => {
  if (pet.value.exp >= expToNextLevel.value) {
    pet.value.level++
    pet.value.exp = 0
    
    uni.showModal({
      title: '🎉 升级了！',
      content: `${pet.value.name} 升到了 ${pet.value.level} 级！`,
      showCancel: false
    })
  }
}

const changePet = () => {
  showPetSelector.value = true
}

const selectPet = (petType: PetType) => {
  pet.value.id = petType.id
  pet.value.emoji = petType.emoji
  showPetSelector.value = false
  savePet()
  
  uni.showToast({ title: `换成了 ${petType.name}`, icon: 'success' })
}

const formatLastInteraction = () => {
  const now = Date.now()
  const diff = now - pet.value.lastInteraction
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)
  
  if (days > 0) return `${days} 天前`
  if (hours > 0) return `${hours} 小时前`
  if (minutes > 0) return `${minutes} 分钟前`
  return '刚刚'
}
</script>

<style lang="scss" scoped>
.pet-page {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  padding-bottom: 30rpx;
}

.pet-stage {
  position: relative;
  height: 600rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  overflow: hidden;

  .sky {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;

    .cloud {
      position: absolute;
      font-size: 60rpx;
      animation: float-cloud 20s linear infinite;

      &:nth-child(1) {
        top: 50rpx;
        left: -100rpx;
      }

      &:nth-child(2) {
        top: 150rpx;
        left: -100rpx;
        animation-delay: 5s;
      }

      &:nth-child(3) {
        top: 250rpx;
        left: -100rpx;
        animation-delay: 10s;
      }
    }
  }

  .pet-container {
    position: relative;
    z-index: 1;
    perspective: 1000rpx;

    .pet-emoji {
      font-size: 200rpx;
      display: block;
      transition: all 0.3s;
      transform-style: preserve-3d;
      animation: float-idle 3s ease-in-out infinite;
      filter: drop-shadow(0 20rpx 40rpx rgba(0, 0, 0, 0.3));

      &.happy {
        animation: jump-happy-3d 0.8s ease, float-idle 3s ease-in-out infinite;
      }

      &.sad {
        opacity: 0.7;
        filter: grayscale(50%) drop-shadow(0 10rpx 20rpx rgba(0, 0, 0, 0.2));
        animation: shake-sad 2s ease-in-out infinite;
      }
    }

    .love-effect {
      position: absolute;
      top: -50rpx;
      left: 50%;
      transform: translateX(-50%);
      font-size: 60rpx;
      animation: float-up-3d 1.5s ease-out;
      filter: drop-shadow(0 0 10rpx rgba(255, 105, 180, 0.8));
    }
  }

  .pet-name {
    margin-top: 20rpx;
    text-align: center;
    color: #ffffff;
    font-size: 36rpx;
    font-weight: bold;
    text-shadow: 0 2rpx 4rpx rgba(0, 0, 0, 0.2);

    .pet-level {
      margin-left: 15rpx;
      font-size: 28rpx;
      background: rgba(255, 255, 255, 0.3);
      padding: 5rpx 15rpx;
      border-radius: 20rpx;
    }
  }
}

@keyframes float-cloud {
  from {
    transform: translateX(0);
  }
  to {
    transform: translateX(calc(100vw + 100rpx));
  }
}

@keyframes float-idle {
  0%, 100% {
    transform: translateY(0) rotateY(0deg);
  }
  25% {
    transform: translateY(-15rpx) rotateY(-5deg);
  }
  50% {
    transform: translateY(-10rpx) rotateY(0deg);
  }
  75% {
    transform: translateY(-15rpx) rotateY(5deg);
  }
}

@keyframes jump-happy-3d {
  0% {
    transform: translateY(0) rotateY(0deg) scale(1);
  }
  25% {
    transform: translateY(-80rpx) rotateY(180deg) scale(1.2);
  }
  50% {
    transform: translateY(-100rpx) rotateY(360deg) scale(1.3);
  }
  75% {
    transform: translateY(-50rpx) rotateY(540deg) scale(1.15);
  }
  100% {
    transform: translateY(0) rotateY(720deg) scale(1);
  }
}

@keyframes shake-sad {
  0%, 100% {
    transform: translateX(0) rotateZ(0deg);
  }
  25% {
    transform: translateX(-10rpx) rotateZ(-3deg);
  }
  75% {
    transform: translateX(10rpx) rotateZ(3deg);
  }
}

@keyframes float-up-3d {
  0% {
    opacity: 1;
    transform: translateX(-50%) translateY(0) scale(1) rotateZ(0deg);
  }
  50% {
    opacity: 1;
    transform: translateX(-50%) translateY(-80rpx) scale(1.5) rotateZ(180deg);
  }
  100% {
    opacity: 0;
    transform: translateX(-50%) translateY(-150rpx) scale(0.5) rotateZ(360deg);
  }
}

.status-bar {
  padding: 30rpx;
  background: rgba(255, 255, 255, 0.9);
  margin: 20rpx 30rpx;
  border-radius: 20rpx;

  .status-item {
    display: flex;
    align-items: center;
    margin-bottom: 20rpx;

    &:last-child {
      margin-bottom: 0;
    }

    .status-icon {
      font-size: 40rpx;
      margin-right: 15rpx;
    }

    .progress-bar {
      flex: 1;
      height: 20rpx;
      background: #e0e0e0;
      border-radius: 10rpx;
      overflow: hidden;

      .progress {
        height: 100%;
        background: linear-gradient(90deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
        transition: width 0.3s;
      }
    }

    .status-value {
      margin-left: 15rpx;
      font-size: 24rpx;
      color: var(--theme-text-secondary);
      min-width: 60rpx;
      text-align: right;
    }
  }
}

.actions {
  padding: 0 30rpx;
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 15rpx;
  margin-bottom: 20rpx;

  .action-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 25rpx 10rpx;
    background: rgba(255, 255, 255, 0.9);
    border-radius: 20rpx;
    border: none;
    box-shadow: 0 5rpx 15rpx rgba(0, 0, 0, 0.1);

    &::after {
      border: none;
    }

    &:active {
      transform: scale(0.95);
    }

    .btn-icon {
      font-size: 48rpx;
      margin-bottom: 8rpx;
    }

    .btn-text {
      font-size: 22rpx;
      color: var(--theme-text-secondary);
    }
  }
}

.pet-info {
  background: rgba(255, 255, 255, 0.9);
  margin: 0 30rpx;
  padding: 30rpx;
  border-radius: 20rpx;

  .info-row {
    display: flex;
    justify-content: space-between;
    margin-bottom: 15rpx;

    &:last-child {
      margin-bottom: 0;
    }

    .info-label {
      font-size: 28rpx;
      color: var(--theme-text-secondary);
    }

    .info-value {
      font-size: 28rpx;
      color: var(--theme-text);
      font-weight: bold;
    }
  }
}

.pet-selector-mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
}

.pet-selector {
  width: 90%;
  max-height: 80%;
  background: var(--theme-surface);
  border-radius: 30rpx;
  overflow: hidden;

  .selector-header {
    padding: 40rpx;
    text-align: center;
    border-bottom: 1rpx solid #f0f0f0;

    .selector-title {
      font-size: 36rpx;
      font-weight: bold;
      color: var(--theme-text);
    }
  }

  .pet-grid {
    padding: 30rpx;
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 20rpx;
    max-height: 600rpx;
    overflow-y: auto;

    .pet-option {
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 20rpx;
      background: var(--theme-background);
      border-radius: 20rpx;
      border: 3rpx solid transparent;

      &.selected {
        border-color: var(--theme-primary);
        background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
      }

      .pet-option-emoji {
        font-size: 60rpx;
        margin-bottom: 10rpx;
      }

      .pet-option-name {
        font-size: 22rpx;
        color: var(--theme-text-secondary);
      }
    }
  }
}
</style>

