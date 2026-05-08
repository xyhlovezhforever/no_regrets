<template>
  <view class="chat-list-page">
    <!-- 新建对话按钮 -->
    <view class="header">
      <view class="create-btn" @click="createSession">
        <text>➕ 新建对话</text>
      </view>
      <view class="create-btn phone-btn" @click="makePhoneCall">
        <text>📞 语音通话</text>
      </view>
      <view class="create-btn custom-btn" @click="createCustomAI">
        <text>✨ 创建专属AI</text>
      </view>
    </view>

    <!-- 会话列表 -->
    <view class="session-list" v-if="chatList.length > 0">
      <view
        v-for="chat in chatList"
        :key="chat.id"
        class="session-item"
        @click="openChat(chat.id)"
        @longpress="deleteChat(chat.id)"
      >
        <view class="session-avatar">
          <text>🤖</text>
        </view>
        <view class="session-info">
          <text class="session-title">AI 陪伴</text>
          <text class="session-message">{{ chat.lastMessage }}</text>
        </view>
        <view class="session-meta">
          <text class="session-time">{{ formatTime(chat.updatedAt) }}</text>
          <text class="session-arrow">›</text>
        </view>
      </view>
    </view>

    <!-- 空状态 -->
    <view v-else class="empty-state">
      <text class="empty-icon">💬</text>
      <text class="empty-text">还没有对话</text>
      <text class="empty-hint">开始你的第一次倾诉吧</text>
      <button class="empty-btn" @click="createSession">创建对话</button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { getStorage, setStorage } from '@/utils/storage'
import { navigateTo } from '@/utils'

interface ChatItem {
  id: string
  lastMessage: string
  updatedAt: number
}

const chatList = ref<ChatItem[]>([])

onMounted(() => {
  loadChatList()
})

onShow(() => {
  loadChatList()
})

const loadChatList = () => {
  const stored = getStorage<ChatItem[]>('aiChatList', [])
  chatList.value = stored.sort((a, b) => b.updatedAt - a.updatedAt)
}

const createSession = () => {
  navigateTo('/pages/ai/chat')
}

const createCustomAI = () => {
  navigateTo('/pages/ai/custom-ai')
}

const makePhoneCall = () => {
  navigateTo('/pages/ai/phone-call')
}

const openChat = (id: string) => {
  navigateTo('/pages/ai/chat', { id })
}

const formatTime = (timestamp: number): string => {
  const now = Date.now()
  const diff = now - timestamp
  
  const minute = 60 * 1000
  const hour = 60 * minute
  const day = 24 * hour
  
  if (diff < minute) {
    return '刚刚'
  } else if (diff < hour) {
    return `${Math.floor(diff / minute)}分钟前`
  } else if (diff < day) {
    return `${Math.floor(diff / hour)}小时前`
  } else if (diff < 7 * day) {
    return `${Math.floor(diff / day)}天前`
  } else {
    const date = new Date(timestamp)
    return `${date.getMonth() + 1}月${date.getDate()}日`
  }
}

// 长按删除聊天
const deleteChat = (id: string) => {
  uni.showModal({
    title: '确认删除',
    content: '确定要删除这个对话吗？',
    success: (res) => {
      if (res.confirm) {
        // 删除聊天记录
        const chats = getStorage<any>('aiChats', {})
        delete chats[id]
        setStorage('aiChats', chats)
        
        // 更新列表
        chatList.value = chatList.value.filter(c => c.id !== id)
        setStorage('aiChatList', chatList.value)
        
        uni.showToast({ title: '已删除', icon: 'success' })
      }
    }
  })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.chat-list-page {
  min-height: 100vh;
  @include cyber-page-bg;
  position: relative;
  display: flex;
  flex-direction: column;
}

.header {
  padding: 25rpx 30rpx;
  background: linear-gradient(180deg, rgba(30, 36, 66, 0.95) 0%, rgba(30, 36, 66, 0.85) 100%);
  backdrop-filter: blur(30rpx);
  display: flex;
  gap: 15rpx;
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.6),
    0 0 40rpx rgba(0, 217, 255, 0.25),
    inset 0 1rpx 0 rgba(255, 255, 255, 0.1),
    inset 0 -1rpx 0 rgba(0, 217, 255, 0.4);
  border-bottom: 2rpx solid rgba(0, 217, 255, 0.4);
  position: sticky;
  top: 0;
  z-index: 10;

  .create-btn {
    flex: 1;
    padding: 25rpx 20rpx;
    border-radius: 20rpx;
    text-align: center;
    font-size: 26rpx;
    font-weight: bold;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(79, 172, 254, 0.9) 100%);
    border: 2rpx solid rgba(0, 217, 255, 0.6);
    box-shadow: 
      0 8rpx 24rpx rgba(0, 217, 255, 0.4),
      0 0 40rpx rgba(0, 217, 255, 0.3),
      inset 0 0 30rpx rgba(0, 217, 255, 0.1);
    animation: btnFloat 3s ease-in-out infinite;
    
    text {
      position: relative;
      z-index: 2;
      @include neon-text(#ffffff);
      text-shadow: 
        0 0 10rpx rgba(0, 217, 255, 0.8),
        0 2rpx 4rpx rgba(0, 0, 0, 0.5);
    }
    
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
        transparent 180deg,
        rgba(255, 255, 255, 0.3) 270deg,
        transparent 360deg
      );
      animation: rotate 4s linear infinite;
      z-index: 1;
    }
    
    &:active {
      transform: scale(0.96) translateY(2rpx);
      box-shadow: 
        0 4rpx 12rpx rgba(0, 217, 255, 0.6),
        0 0 30rpx rgba(0, 217, 255, 0.5),
        inset 0 0 40rpx rgba(0, 217, 255, 0.2);
    }
  }

  .phone-btn {
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(245, 87, 108, 0.9) 100%);
    border: 2rpx solid rgba(255, 0, 214, 0.6);
    box-shadow: 
      0 8rpx 24rpx rgba(255, 0, 214, 0.4),
      0 0 40rpx rgba(255, 0, 214, 0.3),
      inset 0 0 30rpx rgba(255, 0, 214, 0.1);
    animation: btnFloat 3s ease-in-out infinite 0.5s;
    
    text {
      text-shadow: 
        0 0 10rpx rgba(255, 0, 214, 0.8),
        0 2rpx 4rpx rgba(0, 0, 0, 0.5);
    }
    
    &:active {
      box-shadow: 
        0 4rpx 12rpx rgba(255, 0, 214, 0.6),
        0 0 30rpx rgba(255, 0, 214, 0.5),
        inset 0 0 40rpx rgba(255, 0, 214, 0.2);
    }
  }

  .custom-btn {
    background: linear-gradient(135deg, rgba(255, 214, 0, 0.9) 0%, rgba(254, 225, 64, 0.9) 100%);
    border: 2rpx solid rgba(255, 214, 0, 0.6);
    box-shadow: 
      0 8rpx 24rpx rgba(255, 214, 0, 0.4),
      0 0 40rpx rgba(255, 214, 0, 0.3),
      inset 0 0 30rpx rgba(255, 214, 0, 0.1);
    animation: btnFloat 3s ease-in-out infinite 1s;
    
    text {
      text-shadow: 
        0 0 10rpx rgba(255, 214, 0, 0.8),
        0 2rpx 4rpx rgba(0, 0, 0, 0.5);
    }
    
    &:active {
      box-shadow: 
        0 4rpx 12rpx rgba(255, 214, 0, 0.6),
        0 0 30rpx rgba(255, 214, 0, 0.5),
        inset 0 0 40rpx rgba(255, 214, 0, 0.2);
    }
  }
}

@keyframes btnFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-4rpx);
  }
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.session-list {
  flex: 1;
  padding: 20rpx 30rpx;
  position: relative;
  z-index: 1;
}

.session-item {
  @include neon-card;
  border-radius: 15rpx;
  padding: 25rpx;
  margin-bottom: 15rpx;
  display: flex;
  align-items: center;
  animation: fadeInRight 0.5s ease-out backwards;
  position: relative;
  
  &:nth-child(odd) {
    animation-delay: 0.1s;
  }
  
  &:nth-child(even) {
    animation-delay: 0.2s;
  }

  &:active {
    transform: translateX(10rpx);
    box-shadow: 
      0 15rpx 50rpx rgba(0, 0, 0, 0.6),
      0 0 40rpx rgba(0, 217, 255, 0.5),
      inset 0 0 40rpx rgba(0, 217, 255, 0.15);
  }

  .session-avatar {
    width: 80rpx;
    height: 80rpx;
    border-radius: 40rpx;
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 40rpx;
    margin-right: 20rpx;
    flex-shrink: 0;
    border: 2rpx solid rgba(0, 217, 255, 0.6);
    box-shadow: 
      0 0 20rpx rgba(0, 217, 255, 0.5),
      inset 0 0 20rpx rgba(0, 217, 255, 0.1);
    animation: avatarGlow 2s ease-in-out infinite;
  }

  .session-info {
    flex: 1;
    min-width: 0;

    .session-title {
      display: block;
      font-size: 30rpx;
      @include neon-title(#ffffff);
      font-weight: bold;
      margin-bottom: 8rpx;
    }

    .session-message {
      display: block;
      font-size: 26rpx;
      color: #b8c5d6;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      text-shadow: 0 0 5rpx rgba(184, 197, 214, 0.3);
    }
  }

  .session-meta {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    margin-left: 20rpx;

    .session-time {
      font-size: 22rpx;
      @include neon-text(#6b7b93);
      margin-bottom: 5rpx;
    }

    .session-arrow {
      font-size: 40rpx;
      color: rgba(0, 217, 255, 0.6);
      text-shadow: 0 0 10rpx rgba(0, 217, 255, 0.5);
      transition: all 0.3s ease;
    }
  }
  
  &:active .session-arrow {
    transform: translateX(10rpx);
    color: rgba(0, 217, 255, 1);
    text-shadow: 0 0 20rpx rgba(0, 217, 255, 0.8);
  }
}

@keyframes fadeInRight {
  from {
    opacity: 0;
    transform: translateX(-30rpx);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes avatarGlow {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(0, 217, 255, 0.5),
      inset 0 0 20rpx rgba(0, 217, 255, 0.1);
  }
  50% {
    box-shadow: 
      0 0 30rpx rgba(0, 217, 255, 0.8),
      inset 0 0 30rpx rgba(0, 217, 255, 0.2);
  }
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60rpx 30rpx;
  position: relative;
  z-index: 1;

  .empty-icon {
    font-size: 120rpx;
    margin-bottom: 30rpx;
    filter: drop-shadow(0 0 30rpx rgba(0, 217, 255, 0.8));
    animation: float 3s ease-in-out infinite;
  }

  .empty-text {
    font-size: 32rpx;
    @include neon-title(#ffffff);
    margin-bottom: 10rpx;
  }

  .empty-hint {
    font-size: 26rpx;
    color: #b8c5d6;
    margin-bottom: 50rpx;
    text-shadow: 0 0 8rpx rgba(184, 197, 214, 0.5);
  }

  .empty-btn {
    width: 300rpx;
    height: 80rpx;
    border-radius: 50rpx;
    border: none;
    font-size: 30rpx;
    font-weight: bold;
    line-height: 80rpx;
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    border: 2rpx solid rgba(0, 217, 255, 0.6);
    box-shadow: 
      0 10rpx 30rpx rgba(0, 217, 255, 0.5),
      0 0 50rpx rgba(0, 217, 255, 0.4),
      inset 0 0 40rpx rgba(0, 217, 255, 0.15);
    @include neon-text(#ffffff);
    text-shadow: 
      0 0 15rpx rgba(0, 217, 255, 1),
      0 2rpx 6rpx rgba(0, 0, 0, 0.6);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
    animation: emptyBtnPulse 3s ease-in-out infinite;
    
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
        rgba(255, 255, 255, 0.4) 90deg,
        transparent 180deg,
        rgba(255, 255, 255, 0.4) 270deg,
        transparent 360deg
      );
      animation: rotate 5s linear infinite;
    }
    
    &::after {
      border: none;
    }
    
    &:active {
      transform: scale(0.95) translateY(2rpx);
      box-shadow: 
        0 5rpx 20rpx rgba(0, 217, 255, 0.7),
        0 0 40rpx rgba(0, 217, 255, 0.6),
        inset 0 0 50rpx rgba(0, 217, 255, 0.25);
    }
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-20rpx) rotate(5deg);
  }
}

@keyframes emptyBtnPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 30rpx rgba(0, 217, 255, 0.5),
      0 0 50rpx rgba(0, 217, 255, 0.4),
      inset 0 0 40rpx rgba(0, 217, 255, 0.15);
  }
  50% {
    box-shadow: 
      0 12rpx 40rpx rgba(0, 217, 255, 0.7),
      0 0 60rpx rgba(0, 217, 255, 0.6),
      inset 0 0 50rpx rgba(0, 217, 255, 0.25);
  }
}
</style>

