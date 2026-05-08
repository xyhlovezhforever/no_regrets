<template>
  <view class="account-page">
    <!-- 查看模式提示 -->
    <view v-if="isViewMode && viewedUserInfo" class="view-mode-banner">
      <text class="banner-icon">👁️</text>
      <text class="banner-text">正在查看 {{ viewedUserInfo.nickname }} 的记账（只读模式）</text>
    </view>

    <!-- 汇总卡片 -->
    <view class="summary-card">
      <view class="summary-header">
        <text class="summary-title">💰 账户概览</text>
        <text class="summary-balance">
          结余：¥{{ ((summary.balance || 0) / 100).toFixed(2) }}
        </text>
      </view>
      <view class="summary-row">
        <view class="summary-item income-item">
          <text class="summary-icon">📈</text>
          <view class="summary-detail">
            <text class="summary-label">总收入</text>
            <text class="summary-value">¥{{ ((summary.total_income || 0) / 100).toFixed(2) }}</text>
          </view>
        </view>
        <view class="divider"></view>
        <view class="summary-item expense-item">
          <text class="summary-icon">📉</text>
          <view class="summary-detail">
            <text class="summary-label">总支出</text>
            <text class="summary-value">¥{{ ((summary.total_expense || 0) / 100).toFixed(2) }}</text>
          </view>
        </view>
      </view>
    </view>

    <!-- 账单统计按钮 -->
    <view class="bill-buttons">
      <view class="bill-btn" @click="goToMonthlyBill">
        <text class="bill-icon">📅</text>
        <text class="bill-text">月账单</text>
      </view>
      <view class="bill-btn" @click="goToYearlyBill">
        <text class="bill-icon">📊</text>
        <text class="bill-text">年账单</text>
      </view>
    </view>

    <!-- 调试信息 -->
    <!-- <view class="debug-info" style="padding: 20rpx; background: var(--theme-surface); margin: 20rpx; border-radius: 10rpx;">
      <text style="font-size: 24rpx; color: var(--theme-text-secondary);">调试: records.length={{ records.length }}, sortedRecords.length={{ sortedRecords.length }}</text>
    </view> -->

    <!-- 记录列表 -->
    <view class="record-section" v-if="sortedRecords.length > 0">
      <view class="section-header">
        <text class="section-title">记账记录</text>
        <text class="record-count">共{{ sortedRecords.length }}条</text>
      </view>
      <view class="record-list">
        <view
          v-for="record in sortedRecords"
          :key="record.id"
          class="record-item"
          :class="record.record_type"
          @click="editRecord(record)"
        >
          <view class="record-left">
            <text class="record-icon">{{ record.record_type === 'income' ? '💰' : '💸' }}</text>
            <view class="record-info">
              <text class="record-category">{{ record.category }}</text>
              <text class="record-desc">{{ record.description || '无备注' }}</text>
              <text class="record-date">{{ formatDisplayDate(record.date) }}</text>
            </view>
          </view>
          <text class="record-amount" :class="record.record_type">
            {{ record.record_type === 'income' ? '+' : '-' }}{{ (record.amount / 100).toFixed(2) }}
          </text>
        </view>
      </view>
    </view>

    <!-- 空状态 -->
    <view v-else class="empty-state">
      <text class="empty-icon">📊</text>
      <text class="empty-text">还没有记账记录</text>
      <text class="empty-hint">点击右下角 + 号开始记账</text>
    </view>

    <!-- 添加按钮 -->
    <view v-if="!isViewMode" class="add-btn" @click="addRecord">
      <text class="add-icon">+</text>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { onShow, onLoad } from '@dcloudio/uni-app'
import { navigateTo, showToast } from '@/utils'
import { getAccountRecordsApi, getAccountSummaryApi, type AccountSummary } from '@/api/account'
import { getBoundUserAccountsApi, getBoundUserAccountStatisticsApi } from '@/api/boundData'
import { useViewMode } from '@/composables/useViewMode'

interface AccountRecord {
  id: string
  record_type: 'income' | 'expense'
  category: string
  description: string
  amount: number
  date: string
  created_at: string
}

// 查看模式
const { isViewMode, viewedUserId, viewedUserInfo, canEdit, canCreate, setViewMode, clearViewMode } = useViewMode()

const records = ref<AccountRecord[]>([])
const summary = ref<AccountSummary>({
  total_income: 0,
  total_expense: 0,
  balance: 0
})
const loading = ref(false)
const pageParams = ref<any>({})

const loadRecords = async () => {
  try {
    loading.value = true
    
    // 如果是查看模式，加载绑定用户的数据
    if (isViewMode.value && viewedUserId.value) {
      const boundAccounts = await getBoundUserAccountsApi(viewedUserId.value)
      records.value = boundAccounts.map((record: any) => ({
        id: record.id,
        record_type: record.type_ as 'income' | 'expense',
        amount: Math.round(record.amount * 100), // 转换为分
        category: record.category || '',
        description: record.description || '',
        date: record.date || '',
        created_at: record.created_at || ''
      }))
    } else {
      // 普通模式，加载自己的数据
      const res = await getAccountRecordsApi({ page: 1, page_size: 1000 })
      
      if (Array.isArray(res)) {
        records.value = res.map((record: any) => ({
          id: record.id,
          record_type: record.record_type || 'expense',
          amount: record.amount || 0,
          category: record.category || '',
          description: record.description || '',
          date: record.date || '',
          created_at: record.created_at || ''
        }))
      } else {
        records.value = []
      }
    }
  } catch (error: any) {
    console.error('加载记录失败:', error)
    if (error?.message?.includes('401') || error?.message?.includes('Unauthorized')) {
      uni.showToast({ title: '请先登录', icon: 'none' })
      setTimeout(() => {
        navigateTo('/pages/auth/login')
      }, 1500)
    } else {
      uni.showToast({ title: '加载失败: ' + (error?.message || '未知错误'), icon: 'none' })
    }
  } finally {
    loading.value = false
  }
}

const loadSummary = async () => {
  try {
    // 如果是查看模式，加载绑定用户的统计数据
    if (isViewMode.value && viewedUserId.value) {
      const stats = await getBoundUserAccountStatisticsApi(viewedUserId.value)
      summary.value = {
        total_income: Math.round(stats.total_income * 100), // 转换为分
        total_expense: Math.round(stats.total_expense * 100),
        balance: Math.round(stats.balance * 100)
      }
    } else {
      // 普通模式，加载自己的数据
      const res = await getAccountSummaryApi()
      
      if (res) {
        summary.value = {
          total_income: res.total_income || 0,
          total_expense: res.total_expense || 0,
          balance: res.balance || 0
        }
      } else {
        summary.value = { total_income: 0, total_expense: 0, balance: 0 }
      }
    }
  } catch (error: any) {
    console.error('加载汇总失败:', error)
    uni.showToast({ title: '加载汇总失败: ' + (error?.message || '未知错误'), icon: 'none' })
  }
}

onLoad((options: any) => {
  pageParams.value = options || {}
  
  // 如果传入了查看用户的参数，设置查看模式
  if (options?.viewUserId) {
    const userInfo = options.userInfo ? JSON.parse(decodeURIComponent(options.userInfo)) : null
    setViewMode({
      isViewMode: true,
      viewedUserId: options.viewUserId,
      viewedUserInfo: userInfo,
      appType: 'account',
    })
  } else {
    clearViewMode()
  }
})

onMounted(() => {
  loadRecords()
  loadSummary()
  
  // 监听账本更新事件
  uni.$on('refreshAccountList', () => {
    // console.log('收到账本刷新事件')
    loadRecords()
    loadSummary()
  })
})

onShow(() => {
  // console.log('账本页面显示，刷新数据')
  loadRecords()
  loadSummary()
})

onUnmounted(() => {
  // 移除事件监听
  uni.$off('refreshAccountList')
})

// 按日期排序显示所有记录
const sortedRecords = computed(() => {
  if (!records.value || !Array.isArray(records.value)) {
    // console.warn('records.value 不是数组:', records.value)
    return []
  }
  // console.log('sortedRecords 计算，records.value:', records.value)
  // console.log('records.value.length:', records.value.length)
  const sorted = [...records.value].sort((a, b) => {
    return new Date(b.date).getTime() - new Date(a.date).getTime()
  })
  // console.log('排序后的记录:', sorted)
  return sorted
})

// 格式化显示日期
const formatDisplayDate = (dateStr: string) => {
  const date = new Date(dateStr)
  const today = new Date()
  const yesterday = new Date(today)
  yesterday.setDate(yesterday.getDate() - 1)
  
  const dateOnly = dateStr.split('T')[0]
  const todayStr = today.toISOString().split('T')[0]
  const yesterdayStr = yesterday.toISOString().split('T')[0]
  
  if (dateOnly === todayStr) {
    return '今天'
  } else if (dateOnly === yesterdayStr) {
    return '昨天'
  } else {
    return dateStr
  }
}

const addRecord = () => {
  if (!canCreate.value) {
    showToast('查看模式下不能创建记账记录')
    return
  }
  navigateTo('/pages/self/account-edit')
}

const goToMonthlyBill = () => {
  navigateTo('/pages/self/account-monthly')
}

const goToYearlyBill = () => {
  navigateTo('/pages/self/account-yearly')
}

const editRecord = (record: AccountRecord) => {
  if (!canEdit.value) {
    showToast('查看模式下不能编辑记账记录')
    return
  }
  navigateTo('/pages/self/account-edit', { id: record.id })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.account-page {
  min-height: 100vh;
  @include cyber-page-bg;
  padding: 20rpx 30rpx 150rpx;
  position: relative;
}

// 查看模式横幅
.view-mode-banner {
  @include neon-card;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15rpx;
  padding: 20rpx 30rpx;
  margin-bottom: 20rpx;
  background: linear-gradient(135deg, rgba(255, 152, 0, 0.2) 0%, rgba(255, 193, 7, 0.2) 100%);
  border: 2rpx solid rgba(255, 152, 0, 0.5);
  border-radius: 20rpx;
  box-shadow: 
    0 8rpx 32rpx rgba(255, 152, 0, 0.3),
    inset 0 0 30rpx rgba(255, 152, 0, 0.1);

  .banner-icon {
    font-size: 32rpx;
  }

  .banner-text {
    font-size: 28rpx;
    color: #fff;
    font-weight: 500;
  }
}

.summary-card {
  @include neon-card;
  @include rainbow-border;
  border-radius: 25rpx;
  padding: 40rpx 35rpx;
  margin-bottom: 20rpx;
  animation: fadeInUp 0.6s ease-out;

  .summary-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 30rpx;
    padding-bottom: 20rpx;
    border-bottom: 1rpx solid rgba(255, 255, 255, 0.2);

    .summary-title {
      font-size: 32rpx;
      font-weight: bold;
      @include neon-title(#FFD600);
    }

    .summary-balance {
      font-size: 26rpx;
      @include neon-text(#00FF88);
      background: rgba(30, 36, 66, 0.6);
      padding: 10rpx 20rpx;
      border-radius: 25rpx;
      border: 1rpx solid rgba(0, 255, 136, 0.5);
      box-shadow: 0 0 15rpx rgba(0, 255, 136, 0.3);
    }
  }

  .summary-row {
    display: flex;
    justify-content: space-around;
    align-items: center;

    .divider {
      width: 2rpx;
      height: 80rpx;
      background: rgba(255, 255, 255, 0.2);
    }

    .summary-item {
      flex: 1;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 20rpx;

      .summary-icon {
        font-size: 48rpx;
        filter: drop-shadow(0 0 15rpx rgba(0, 217, 255, 0.6));
        animation: iconPulse 2s ease-in-out infinite;
      }

      .summary-detail {
        display: flex;
        flex-direction: column;
        gap: 8rpx;

        .summary-label {
          font-size: 24rpx;
          @include neon-text(#8B5CF6);
        }

        .summary-value {
          font-size: 36rpx;
          font-weight: bold;
          @include neon-title(#ffffff);
        }
      }
    }
  }
}

.bill-buttons {
  display: flex;
  gap: 20rpx;
  margin-bottom: 30rpx;

  .bill-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12rpx;
    padding: 28rpx;
    @include neon-card;
    border-radius: 16rpx;
    transition: all 0.3s ease;
    animation: fadeInUp 0.6s ease-out 0.1s backwards;

    &:active {
      transform: scale(0.95);
      box-shadow: 0 0 30rpx rgba(0, 217, 255, 0.6);
    }

    .bill-icon {
      font-size: 44rpx;
      filter: drop-shadow(0 0 15rpx rgba(138, 92, 246, 0.6));
    }

    .bill-text {
      font-size: 28rpx;
      font-weight: 600;
      @include neon-text(#ffffff);
    }
  }
}

@keyframes fadeInUp {
  from { opacity: 0; transform: translateY(20rpx); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes iconPulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

.record-section {
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20rpx;
    padding: 0 10rpx;

    .section-title {
      font-size: 30rpx;
      font-weight: bold;
      @include neon-title(#00D9FF);
    }

    .record-count {
      font-size: 24rpx;
      @include neon-text(#8B5CF6);
    }
  }

  .record-list {
    .record-item {
      @include neon-card;
      border-radius: 20rpx;
      padding: 30rpx;
      margin-bottom: 15rpx;
      display: flex;
      align-items: center;
      justify-content: space-between;
      transition: all 0.3s ease;
      animation: fadeInUp 0.4s ease-out backwards;

      &:nth-child(1) { animation-delay: 0.05s; }
      &:nth-child(2) { animation-delay: 0.1s; }
      &:nth-child(3) { animation-delay: 0.15s; }

      &:active {
        transform: scale(0.98);
        box-shadow: 0 0 30rpx rgba(0, 217, 255, 0.5);
      }

      .record-left {
        display: flex;
        align-items: center;
        flex: 1;

        .record-icon {
          font-size: 45rpx;
          margin-right: 20rpx;
        }

        .record-info {
          flex: 1;

          .record-category {
            display: block;
            font-size: 30rpx;
            font-weight: 500;
            @include neon-text(#ffffff);
            margin-bottom: 8rpx;
          }

          .record-desc {
            display: block;
            font-size: 24rpx;
            color: #b8c5d6;
            margin-bottom: 5rpx;
          }

          .record-date {
            display: block;
            font-size: 22rpx;
            @include neon-text(#8B5CF6);
          }
        }
      }

      .record-amount {
        font-size: 34rpx;
        font-weight: bold;

        &.income {
          @include neon-text(#00FF88);
        }

        &.expense {
          @include neon-text(#FF0055);
        }
      }
    }
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 200rpx 0;

  .empty-icon {
    font-size: 140rpx;
    margin-bottom: 30rpx;
    filter: drop-shadow(0 0 30rpx rgba(0, 217, 255, 0.6));
    animation: floatIcon 3s ease-in-out infinite;
  }

  .empty-text {
    font-size: 32rpx;
    @include neon-text(#00D9FF);
    margin-bottom: 15rpx;
    font-weight: 500;
  }

  .empty-hint {
    font-size: 26rpx;
    @include neon-text(#8B5CF6);
  }
}

@keyframes floatIcon {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-15rpx); }
}

.add-btn {
  position: fixed;
  bottom: 100rpx;
  right: 50rpx;
  width: 110rpx;
  height: 110rpx;
  @include glow-button(#00D9FF);
  background: linear-gradient(135deg, rgba(0, 217, 255, 0.8) 0%, rgba(138, 92, 246, 0.8) 100%);
  border-radius: 55rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  animation: float 3s ease-in-out infinite, addPulse 2s ease-in-out infinite;

  .add-icon {
    font-size: 60rpx;
    @include neon-text(#ffffff);
    font-weight: 300;
  }

  &:active {
    transform: scale(0.9);
    animation: none;
  }
}

@keyframes addPulse {
  0%, 100% {
    box-shadow: 
      0 0 30rpx rgba(0, 217, 255, 0.6),
      0 12rpx 35rpx rgba(0, 0, 0, 0.4);
  }
  50% {
    box-shadow: 
      0 0 50rpx rgba(0, 217, 255, 0.9),
      0 15rpx 45rpx rgba(0, 0, 0, 0.5);
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10rpx);
  }
}
</style>

