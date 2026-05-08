<template>
  <view class="write-letter-page">
    <view class="page-header">
      <text class="page-title">✉️ 写情书</text>
      <text class="page-subtitle">给TA写一封真挚的情书</text>
    </view>

    <view class="letter-form">
      <view class="recipient-block">
        <view class="form-label">
          <text class="label-icon">💌</text>
          <text>收信人</text>
        </view>
        <view class="receiver-selector" @click="showFriendPicker = true">
          <view class="selector-info">
            <text v-if="selectedFriend" class="selected-name">{{ selectedFriend.friend_name }}</text>
            <text v-else class="placeholder">请选择好友</text>
            <text class="selector-hint">点击选择好友</text>
          </view>
          <text class="arrow">›</text>
        </view>
      </view>

      <view class="paper-wrapper">
        <view class="paper">
          <view class="paper-pin left"></view>
          <view class="paper-pin right"></view>

          <view class="paper-header">
            <text class="paper-label">TO.</text>
            <text class="paper-recipient">
              {{ selectedFriend ? selectedFriend.friend_name : '亲爱的...' }}
            </text>
          </view>

          <input
            v-model="letterForm.title"
            class="paper-title"
            placeholder="给这封情书取一个温柔的标题"
            maxlength="50"
          />

          <textarea
            v-model="letterForm.content"
            class="paper-content"
            placeholder="在这里写下你想对TA说的每一句悄悄话..."
            maxlength="2000"
            :show-confirm-bar="false"
          />

          <view class="paper-footer">
            <text class="paper-label">FROM.</text>
            <text class="paper-sender">{{ senderName }}</text>
          </view>

          <text class="char-count">{{ letterForm.content.length }}/2000</text>
        </view>
      </view>

      <view class="form-actions">
        <button class="cancel-btn" @click="goBack">取消</button>
        <button class="submit-btn" @click="submitLetter">
          <text class="btn-icon">📮</text>
          <text>发送情书</text>
        </button>
      </view>
    </view>

    <!-- 好友选择弹窗 -->
    <view v-if="showFriendPicker" class="modal-mask" @click="showFriendPicker = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">选择收信人</text>
          <text class="modal-close" @click="showFriendPicker = false">✕</text>
        </view>
        <view class="friends-list">
          <view 
            v-for="friend in friends" 
            :key="friend.id"
            class="friend-item"
            :class="{ selected: selectedFriend?.id === friend.id }"
            @click="selectFriend(friend)"
          >
            <view class="friend-avatar">
              <image v-if="friend.friend_avatar" :src="friend.friend_avatar" mode="aspectFill" />
              <text v-else class="avatar-placeholder">👤</text>
            </view>
            <text class="friend-name">{{ friend.friend_name }}</text>
            <text v-if="selectedFriend?.id === friend.id" class="check-icon">✓</text>
          </view>
          <view v-if="friends.length === 0" class="empty-friends">
            <text class="empty-icon">👥</text>
            <text class="empty-text">暂无好友</text>
            <text class="empty-hint">快去添加好友吧</text>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { navigateBack } from '@/utils'
import { useUserStore } from '@/store'
import type { Friend } from '@/types'
import { getFriendsApi, createLoveLetterApi } from '@/api/loveLetter'

const userStore = useUserStore()
const userInfo = computed(() => userStore.userInfo)

const senderName = computed(() => {
  const info = userInfo.value
  if (!info) return '匿名寄信人'
  if (info.nickname && info.nickname.trim()) {
    return info.nickname
  }
  return info.username || '匿名寄信人'
})

const friends = ref<Friend[]>([])
const selectedFriend = ref<Friend | null>(null)
const showFriendPicker = ref(false)
const isReply = ref(false)
const replyToUserId = ref('')

const letterForm = ref({
  title: '',
  content: ''
})

// 接收页面参数
onLoad((options: any) => {
  if (options.replyTo) {
    isReply.value = true
    replyToUserId.value = options.replyTo
    
    // 如果有回复的收信人名字，自动填充标题
    if (options.replyName) {
      letterForm.value.title = `回复给 ${options.replyName}`
    }
  }
})

onMounted(() => {
  loadFriends()
})

const loadFriends = async () => {
  try {
    const data = await getFriendsApi()
    friends.value = data
    
    // 如果是回复，自动选择对应的好友
    if (isReply.value && replyToUserId.value) {
      const targetFriend = data.find(f => f.user_id === replyToUserId.value)
      if (targetFriend) {
        selectedFriend.value = targetFriend
      }
    }
  } catch (error) {
    console.error('加载好友列表失败:', error)
    uni.showToast({ title: '加载好友失败', icon: 'none' })
  }
}

const selectFriend = (friend: Friend) => {
  selectedFriend.value = friend
  showFriendPicker.value = false
}

const submitLetter = async () => {
  if (!selectedFriend.value) {
    return uni.showToast({ title: '请选择收信人', icon: 'none' })
  }
  
  if (!letterForm.value.title.trim()) {
    return uni.showToast({ title: '请输入标题', icon: 'none' })
  }
  
  if (!letterForm.value.content.trim()) {
    return uni.showToast({ title: '请输入内容', icon: 'none' })
  }

  try {
    await createLoveLetterApi({
      title: letterForm.value.title.trim(),
      content: letterForm.value.content.trim(),
      to_user_id: selectedFriend.value.friend_id
    })
    
    uni.showToast({ title: '发送成功', icon: 'success' })
    
    setTimeout(() => {
      navigateBack()
    }, 1500)
  } catch (error: any) {
    console.error('发送情书失败:', error)
    uni.showToast({ title: '发送失败: ' + (error.message || '未知错误'), icon: 'none' })
  }
}

const goBack = () => {
  navigateBack()
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.write-letter-page {
  min-height: 100vh;
  @include cyber-page-bg;
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
      radial-gradient(circle at 50% 20%, rgba(255, 107, 157, 0.15) 0%, transparent 40%),
      radial-gradient(circle at 50% 80%, rgba(240, 147, 251, 0.15) 0%, transparent 40%);
    pointer-events: none;
    animation: heartbeat 6s ease-in-out infinite;
  }
}

@keyframes heartbeat {
  0%, 100% { opacity: 0.4; transform: scale(1); }
  50% { opacity: 0.7; transform: scale(1.05); }
}

.page-header {
  text-align: center;
  padding: 40rpx 0 50rpx;
  position: relative;
  z-index: 1;
  animation: fadeInDown 0.8s ease-out;

  .page-title {
    display: block;
    font-size: 48rpx;
    font-weight: bold;
    @include neon-title(#FF6B9D);
    margin-bottom: 15rpx;
    animation: loveGlow 3s ease-in-out infinite;
  }

  .page-subtitle {
    display: block;
    font-size: 26rpx;
    @include neon-text(#F093FB);
  }
}

@keyframes fadeInDown {
  from {
    opacity: 0;
    transform: translateY(-30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes loveGlow {
  0%, 100% {
    text-shadow: 
      0 0 20rpx rgba(255, 107, 157, 0.8),
      0 0 40rpx rgba(255, 107, 157, 0.5),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
  50% {
    text-shadow: 
      0 0 30rpx rgba(255, 107, 157, 1),
      0 0 60rpx rgba(255, 107, 157, 0.8),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
}

.letter-form {
  @include neon-card;
  border-radius: 30rpx;
  padding: 40rpx;
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(255, 107, 157, 0.4),
    inset 0 0 80rpx rgba(255, 107, 157, 0.1);
  border: 2rpx solid rgba(255, 107, 157, 0.6);
  position: relative;
  z-index: 1;
  animation: fadeInUp 0.8s ease-out;

  .form-label {
    display: flex;
    align-items: center;
    gap: 10rpx;
    margin-bottom: 15rpx;
    font-size: 28rpx;
    @include neon-text(#FF6B9D);
    font-weight: bold;

    .label-icon {
      font-size: 32rpx;
      filter: drop-shadow(0 0 15rpx rgba(255, 107, 157, 0.8));
      animation: iconPulse 3s ease-in-out infinite;
    }
  }

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes iconPulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

  .recipient-block {
    margin-bottom: 40rpx;
  }

  .receiver-selector {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 30rpx 35rpx;
    background: rgba(30, 36, 66, 0.6);
    backdrop-filter: blur(10rpx);
    border-radius: 20rpx;
    border: 2rpx dashed rgba(255, 107, 157, 0.6);
    box-shadow: 0 0 30rpx rgba(255, 107, 157, 0.3);
    transition: all 0.3s;

    &:active {
      transform: scale(0.98);
      border-color: rgba(255, 107, 157, 0.9);
      box-shadow: 0 0 40rpx rgba(255, 107, 157, 0.5);
    }

    .selector-info {
      display: flex;
      flex-direction: column;
      gap: 6rpx;
    }

    .selected-name {
      font-size: 30rpx;
      @include neon-text(#FF6B9D);
      font-weight: 600;
    }

    .placeholder {
      font-size: 30rpx;
      color: #6b7b93;
      font-weight: 500;
    }

    .selector-hint {
      font-size: 24rpx;
      @include neon-text(#F093FB);
    }

    .arrow {
      font-size: 44rpx;
      @include neon-text(#FF6B9D);
      font-weight: 500;
      animation: arrowFloat 2s ease-in-out infinite;
    }
  }

@keyframes arrowFloat {
  0%, 100% { transform: translateX(0); }
  50% { transform: translateX(5rpx); }
}

  .paper-wrapper {
    position: relative;
    padding: 20rpx 0 0;
  }

  .paper {
    position: relative;
    background: rgba(255, 253, 245, 0.15);
    backdrop-filter: blur(10rpx);
    border-radius: 24rpx;
    padding: 50rpx 40rpx 80rpx;
    box-shadow: 
      0 18rpx 45rpx rgba(255, 143, 171, 0.4),
      0 0 40rpx rgba(255, 107, 157, 0.3),
      inset 0 0 60rpx rgba(255, 107, 157, 0.05);
    overflow: hidden;
    border: 2rpx solid rgba(255, 107, 157, 0.6);
    background-image: linear-gradient(
        rgba(255, 107, 157, 0.15) 1px,
        transparent 1px
      );
    background-size: 100% 70rpx;
  }

  .paper::after {
    content: '';
    position: absolute;
    inset: 0;
    background: radial-gradient(circle at top left, rgba(255, 255, 255, 0.6), transparent 55%);
    pointer-events: none;
  }

  .paper-pin {
    position: absolute;
    top: 20rpx;
    width: 48rpx;
    height: 48rpx;
    background: rgba(255, 107, 157, 0.3);
    border-radius: 50%;
    box-shadow: 
      inset 0 0 0 8rpx rgba(255, 107, 157, 0.6),
      0 0 20rpx rgba(255, 107, 157, 0.6),
      0 6rpx 12rpx rgba(255, 143, 171, 0.3);
    z-index: 2;
    animation: pinPulse 3s ease-in-out infinite;

    &.left {
      left: 60rpx;
    }

    &.right {
      right: 60rpx;
      animation-delay: 1.5s;
    }
  }

@keyframes pinPulse {
  0%, 100% {
    box-shadow: 
      inset 0 0 0 8rpx rgba(255, 107, 157, 0.6),
      0 0 20rpx rgba(255, 107, 157, 0.6),
      0 6rpx 12rpx rgba(255, 143, 171, 0.3);
  }
  50% {
    box-shadow: 
      inset 0 0 0 8rpx rgba(255, 107, 157, 0.8),
      0 0 30rpx rgba(255, 107, 157, 0.9),
      0 8rpx 16rpx rgba(255, 143, 171, 0.5);
  }
}

  .paper-header,
  .paper-footer {
    display: flex;
    align-items: baseline;
    gap: 16rpx;
    font-size: 30rpx;
    @include neon-text(#F093FB);
    letter-spacing: 4rpx;
  }

  .paper-recipient {
    flex: 1;
    font-size: 34rpx;
    @include neon-title(#FF6B9D);
    font-weight: 600;
    letter-spacing: 2rpx;
    text-align: right;
  }

  .paper-footer {
    margin-top: 50rpx;
    justify-content: flex-end;
  }

  .paper-sender {
    font-size: 32rpx;
    @include neon-text(#FF6B9D);
    font-weight: 600;
  }

  .paper-label {
    font-family: 'Times New Roman', serif;
    font-size: 32rpx;
  }

  .paper-title {
    margin: 40rpx 0 10rpx;
    width: 100%;
    border: none;
    background: transparent;
    font-size: 36rpx;
    @include neon-text(#FF6B9D);
    font-weight: 600;
    padding: 0 4rpx 20rpx;
    border-bottom: 2rpx solid rgba(255, 107, 157, 0.5);
    box-shadow: 0 2rpx 0 rgba(255, 107, 157, 0.3);
    transition: all 0.3s ease;
    
    &:focus {
      border-bottom-color: rgba(255, 107, 157, 0.9);
      box-shadow: 0 3rpx 15rpx rgba(255, 107, 157, 0.5);
    }
  }

  .paper-title::placeholder {
    color: #6b7b93;
  }

  .paper-content {
    width: 100%;
    min-height: 460rpx;
    border: none;
    background: transparent;
    font-size: 30rpx;
    line-height: 70rpx;
    color: #b8c5d6;
    text-shadow: 0 0 8rpx rgba(184, 197, 214, 0.2);
    padding: 10rpx 4rpx;
    transition: all 0.3s ease;
    
    &:focus {
      color: #ffffff;
      text-shadow: 0 0 10rpx rgba(255, 255, 255, 0.3);
    }
  }

  .paper-content::placeholder {
    color: #6b7b93;
  }

  .paper-title:focus,
  .paper-content:focus {
    outline: none;
  }

  .char-count {
    position: absolute;
    right: 40rpx;
    bottom: 30rpx;
    font-size: 24rpx;
    @include neon-text(#F093FB);
  }

  .form-actions {
    display: flex;
    gap: 20rpx;
    margin-top: 50rpx;

    button {
      flex: 1;
      height: 90rpx;
      border-radius: 50rpx;
      font-size: 30rpx;
      font-weight: bold;
      border: none;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 10rpx;
      transition: all 0.3s;

      &::after {
        border: none;
      }

      &.cancel-btn {
        background: #f0f0f0;
        color: var(--theme-text-secondary);

        &:active {
          background: #e0e0e0;
        }
      }

      &.submit-btn {
        background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
        color: #ffffff;
        box-shadow: 0 8rpx 20rpx rgba(255, 107, 157, 0.3);

        &:active {
          transform: scale(0.98);
        }

        .btn-icon {
          font-size: 32rpx;
        }
      }
    }
  }
}

.modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: flex-end;
  justify-content: center;
  z-index: 999;
}

.modal-content {
  width: 100%;
  max-height: 80vh;
  background: var(--theme-surface);
  border-radius: 30rpx 30rpx 0 0;
  padding: 40rpx 30rpx;
  animation: slideUp 0.3s;

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 30rpx;

    .modal-title {
      font-size: 36rpx;
      font-weight: bold;
      color: var(--theme-text);
    }

    .modal-close {
      font-size: 40rpx;
      color: var(--theme-text-secondary);
      padding: 10rpx;
    }
  }

  .friends-list {
    max-height: 60vh;
    overflow-y: auto;

    .friend-item {
      display: flex;
      align-items: center;
      gap: 20rpx;
      padding: 25rpx;
      border-radius: 20rpx;
      margin-bottom: 15rpx;
      background: var(--theme-background);
      transition: all 0.3s;
      position: relative;

      &.selected {
        background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
        border: 2rpx solid #ff6b9d;
      }

      &:active {
        transform: scale(0.98);
      }

      .friend-avatar {
        width: 80rpx;
        height: 80rpx;
        border-radius: 50%;
        overflow: hidden;
        background: linear-gradient(135deg, rgba(255, 107, 157, 0.3) 0%, rgba(240, 147, 251, 0.3) 100%);
        border: 2rpx solid rgba(255, 107, 157, 0.5);
        box-shadow: 0 0 15rpx rgba(255, 107, 157, 0.4);
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;

        image {
          width: 100%;
          height: 100%;
        }

        .avatar-placeholder {
          font-size: 40rpx;
        }
      }

      .friend-name {
        flex: 1;
        font-size: 30rpx;
        @include neon-text(#ffffff);
        font-weight: 500;
      }

      .check-icon {
        font-size: 36rpx;
        @include neon-text(#FF6B9D);
        font-weight: bold;
        animation: checkPulse 1s ease-in-out infinite;
      }
    }

@keyframes checkPulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.2); }
}

    .empty-friends {
      padding: 100rpx 0;
      text-align: center;

      .empty-icon {
        display: block;
        font-size: 100rpx;
        margin-bottom: 20rpx;
        filter: drop-shadow(0 0 30rpx rgba(255, 107, 157, 0.6));
        animation: floatIcon 3s ease-in-out infinite;
      }

      .empty-text {
        display: block;
        font-size: 32rpx;
        @include neon-text(#FF6B9D);
        margin-bottom: 10rpx;
      }

      .empty-hint {
        display: block;
        font-size: 26rpx;
        @include neon-text(#F093FB);
      }
    }
  }
}

@keyframes floatIcon {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10rpx); }
}

@keyframes slideUp {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}
</style>
