<template>
  <view class="games-page">
    <view class="header">
      <text class="title">🎮 游戏中心</text>
      <text class="subtitle">轻松娱乐，释放压力</text>
    </view>

    <view class="games-grid">
      <view class="game-card card-1" @click="startGame('bubble_pop', '/pages/games/bubble-pop')">
        <view class="card-bg"></view>
        <view class="card-content">
          <text class="game-icon">🎈</text>
          <text class="game-title">扎气球</text>
          <text class="game-desc">泡泡大作战</text>
          <view class="level-badge">关卡 {{ levels.bubble_pop || 1 }}</view>
          <view class="play-btn">
            <text>开始游戏</text>
          </view>
        </view>
      </view>

      <view class="game-card card-2" @click="startGame('memory_card', '/pages/games/memory-card')">
        <view class="card-bg"></view>
        <view class="card-content">
          <text class="game-icon">🃏</text>
          <text class="game-title">记忆翻牌</text>
          <text class="game-desc">挑战记忆力</text>
          <view class="level-badge">关卡 {{ levels.memory_card || 1 }}</view>
          <view class="play-btn">
            <text>开始游戏</text>
          </view>
        </view>
      </view>

      <view class="game-card card-3" @click="startGame('puzzle', '/pages/games/puzzle')">
        <view class="card-bg"></view>
        <view class="card-content">
          <text class="game-icon">🧩</text>
          <text class="game-title">拼图游戏</text>
          <text class="game-desc">考验智力</text>
          <view class="level-badge">关卡 {{ levels.puzzle || 1 }}</view>
          <view class="play-btn">
            <text>开始游戏</text>
          </view>
        </view>
      </view>

      <view class="game-card card-4" @click="startGame('whack_mole', '/pages/games/whack-mole')">
        <view class="card-bg"></view>
        <view class="card-content">
          <text class="game-icon">🔨</text>
          <text class="game-title">打地鼠</text>
          <text class="game-desc">反应速度</text>
          <view class="level-badge">关卡 {{ levels.whack_mole || 1 }}</view>
          <view class="play-btn">
            <text>开始游戏</text>
          </view>
        </view>
      </view>
    </view>

    <!-- 游戏说明 -->
    <view class="tips">
      <text class="tips-icon">💡</text>
      <text class="tips-text">放松心情，享受游戏乐趣～</text>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { getStorage, setStorage } from '@/utils/storage'

const levels = ref<Record<string, number>>({
  bubble_pop: 1,
  memory_card: 1,
  puzzle: 1,
  whack_mole: 1
})

onMounted(() => {
  loadLevels()
})

onShow(() => {
  loadLevels()
})

const loadLevels = () => {
  const savedLevels = getStorage('game_levels', {})
  levels.value = {
    bubble_pop: savedLevels.bubble_pop || 1,
    memory_card: savedLevels.memory_card || 1,
    puzzle: savedLevels.puzzle || 1,
    whack_mole: savedLevels.whack_mole || 1
  }
}

const startGame = (gameType: string, page: string) => {
  const currentLevel = levels.value[gameType] || 1
  uni.navigateTo({
    url: `${page}?level=${currentLevel}&gameType=${gameType}`
  })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.games-page {
  @include cyber-page-bg;
  padding: 40rpx 30rpx;
  position: relative;
  
  &::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 20% 30%, rgba(255, 0, 214, 0.1) 0%, transparent 50%),
      radial-gradient(circle at 80% 70%, rgba(0, 217, 255, 0.1) 0%, transparent 50%);
    pointer-events: none;
    z-index: 0;
  }
}

.header {
  text-align: center;
  margin-bottom: 50rpx;
  position: relative;
  z-index: 1;
  animation: fadeInDown 0.8s ease-out;

  .title {
    display: block;
    font-size: 52rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
    margin-bottom: 15rpx;
    animation: neonFlicker 5s ease-in-out infinite;
  }

  .subtitle {
    display: block;
    font-size: 28rpx;
    @include neon-text(#ffffff);
  }
}

@keyframes fadeInDown {
  from {
    opacity: 0;
    transform: translateY(-30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.games-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 25rpx;
  margin-bottom: 50rpx;
  position: relative;
  z-index: 1;
}

.game-card {
  position: relative;
  border-radius: 25rpx;
  overflow: visible;
  min-height: 300rpx;
  @include rainbow-border;
  box-shadow: 0 15rpx 40rpx rgba(0, 0, 0, 0.5);
  transition: all 0.3s;
  animation: fadeInUp 0.6s ease-out backwards, cardFloat 4s ease-in-out infinite;
  
  &:nth-child(1) { animation-delay: 0.1s; }
  &:nth-child(2) { animation-delay: 0.2s; }
  &:nth-child(3) { animation-delay: 0.3s; }
  &:nth-child(4) { animation-delay: 0.4s; }

  &:active {
    transform: scale(0.95) translateY(-10rpx);
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.6),
      0 0 50rpx currentColor;
  }

  .card-bg {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 0;
    border-radius: 25rpx;
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
        rgba(255, 255, 255, 0.1) 90deg,
        transparent 180deg,
        rgba(255, 255, 255, 0.1) 270deg,
        transparent 360deg
      );
      animation: rotateGlow 6s linear infinite;
    }
  }

  .card-content {
    position: relative;
    z-index: 1;
    height: 100%;
    padding: 35rpx 25rpx;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    .game-icon {
      font-size: 80rpx;
      margin-bottom: 20rpx;
      filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 0.8));
      animation: iconBounce 3s ease-in-out infinite, glowPulse 2s ease-in-out infinite;
    }

    .game-title {
      font-size: 34rpx;
      font-weight: bold;
      @include neon-title(#ffffff);
      margin-bottom: 10rpx;
    }

    .game-desc {
      font-size: 24rpx;
      @include neon-text(#00D9FF);
      margin-bottom: 15rpx;
    }

    .level-badge {
      @include neon-tag(#FFD600);
      font-size: 22rpx;
      font-weight: bold;
      margin-bottom: 15rpx;
    }

    .play-btn {
      @include glow-button(#FF00D6);
      padding: 15rpx 35rpx;
      border-radius: 50rpx;
      font-size: 26rpx;
      font-weight: bold;
      transition: all 0.3s;
      animation: btnPulse 3s ease-in-out infinite;

      &:not(.disabled):active {
        transform: scale(0.95);
      }

      &.disabled {
        opacity: 0.6;
        animation: none;
      }
    }
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(255, 0, 214, 0.5),
      0 8rpx 24rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(255, 0, 214, 0.2);
  }
  50% {
    box-shadow: 
      0 0 30rpx rgba(255, 0, 214, 0.8),
      0 12rpx 32rpx rgba(0, 0, 0, 0.4),
      inset 0 0 30rpx rgba(255, 0, 214, 0.3);
  }
}

.card-1 .card-bg {
  background: linear-gradient(135deg, rgba(240, 147, 251, 0.9) 0%, rgba(245, 87, 108, 0.9) 100%);
  box-shadow: inset 0 0 40rpx rgba(240, 147, 251, 0.3);
}

.card-2 .card-bg {
  background: linear-gradient(135deg, rgba(67, 233, 123, 0.9) 0%, rgba(56, 249, 215, 0.9) 100%);
  box-shadow: inset 0 0 40rpx rgba(67, 233, 123, 0.3);
}

.card-3 .card-bg {
  background: linear-gradient(135deg, rgba(79, 172, 254, 0.9) 0%, rgba(0, 242, 254, 0.9) 100%);
  box-shadow: inset 0 0 40rpx rgba(79, 172, 254, 0.3);
}

.card-4 .card-bg {
  background: linear-gradient(135deg, rgba(250, 112, 154, 0.9) 0%, rgba(254, 225, 64, 0.9) 100%);
  box-shadow: inset 0 0 40rpx rgba(250, 112, 154, 0.3);
}

.tips {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15rpx;
  padding: 30rpx;
  @include neon-card;
  border-radius: 20rpx;
  position: relative;
  z-index: 1;
  animation: fadeIn 1s ease-out;

  .tips-icon {
    font-size: 36rpx;
    filter: drop-shadow(0 0 15rpx rgba(255, 214, 0, 0.8));
    animation: iconPulse 2s ease-in-out infinite;
  }

  .tips-text {
    font-size: 26rpx;
    @include neon-text(#ffffff);
  }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes iconPulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

@keyframes iconBounce {
  0%, 100% {
    transform: translateY(0) scale(1);
  }
  50% {
    transform: translateY(-8rpx) scale(1.1);
  }
}

@keyframes glowPulse {
  0%, 100% {
    filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 0.8));
  }
  50% {
    filter: drop-shadow(0 0 30rpx rgba(255, 214, 0, 1));
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(40rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes cardFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-8rpx);
  }
}

@keyframes rotateGlow {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>

