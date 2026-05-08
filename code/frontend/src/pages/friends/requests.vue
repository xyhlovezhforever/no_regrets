<template>
  <view class="requests-page">
    <view v-if="requests.length > 0" class="requests-list">
      <view
        v-for="request in requests"
        :key="request.id"
        class="request-item"
      >
        <view v-if="request.avatar" class="request-avatar-wrapper">
          <image class="request-avatar" :src="request.avatar" mode="aspectFill" />
        </view>
        <view v-else class="request-avatar-wrapper request-avatar-emoji">
          <text>👤</text>
        </view>
        <view class="request-info">
          <text class="request-name">{{ request.name }}</text>
          <text class="request-message">{{ request.message || '请求添加你为好友' }}</text>
        </view>
        <view class="request-actions">
          <button class="btn-reject" @click="rejectRequest(request)">拒绝</button>
          <button class="btn-accept" @click="acceptRequest(request)">接受</button>
        </view>
      </view>
    </view>

    <view v-else class="empty-state">
      <text class="empty-icon">📭</text>
      <text class="empty-text">暂无好友申请</text>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { showToast } from '@/utils'
import { getFriendRequestsApi, handleFriendRequestApi, type FriendRequest as ApiFriendRequest } from '@/api/friend'

interface FriendRequest {
  id: string
  name: string
  avatar: string
  message?: string
  userId: string
}

const requests = ref<FriendRequest[]>([])
const loading = ref(false)

onMounted(() => {
  loadRequests()
})

onShow(() => {
  loadRequests()
})

const loadRequests = async () => {
  try {
    loading.value = true
    const data = await getFriendRequestsApi()
    
    // 转换为页面需要的格式，只显示待处理的申请
    requests.value = (data as any[])
      .filter((r: any) => r.status === 'pending')
      .map((r: any) => ({
        id: r.id,
        name: r.from_nickname || r.from_username || '用户',
        avatar: r.from_avatar || '',
        message: r.message || '请求添加你为好友',
        userId: r.from_user_id
      }))
  } catch (error: any) {
    // console.error('加载好友申请失败:', error)
    showToast(error.message || '加载失败', 'none')
  } finally {
    loading.value = false
  }
}

const acceptRequest = async (request: FriendRequest) => {
  try {
    await handleFriendRequestApi(request.id, {
      friendship_id: request.id,
      action: 'accept'
    })
    
    showToast('已添加为好友', 'success')
    
    // 刷新列表
    loadRequests()
  } catch (error: any) {
    // console.error('接受好友申请失败:', error)
    showToast(error.message || '操作失败', 'none')
  }
}

const rejectRequest = (request: FriendRequest) => {
  uni.showModal({
    title: '确认操作',
    content: '确定要拒绝此好友申请吗？',
    success: async (res) => {
      if (res.confirm) {
        try {
          await handleFriendRequestApi(request.id, {
            friendship_id: request.id,
            action: 'reject'
          })
          
          showToast('已拒绝', 'success')
          
          // 刷新列表
          loadRequests()
        } catch (error: any) {
          // console.error('拒绝好友申请失败:', error)
          showToast(error.message || '操作失败', 'none')
        }
      }
    }
  })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.requests-page {
  min-height: 100vh;
  @include cyber-page-bg;
  position: relative;
  display: flex;
  flex-direction: column;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 25% 30%, rgba(255, 0, 214, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 75% 70%, rgba(0, 217, 255, 0.12) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
  }
}

.requests-list {
  flex: 1;
  padding: 30rpx;
  position: relative;
  z-index: 1;
  
  .request-item {
    display: flex;
    align-items: center;
    padding: 30rpx;
    @include neon-card;
    border-radius: 25rpx;
    border: 2rpx solid rgba(255, 0, 214, 0.5);
    box-shadow: 
      0 10rpx 40rpx rgba(0, 0, 0, 0.5),
      0 0 40rpx rgba(255, 0, 214, 0.3),
      inset 0 0 40rpx rgba(255, 0, 214, 0.1);
    margin-bottom: 20rpx;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    animation: itemSlideIn 0.5s ease-out backwards;
    
    &:nth-child(1) { animation-delay: 0.1s; }
    &:nth-child(2) { animation-delay: 0.2s; }
    &:nth-child(3) { animation-delay: 0.3s; }
    &:nth-child(4) { animation-delay: 0.4s; }
    &:nth-child(5) { animation-delay: 0.5s; }
    
    &:last-child {
      margin-bottom: 0;
    }
    
    &:active {
      transform: translateY(2rpx);
      box-shadow: 
        0 6rpx 24rpx rgba(0, 0, 0, 0.6),
        0 0 30rpx rgba(255, 0, 214, 0.4),
        inset 0 0 30rpx rgba(255, 0, 214, 0.15);
    }

    .request-avatar-wrapper {
      width: 100rpx;
      height: 100rpx;
      margin-right: 20rpx;
      flex-shrink: 0;

      .request-avatar {
        width: 100%;
        height: 100%;
        border-radius: 50rpx;
        border: 2rpx solid rgba(255, 0, 214, 0.6);
        box-shadow: 
          0 0 20rpx rgba(255, 0, 214, 0.5),
          inset 0 0 20rpx rgba(255, 0, 214, 0.1);
      }

      &.request-avatar-emoji {
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(135deg, rgba(255, 0, 214, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
        border: 2rpx solid rgba(255, 0, 214, 0.6);
        border-radius: 50rpx;
        font-size: 56rpx;
        box-shadow: 
          0 0 20rpx rgba(255, 0, 214, 0.5),
          inset 0 0 20rpx rgba(255, 0, 214, 0.1);
        animation: avatarPulse 2s ease-in-out infinite;
      }
    }

    .request-info {
      flex: 1;
      min-width: 0;

      .request-name {
        display: block;
        font-size: 32rpx;
        @include neon-text(#ffffff);
        font-weight: bold;
        margin-bottom: 10rpx;
      }

      .request-message {
        display: block;
        font-size: 26rpx;
        @include neon-text(#FF00D6);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }

    .request-actions {
      display: flex;
      gap: 15rpx;

      .btn-reject,
      .btn-accept {
        width: 110rpx;
        height: 65rpx;
        border-radius: 33rpx;
        font-size: 26rpx;
        font-weight: bold;
        line-height: 65rpx;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        position: relative;
        overflow: hidden;
        
        &::after {
          border: none;
        }
      }

      .btn-reject {
        background: linear-gradient(135deg, rgba(30, 36, 66, 0.8) 0%, rgba(30, 36, 66, 0.6) 100%);
        @include neon-text(#6b7b93);
        border: 2rpx solid rgba(138, 92, 246, 0.5);
        box-shadow: 
          0 4rpx 16rpx rgba(0, 0, 0, 0.3),
          0 0 20rpx rgba(138, 92, 246, 0.2);
        
        &:active {
          transform: scale(0.92) translateY(2rpx);
          @include neon-text(#FF00D6);
          border-color: rgba(255, 0, 214, 0.7);
          box-shadow: 
            0 2rpx 10rpx rgba(255, 0, 214, 0.4),
            0 0 30rpx rgba(255, 0, 214, 0.3);
        }
      }

      .btn-accept {
        background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
        @include neon-text(#ffffff);
        border: 2rpx solid rgba(0, 217, 255, 0.7);
        box-shadow: 
          0 8rpx 24rpx rgba(0, 217, 255, 0.4),
          0 0 40rpx rgba(0, 217, 255, 0.3),
          inset 0 0 30rpx rgba(0, 217, 255, 0.15);
        
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
            rgba(255, 255, 255, 0.3) 90deg,
            transparent 180deg
          );
          animation: rotate 3s linear infinite;
        }
        
        &:active {
          transform: scale(0.92) translateY(2rpx);
          box-shadow: 
            0 4rpx 16rpx rgba(0, 217, 255, 0.6),
            0 0 50rpx rgba(0, 217, 255, 0.5),
            inset 0 0 40rpx rgba(0, 217, 255, 0.25);
        }
      }
    }
  }
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 100rpx 30rpx;
  position: relative;
  z-index: 1;

  .empty-icon {
    font-size: 140rpx;
    margin-bottom: 40rpx;
    filter: drop-shadow(0 0 40rpx rgba(138, 92, 246, 0.8));
    animation: emptyFloat 3s ease-in-out infinite;
  }

  .empty-text {
    font-size: 36rpx;
    font-weight: 500;
    @include neon-title(#8B5CF6);
    animation: textPulse 2s ease-in-out infinite;
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 0.8; }
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

@keyframes avatarPulse {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(255, 0, 214, 0.5),
      inset 0 0 20rpx rgba(255, 0, 214, 0.1);
    transform: scale(1);
  }
  50% {
    box-shadow: 
      0 0 30rpx rgba(255, 0, 214, 0.8),
      inset 0 0 30rpx rgba(255, 0, 214, 0.2);
    transform: scale(1.05);
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes emptyFloat {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-20rpx) rotate(5deg);
  }
}

@keyframes textPulse {
  0%, 100% {
    text-shadow: 
      0 0 15rpx currentColor,
      0 0 30rpx currentColor;
    opacity: 0.8;
  }
  50% {
    text-shadow: 
      0 0 25rpx currentColor,
      0 0 50rpx currentColor;
    opacity: 1;
  }
}
</style>

