<template>
  <view class="forum-page">
    <!-- 顶部标签栏 -->
    <view class="tabs-bar">
      <view
        v-for="tab in tabs"
        :key="tab.type"
        class="tab-item"
        :class="{ active: activeTab === tab.type }"
        @click="switchTab(tab.type)"
      >
        <text class="tab-icon">{{ tab.icon }}</text>
        <text class="tab-text">{{ tab.label }}</text>
      </view>
    </view>

    <!-- 发布按钮 -->
    <view class="fab" @click="handleCreate">
      <text class="fab-icon">+</text>
    </view>

    <!-- 帖子列表 -->
    <scroll-view
      class="posts-list"
      scroll-y
      refresher-enabled
      :refresher-triggered="refreshing"
      @refresherrefresh="handleRefresh"
      @scrolltolower="handleLoadMore"
    >
      <view v-if="loading && posts.length === 0" class="loading">
        <text>加载中...</text>
      </view>

      <view v-else-if="posts.length === 0" class="empty">
        <text class="empty-icon">📭</text>
        <text class="empty-text">暂无内容，快来发布第一条吧~</text>
      </view>

      <view v-else>
        <view
          v-for="post in posts"
          :key="post.id"
          class="post-card"
          @click="handleViewPost(post.id)"
        >
          <!-- 帖子头部 -->
          <view class="post-header">
            <view class="post-type-badge" :class="`type-${post.post_type}`">
              <text>{{ getTypeLabel(post.post_type) }}</text>
            </view>
            <text class="post-time">{{ formatTime(post.created_at) }}</text>
          </view>

          <!-- 帖子标题 -->
          <text class="post-title">{{ post.title }}</text>

          <!-- 帖子内容预览 -->
          <view class="post-content-preview">
            <text>{{ truncateContent(post.content) }}</text>
          </view>

          <!-- 特殊内容展示 -->
          <view v-if="post.post_type === 'inspiration'" class="post-special">
            <text v-if="post.card_category" class="category-tag">
              {{ post.card_category === 'encourage' ? '💪 鼓励' : '📚 哲理' }}
            </text>
            <text v-if="post.author" class="author-tag">— {{ post.author }}</text>
          </view>

          <view v-if="post.post_type === 'topic'" class="post-special">
            <!-- 话题类型不显示额外信息，只显示标题和内容 -->
          </view>

          <view v-if="post.post_type === 'creation'" class="post-special">
            <text v-if="post.creation_category" class="category-tag">
              {{ post.creation_category }}
            </text>
            <view v-if="post.creation_tags && post.creation_tags.length > 0" class="tags">
              <text
                v-for="tag in post.creation_tags"
                :key="tag"
                class="tag"
              >#{{ tag }}</text>
            </view>
          </view>

          <!-- 帖子底部统计 -->
          <view class="post-footer">
            <view class="post-stats">
              <text class="stat-item">👍 {{ post.likes }}</text>
              <text class="stat-item">👁️ {{ post.views }}</text>
              <text class="stat-item">💬 {{ post.comments_count }}</text>
            </view>
          </view>
        </view>
      </view>

      <!-- 加载更多 -->
      <view v-if="hasMore && !loading" class="load-more">
        <text>上拉加载更多</text>
      </view>
      <view v-if="!hasMore && posts.length > 0" class="load-more">
        <text>没有更多了</text>
      </view>
    </scroll-view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getPostsApi, type ForumPost } from '@/api/forum'
import { useUserStore } from '@/store'

const userStore = useUserStore()

const tabs = [
  { type: 'all', icon: '🏠', label: '全部' },
  { type: 'inspiration', icon: '✨', label: '灵感' },
  { type: 'topic', icon: '💭', label: '话题' },
  { type: 'creation', icon: '✍️', label: '创作' }
]

const activeTab = ref<string>('all')
const posts = ref<ForumPost[]>([])
const loading = ref(false)
const refreshing = ref(false)
const hasMore = ref(true)
const page = ref(1)
const pageSize = 10

// 监听刷新事件
const handleRefreshEvent = () => {
  page.value = 1
  posts.value = []
  hasMore.value = true
  loadPosts()
}

onMounted(() => {
  const pages = getCurrentPages()
  const currentPage = pages[pages.length - 1]
  const type = (currentPage as any).options?.type
  if (type && ['inspiration', 'topic', 'creation'].includes(type)) {
    activeTab.value = type
  }
  loadPosts()
  
  // 监听发布成功事件
  uni.$on('refreshForumList', handleRefreshEvent)
})

onUnmounted(() => {
  // 移除监听
  uni.$off('refreshForumList', handleRefreshEvent)
})

const switchTab = (type: string) => {
  activeTab.value = type
  page.value = 1
  posts.value = []
  hasMore.value = true
  loadPosts()
}

const loadPosts = async () => {
  if (loading.value) return
  
  loading.value = true
  try {
    const params: any = {
      page: page.value,
      page_size: pageSize
    }
    
    if (activeTab.value !== 'all') {
      params.post_type = activeTab.value
    }
    
    const res = await getPostsApi(params)
    if (res) {
      const responseData = (res as any).data || res
      const newPosts = Array.isArray(responseData) ? responseData : responseData.list || []
      if (page.value === 1) {
        posts.value = newPosts
      } else {
        posts.value.push(...newPosts)
      }
      hasMore.value = newPosts.length >= pageSize
    }
  } catch (error) {
    console.error('加载帖子失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  } finally {
    loading.value = false
    refreshing.value = false
  }
}

const handleRefresh = () => {
  refreshing.value = true
  page.value = 1
  posts.value = []
  hasMore.value = true
  loadPosts()
}

const handleLoadMore = () => {
  if (!hasMore.value || loading.value) return
  page.value++
  loadPosts()
}

const handleCreate = () => {
  uni.navigateTo({
    url: '/pages/forum/create?type=' + (activeTab.value === 'all' ? 'inspiration' : activeTab.value)
  })
}

const handleViewPost = (id: string) => {
  uni.navigateTo({
    url: `/pages/forum/detail?id=${id}`
  })
}

const getTypeLabel = (type: string) => {
  const map: Record<string, string> = {
    inspiration: '灵感',
    topic: '话题',
    creation: '创作'
  }
  return map[type] || type
}

const truncateContent = (content: string, maxLength = 100) => {
  if (content.length <= maxLength) return content
  return content.substring(0, maxLength) + '...'
}

const formatTime = (time: string) => {
  const date = new Date(time)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)

  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  if (days < 7) return `${days}天前`
  
  return `${date.getMonth() + 1}-${date.getDate()}`
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.forum-page {
  @include cyber-page-bg;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 20% 30%, rgba(0, 217, 255, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 70%, rgba(255, 0, 214, 0.15) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.tabs-bar {
  position: sticky;
  top: 0;
  z-index: 100;
  background: rgba(30, 36, 66, 0.9);
  backdrop-filter: blur(20rpx);
  display: flex;
  padding: 20rpx 0;
  box-shadow: 
    0 8rpx 30rpx rgba(0, 0, 0, 0.5),
    0 0 30rpx rgba(0, 217, 255, 0.2),
    inset 0 -1rpx 0 rgba(0, 217, 255, 0.3);
  border-bottom: 1rpx solid rgba(0, 217, 255, 0.3);

  .tab-item {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 10rpx 0;
    transition: all 0.3s;
    position: relative;

    .tab-icon {
      font-size: 36rpx;
      margin-bottom: 8rpx;
      transition: all 0.3s ease;
    }

    .tab-text {
      font-size: 24rpx;
      color: #b8c5d6;
      transition: all 0.3s ease;
    }

    &.active {
      .tab-icon {
        filter: drop-shadow(0 0 15rpx rgba(0, 217, 255, 0.8));
        animation: iconPulse 2s ease-in-out infinite;
      }
      
      .tab-text {
        @include neon-text(#00D9FF);
        font-weight: bold;
      }
      
      &::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 50%;
        transform: translateX(-50%);
        width: 60rpx;
        height: 4rpx;
        background: linear-gradient(90deg, transparent, #00D9FF, transparent);
        border-radius: 2rpx;
        box-shadow: 0 0 10rpx rgba(0, 217, 255, 0.6);
      }
    }
  }

@keyframes iconPulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}
}

.fab {
  position: fixed;
  right: 40rpx;
  bottom: 120rpx;
  width: 100rpx;
  height: 100rpx;
  @include glow-button(#FF00D6);
  background: linear-gradient(135deg, rgba(255, 0, 214, 0.8) 0%, rgba(138, 92, 246, 0.8) 100%);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 99;
  animation: fabPulse 2s ease-in-out infinite;
  transition: all 0.3s ease;

  &:active {
    transform: scale(0.9) rotate(90deg);
  }

  .fab-icon {
    @include neon-text(#ffffff);
    font-size: 50rpx;
    font-weight: 300;
  }
}

@keyframes fabPulse {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(255, 0, 214, 0.5),
      0 8rpx 24rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(255, 0, 214, 0.2);
  }
  50% {
    box-shadow: 
      0 0 40rpx rgba(255, 0, 214, 0.8),
      0 12rpx 32rpx rgba(0, 0, 0, 0.4),
      inset 0 0 30rpx rgba(255, 0, 214, 0.3);
  }
}

.posts-list {
  flex: 1;
  height: 0;
  padding: 20rpx;
  position: relative;
  z-index: 1;
}

.loading,
.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 100rpx 0;
  @include neon-text(#b8c5d6);

  .empty-icon {
    font-size: 80rpx;
    margin-bottom: 20rpx;
    filter: drop-shadow(0 0 20rpx rgba(0, 217, 255, 0.6));
    animation: float 3s ease-in-out infinite;
  }
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-15rpx); }
}

.post-card {
  @include neon-card;
  border-radius: 20rpx;
  padding: 30rpx;
  margin-bottom: 20rpx;
  animation: fadeInUp 0.5s ease-out backwards;
  position: relative;
  
  &:nth-child(odd) {
    animation-delay: 0.1s;
  }
  
  &:nth-child(even) {
    animation-delay: 0.2s;
  }
  
  &:active {
    transform: translateY(-5rpx);
    box-shadow: 
      0 15rpx 50rpx rgba(0, 0, 0, 0.6),
      0 0 40rpx rgba(0, 217, 255, 0.5),
      inset 0 0 40rpx rgba(0, 217, 255, 0.15);
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.post-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20rpx;

  .post-type-badge {
    @include neon-tag(#00D9FF);
    font-size: 22rpx;
    font-weight: bold;

    &.type-inspiration {
      @include neon-tag(#FFD600);
    }

    &.type-topic {
      @include neon-tag(#00D9FF);
    }

    &.type-creation {
      @include neon-tag(#FF00D6);
    }
  }

  .post-time {
    font-size: 22rpx;
    @include neon-text(#6b7b93);
  }
}

.post-title {
  display: block;
  font-size: 32rpx;
  font-weight: bold;
  @include neon-title(#ffffff);
  margin-bottom: 15rpx;
  line-height: 1.5;
}

.post-content-preview {
  font-size: 28rpx;
  color: #b8c5d6;
  line-height: 1.6;
  margin-bottom: 15rpx;
  text-shadow: 0 0 5rpx rgba(184, 197, 214, 0.3);
}

.post-special {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 15rpx;
  margin-bottom: 15rpx;

  .category-tag,
  .author-tag {
    @include neon-tag(#8B5CF6);
    font-size: 24rpx;
  }

  .tags {
    display: flex;
    flex-wrap: wrap;
    gap: 10rpx;

    .tag {
      font-size: 22rpx;
      @include neon-tag(#00D9FF);
    }
  }
}

.post-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 15rpx;
  border-top: 1rpx solid rgba(0, 217, 255, 0.2);

  .post-stats {
    display: flex;
    gap: 30rpx;

    .stat-item {
      font-size: 24rpx;
      @include neon-text(#6b7b93);
      transition: all 0.3s ease;
      
      &:active {
        transform: scale(1.2);
        @include neon-text(#00D9FF);
      }
    }
  }
}

.load-more {
  text-align: center;
  padding: 30rpx 0;
  @include neon-text(#6b7b93);
  font-size: 24rpx;
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}
</style>

