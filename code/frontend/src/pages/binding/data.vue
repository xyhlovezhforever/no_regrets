<template>
  <view class="data-page">
    <!-- 动态背景 -->
    <view class="bg-gradient">
      <view class="bg-orb orb-1"></view>
      <view class="bg-orb orb-2"></view>
      <view class="bg-orb orb-3"></view>
    </view>

    <!-- 用户信息 -->
    <view v-if="userInfo" class="user-card">
      <view class="user-glow"></view>
      <image :src="userInfo.avatar || '/static/default-avatar.png'" class="user-avatar" />
      <view class="user-info">
        <view class="user-name">{{ userInfo.nickname }}</view>
        <view class="user-desc">查看TA授权的数据</view>
      </view>
    </view>

    <!-- Tab 切换 -->
    <view class="tabs">
      <view class="tab-bg-indicator" :style="{ transform: `translateX(${activeTab === 'notes' ? '0' : '100%'})` }"></view>
      <view
        v-for="tab in tabs"
        :key="tab.value"
        :class="['tab-item', { active: activeTab === tab.value }]"
        @click="switchTab(tab.value)"
      >
        <text class="tab-icon">{{ tab.icon }}</text>
        <text class="tab-text">{{ tab.label }}</text>
      </view>
    </view>

    <!-- 便签列表 -->
    <view v-if="activeTab === 'notes'" class="content">
      <view v-if="notes.length === 0" class="empty">
        <view class="empty-icon">📝</view>
        <text class="empty-text">暂无便签数据</text>
        <text class="empty-desc">对方还没有授权或没有便签</text>
      </view>
      <view v-else class="notes-list">
        <view
          v-for="(note, index) in notes"
          :key="note.id"
          class="note-item"
          :style="{ animationDelay: `${index * 0.1}s` }"
          @click="viewNoteDetail(note)"
        >
          <view class="note-glow"></view>
          <view class="note-header">
            <view class="note-title">
              <text v-if="note.is_pinned" class="pin-icon">📌</text>
              <text>{{ note.title }}</text>
            </view>
            <view v-if="note.category" class="note-category">{{ note.category }}</view>
          </view>
          <view class="note-content">{{ note.content }}</view>
          <view class="note-footer">
            <view class="note-tags">
              <text v-for="tag in note.tags" :key="tag" class="tag">{{ tag }}</text>
            </view>
            <view class="note-time">{{ formatTime(note.updated_at) }}</view>
          </view>
        </view>
      </view>
    </view>

    <!-- 记账列表 -->
    <view v-if="activeTab === 'accounts'" class="content">
      <!-- 查看完整记账按钮 -->
      <view class="view-full-btn" @click="viewFullAccounts">
        <text class="btn-icon">💰</text>
        <text class="btn-text">查看完整记账</text>
        <text class="btn-arrow">›</text>
      </view>

      <!-- 统计卡片 -->
      <view v-if="statistics" class="statistics-card">
        <view class="stat-glow"></view>
        <view class="stat-item income">
          <view class="stat-icon">💰</view>
          <view class="stat-info">
            <view class="stat-label">总收入</view>
            <view class="stat-value">¥{{ statistics.total_income.toFixed(2) }}</view>
          </view>
        </view>
        <view class="stat-item expense">
          <view class="stat-icon">💸</view>
          <view class="stat-info">
            <view class="stat-label">总支出</view>
            <view class="stat-value">¥{{ statistics.total_expense.toFixed(2) }}</view>
          </view>
        </view>
        <view class="stat-item balance">
          <view class="stat-icon">📊</view>
          <view class="stat-info">
            <view class="stat-label">余额</view>
            <view class="stat-value" :class="{ negative: statistics.balance < 0 }">
              ¥{{ statistics.balance.toFixed(2) }}
            </view>
          </view>
        </view>
      </view>

      <!-- 记账记录 -->
      <view v-if="accounts.length === 0" class="empty">
        <view class="empty-icon">💰</view>
        <text class="empty-text">暂无记账数据</text>
        <text class="empty-desc">对方还没有授权或没有记账记录</text>
      </view>
      <view v-else class="accounts-list">
        <view
          v-for="(account, index) in accounts"
          :key="account.id"
          class="account-item"
          :style="{ animationDelay: `${index * 0.1}s` }"
        >
          <view class="account-glow"></view>
          <view class="account-icon" :class="account.type_">
            {{ account.type_ === 'income' ? '💰' : '💸' }}
          </view>
          <view class="account-info">
            <view class="account-category">{{ account.category }}</view>
            <view v-if="account.description" class="account-desc">{{ account.description }}</view>
            <view class="account-date">{{ formatDate(account.date) }}</view>
          </view>
          <view class="account-amount" :class="account.type_">
            {{ account.type_ === 'income' ? '+' : '-' }}¥{{ account.amount.toFixed(2) }}
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import {
  getBoundUserNotesApi,
  getBoundUserAccountsApi,
  getBoundUserAccountStatisticsApi,
  type BoundNote,
  type BoundAccount,
  type AccountStatistics,
} from '@/api/boundData'
import { showToast, showLoading, hideLoading } from '@/utils'

const userId = ref('')
const userInfo = ref<any>(null)
const activeTab = ref('notes')

const tabs = [
  { label: '便签', value: 'notes', icon: '📝' },
  { label: '记账', value: 'accounts', icon: '💰' },
]

const notes = ref<BoundNote[]>([])
const accounts = ref<BoundAccount[]>([])
const statistics = ref<AccountStatistics | null>(null)

onLoad((options: any) => {
  if (options.userId) {
    userId.value = options.userId
    if (options.userInfo) {
      try {
        userInfo.value = JSON.parse(decodeURIComponent(options.userInfo))
      } catch (e) {
        console.error('解析用户信息失败:', e)
      }
    }
    loadData()
  }
})

const switchTab = (tab: string) => {
  activeTab.value = tab
  loadData()
}

const loadData = async () => {
  if (activeTab.value === 'notes') {
    await loadNotes()
  } else {
    await loadAccounts()
  }
}

const loadNotes = async () => {
  try {
    console.log('加载便签数据...')
    showLoading('加载中...')
    const data = await getBoundUserNotesApi(userId.value)
    notes.value = Array.isArray(data) ? data : []
    console.log('便签数据:', notes.value)
  } catch (error: any) {
    console.error('加载便签失败:', error)
    if (error.message.includes('权限')) {
      showToast('对方未授权查看便签')
    } else {
      showToast('加载失败')
    }
    notes.value = []
  } finally {
    hideLoading()
  }
}

const loadAccounts = async () => {
  try {
    console.log('加载记账数据...')
    showLoading('加载中...')
    
    // 加载记账记录
    const data = await getBoundUserAccountsApi(userId.value)
    accounts.value = Array.isArray(data) ? data : []
    console.log('记账数据:', accounts.value)
    
    // 加载统计数据
    const stats = await getBoundUserAccountStatisticsApi(userId.value)
    statistics.value = stats
    console.log('统计数据:', statistics.value)
  } catch (error: any) {
    console.error('加载记账失败:', error)
    if (error.message.includes('权限')) {
      showToast('对方未授权查看记账')
    } else {
      showToast('加载失败')
    }
    accounts.value = []
    statistics.value = null
  } finally {
    hideLoading()
  }
}

const viewNoteDetail = (note: BoundNote) => {
  // 跳转到便签页面，传入查看模式参数
  const userInfoStr = encodeURIComponent(JSON.stringify(userInfo.value))
  uni.navigateTo({
    url: `/pages/self/note?viewUserId=${userId.value}&userInfo=${userInfoStr}`,
  })
}

const viewFullAccounts = () => {
  // 跳转到记账页面，传入查看模式参数
  const userInfoStr = encodeURIComponent(JSON.stringify(userInfo.value))
  uni.navigateTo({
    url: `/pages/self/account?viewUserId=${userId.value}&userInfo=${userInfoStr}`,
  })
}

const goBack = () => {
  uni.navigateBack()
}

const formatTime = (time: string) => {
  const date = new Date(time)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const days = Math.floor(diff / (1000 * 60 * 60 * 24))
  
  if (days === 0) {
    return '今天'
  } else if (days === 1) {
    return '昨天'
  } else if (days < 7) {
    return `${days}天前`
  } else {
    return date.toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit' })
  }
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return date.toLocaleDateString('zh-CN', { 
    year: 'numeric',
    month: '2-digit', 
    day: '2-digit' 
  })
}
</script>

<style scoped lang="scss">
.data-page {
  min-height: 100vh;
  position: relative;
  overflow: hidden;
  background: linear-gradient(135deg, #0a0e27 0%, #1a1f3a 50%, #0f1429 100%);
  padding-bottom: 150rpx;
}

// 动态背景
.bg-gradient {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 0;
  pointer-events: none;
  overflow: hidden;
}

.bg-orb {
  position: absolute;
  border-radius: 50%;
  filter: blur(80rpx);
  opacity: 0.4;
  animation: float 20s ease-in-out infinite;
  
  &.orb-1 {
    width: 600rpx;
    height: 600rpx;
    top: -200rpx;
    left: -100rpx;
    background: radial-gradient(circle, rgba(16, 185, 129, 0.6) 0%, transparent 70%);
    animation-delay: 0s;
  }
  
  &.orb-2 {
    width: 800rpx;
    height: 800rpx;
    top: 40%;
    right: -200rpx;
    background: radial-gradient(circle, rgba(139, 92, 246, 0.5) 0%, transparent 70%);
    animation-delay: 5s;
  }
  
  &.orb-3 {
    width: 500rpx;
    height: 500rpx;
    bottom: -100rpx;
    left: 30%;
    background: radial-gradient(circle, rgba(0, 217, 255, 0.4) 0%, transparent 70%);
    animation-delay: 10s;
  }
}

@keyframes float {
  0%, 100% {
    transform: translate(0, 0) scale(1);
  }
  33% {
    transform: translate(50rpx, -50rpx) scale(1.1);
  }
  66% {
    transform: translate(-50rpx, 50rpx) scale(0.9);
  }
}

// 用户卡片
.user-card {
  margin-top: 20rpx;
  position: relative;
  z-index: 10;
  margin: 30rpx 20rpx;
  padding: 30rpx;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
  backdrop-filter: blur(20rpx);
  border-radius: 24rpx;
  border: 2rpx solid rgba(16, 185, 129, 0.3);
  display: flex;
  align-items: center;
  gap: 25rpx;
  overflow: hidden;
  box-shadow: 0 10rpx 40rpx rgba(0, 0, 0, 0.3);

  .user-glow {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(139, 92, 246, 0.1) 100%);
    filter: blur(20rpx);
    opacity: 0.5;
    pointer-events: none;
  }

  .user-avatar {
    width: 100rpx;
    height: 100rpx;
    border-radius: 50%;
    border: 4rpx solid rgba(16, 185, 129, 0.5);
    box-shadow: 0 8rpx 30rpx rgba(16, 185, 129, 0.4);
    z-index: 1;
  }

  .user-info {
    flex: 1;
    z-index: 1;

    .user-name {
      font-size: 36rpx;
      font-weight: bold;
      color: #fff;
      margin-bottom: 8rpx;
    }

    .user-desc {
      font-size: 26rpx;
      color: rgba(255, 255, 255, 0.6);
    }
  }
}

// Tab 切换
.tabs {
  position: relative;
  z-index: 10;
  display: flex;
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(139, 92, 246, 0.08) 100%);
  backdrop-filter: blur(20rpx);
  padding: 10rpx;
  margin: 20rpx 20rpx 0;
  border-radius: 20rpx;
  box-shadow: 0 8rpx 32rpx rgba(0, 0, 0, 0.3);

  .tab-bg-indicator {
    position: absolute;
    top: 10rpx;
    left: 10rpx;
    width: calc(50% - 10rpx);
    height: calc(100% - 20rpx);
    background: linear-gradient(135deg, rgba(16, 185, 129, 0.3) 0%, rgba(16, 185, 129, 0.2) 100%);
    border-radius: 15rpx;
    transition: transform 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    box-shadow: 0 4rpx 20rpx rgba(16, 185, 129, 0.4);
    z-index: 0;
  }

  .tab-item {
    flex: 1;
    padding: 25rpx 0;
    text-align: center;
    position: relative;
    z-index: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8rpx;

    .tab-icon {
      font-size: 32rpx;
      opacity: 0.6;
      transition: all 0.3s ease;
    }

    .tab-text {
      font-size: 28rpx;
      color: rgba(255, 255, 255, 0.6);
      font-weight: 600;
      transition: all 0.3s ease;
    }

    &.active {
      .tab-icon {
        opacity: 1;
        filter: drop-shadow(0 0 15rpx rgba(16, 185, 129, 0.8));
      }

      .tab-text {
        color: #fff;
        text-shadow: 0 0 20rpx rgba(16, 185, 129, 0.8);
      }
    }
  }
}

// 内容区域
.content {
  position: relative;
  z-index: 10;
  padding: 30rpx 20rpx;
}

// 查看完整按钮
.view-full-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15rpx;
  padding: 25rpx 30rpx;
  margin-bottom: 30rpx;
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.3) 0%, rgba(139, 92, 246, 0.3) 100%);
  border: 2rpx solid rgba(16, 185, 129, 0.5);
  border-radius: 20rpx;
  box-shadow: 
    0 8rpx 32rpx rgba(16, 185, 129, 0.3),
    inset 0 0 30rpx rgba(16, 185, 129, 0.1);
  transition: all 0.3s ease;

  &:active {
    transform: scale(0.98);
    box-shadow: 
      0 4rpx 16rpx rgba(16, 185, 129, 0.4),
      inset 0 0 40rpx rgba(16, 185, 129, 0.2);
  }

  .btn-icon {
    font-size: 36rpx;
  }

  .btn-text {
    font-size: 30rpx;
    color: #fff;
    font-weight: 600;
  }

  .btn-arrow {
    font-size: 40rpx;
    color: rgba(255, 255, 255, 0.8);
    font-weight: bold;
  }
}

// 空状态
.empty {
  text-align: center;
  padding: 180rpx 40rpx;
  
  .empty-icon {
    font-size: 120rpx;
    margin-bottom: 30rpx;
    filter: drop-shadow(0 0 30rpx rgba(16, 185, 129, 0.5));
  }

  .empty-text {
    font-size: 32rpx;
    color: rgba(255, 255, 255, 0.9);
    font-weight: bold;
    display: block;
    margin-bottom: 15rpx;
  }
  
  .empty-desc {
    font-size: 26rpx;
    color: rgba(255, 255, 255, 0.6);
    display: block;
  }
}

// 便签列表
.notes-list {
  .note-item {
    position: relative;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
    backdrop-filter: blur(20rpx);
    padding: 30rpx;
    margin-bottom: 20rpx;
    border-radius: 24rpx;
    border: 2rpx solid rgba(16, 185, 129, 0.2);
    overflow: hidden;
    animation: slideIn 0.5s ease-out forwards;
    opacity: 0;

    .note-glow {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(139, 92, 246, 0.1) 100%);
      filter: blur(20rpx);
      opacity: 0.5;
      pointer-events: none;
    }

    .note-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 15rpx;
      z-index: 1;
      position: relative;

      .note-title {
        font-size: 32rpx;
        font-weight: bold;
        color: #fff;
        display: flex;
        align-items: center;
        gap: 10rpx;

        .pin-icon {
          font-size: 24rpx;
        }
      }

      .note-category {
        padding: 8rpx 16rpx;
        background: linear-gradient(135deg, rgba(16, 185, 129, 0.3) 0%, rgba(16, 185, 129, 0.2) 100%);
        border-radius: 12rpx;
        font-size: 24rpx;
        color: #10B981;
      }
    }

    .note-content {
      font-size: 28rpx;
      color: rgba(255, 255, 255, 0.8);
      line-height: 1.6;
      margin-bottom: 15rpx;
      overflow: hidden;
      text-overflow: ellipsis;
      display: -webkit-box;
      -webkit-line-clamp: 3;
      -webkit-box-orient: vertical;
      z-index: 1;
      position: relative;
    }

    .note-footer {
      display: flex;
      justify-content: space-between;
      align-items: center;
      z-index: 1;
      position: relative;

      .note-tags {
        display: flex;
        gap: 10rpx;
        flex-wrap: wrap;

        .tag {
          padding: 6rpx 12rpx;
          background: linear-gradient(135deg, rgba(139, 92, 246, 0.2) 0%, rgba(139, 92, 246, 0.1) 100%);
          border-radius: 8rpx;
          font-size: 22rpx;
          color: #8B5CF6;
        }
      }

      .note-time {
        font-size: 24rpx;
        color: rgba(255, 255, 255, 0.5);
      }
    }
  }
}

// 统计卡片
.statistics-card {
  position: relative;
  background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
  backdrop-filter: blur(20rpx);
  padding: 30rpx;
  margin-bottom: 30rpx;
  border-radius: 24rpx;
  border: 2rpx solid rgba(16, 185, 129, 0.3);
  overflow: hidden;
  box-shadow: 0 10rpx 40rpx rgba(0, 0, 0, 0.3);

  .stat-glow {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(139, 92, 246, 0.1) 100%);
    filter: blur(20rpx);
    opacity: 0.5;
    pointer-events: none;
  }

  .stat-item {
    display: flex;
    align-items: center;
    gap: 20rpx;
    padding: 20rpx 0;
    border-bottom: 2rpx solid rgba(255, 255, 255, 0.1);
    z-index: 1;
    position: relative;

    &:last-child {
      border-bottom: none;
    }

    .stat-icon {
      font-size: 48rpx;
      width: 80rpx;
      height: 80rpx;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 50%;
      background: linear-gradient(135deg, rgba(16, 185, 129, 0.2) 0%, rgba(16, 185, 129, 0.1) 100%);
    }

    .stat-info {
      flex: 1;

      .stat-label {
        font-size: 26rpx;
        color: rgba(255, 255, 255, 0.6);
        margin-bottom: 8rpx;
      }

      .stat-value {
        font-size: 36rpx;
        font-weight: bold;
        color: #10B981;

        &.negative {
          color: #EF4444;
        }
      }
    }
  }
}

// 记账列表
.accounts-list {
  .account-item {
    position: relative;
    display: flex;
    align-items: center;
    gap: 20rpx;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
    backdrop-filter: blur(20rpx);
    padding: 25rpx;
    margin-bottom: 15rpx;
    border-radius: 20rpx;
    border: 2rpx solid rgba(16, 185, 129, 0.2);
    overflow: hidden;
    animation: slideIn 0.5s ease-out forwards;
    opacity: 0;

    .account-glow {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(139, 92, 246, 0.1) 100%);
      filter: blur(20rpx);
      opacity: 0.5;
      pointer-events: none;
    }

    .account-icon {
      width: 70rpx;
      height: 70rpx;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 36rpx;
      z-index: 1;

      &.income {
        background: linear-gradient(135deg, rgba(16, 185, 129, 0.3) 0%, rgba(16, 185, 129, 0.2) 100%);
      }

      &.expense {
        background: linear-gradient(135deg, rgba(239, 68, 68, 0.3) 0%, rgba(239, 68, 68, 0.2) 100%);
      }
    }

    .account-info {
      flex: 1;
      z-index: 1;

      .account-category {
        font-size: 30rpx;
        font-weight: bold;
        color: #fff;
        margin-bottom: 6rpx;
      }

      .account-desc {
        font-size: 24rpx;
        color: rgba(255, 255, 255, 0.6);
        margin-bottom: 6rpx;
      }

      .account-date {
        font-size: 22rpx;
        color: rgba(255, 255, 255, 0.5);
      }
    }

    .account-amount {
      font-size: 32rpx;
      font-weight: bold;
      z-index: 1;

      &.income {
        color: #10B981;
      }

      &.expense {
        color: #EF4444;
      }
    }
  }
}

@keyframes slideIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
