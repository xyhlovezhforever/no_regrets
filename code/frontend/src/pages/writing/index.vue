<template>
  <view class="writing-page">
    <view class="header">
      <text class="title">✍️ 创作交流</text>
      <text class="subtitle">记录心情，分享故事</text>
    </view>

    <!-- 快捷操作 -->
    <view class="quick-actions">
      <view class="action-card publish-card" @click="handleNavigate('/pages/release/writing-create')">
        <text class="action-icon">✨</text>
        <view class="action-info">
          <text class="action-title">发布作品</text>
          <text class="action-desc">开始创作</text>
        </view>
        <text class="action-arrow">→</text>
      </view>
    </view>

    <!-- 作品预览 -->
    <view class="works-preview">
      <view class="preview-header">
        <text class="preview-title">最新作品</text>
        <view class="more-btn" @click="handleNavigate('/pages/release/writing-list')">
          <text>查看全部</text>
          <text class="more-arrow">›</text>
        </view>
      </view>

      <view v-if="works.length > 0" class="works-list">
        <view
          v-for="work in works.slice(0, 5)"
          :key="work.id"
          class="work-item"
          @click="viewWork(work)"
        >
          <view class="work-header">
            <view class="author-info">
              <text class="author-avatar">{{ work.authorAvatar || '✍️' }}</text>
              <text class="author-name">{{ work.author }}</text>
            </view>
            <text class="work-category">{{ work.category }}</text>
          </view>
          <text class="work-title">{{ work.title }}</text>
          <text class="work-content">{{ work.content }}</text>
          <view class="work-footer">
            <text class="work-time">{{ formatTime(work.time) }}</text>
            <view class="work-stats">
              <text class="stat-item">❤️ {{ work.likes || 0 }}</text>
              <text class="stat-item">💬 {{ work.comments?.length || 0 }}</text>
            </view>
          </view>
        </view>
      </view>

      <view v-else class="empty-works">
        <text class="empty-icon">📝</text>
        <text class="empty-text">还没有作品</text>
        <text class="empty-hint">快来发布第一篇作品吧~</text>
        <button class="empty-btn" @click="handleNavigate('/pages/release/writing-create')">
          开始创作
        </button>
      </view>
    </view>

    <!-- 分类导航 -->
    <view class="categories">
      <view class="category-header">
        <text class="category-title">浏览分类</text>
      </view>
      <view class="category-grid">
        <view
          v-for="cat in categories"
          :key="cat.name"
          class="category-item"
          @click="browseCategory(cat.name)"
        >
          <text class="category-icon">{{ cat.icon }}</text>
          <text class="category-name">{{ cat.name }}</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { getStorage } from '@/utils/storage'
import { navigateTo } from '@/utils'

interface Work {
  id: string
  title: string
  content: string
  author: string
  authorAvatar?: string
  category: string
  time: number
  likes?: number
  comments?: any[]
}

const works = ref<Work[]>([])

const categories = [
  { name: '随笔', icon: '📖' },
  { name: '日记', icon: '📔' },
  { name: '诗歌', icon: '🌸' },
  { name: '小说', icon: '📚' },
  { name: '感悟', icon: '💭' },
  { name: '其他', icon: '✨' }
]

onMounted(() => {
  loadWorks()
})

onShow(() => {
  loadWorks()
})

const loadWorks = () => {
  works.value = getStorage<Work[]>('writingWorks', [])
}

const handleNavigate = (url: string) => {
  navigateTo(url)
}

const viewWork = (work: Work) => {
  uni.showModal({
    title: work.title,
    content: work.content,
    showCancel: false
  })
}

const browseCategory = (category: string) => {
  navigateTo('/pages/release/writing-list', { category })
}

const formatTime = (timestamp: number): string => {
  const now = Date.now()
  const diff = now - timestamp

  const minute = 60 * 1000
  const hour = 60 * minute
  const day = 24 * hour

  if (diff < minute) return '刚刚'
  if (diff < hour) return `${Math.floor(diff / minute)}分钟前`
  if (diff < day) return `${Math.floor(diff / hour)}小时前`
  if (diff < 7 * day) return `${Math.floor(diff / day)}天前`

  const date = new Date(timestamp)
  return `${date.getMonth() + 1}月${date.getDate()}日`
}
</script>

<style lang="scss" scoped>
.writing-page {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  padding: 40rpx 30rpx;
}

.header {
  text-align: center;
  margin-bottom: 40rpx;

  .title {
    display: block;
    font-size: 52rpx;
    font-weight: bold;
    color: var(--theme-text);
    margin-bottom: 15rpx;
  }

  .subtitle {
    display: block;
    font-size: 28rpx;
    color: var(--theme-text-secondary);
  }
}

.quick-actions {
  margin-bottom: 40rpx;

  .action-card {
    background: var(--theme-surface);
    border-radius: 25rpx;
    padding: 30rpx;
    display: flex;
    align-items: center;
    box-shadow: 0 8rpx 20rpx rgba(0, 0, 0, 0.1);
    transition: all 0.3s;

    &:active {
      transform: scale(0.98);
    }

    .action-icon {
      font-size: 60rpx;
      margin-right: 20rpx;
    }

    .action-info {
      flex: 1;

      .action-title {
        display: block;
        font-size: 32rpx;
        font-weight: bold;
        color: var(--theme-text);
        margin-bottom: 8rpx;
      }

      .action-desc {
        font-size: 24rpx;
        color: var(--theme-text-secondary);
      }
    }

    .action-arrow {
      font-size: 40rpx;
      color: #cccccc;
    }
  }
}

.works-preview {
  background: var(--theme-surface);
  border-radius: 25rpx;
  padding: 30rpx;
  margin-bottom: 30rpx;
  box-shadow: 0 8rpx 20rpx rgba(0, 0, 0, 0.1);

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 30rpx;

    .preview-title {
      font-size: 32rpx;
      font-weight: bold;
      color: var(--theme-text);
    }

    .more-btn {
      display: flex;
      align-items: center;
      font-size: 26rpx;
      color: var(--theme-primary);

      .more-arrow {
        font-size: 32rpx;
        margin-left: 5rpx;
      }
    }
  }

  .works-list {
    .work-item {
      padding: 25rpx;
      background: var(--theme-background);
      border-radius: 20rpx;
      margin-bottom: 20rpx;

      &:last-child {
        margin-bottom: 0;
      }

      &:active {
        background: #f0f1f3;
      }

      .work-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 15rpx;

        .author-info {
          display: flex;
          align-items: center;

          .author-avatar {
            font-size: 32rpx;
            margin-right: 10rpx;
          }

          .author-name {
            font-size: 26rpx;
            color: var(--theme-text-secondary);
          }
        }

        .work-category {
          font-size: 22rpx;
          color: var(--theme-primary);
          background: rgba(102, 126, 234, 0.1);
          padding: 5rpx 15rpx;
          border-radius: 20rpx;
        }
      }

      .work-title {
        display: block;
        font-size: 30rpx;
        font-weight: bold;
        color: var(--theme-text);
        margin-bottom: 10rpx;
      }

      .work-content {
        display: block;
        font-size: 26rpx;
        color: var(--theme-text-secondary);
        line-height: 1.6;
        margin-bottom: 15rpx;
        overflow: hidden;
        text-overflow: ellipsis;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
      }

      .work-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;

        .work-time {
          font-size: 22rpx;
          color: var(--theme-text-secondary);
        }

        .work-stats {
          display: flex;
          gap: 20rpx;

          .stat-item {
            font-size: 22rpx;
            color: var(--theme-text-secondary);
          }
        }
      }
    }
  }

  .empty-works {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 80rpx 0;

    .empty-icon {
      font-size: 100rpx;
      margin-bottom: 20rpx;
    }

    .empty-text {
      font-size: 30rpx;
      color: var(--theme-text-secondary);
      margin-bottom: 10rpx;
    }

    .empty-hint {
      font-size: 24rpx;
      color: var(--theme-text-secondary);
      margin-bottom: 40rpx;
    }

    .empty-btn {
      width: 250rpx;
      height: 70rpx;
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      color: #ffffff;
      border-radius: 50rpx;
      border: none;
      font-size: 28rpx;
      line-height: 70rpx;

      &::after {
        border: none;
      }
    }
  }
}

.categories {
  background: var(--theme-surface);
  border-radius: 25rpx;
  padding: 30rpx;
  box-shadow: 0 8rpx 20rpx rgba(0, 0, 0, 0.1);

  .category-header {
    margin-bottom: 25rpx;

    .category-title {
      font-size: 32rpx;
      font-weight: bold;
      color: var(--theme-text);
    }
  }

  .category-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 20rpx;

    .category-item {
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 30rpx 20rpx;
      background: var(--theme-background);
      border-radius: 20rpx;
      transition: all 0.3s;

      &:active {
        background: #f0f1f3;
        transform: scale(0.95);
      }

      .category-icon {
        font-size: 48rpx;
        margin-bottom: 10rpx;
      }

      .category-name {
        font-size: 24rpx;
        color: var(--theme-text-secondary);
      }
    }
  }
}
</style>

