<template>
  <view class="detail-page">
    <view class="work-detail">
      <text class="work-title">{{ work.title }}</text>
      <view class="work-author">
        <image class="author-avatar" :src="work.userAvatar" mode="aspectFill" />
        <text class="author-name">{{ work.userName }}</text>
        <text class="work-time">{{ work.createdAt }}</text>
      </view>
      <text class="work-content">{{ work.content }}</text>
      <view class="work-actions">
        <button class="btn-action" @click="toggleLike">
          <text>{{ work.isLiked ? '❤️' : '🤍' }}</text>
          <text>{{ work.likes }}</text>
        </button>
        <button class="btn-action">
          <text>💬</text>
          <text>{{ work.comments }}</text>
        </button>
        <button class="btn-action">
          <text>📤</text>
          <text>分享</text>
        </button>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const work = ref({
  title: '致青春的一封信',
  userName: '文学青年',
  userAvatar: '',
  content: '亲爱的青春，感谢你赋予我勇气和梦想...\n\n在这段美好的时光里，我经历了太多的喜怒哀乐...',
  likes: 128,
  comments: 45,
  isLiked: false,
  createdAt: '2024-01-15 14:30',
})

const toggleLike = () => {
  work.value.isLiked = !work.value.isLiked
  work.value.likes += work.value.isLiked ? 1 : -1
}
</script>

<style lang="scss" scoped>
.detail-page {
  min-height: 100vh;
  background: var(--theme-background);
}

.work-detail {
  background: var(--theme-surface);
  padding: 40rpx 30rpx;

  .work-title {
    display: block;
    font-size: 40rpx;
    font-weight: bold;
    color: var(--theme-text);
    margin-bottom: 30rpx;
  }

  .work-author {
    display: flex;
    align-items: center;
    margin-bottom: 30rpx;

    .author-avatar-wrapper {
      width: 60rpx;
      height: 60rpx;
      flex-shrink: 0;

      .author-avatar {
        width: 100%;
        height: 100%;
        border-radius: 50%;
      }

      &.author-avatar-emoji {
        display: flex;
        align-items: center;
        justify-content: center;
        background: #f0f0f0;
        border-radius: 50%;
        font-size: 35rpx;
      }
    }

    .author-name {
      flex: 1;
      font-size: 28rpx;
      color: var(--theme-text);
    }

    .work-time {
      font-size: 24rpx;
      color: var(--theme-text-secondary);
    }
  }

  .work-content {
    display: block;
    font-size: 30rpx;
    color: var(--theme-text-secondary);
    line-height: 1.8;
    margin-bottom: 40rpx;
    white-space: pre-wrap;
  }

  .work-actions {
    display: flex;
    gap: 20rpx;
    padding-top: 30rpx;
    border-top: 1rpx solid #e5e5e5;

    .btn-action {
      flex: 1;
      height: 70rpx;
      background: var(--theme-background);
      border-radius: 50rpx;
      border: none;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 10rpx;
      font-size: 26rpx;
      color: var(--theme-text-secondary);
    }
  }
}
</style>

