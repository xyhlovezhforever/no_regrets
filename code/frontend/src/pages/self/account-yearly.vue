<template>
  <view class="account-yearly-page">
    <!-- 年份选择器 -->
    <view class="year-selector">
      <button class="selector-btn" @click="prevYear">‹</button>
      <picker mode="date" fields="year" :value="currentYear" @change="onYearChange">
        <view class="year-text">{{ currentYear }}年</view>
      </picker>
      <button class="selector-btn" @click="nextYear">›</button>
    </view>

    <!-- 统计卡片 -->
    <view class="summary-cards">
      <view class="summary-card income">
        <text class="label">全年收入</text>
        <text class="amount">¥{{ (billData.total_income / 100).toFixed(2) }}</text>
      </view>
      <view class="summary-card expense">
        <text class="label">全年支出</text>
        <text class="amount">¥{{ (billData.total_expense / 100).toFixed(2) }}</text>
      </view>
      <view class="summary-card balance">
        <text class="label">全年结余</text>
        <text class="amount">¥{{ (billData.balance / 100).toFixed(2) }}</text>
      </view>
    </view>

    <!-- 每月统计 -->
    <view class="section">
      <view class="section-title">
        <text class="title-text">📊 每月统计</text>
      </view>
      <view v-if="billData.monthly && billData.monthly.length > 0" class="monthly-list">
        <view v-for="item in billData.monthly" :key="item.month" class="monthly-item">
          <view class="monthly-date">{{ item.month }}</view>
          <view class="monthly-amounts">
            <text class="income-text" v-if="item.income > 0">+¥{{ (item.income / 100).toFixed(2) }}</text>
            <text class="expense-text" v-if="item.expense > 0">-¥{{ (item.expense / 100).toFixed(2) }}</text>
          </view>
        </view>
      </view>
      <view v-else class="empty-state">
        <text>本年暂无记账记录</text>
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
        <text>本年暂无分类数据</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getYearlyBillApi } from '@/api/account'

interface MonthlyStat {
  month: string
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
  total_income: number
  total_expense: number
  balance: number
  monthly: MonthlyStat[]
  by_category: CategoryStat[]
}

const currentYear = ref('')
const billData = ref<BillData>({
  year: 0,
  total_income: 0,
  total_expense: 0,
  balance: 0,
  monthly: [],
  by_category: []
})
const loading = ref(false)

onMounted(() => {
  // 初始化为当前年份
  currentYear.value = String(new Date().getFullYear())
  loadBillData()
})

const onYearChange = (e: any) => {
  currentYear.value = e.detail.value
  loadBillData()
}

const prevYear = () => {
  currentYear.value = String(Number(currentYear.value) - 1)
  loadBillData()
}

const nextYear = () => {
  currentYear.value = String(Number(currentYear.value) + 1)
  loadBillData()
}

const loadBillData = async () => {
  const year = Number(currentYear.value)
  
  loading.value = true
  try {
    const data = await getYearlyBillApi(year)
    billData.value = data
  } catch (error) {
    console.error('加载年账单失败:', error)
    uni.showToast({
      title: '加载失败',
      icon: 'none'
    })
    // 重置数据
    billData.value = {
      year,
      total_income: 0,
      total_expense: 0,
      balance: 0,
      monthly: [],
      by_category: []
    }
  } finally {
    loading.value = false
  }
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.account-yearly-page {
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

.year-selector {
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

  .year-text {
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

  .monthly-list {
    .monthly-item {
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

      .monthly-date {
        font-size: 28rpx;
        font-weight: 500;
        @include neon-text(#00D9FF);
      }

      .monthly-amounts {
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

