<template>
  <view class="settings-page">
    <view class="settings-list">
      <view class="settings-item">
        <text class="settings-label">主题模式</text>
        <switch :checked="appStore.theme === 'dark'" @change="toggleTheme" />
      </view>

      <view class="settings-item" @click="clearCache">
        <text class="settings-label">清除缓存</text>
        <text class="settings-value">›</text>
      </view>

      <view class="settings-item" @click="checkUpdate">
        <text class="settings-label">检查更新</text>
        <text class="settings-value">v1.0.0 ›</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { useAppStore } from '@/store'

const appStore = useAppStore()

const toggleTheme = (e: any) => {
  appStore.setTheme(e.detail.value ? 'dark' : 'light')
}

const clearCache = () => {
  uni.showModal({
    title: '提示',
    content: '确定要清除缓存吗？',
    success: (res) => {
      if (res.confirm) {
        uni.showToast({ title: '清除成功', icon: 'success' })
      }
    },
  })
}

const checkUpdate = () => {
  uni.showToast({ title: '已是最新版本', icon: 'success' })
}
</script>

<style lang="scss" scoped>
.settings-page {
  min-height: 100vh;
  background: #f8f9fa;
  padding: 20rpx 30rpx;
}

.settings-list {
  background: #ffffff;
  border-radius: 20rpx;
  overflow: hidden;
}

.settings-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 30rpx;
  border-bottom: 1rpx solid #f0f0f0;

  &:last-child {
    border-bottom: none;
  }

  &:active {
    background: #f8f9fa;
  }

  .settings-label {
    font-size: 28rpx;
    color: #333333;
  }

  .settings-value {
    font-size: 28rpx;
    color: #999999;
  }
}
</style>

