<template>
  <view class="user-center">
    <!-- 用户信息卡片 -->
    <view class="user-card">
      <view class="user-info">
        <view v-if="userStore.avatar" class="avatar-wrapper">
          <image
            class="avatar"
            :src="getFullUrl(userStore.avatar)"
            mode="aspectFill"
          />
        </view>
        <view v-else class="avatar-wrapper avatar-emoji">
          <text class="avatar-text">👤</text>
        </view>
        <view class="info">
          <text class="nickname">{{ userStore.nickname || userStore.username || '未登录' }}</text>
          <text class="account-label">账号：{{ userStore.username || 'guest' }}</text>
        </view>
        <view class="edit-btn" @click="handleNavigate('/pages/user/profile')">
          <text>✏️</text>
        </view>
      </view>
    </view>

    <!-- 功能列表 -->
    <view class="menu-list">
      <view class="menu-section">
        <view class="menu-item" @click="handleNavigate('/pages/ai/chat-list')">
          <text class="menu-icon">🤖</text>
          <text class="menu-title">我的对话</text>
          <text class="menu-arrow">›</text>
        </view>

        <view class="menu-item" @click="handleNavigate('/pages/release/writing-list')">
          <text class="menu-icon">✍️</text>
          <text class="menu-title">我的作品</text>
          <text class="menu-arrow">›</text>
        </view>

        <view class="menu-item" @click="handleNavigate('/pages/self/note')">
          <text class="menu-icon">📝</text>
          <text class="menu-title">我的便签</text>
          <text class="menu-arrow">›</text>
        </view>

        <view class="menu-item" @click="handleNavigate('/pages/self/account')">
          <text class="menu-icon">💰</text>
          <text class="menu-title">我的记账</text>
          <text class="menu-arrow">›</text>
        </view>
      </view>

      <view class="menu-section">
        <view class="menu-item" @click="handleNavigate('/pages/user/settings')">
          <text class="menu-icon">⚙️</text>
          <text class="menu-title">设置</text>
          <text class="menu-arrow">›</text>
        </view>

        <view class="menu-item" @click="handleAbout">
          <text class="menu-icon">ℹ️</text>
          <text class="menu-title">关于我们</text>
          <text class="menu-arrow">›</text>
        </view>
      </view>

      <view class="menu-section" v-if="userStore.isLoggedIn">
        <view class="menu-item danger" @click="handleLogout">
          <text class="menu-icon">🚪</text>
          <text class="menu-title">退出登录</text>
        </view>
      </view>

      <view class="menu-section" v-else>
        <view class="menu-item primary" @click="goToLogin">
          <text class="menu-icon">🔐</text>
          <text class="menu-title">登录 / 注册</text>
        </view>
      </view>
    </view>
    
    <!-- 底部间距 -->
    <view class="bottom-space"></view>
  </view>
</template>

<script setup lang="ts">
import { onShow } from '@dcloudio/uni-app'
import { useUserStore } from '@/store'
import { showModal, navigateTo } from '@/utils'
import { BASE_API } from '@/config'

const userStore = useUserStore()

// 将相对路径转换为完整URL
const getFullUrl = (url: string | null | undefined) => {
  if (!url) return ''
  if (url.startsWith('http')) return url
  return `${BASE_API.replace('/api/v1', '')}${url}`
}

const handleNavigate = (url: string) => {
  if (!userStore.isLoggedIn && url !== '/pages/auth/login') {
    return goToLogin()
  }
  navigateTo(url)
}

const goToLogin = () => {
  uni.navigateTo({
    url: '/pages/auth/login',
  })
}

const handleAbout = () => {
  uni.showModal({
    title: '关于我们',
    content: '你没有遗憾 v1.0.0\n一个为人们提供情绪价值的应用',
    showCancel: false,
  })
}

const handleLogout = async () => {
  const res = await showModal('提示', '确定要退出登录吗？')
  if (res.confirm) {
    await userStore.logout()
  }
}

onShow(() => {
  // TabBar组件会自动检测当前页面路由
})
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.user-center {
  @include cyber-page-bg;
  min-height: 100vh;
  padding-bottom: 30rpx;
  position: relative;
}

.bottom-space {
  height: 180rpx;
}

.user-card {
  background: rgba(30, 36, 66, 0.8);
  backdrop-filter: blur(20rpx);
  padding: 60rpx 30rpx 40rpx;
  margin-bottom: 20rpx;
  border-radius: 0 0 50rpx 50rpx;
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(0, 217, 255, 0.3),
    inset 0 -2rpx 20rpx rgba(0, 217, 255, 0.2);
  border: 1rpx solid rgba(0, 217, 255, 0.3);
  position: relative;
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
      rgba(0, 217, 255, 0.1) 90deg,
      transparent 180deg,
      rgba(255, 0, 214, 0.1) 270deg,
      transparent 360deg
    );
    animation: rotate 6s linear infinite;
  }

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

  .user-info {
    display: flex;
    align-items: center;
    position: relative;
    z-index: 1;

    .avatar-wrapper {
      width: 120rpx;
      height: 120rpx;
      margin-right: 20rpx;
      flex-shrink: 0;

      .avatar {
        width: 100%;
        height: 100%;
        border-radius: 60rpx;
        border: 4rpx solid rgba(0, 217, 255, 0.6);
        box-shadow: 
          0 0 30rpx rgba(0, 217, 255, 0.6),
          inset 0 0 20rpx rgba(0, 217, 255, 0.1);
        animation: glowPulse 2s ease-in-out infinite;
      }

      &.avatar-emoji {
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(30, 36, 66, 0.6);
        border-radius: 60rpx;
        border: 4rpx solid rgba(255, 214, 0, 0.6);
        box-shadow: 0 0 20rpx rgba(255, 214, 0, 0.5);

        .avatar-text {
          font-size: 60rpx;
        }
      }
    }

    .info {
      flex: 1;

      .nickname {
        display: block;
        font-size: 36rpx;
        font-weight: bold;
        @include neon-title(#ffffff);
        margin-bottom: 8rpx;
      }

      .account-label {
        display: block;
        font-size: 24rpx;
        @include neon-text(#00D9FF);
      }
    }

    .edit-btn {
      width: 60rpx;
      height: 60rpx;
      background: rgba(30, 36, 66, 0.6);
      border-radius: 30rpx;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 32rpx;
      border: 2rpx solid rgba(0, 217, 255, 0.4);
      box-shadow: 0 0 15rpx rgba(0, 217, 255, 0.3);
      transition: all 0.3s ease;

      &:active {
        transform: scale(0.9);
        box-shadow: 0 0 25rpx rgba(0, 217, 255, 0.6);
      }
    }
  }
}

.menu-list {
  padding: 0 30rpx;
  position: relative;
  z-index: 1;
}

.menu-section {
  @include neon-card;
  border-radius: 20rpx;
  margin-bottom: 20rpx;
  overflow: hidden;
  padding: 0;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 30rpx 20rpx;
  border-bottom: 1rpx solid rgba(0, 217, 255, 0.1);
  transition: all 0.3s ease;
  position: relative;

  &:last-child {
    border-bottom: none;
  }

  &:active {
    background: rgba(0, 217, 255, 0.1);
    transform: translateX(10rpx);
    
    .menu-arrow {
      transform: translateX(10rpx);
    }
  }

  .menu-icon {
    font-size: 36rpx;
    margin-right: 20rpx;
    filter: drop-shadow(0 0 10rpx rgba(0, 217, 255, 0.6));
    animation: iconPulse 2s ease-in-out infinite;
  }

  .menu-title {
    flex: 1;
    font-size: 28rpx;
    @include neon-text(#ffffff);
  }

  .menu-arrow {
    font-size: 40rpx;
    color: rgba(0, 217, 255, 0.6);
    transition: all 0.3s ease;
    text-shadow: 0 0 10rpx rgba(0, 217, 255, 0.5);
  }

  &.danger {
    .menu-icon {
      filter: drop-shadow(0 0 10rpx rgba(255, 0, 85, 0.6));
    }
    
    .menu-title {
      color: #FF0055;
      text-shadow: 0 0 10rpx rgba(255, 0, 85, 0.6);
    }
  }

  &.primary {
    .menu-icon {
      filter: drop-shadow(0 0 10rpx rgba(102, 126, 234, 0.8));
    }
    
    .menu-title {
      @include neon-title(#667eea);
      font-weight: bold;
    }
  }
}

@keyframes iconPulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
}
</style>

