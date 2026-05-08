<template>
  <view class="friends-page">
    <!-- 顶部操作栏 -->
    <view class="header-actions">
      <view class="action-btn" @click="addFriend">
        <text class="action-icon">👥</text>
        <text class="action-text">添加好友</text>
      </view>
      <view class="action-btn" @click="viewRequests">
        <text class="action-icon">📬</text>
        <text class="action-text">好友申请</text>
        <text v-if="requestCount > 0" class="badge">{{ requestCount }}</text>
      </view>
    </view>

    <!-- 好友列表 -->
    <view class="friends-list" v-if="friends.length > 0">
      <view class="section-title">好友列表 ({{ friends.length }})</view>
      <view
        v-for="friend in friends"
        :key="friend.id"
        class="friend-item"
        @click="() => openChat(friend)"
      >
        <view v-if="friend.avatar" class="friend-avatar-wrapper">
          <image class="friend-avatar" :src="friend.avatar" mode="aspectFill" />
        </view>
        <view v-else class="friend-avatar-wrapper friend-avatar-emoji">
          <text>👤</text>
        </view>
        <view class="friend-info">
          <text class="friend-name">{{ friend.name }}</text>
          <text class="friend-status" :class="{ online: friend.online }">{{ friend.online ? '在线' : '离线' }}</text>
        </view>
        <view class="friend-meta">
          <text class="friend-time" v-if="friend.lastTime">{{ formatTime(friend.lastTime) }}</text>
          <text class="unread-badge" v-if="friend.unreadCount > 0">{{ friend.unreadCount }}</text>
        </view>
      </view>
    </view>

    <!-- 空状态 -->
    <view v-else class="empty-state">
      <text class="empty-icon">👋</text>
      <text class="empty-text">还没有好友</text>
      <text class="empty-hint">点击上方"添加好友"开始添加吧</text>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { navigateTo, showToast } from '@/utils'
import { getFriendsApi, getFriendRequestsApi, type Friend as ApiFriend, type FriendRequest } from '@/api/friend'
import { getWebSocketClient, type WsMessage } from '@/utils/websocket'
import { useUserStore } from '@/store'

interface Friend {
  id: string
  name: string
  avatar: string
  online: boolean
  lastMessage?: string
  lastTime?: number
  unreadCount?: number
}

const friends = ref<Friend[]>([])
const friendRequests = ref<FriendRequest[]>([])
const loading = ref(false)
const wsClient = getWebSocketClient()
const userStore = useUserStore()

const requestCount = computed(() => friendRequests.value.filter(r => r.status === 'pending').length)

// WebSocket 消息处理：收到新消息时更新未读数量
const handleWsMessage = (message: WsMessage) => {
  if (message.type === 'message') {
    const msg = message as any
    const msgFromId = String(msg.from_user_id || '')
    const msgToId = String(msg.to_user_id || '')
    const currentUserId = String(userStore.userId || '')
    
    // 检查是否是发送给我的消息（不是我自己发送的）
    if (msgToId === currentUserId && msgFromId !== currentUserId) {
      // console.log('[FriendsList] 收到新消息，更新未读数量:', {
      //   from_user_id: msgFromId,
      //   to_user_id: msgToId
      // })
      
      // 找到对应的好友，增加未读数量
      const friendIndex = friends.value.findIndex(f => f.id === msgFromId)
      if (friendIndex !== -1) {
        // 增加未读数量
        friends.value[friendIndex].unreadCount = (friends.value[friendIndex].unreadCount || 0) + 1
        // 更新最后消息时间
        friends.value[friendIndex].lastTime = new Date(msg.created_at).getTime()
        
        // 重新排序（未读数多的在前）
        friends.value.sort((a, b) => {
          if (b.unreadCount !== a.unreadCount) {
            return (b.unreadCount || 0) - (a.unreadCount || 0)
          }
          return (b.lastTime || 0) - (a.lastTime || 0)
        })
      } else {
        // 如果好友不在列表中，刷新整个列表（可能是新好友）
        loadFriends()
      }
    }
  }
}

onMounted(() => {
  loadFriends()
  loadRequests()
  
  // 注册 WebSocket 消息处理器
  wsClient.onMessage(handleWsMessage)
  
  // 确保 WebSocket 已连接
  if (!wsClient.isConnected()) {
    wsClient.connect()
  }
})

onShow(() => {
  loadFriends()
  loadRequests()
  
  // 确保 WebSocket 消息处理器已注册
  wsClient.onMessage(handleWsMessage)
  
  // 确保 WebSocket 已连接
  if (!wsClient.isConnected()) {
    wsClient.connect()
  }
})

onUnmounted(() => {
  // 移除 WebSocket 消息处理器
  wsClient.offMessage(handleWsMessage)
})

const loadFriends = async () => {
  try {
    loading.value = true
    const data = await getFriendsApi()
    
    // 转换后端数据格式为页面需要的格式
    // 去重：使用user_id作为唯一标识
    const friendMap = new Map<string, Friend>()
    
    ;(data as any[]).forEach((f: any) => {
      const friendId = f.user_id || f.id
      // 如果已存在，跳过（去重）
      if (friendMap.has(friendId)) {
        return
      }
      
      friendMap.set(friendId, {
        id: friendId,
        name: f.nickname || f.username || '未知用户',
        avatar: f.avatar || '',
        online: f.online || false,
        lastMessage: f.last_message || undefined,
        lastTime: f.last_time ? new Date(f.last_time).getTime() : undefined,
        unreadCount: f.unread_count || 0
      })
    })
    
    // 转换为数组并排序
    friends.value = Array.from(friendMap.values()).sort((a, b) => {
      // 优先按未读数排序，然后按最后消息时间排序
      if (b.unreadCount !== a.unreadCount) {
        return (b.unreadCount || 0) - (a.unreadCount || 0)
      }
      return (b.lastTime || 0) - (a.lastTime || 0)
    })
  } catch (error: any) {
    // console.error('加载好友列表失败:', error)
    showToast(error.message || '加载好友列表失败', 'none')
  } finally {
    loading.value = false
  }
}

const loadRequests = async () => {
  try {
    const data = await getFriendRequestsApi()
    friendRequests.value = data as FriendRequest[]
  } catch (error: any) {
    // console.error('加载好友申请失败:', error)
  }
}

const addFriend = () => {
  navigateTo('/pages/friends/add-friend')
}

const viewRequests = () => {
  navigateTo('/pages/friends/requests')
}

const openChat = (friend: Friend) => {
  if (!friend || !friend.id) {
    // console.error('[FriendsList] openChat: friend对象无效', friend)
    return
  }
  navigateTo('/pages/friends/chat', { friendId: String(friend.id) })
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
  } else if (diff < 2 * day) {
    return '昨天'
  } else {
    const date = new Date(timestamp)
    return `${date.getMonth() + 1}/${date.getDate()}`
  }
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.friends-page {
  min-height: 100vh;
  @include cyber-page-bg;
}

.header-actions {
  display: flex;
  gap: 20rpx;
  padding: 20rpx 30rpx;
  background: rgba(30, 36, 66, 0.8);
  backdrop-filter: blur(10rpx);
  border-bottom: 2rpx solid rgba(0, 217, 255, 0.3);

  .action-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 20rpx;
    @include glow-button(#00D9FF);
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
    border-radius: 15rpx;
    position: relative;
    transition: all 0.3s;

    &:active {
      transform: scale(0.95);
    }

    .action-icon {
      font-size: 40rpx;
      margin-bottom: 10rpx;
      filter: drop-shadow(0 0 10rpx rgba(0, 217, 255, 0.6));
    }

    .action-text {
      font-size: 24rpx;
      @include neon-text(#ffffff);
    }

    .badge {
      position: absolute;
      top: 10rpx;
      right: 10rpx;
      background: rgba(255, 0, 85, 0.9);
      @include neon-text(#ffffff);
      font-size: 20rpx;
      padding: 4rpx 10rpx;
      border-radius: 20rpx;
      min-width: 32rpx;
      text-align: center;
      box-shadow: 0 0 15rpx rgba(255, 0, 85, 0.8);
      animation: badgePulse 2s ease-in-out infinite;
    }
  }
}

@keyframes badgePulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

.section-title {
  padding: 20rpx 30rpx;
  font-size: 26rpx;
  @include neon-text(#8B5CF6);
}

.friends-list {
  .friend-item {
    display: flex;
    align-items: center;
    padding: 25rpx 30rpx;
    @include neon-card;
    border-bottom: 1rpx solid rgba(0, 217, 255, 0.1);
    margin: 0 20rpx 2rpx;
    border-radius: 10rpx;
    transition: all 0.3s;

    &:active {
      transform: translateX(5rpx);
      box-shadow: 0 0 25rpx rgba(0, 217, 255, 0.4);
    }

    .friend-avatar-wrapper {
      width: 90rpx;
      height: 90rpx;
      margin-right: 20rpx;
      flex-shrink: 0;

      .friend-avatar {
        width: 100%;
        height: 100%;
        border-radius: 45rpx;
        border: 2rpx solid rgba(0, 217, 255, 0.5);
        box-shadow: 0 0 15rpx rgba(0, 217, 255, 0.4);
      }

      &.friend-avatar-emoji {
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(30, 36, 66, 0.6);
        border: 2rpx solid rgba(0, 217, 255, 0.5);
        border-radius: 45rpx;
        font-size: 50rpx;
        box-shadow: 0 0 15rpx rgba(0, 217, 255, 0.4);
      }
    }

    .friend-info {
      flex: 1;
      min-width: 0;

      .friend-name {
        display: block;
        font-size: 30rpx;
        @include neon-text(#ffffff);
        font-weight: bold;
        margin-bottom: 8rpx;
      }

      .friend-status {
        display: block;
        font-size: 24rpx;
        color: #6b7b93;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;

        &.online {
          @include neon-text(#00FF88);
        }
      }
    }

    .friend-meta {
      display: flex;
      flex-direction: column;
      align-items: flex-end;
      gap: 10rpx;

      .friend-time {
        font-size: 22rpx;
        @include neon-text(#8B5CF6);
      }

      .unread-badge {
        background: rgba(255, 0, 85, 0.9);
        @include neon-text(#ffffff);
        font-size: 20rpx;
        padding: 4rpx 10rpx;
        border-radius: 20rpx;
        min-width: 32rpx;
        text-align: center;
        box-shadow: 0 0 15rpx rgba(255, 0, 85, 0.8);
        animation: badgePulse 2s ease-in-out infinite;
      }
    }
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 200rpx 30rpx;

  .empty-icon {
    font-size: 120rpx;
    margin-bottom: 30rpx;
    filter: drop-shadow(0 0 30rpx rgba(0, 217, 255, 0.6));
    animation: floatIcon 3s ease-in-out infinite;
  }

  .empty-text {
    font-size: 32rpx;
    @include neon-text(#00D9FF);
    margin-bottom: 10rpx;
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
</style>

