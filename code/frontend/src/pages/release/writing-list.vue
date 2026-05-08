<template>
  <view class="writing-list-page">
    <view class="create-btn" @click="createWriting">
      <text>✍️ 发表作品</text>
    </view>

    <view class="writing-list">
      <view
        v-for="work in works"
        :key="work.id"
        class="work-item"
        @click="viewDetail(work.id)"
      >
        <view class="work-header">
          <view v-if="work.userAvatar" class="user-avatar-wrapper">
            <image class="user-avatar" :src="work.userAvatar" mode="aspectFill" />
          </view>
          <view v-else class="user-avatar-wrapper user-avatar-emoji">
            <text>👤</text>
          </view>
          <view class="user-info">
            <text class="user-name">{{ work.userName }}</text>
            <text class="work-time">{{ work.createdAt }}</text>
          </view>
        </view>

        <text class="work-title">{{ work.title }}</text>
        <text class="work-content">{{ work.content }}</text>

        <view class="work-footer">
          <text class="work-stat">👁️ {{ work.views }}</text>
          <text class="work-stat">❤️ {{ work.likes }}</text>
          <text class="work-stat">💬 {{ work.comments }}</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const works = ref([
  {
    id: '1',
    userId: '1',
    userName: '文学青年',
    userAvatar: '',
    title: '致青春的一封信',
    content: '亲爱的青春，感谢你赋予我勇气和梦想...',
    views: 567,
    likes: 128,
    comments: 45,
    createdAt: '2024-01-15 14:30',
  },
])

const createWriting = () => {
  uni.navigateTo({ url: '/pages/release/writing-create' })
}

const viewDetail = (id: string) => {
  uni.navigateTo({ url: `/pages/release/writing-detail?id=${id}` })
}
</script>

<style lang="scss" scoped>
.writing-list-page {
  min-height: 100vh;
  background: var(--theme-background);
}

.create-btn {
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  color: #ffffff;
  padding: 25rpx;
  margin: 20rpx 30rpx;
  border-radius: 50rpx;
  text-align: center;
  font-size: 30rpx;
  font-weight: bold;
}

.writing-list {
  padding: 0 30rpx 30rpx;
}

.work-item {
  background: var(--theme-surface);
  border-radius: 20rpx;
  padding: 30rpx;
  margin-bottom: 20rpx;

  .work-header {
    display: flex;
    align-items: center;
    margin-bottom: 20rpx;

    .user-avatar-wrapper {
      width: 60rpx;
      height: 60rpx;
      flex-shrink: 0;

      .user-avatar {
        width: 100%;
        height: 100%;
        border-radius: 50%;
      }

      &.user-avatar-emoji {
        display: flex;
        align-items: center;
        justify-content: center;
        background: #f0f0f0;
        border-radius: 50%;
        font-size: 35rpx;
      }
    }

    .user-info {
      flex: 1;

      .user-name {
        display: block;
        font-size: 28rpx;
        color: var(--theme-text);
        margin-bottom: 5rpx;
      }

      .work-time {
        display: block;
        font-size: 22rpx;
        color: var(--theme-text-secondary);
      }
    }
  }

  .work-title {
    display: block;
    font-size: 32rpx;
    font-weight: bold;
    color: var(--theme-text);
    margin-bottom: 15rpx;
  }

  .work-content {
    display: block;
    font-size: 26rpx;
    color: var(--theme-text-secondary);
    line-height: 1.6;
    margin-bottom: 20rpx;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
  }

  .work-footer {
    display: flex;
    gap: 30rpx;

    .work-stat {
      font-size: 24rpx;
      color: var(--theme-text-secondary);
    }
  }
}
</style>

