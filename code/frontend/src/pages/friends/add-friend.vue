<template>
  <view class="add-friend-page">
    <!-- 搜索框 -->
    <view class="search-box">
      <view class="search-input-wrapper">
        <text class="search-icon">🔍</text>
        <input
          v-model="searchText"
          class="search-input"
          placeholder="输入好友ID或昵称"
          @confirm="searchFriend"
        />
        <text v-if="searchText" class="clear-icon" @click="clearSearch">✕</text>
      </view>
      <button class="search-btn" @click="searchFriend">搜索</button>
    </view>

    <!-- 快速添加 -->
    <view class="quick-add">
      <text class="section-title">快速添加</text>
      <view class="quick-item" @click="scanQRCode">
        <text class="quick-icon">📷</text>
        <text class="quick-text">扫一扫</text>
      </view>
      <view class="quick-item" @click="showMyQRCode">
        <text class="quick-icon">🔲</text>
        <text class="quick-text">我的二维码</text>
      </view>
    </view>

    <!-- 搜索结果 -->
    <view v-if="searchResults.length > 0" class="search-result">
      <text class="section-title">搜索结果 ({{ searchResults.length }})</text>
      <view
        v-for="user in searchResults"
        :key="user.id"
        class="result-item"
      >
        <view v-if="user.avatar" class="result-avatar-wrapper">
          <image class="result-avatar" :src="user.avatar" mode="aspectFill" />
        </view>
        <view v-else class="result-avatar-wrapper result-avatar-emoji">
          <text>👤</text>
        </view>
        <view class="result-info">
          <text class="result-name">{{ user.name }}</text>
          <text class="result-id">账号: {{ user.username }}</text>
          <text v-if="user.desc" class="result-desc">{{ user.desc }}</text>
        </view>
        <button class="add-btn" @click="sendRequest(user)">添加</button>
      </view>
    </view>

    <!-- 推荐好友 -->
    <view class="recommend-friends">
      <text class="section-title">推荐好友</text>
      <view
        v-for="user in recommendUsers"
        :key="user.id"
        class="recommend-item"
      >
        <view v-if="user.avatar" class="recommend-avatar-wrapper">
          <image class="recommend-avatar" :src="user.avatar" mode="aspectFill" />
        </view>
        <view v-else class="recommend-avatar-wrapper recommend-avatar-emoji">
          <text>👤</text>
        </view>
        <view class="recommend-info">
          <text class="recommend-name">{{ user.name }}</text>
          <text class="recommend-desc">{{ user.desc }}</text>
        </view>
        <button class="add-btn" @click="sendRequest(user)">添加</button>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { showToast } from '@/utils'
import { getStorage } from '@/utils/storage'
import { searchUserApi, addFriendApi } from '@/api/friend'
import { useUserStore } from '@/store'

interface User {
  id: string
  name: string
  avatar: string
  desc?: string
  username?: string
}

const userStore = useUserStore()
const searchText = ref('')
const searchResults = ref<User[]>([])
const recommendUsers = ref<User[]>([])
const searching = ref(false)

onMounted(() => {
  loadRecommendUsers()
})

const loadRecommendUsers = async () => {
  // 可以从后端获取推荐用户，这里暂时留空
  recommendUsers.value = []
}

const searchFriend = async () => {
  if (!searchText.value.trim()) {
    return showToast('请输入搜索内容', 'none')
  }

  try {
    searching.value = true
    const users = await searchUserApi(searchText.value.trim())
    
    if (users && users.length > 0) {
      // 显示所有搜索结果
      searchResults.value = users.map((user: any) => ({
        id: user.id,
        name: user.nickname || user.username,
        avatar: user.avatar || '',
        username: user.username,
        desc: user.bio || ''
      }))
    } else {
      searchResults.value = []
      showToast('未找到用户', 'none')
    }
  } catch (error: any) {
    // console.error('搜索用户失败:', error)
    showToast(error.message || '搜索失败', 'none')
    searchResults.value = []
  } finally {
    searching.value = false
  }
}

const clearSearch = () => {
  searchText.value = ''
  searchResults.value = []
}

const scanQRCode = () => {
  // #ifdef H5
  uni.showToast({ title: 'H5环境不支持扫码', icon: 'none' })
  // #endif
  
  // #ifndef H5
  uni.scanCode({
    success: (res) => {
      searchText.value = res.result
      searchFriend()
    }
  })
  // #endif
}

const showMyQRCode = () => {
  const userInfo = getStorage<any>('userInfo', {})
  const myId = userInfo.id || 'my_id_' + Date.now()
  
  uni.showModal({
    title: '我的二维码',
    content: `我的ID: ${myId}\n\n让好友搜索此ID添加你为好友`,
    showCancel: false
  })
}

const sendRequest = async (user: User) => {
  if (!user) return

  // 不能添加自己为好友
  if (user.id === userStore.userId) {
    return showToast('不能添加自己为好友', 'none')
  }

  uni.showModal({
    title: '添加好友',
    content: `确定要添加 ${user.name} (${user.username}) 为好友吗？`,
    success: async (res) => {
      if (res.confirm) {
        try {
          await addFriendApi({
            friend_id: user.id,
            message: '我是 ' + (userStore.nickname || userStore.username)
          })
          
          showToast('好友申请已发送', 'success')
        } catch (error: any) {
          // console.error('发送好友申请失败:', error)
          showToast(error.message || '发送失败', 'none')
        }
      }
    }
  })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.add-friend-page {
  min-height: 100vh;
  @include cyber-page-bg;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 30% 20%, rgba(0, 217, 255, 0.1) 0%, transparent 50%),
      radial-gradient(circle at 70% 80%, rgba(255, 0, 214, 0.1) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
  }
}

.search-box {
  display: flex;
  gap: 15rpx;
  padding: 25rpx 30rpx;
  background: linear-gradient(180deg, rgba(30, 36, 66, 0.95) 0%, rgba(30, 36, 66, 0.85) 100%);
  backdrop-filter: blur(30rpx);
  border-bottom: 2rpx solid rgba(0, 217, 255, 0.4);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.6),
    0 0 40rpx rgba(0, 217, 255, 0.2),
    inset 0 -2rpx 0 rgba(0, 217, 255, 0.4);
  position: sticky;
  top: 0;
  z-index: 10;

  .search-input-wrapper {
    flex: 1;
    display: flex;
    align-items: center;
    @include neon-input;
    border-radius: 50rpx;
    padding: 0 30rpx;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

    .search-icon {
      font-size: 32rpx;
      margin-right: 15rpx;
      filter: drop-shadow(0 0 10rpx rgba(0, 217, 255, 0.8));
    }

    .search-input {
      flex: 1;
      font-size: 28rpx;
      height: 70rpx;
      background: transparent;
      border: none;
      color: #ffffff;
      
      &::placeholder {
        color: #6b7b93;
      }
    }

    .clear-icon {
      font-size: 32rpx;
      @include neon-text(#FF00D6);
      padding: 10rpx;
      transition: all 0.3s ease;
      
      &:active {
        transform: rotate(90deg) scale(1.2);
      }
    }
    
    &:focus-within {
      border-color: rgba(0, 217, 255, 0.8);
      box-shadow: 
        0 0 40rpx rgba(0, 217, 255, 0.5),
        inset 0 0 30rpx rgba(0, 217, 255, 0.1);
      transform: translateY(-2rpx);
    }
  }

  .search-btn {
    width: 130rpx;
    height: 70rpx;
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(0, 217, 255, 0.7);
    border-radius: 50rpx;
    font-size: 28rpx;
    font-weight: bold;
    line-height: 70rpx;
    box-shadow: 
      0 8rpx 24rpx rgba(0, 217, 255, 0.4),
      0 0 40rpx rgba(0, 217, 255, 0.3),
      inset 0 0 30rpx rgba(0, 217, 255, 0.15);
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
      background: conic-gradient(
        from 0deg,
        transparent 0deg,
        rgba(255, 255, 255, 0.3) 90deg,
        transparent 180deg
      );
      animation: rotate 3s linear infinite;
    }
    
    &::after {
      border: none;
    }
    
    &:active {
      transform: scale(0.95) translateY(2rpx);
      box-shadow: 
        0 4rpx 16rpx rgba(0, 217, 255, 0.6),
        0 0 50rpx rgba(0, 217, 255, 0.5),
        inset 0 0 40rpx rgba(0, 217, 255, 0.25);
    }
  }
}

.section-title {
  display: block;
  padding: 25rpx 30rpx;
  font-size: 30rpx;
  font-weight: bold;
  @include neon-title(#8B5CF6);
  position: relative;
  
  &::after {
    content: '';
    position: absolute;
    bottom: 10rpx;
    left: 30rpx;
    width: 60rpx;
    height: 2rpx;
    background: linear-gradient(90deg, rgba(138, 92, 246, 1) 0%, transparent 100%);
    box-shadow: 0 0 10rpx rgba(138, 92, 246, 0.8);
  }
}

.quick-add {
  margin: 30rpx;
  @include neon-card;
  border-radius: 25rpx;
  padding: 20rpx 0;
  border: 2rpx solid rgba(138, 92, 246, 0.5);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(138, 92, 246, 0.3),
    inset 0 0 40rpx rgba(138, 92, 246, 0.1);
  animation: sectionAppear 0.6s ease-out;

  .quick-item {
    display: flex;
    align-items: center;
    padding: 30rpx;
    border-bottom: 1rpx solid rgba(138, 92, 246, 0.2);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

    &:active {
      background: rgba(138, 92, 246, 0.1);
      transform: translateX(10rpx);
    }

    &:last-child {
      border-bottom: none;
    }

    .quick-icon {
      font-size: 48rpx;
      margin-right: 20rpx;
      filter: drop-shadow(0 0 15rpx rgba(138, 92, 246, 0.8));
      animation: iconFloat 3s ease-in-out infinite;
    }

    .quick-text {
      flex: 1;
      font-size: 32rpx;
      font-weight: 500;
      @include neon-text(#ffffff);
    }
    
    &::after {
      content: '›';
      font-size: 40rpx;
      @include neon-text(#00D9FF);
      transition: all 0.3s ease;
    }
    
    &:active::after {
      transform: translateX(10rpx);
    }
  }
}

.search-result,
.recommend-friends {
  margin: 30rpx;
  @include neon-card;
  border-radius: 25rpx;
  padding: 20rpx 0;
  border: 2rpx solid rgba(0, 217, 255, 0.5);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(0, 217, 255, 0.3),
    inset 0 0 40rpx rgba(0, 217, 255, 0.1);
  animation: sectionAppear 0.6s ease-out 0.1s backwards;
}

.result-item,
.recommend-item {
  display: flex;
  align-items: center;
  padding: 25rpx 30rpx;
  border-bottom: 1rpx solid rgba(0, 217, 255, 0.2);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  animation: itemSlideIn 0.5s ease-out backwards;
  
  &:nth-child(2) { animation-delay: 0.1s; }
  &:nth-child(3) { animation-delay: 0.2s; }
  &:nth-child(4) { animation-delay: 0.3s; }
  &:nth-child(5) { animation-delay: 0.4s; }

  &:last-child {
    border-bottom: none;
  }
  
  &:active {
    background: rgba(0, 217, 255, 0.05);
    transform: translateX(10rpx);
  }

  .result-avatar-wrapper,
  .recommend-avatar-wrapper {
    width: 90rpx;
    height: 90rpx;
    margin-right: 20rpx;
    flex-shrink: 0;

    .result-avatar,
    .recommend-avatar {
      width: 100%;
      height: 100%;
      border-radius: 45rpx;
    }

    &.result-avatar-emoji,
    &.recommend-avatar-emoji {
      display: flex;
      align-items: center;
      justify-content: center;
      background: linear-gradient(135deg, rgba(0, 217, 255, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
      border: 2rpx solid rgba(0, 217, 255, 0.6);
      border-radius: 45rpx;
      font-size: 50rpx;
      box-shadow: 
        0 0 20rpx rgba(0, 217, 255, 0.5),
        inset 0 0 20rpx rgba(0, 217, 255, 0.1);
      animation: avatarPulse 2s ease-in-out infinite;
    }
  }

  .result-info,
  .recommend-info {
    flex: 1;
    min-width: 0;

    .result-name,
    .recommend-name {
      display: block;
      font-size: 32rpx;
      @include neon-text(#ffffff);
      font-weight: bold;
      margin-bottom: 10rpx;
    }

    .result-id,
    .recommend-desc {
      display: block;
      font-size: 24rpx;
      @include neon-text(#00D9FF);
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      margin-bottom: 6rpx;
    }

    .result-desc {
      display: block;
      font-size: 22rpx;
      color: #b8c5d6;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }

  .add-btn {
    width: 120rpx;
    height: 60rpx;
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(255, 0, 214, 0.7);
    border-radius: 30rpx;
    font-size: 26rpx;
    font-weight: bold;
    line-height: 60rpx;
    box-shadow: 
      0 6rpx 20rpx rgba(255, 0, 214, 0.4),
      0 0 30rpx rgba(255, 0, 214, 0.3),
      inset 0 0 20rpx rgba(255, 0, 214, 0.15);
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
      background: conic-gradient(
        from 0deg,
        transparent 0deg,
        rgba(255, 255, 255, 0.3) 90deg,
        transparent 180deg
      );
      animation: rotate 3s linear infinite;
    }
    
    &::after {
      border: none;
    }
    
    &:active {
      transform: scale(0.92) translateY(2rpx);
      box-shadow: 
        0 4rpx 12rpx rgba(255, 0, 214, 0.6),
        0 0 40rpx rgba(255, 0, 214, 0.5),
        inset 0 0 30rpx rgba(255, 0, 214, 0.25);
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 0.8; }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes sectionAppear {
  from {
    opacity: 0;
    transform: translateY(30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes iconFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-8rpx);
  }
}

@keyframes itemSlideIn {
  from {
    opacity: 0;
    transform: translateX(-20rpx);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes avatarPulse {
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
</style>

