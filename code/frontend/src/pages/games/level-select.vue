<template>
  <view class="level-select-page">
    <view class="header">
      <text class="title">{{ gameTitle }}</text>
      <text class="subtitle">选择关卡挑战</text>
    </view>

    <view class="levels-grid">
      <view 
        v-for="level in 10" 
        :key="level"
        class="level-card"
        @click="selectLevel(level)"
      >
        <view class="level-number">{{ level }}</view>
        <view class="level-info">
          <text class="difficulty">{{ getDifficulty(level) }}</text>
          <text class="best-score" v-if="bestScores[level]">
            最佳: {{ bestScores[level] }}
          </text>
        </view>
      </view>
    </view>

    <!-- 排行榜按钮 -->
    <view class="actions">
      <button class="action-btn" @click="showRanking = true">
        <text class="btn-icon">🏆</text>
        <text>查看排行榜</text>
      </button>
    </view>

    <!-- 排行榜弹窗 -->
    <view v-if="showRanking" class="modal-mask" @click="showRanking = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">好友排行榜</text>
          <text class="modal-close" @click="showRanking = false">✕</text>
        </view>
        
        <view class="level-tabs">
          <view 
            v-for="lv in 10" 
            :key="lv"
            class="tab-item"
            :class="{ active: selectedLevel === lv }"
            @click="loadRanking(lv)"
          >
            {{ lv }}
          </view>
        </view>

        <view class="ranking-list">
          <view v-if="rankings.length === 0" class="empty">
            <text>暂无排名记录</text>
          </view>
          <view 
            v-for="item in rankings" 
            :key="item.user_id"
            class="ranking-item"
            :class="{ self: item.is_self }"
          >
            <view class="rank">
              <text v-if="item.rank <= 3" class="medal">{{ getMedal(item.rank) }}</text>
              <text v-else class="rank-number">{{ item.rank }}</text>
            </view>
            <view class="user-info">
              <text class="username">{{ item.nickname || item.username }}</text>
              <text class="score">得分: {{ item.score }}</text>
            </view>
            <text v-if="item.time_spent" class="time">{{ formatTime(item.time_spent) }}</text>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { getMyScoresApi, getFriendsRankingApi, type RankingItem } from '@/api/game'

const gameType = ref('')
const gameTitle = ref('')
const gamePage = ref('')
const bestScores = ref<Record<number, number>>({})
const showRanking = ref(false)
const selectedLevel = ref(1)
const rankings = ref<RankingItem[]>([])

onLoad((options: any) => {
  gameType.value = options.type || 'bubble_pop'
  gameTitle.value = options.title || '游戏'
  gamePage.value = options.page || ''
  loadBestScores()
})

const loadBestScores = async () => {
  try {
    for (let level = 1; level <= 10; level++) {
      const scores = await getMyScoresApi(gameType.value, level)
      if (scores.length > 0) {
        bestScores.value[level] = scores[0].score
      }
    }
  } catch (error) {
    console.error('加载最佳成绩失败:', error)
  }
}

const loadRanking = async (level: number) => {
  selectedLevel.value = level
  try {
    rankings.value = await getFriendsRankingApi(gameType.value, level)
  } catch (error) {
    console.error('加载排行榜失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  }
}

const selectLevel = (level: number) => {
  // 直接使用 uni.navigateTo，避免路径被重复编码
  const url = `${gamePage.value}?level=${level}`
  uni.navigateTo({ url })
}

const getDifficulty = (level: number) => {
  if (level <= 3) return '简单'
  if (level <= 6) return '中等'
  if (level <= 8) return '困难'
  return '噩梦'
}

const getMedal = (rank: number) => {
  const medals = ['🥇', '🥈', '🥉']
  return medals[rank - 1] || ''
}

const formatTime = (seconds: number) => {
  const mins = Math.floor(seconds / 60)
  const secs = seconds % 60
  return mins > 0 ? `${mins}:${secs.toString().padStart(2, '0')}` : `${secs}秒`
}

onMounted(() => {
  loadRanking(1)
})
</script>

<style lang="scss" scoped>
.level-select-page {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  padding: 40rpx 30rpx;
}

.header {
  text-align: center;
  margin-bottom: 50rpx;

  .title {
    display: block;
    font-size: 52rpx;
    font-weight: bold;
    color: #ffffff;
    margin-bottom: 15rpx;
  }

  .subtitle {
    display: block;
    font-size: 28rpx;
    color: rgba(255, 255, 255, 0.9);
  }
}

.levels-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 20rpx;
  margin-bottom: 40rpx;
}

.level-card {
  aspect-ratio: 1;
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10rpx);
  border-radius: 20rpx;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border: 2rpx solid rgba(255, 255, 255, 0.3);
  transition: all 0.3s;

  &:active {
    transform: scale(0.9);
    background: rgba(255, 255, 255, 0.3);
  }

  .level-number {
    font-size: 44rpx;
    font-weight: bold;
    color: #ffffff;
    margin-bottom: 5rpx;
  }

  .level-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5rpx;

    .difficulty {
      font-size: 20rpx;
      color: rgba(255, 255, 255, 0.8);
    }

    .best-score {
      font-size: 18rpx;
      color: #ffd700;
    }
  }
}

.actions {
  display: flex;
  justify-content: center;
  margin-top: 40rpx;

  .action-btn {
    display: flex;
    align-items: center;
    gap: 10rpx;
    padding: 25rpx 50rpx;
    background: rgba(255, 255, 255, 0.3);
    backdrop-filter: blur(10rpx);
    border-radius: 50rpx;
    border: 2rpx solid rgba(255, 255, 255, 0.5);
    color: #ffffff;
    font-size: 28rpx;
    font-weight: bold;

    &::after {
      border: none;
    }

    .btn-icon {
      font-size: 36rpx;
    }
  }
}

/* 弹窗样式 */
.modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}

.modal-content {
  width: 90%;
  max-width: 700rpx;
  max-height: 80vh;
  background: var(--theme-surface);
  border-radius: 30rpx;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 30rpx 40rpx;
  border-bottom: 2rpx solid #f0f0f0;

  .modal-title {
    font-size: 32rpx;
    font-weight: bold;
    color: var(--theme-text);
  }

  .modal-close {
    font-size: 36rpx;
    color: var(--theme-text-secondary);
  }
}

.level-tabs {
  display: flex;
  padding: 20rpx 30rpx;
  gap: 15rpx;
  overflow-x: auto;
  border-bottom: 2rpx solid #f0f0f0;

  .tab-item {
    flex-shrink: 0;
    width: 60rpx;
    height: 60rpx;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--theme-background);
    border-radius: 10rpx;
    font-size: 26rpx;
    color: var(--theme-text-secondary);
    transition: all 0.3s;

    &.active {
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      color: #ffffff;
      font-weight: bold;
    }
  }
}

.ranking-list {
  flex: 1;
  overflow-y: auto;
  padding: 20rpx 30rpx;

  .empty {
    text-align: center;
    padding: 100rpx 0;
    color: var(--theme-text-secondary);
    font-size: 28rpx;
  }

  .ranking-item {
    display: flex;
    align-items: center;
    padding: 25rpx 20rpx;
    margin-bottom: 15rpx;
    background: #f8f8f8;
    border-radius: 15rpx;
    transition: all 0.3s;

    &.self {
      background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
      border: 2rpx solid #667eea;
    }

    .rank {
      width: 60rpx;
      text-align: center;

      .medal {
        font-size: 40rpx;
      }

      .rank-number {
        font-size: 28rpx;
        font-weight: bold;
        color: var(--theme-text-secondary);
      }
    }

    .user-info {
      flex: 1;
      display: flex;
      flex-direction: column;
      gap: 8rpx;

      .username {
        font-size: 28rpx;
        color: var(--theme-text);
        font-weight: bold;
      }

      .score {
        font-size: 24rpx;
        color: var(--theme-text-secondary);
      }
    }

    .time {
      font-size: 24rpx;
      color: var(--theme-text-secondary);
    }
  }
}
</style>
