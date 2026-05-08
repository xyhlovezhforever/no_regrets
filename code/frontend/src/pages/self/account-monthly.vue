<template>
  <view class="account-monthly-page">
    <!-- 月份选择器 -->
    <view class="month-selector">
      <button class="selector-btn" @click="prevMonth">‹</button>
      <picker mode="date" fields="month" :value="currentMonth" @change="onMonthChange">
        <view class="month-text">{{ formatMonth }}</view>
      </picker>
      <button class="selector-btn" @click="nextMonth">›</button>
    </view>

    <!-- 统计卡片 -->
    <view class="summary-cards">
      <view class="summary-card income">
        <text class="label">本月收入</text>
        <text class="amount">¥{{ (billData.total_income / 100).toFixed(2) }}</text>
      </view>
      <view class="summary-card expense">
        <text class="label">本月支出</text>
        <text class="amount">¥{{ (billData.total_expense / 100).toFixed(2) }}</text>
      </view>
      <view class="summary-card balance">
        <text class="label">本月结余</text>
        <text class="amount">¥{{ (billData.balance / 100).toFixed(2) }}</text>
      </view>
    </view>

    <!-- 每日统计 -->
    <view class="section">
      <view class="section-title">
        <text class="title-text">📊 每日统计</text>
      </view>
      <view v-if="billData.daily && billData.daily.length > 0" class="daily-list">
        <view v-for="item in billData.daily" :key="item.date" class="daily-item">
          <view class="daily-date">{{ item.date }}</view>
          <view class="daily-amounts">
            <text class="income-text" v-if="item.income > 0">+¥{{ (item.income / 100).toFixed(2) }}</text>
            <text class="expense-text" v-if="item.expense > 0">-¥{{ (item.expense / 100).toFixed(2) }}</text>
          </view>
        </view>
      </view>
      <view v-else class="empty-state">
        <text>本月暂无记账记录</text>
      </view>
    </view>

    <!-- 分类统计 -->
    <view class="section">
      <view class="section-title">
        <text class="title-text">🏷️ 分类统计</text>
      </view>
      <view v-if="billData.by_category && billData.by_category.length > 0" class="category-list">
        <view v-for="item in billData.by_category" :key="item.category" class="category-item">
          <view class="category-info">
            <text class="category-name">{{ item.category }}</text>
            <text class="category-type" :class="item.record_type">{{ item.record_type === 'income' ? '收入' : '支出' }}</text>
          </view>
          <text class="category-amount" :class="item.record_type">
            {{ item.record_type === 'income' ? '+' : '-' }}¥{{ (item.total / 100).toFixed(2) }}
          </text>
        </view>
      </view>
      <view v-else class="empty-state">
        <text>本月暂无分类数据</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getMonthlyBillApi } from '@/api/account'

interface DailyStat {
  date: string
  income: number
  expense: number
}

interface CategoryStat {
  record_type: string
  category: string
  total: number
}

interface BillData {
  year: number
  month: number
  total_income: number
  total_expense: number
  balance: number
  daily: DailyStat[]
  by_category: CategoryStat[]
}

const currentMonth = ref('')
const billData = ref<BillData>({
  year: 0,
  month: 0,
  total_income: 0,
  total_expense: 0,
  balance: 0,
  daily: [],
  by_category: []
})
const loading = ref(false)

const formatMonth = computed(() => {
  if (!currentMonth.value) return ''
  const [year, month] = currentMonth.value.split('-')
  return `${year}年${month}月`
})

onMounted(() => {
  // 初始化为当前月份
  const now = new Date()
  currentMonth.value = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`
  loadBillData()
})

const onMonthChange = (e: any) => {
  currentMonth.value = e.detail.value
  loadBillData()
}

const prevMonth = () => {
  const [year, month] = currentMonth.value.split('-').map(Number)
  const date = new Date(year, month - 1, 1)
  date.setMonth(date.getMonth() - 1)
  currentMonth.value = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}`
  loadBillData()
}

const nextMonth = () => {
  const [year, month] = currentMonth.value.split('-').map(Number)
  const date = new Date(year, month - 1, 1)
  date.setMonth(date.getMonth() + 1)
  currentMonth.value = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}`
  loadBillData()
}

const loadBillData = async () => {
  const [year, month] = currentMonth.value.split('-').map(Number)
  
  loading.value = true
  try {
    const data = await getMonthlyBillApi(year, month)
    billData.value = data
  } catch (error) {
    console.error('加载月账单失败:', error)
    uni.showToast({
      title: '加载失败',
      icon: 'none'
    })
    // 重置数据
    billData.value = {
      year,
      month,
      total_income: 0,
      total_expense: 0,
      balance: 0,
      daily: [],
      by_category: []
    }
  } finally {
    loading.value = false
  }
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.account-monthly-page {
  @include cyber-page-bg;
  min-height: 100vh;
  padding: 30rpx;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 30% 20%, rgba(0, 217, 255, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 70% 80%, rgba(255, 214, 0, 0.12) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.month-selector {
  @include neon-card;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 35rpx 30rpx;
  border-radius: 25rpx;
  margin-bottom: 30rpx;
  border: 2rpx solid rgba(0, 217, 255, 0.6);
  position: relative;
  z-index: 1;
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(0, 217, 255, 0.4),
    inset 0 0 50rpx rgba(0, 217, 255, 0.15);
  animation: slideDown 0.6s ease-out;

  .selector-btn {
    width: 70rpx;
    height: 70rpx;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
    border: 2rpx solid rgba(0, 217, 255, 0.6);
    border-radius: 50%;
    font-size: 40rpx;
    @include neon-text(#00D9FF);
    box-shadow: 
      0 6rpx 20rpx rgba(0, 217, 255, 0.4),
      0 0 30rpx rgba(0, 217, 255, 0.3);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

    &::after {
      border: none;
    }
    
    &:active {
      transform: scale(0.9);
      box-shadow: 
        0 4rpx 12rpx rgba(0, 217, 255, 0.6),
        0 0 50rpx rgba(0, 217, 255, 0.5);
    }
  }

  .month-text {
    font-size: 40rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
    animation: titlePulse 3s ease-in-out infinite;
  }
}

.summary-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20rpx;
  margin-bottom: 30rpx;
  position: relative;
  z-index: 1;

  .summary-card {
    @include neon-card;
    padding: 35rpx 20rpx;
    border-radius: 25rpx;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20rpx;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
    
    &::before {
      content: '';
      position: absolute;
      top: -50%;
      left: -50%;
      width: 200%;
      height: 200%;
      animation: bgRotate 20s linear infinite;
      z-index: 0;
      opacity: 0.3;
    }

    .label {
      font-size: 24rpx;
      @include neon-text(#b8c5d6);
      position: relative;
      z-index: 1;
    }

    .amount {
      font-size: 32rpx;
      font-weight: bold;
      position: relative;
      z-index: 1;
    }

    &.income {
      border: 2rpx solid rgba(74, 222, 128, 0.6);
      box-shadow: 
        0 10rpx 40rpx rgba(0, 0, 0, 0.5),
        0 0 50rpx rgba(74, 222, 128, 0.4),
        inset 0 0 40rpx rgba(74, 222, 128, 0.15);
      animation: cardSlideIn 0.6s ease-out 0.1s backwards;
      
      &::before {
        background: conic-gradient(
          from 0deg,
          transparent 0deg,
          rgba(74, 222, 128, 0.15) 90deg,
          transparent 180deg
        );
      }
      
      .amount {
        @include neon-text(#4ADE80);
        text-shadow: 
          0 0 20rpx rgba(74, 222, 128, 1),
          0 0 40rpx rgba(74, 222, 128, 0.8);
      }
      
      &:active {
        transform: translateY(-8rpx);
        box-shadow: 
          0 12rpx 45rpx rgba(0, 0, 0, 0.6),
          0 0 70rpx rgba(74, 222, 128, 0.6),
          inset 0 0 50rpx rgba(74, 222, 128, 0.2);
      }
    }

    &.expense {
      border: 2rpx solid rgba(255, 0, 79, 0.6);
      box-shadow: 
        0 10rpx 40rpx rgba(0, 0, 0, 0.5),
        0 0 50rpx rgba(255, 0, 79, 0.4),
        inset 0 0 40rpx rgba(255, 0, 79, 0.15);
      animation: cardSlideIn 0.6s ease-out 0.2s backwards;
      
      &::before {
        background: conic-gradient(
          from 0deg,
          transparent 0deg,
          rgba(255, 0, 79, 0.15) 90deg,
          transparent 180deg
        );
      }
      
      .amount {
        @include neon-text(#FF004F);
        text-shadow: 
          0 0 20rpx rgba(255, 0, 79, 1),
          0 0 40rpx rgba(255, 0, 79, 0.8);
      }
      
      &:active {
        transform: translateY(-8rpx);
        box-shadow: 
          0 12rpx 45rpx rgba(0, 0, 0, 0.6),
          0 0 70rpx rgba(255, 0, 79, 0.6),
          inset 0 0 50rpx rgba(255, 0, 79, 0.2);
      }
    }

    &.balance {
      border: 2rpx solid rgba(255, 214, 0, 0.6);
      box-shadow: 
        0 10rpx 40rpx rgba(0, 0, 0, 0.5),
        0 0 50rpx rgba(255, 214, 0, 0.4),
        inset 0 0 40rpx rgba(255, 214, 0, 0.15);
      animation: cardSlideIn 0.6s ease-out 0.3s backwards;
      
      &::before {
        background: conic-gradient(
          from 0deg,
          transparent 0deg,
          rgba(255, 214, 0, 0.15) 90deg,
          transparent 180deg
        );
      }
      
      .amount {
        @include neon-text(#FFD600);
        text-shadow: 
          0 0 20rpx rgba(255, 214, 0, 1),
          0 0 40rpx rgba(255, 214, 0, 0.8);
      }
      
      &:active {
        transform: translateY(-8rpx);
        box-shadow: 
          0 12rpx 45rpx rgba(0, 0, 0, 0.6),
          0 0 70rpx rgba(255, 214, 0, 0.6),
          inset 0 0 50rpx rgba(255, 214, 0, 0.2);
      }
    }
  }
}

.section {
  @include neon-card;
  border-radius: 25rpx;
  padding: 35rpx;
  margin-bottom: 30rpx;
  border: 2rpx solid rgba(138, 92, 246, 0.5);
  position: relative;
  z-index: 1;
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(138, 92, 246, 0.3),
    inset 0 0 50rpx rgba(138, 92, 246, 0.1);
  animation: slideUp 0.6s ease-out 0.4s backwards;

  .section-title {
    margin-bottom: 30rpx;
    padding-bottom: 20rpx;
    border-bottom: 2rpx solid rgba(138, 92, 246, 0.3);
    position: relative;
    
    &::after {
      content: '';
      position: absolute;
      bottom: -2rpx;
      left: 0;
      width: 100rpx;
      height: 2rpx;
      background: linear-gradient(90deg, #8B5CF6, transparent);
      box-shadow: 0 0 15rpx rgba(138, 92, 246, 0.8);
    }

    .title-text {
      font-size: 32rpx;
      font-weight: bold;
      @include neon-title(#8B5CF6);
    }
  }

  .daily-list {
    .daily-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 25rpx 20rpx;
      margin-bottom: 15rpx;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.4) 0%, rgba(30, 36, 66, 0.2) 100%);
      backdrop-filter: blur(10rpx);
      border-radius: 18rpx;
      border: 2rpx solid rgba(0, 217, 255, 0.3);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      animation: itemSlideIn 0.5s ease-out backwards;

      &:last-child {
        margin-bottom: 0;
      }

      .daily-date {
        font-size: 28rpx;
        font-weight: 500;
        @include neon-text(#00D9FF);
      }

      .daily-amounts {
        display: flex;
        gap: 25rpx;

        .income-text {
          font-size: 28rpx;
          font-weight: bold;
          @include neon-text(#4ADE80);
          text-shadow: 
            0 0 15rpx rgba(74, 222, 128, 0.8),
            0 0 30rpx rgba(74, 222, 128, 0.5);
        }

        .expense-text {
          font-size: 28rpx;
          font-weight: bold;
          @include neon-text(#FF004F);
          text-shadow: 
            0 0 15rpx rgba(255, 0, 79, 0.8),
            0 0 30rpx rgba(255, 0, 79, 0.5);
        }
      }
      
      &:hover,
      &:active {
        transform: translateX(10rpx);
        border-color: rgba(0, 217, 255, 0.6);
        box-shadow: 
          0 6rpx 20rpx rgba(0, 0, 0, 0.4),
          0 0 30rpx rgba(0, 217, 255, 0.4);
      }
    }
  }

  .category-list {
    .category-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 25rpx 20rpx;
      margin-bottom: 15rpx;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.4) 0%, rgba(30, 36, 66, 0.2) 100%);
      backdrop-filter: blur(10rpx);
      border-radius: 18rpx;
      border: 2rpx solid rgba(138, 92, 246, 0.3);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      animation: itemSlideIn 0.5s ease-out backwards;

      &:last-child {
        margin-bottom: 0;
      }

      .category-info {
        display: flex;
        align-items: center;
        gap: 20rpx;

        .category-name {
          font-size: 30rpx;
          font-weight: 500;
          @include neon-text(#ffffff);
        }

        .category-type {
          font-size: 22rpx;
          padding: 6rpx 15rpx;
          border-radius: 12rpx;
          font-weight: 500;
          
          &.income {
            background: linear-gradient(135deg, rgba(74, 222, 128, 0.3) 0%, rgba(74, 222, 128, 0.2) 100%);
            @include neon-text(#4ADE80);
            border: 1rpx solid rgba(74, 222, 128, 0.5);
          }
          
          &.expense {
            background: linear-gradient(135deg, rgba(255, 0, 79, 0.3) 0%, rgba(255, 0, 79, 0.2) 100%);
            @include neon-text(#FF004F);
            border: 1rpx solid rgba(255, 0, 79, 0.5);
          }
        }
      }

      .category-amount {
        font-size: 30rpx;
        font-weight: bold;

        &.income {
          @include neon-text(#4ADE80);
          text-shadow: 
            0 0 15rpx rgba(74, 222, 128, 0.8),
            0 0 30rpx rgba(74, 222, 128, 0.5);
        }

        &.expense {
          @include neon-text(#FF004F);
          text-shadow: 
            0 0 15rpx rgba(255, 0, 79, 0.8),
            0 0 30rpx rgba(255, 0, 79, 0.5);
        }
      }
      
      &:hover,
      &:active {
        transform: translateX(10rpx);
        border-color: rgba(138, 92, 246, 0.6);
        box-shadow: 
          0 6rpx 20rpx rgba(0, 0, 0, 0.4),
          0 0 30rpx rgba(138, 92, 246, 0.4);
      }
    }
  }

  .empty-state {
    padding: 80rpx 0;
    text-align: center;
    font-size: 30rpx;
    @include neon-text(#8B5CF6);
    animation: emptyFloat 3s ease-in-out infinite;
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes bgRotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-40rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(40rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes cardSlideIn {
  from {
    opacity: 0;
    transform: translateY(30rpx) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 20rpx rgba(255, 214, 0, 1),
      0 0 40rpx rgba(255, 214, 0, 0.8);
  }
  50% {
    text-shadow: 
      0 0 30rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(255, 214, 0, 1),
      0 0 90rpx rgba(255, 214, 0, 0.6);
  }
}

@keyframes itemSlideIn {
  from {
    opacity: 0;
    transform: translateX(-30rpx);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes emptyFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-15rpx);
  }
}
</style>

