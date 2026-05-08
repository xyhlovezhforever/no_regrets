<template>
  <view class="detail-page">
    <!-- 偶像信息头部 -->
    <view class="idol-header">
      <view class="header-bg"></view>
      <!-- 编辑按钮 -->
      <view class="edit-header-btn" @click="gotoEdit">
        <text>✏️ 编辑</text>
      </view>
      <view class="idol-info">
        <view v-if="idol.avatar_url" class="idol-avatar-wrapper">
          <image class="idol-avatar" :src="getFullUrl(idol.avatar_url)" mode="aspectFill" />
        </view>
        <view v-else class="idol-avatar-wrapper idol-avatar-emoji">
          <text>⭐</text>
        </view>
        <text class="idol-name">{{ idol.name }}</text>
        <text v-if="idol.profession" class="idol-profession">{{ idol.profession }}</text>
        <text v-if="idol.description" class="idol-description">{{ idol.description }}</text>
        <view v-if="idol.tags && idol.tags.length" class="idol-tags">
          <text v-for="tag in idol.tags" :key="tag" class="tag">{{ tag }}</text>
        </view>
      </view>
    </view>

    <!-- 标签页 -->
    <view class="tabs">
      <view class="tab" :class="{ active: activeTab === 'quotes' }" @click="activeTab = 'quotes'">
        <text>💬 语录 ({{ quotes.length }})</text>
      </view>
      <view class="tab" :class="{ active: activeTab === 'works' }" @click="activeTab = 'works'">
        <text>📚 作品 ({{ works.length }})</text>
      </view>
    </view>

    <!-- 语录列表 -->
    <view v-if="activeTab === 'quotes'" class="content-section">
      <view class="section-header">
        <text class="section-title">经典语录</text>
        <view class="add-btn" @click="addQuote">
          <text>+ 添加</text>
        </view>
      </view>
      <view v-if="quotes.length > 0" class="quotes-list">
        <view v-for="quote in quotes" :key="quote.id" class="quote-item">
          <text class="quote-icon">💬</text>
          <view class="quote-content">
            <text class="quote-text">{{ quote.content }}</text>
            <text v-if="quote.source" class="quote-source">— {{ quote.source }}</text>
            <text v-if="quote.quote_date" class="quote-date">{{ formatDate(quote.quote_date) }}</text>
          </view>
          <view class="quote-actions">
            <text class="action-icon" @click.stop="deleteQuote(quote.id)">🗑️</text>
          </view>
        </view>
      </view>
      <view v-else class="empty-state">
        <text class="empty-icon">💬</text>
        <text class="empty-text">还没有语录</text>
      </view>
    </view>

    <!-- 作品列表 -->
    <view v-if="activeTab === 'works'" class="content-section">
      <view class="section-header">
        <text class="section-title">代表作品</text>
        <view class="add-btn" @click="addWork">
          <text>+ 添加</text>
        </view>
      </view>
      <view v-if="works.length > 0" class="works-list">
        <view v-for="work in works" :key="work.id" class="work-item">
          <!-- 作品缩略图 -->
          <view v-if="work.thumbnail_url" class="work-thumbnail">
            <image :src="getFullUrl(work.thumbnail_url)" mode="aspectFill" />
            <view class="work-type-badge">{{ getWorkTypeLabel(work.work_type) }}</view>
          </view>
          <view v-else class="work-thumbnail work-thumbnail-empty">
            <text class="work-type-icon">{{ getWorkTypeIcon(work.work_type) }}</text>
          </view>
          
          <!-- 作品信息 -->
          <view class="work-info">
            <text class="work-title">{{ work.title }}</text>
            <text v-if="work.description" class="work-desc">{{ work.description }}</text>
            <view class="work-meta">
              <text class="meta-item">👁️ {{ work.view_count }}</text>
              <text class="meta-item">❤️ {{ work.like_count }}</text>
              <text v-if="work.duration" class="meta-item">⏱️ {{ formatDuration(work.duration) }}</text>
            </view>
          </view>
          
          <!-- 作品操作 -->
          <view class="work-actions">
            <text class="action-icon" @click.stop="viewWork(work.id)">👁️</text>
            <text class="action-icon" @click.stop="toggleLike(work.id)">❤️</text>
            <text class="action-icon" @click.stop="deleteWork(work.id)">🗑️</text>
          </view>
        </view>
      </view>
      <view v-else class="empty-state">
        <text class="empty-icon">📚</text>
        <text class="empty-text">还没有作品</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onLoad, onShow } from '@dcloudio/uni-app'
import { navigateTo } from '@/utils'
import { BASE_API } from '@/config'
import type { IdolDetail, IdolQuote, IdolWork } from '@/types/idol'
import {
  getIdolApi,
  createIdolQuoteApi,
  deleteIdolQuoteApi,
  createIdolWorkApi,
  deleteIdolWorkApi,
  viewWorkApi,
  likeWorkApi,
  unlikeWorkApi
} from '@/api/idol'

const idolId = ref('')
const idol = ref<Partial<IdolDetail>>({})
const quotes = ref<IdolQuote[]>([])
const works = ref<IdolWork[]>([])
const activeTab = ref<'quotes' | 'works'>('quotes')
const likedWorks = ref<Set<string>>(new Set())

onLoad((options: any) => {
  if (options.id) {
    idolId.value = options.id
    loadIdolDetail()
  }
})

onShow(() => {
  // 从编辑页面返回时刷新数据
  if (idolId.value) {
    loadIdolDetail()
  }
})

const loadIdolDetail = async () => {
  try {
    const data = await getIdolApi(idolId.value)
    idol.value = data
    quotes.value = data.quotes || []
    works.value = data.works || []
  } catch (error: any) {
    console.error('加载偶像详情失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  }
}

const addQuote = () => {
  uni.showModal({
    title: '添加语录',
    editable: true,
    placeholderText: '输入语录内容',
    success: async (res) => {
      if (res.confirm && res.content) {
        try {
          await createIdolQuoteApi(idolId.value, {
            content: res.content
          })
          await loadIdolDetail()
          uni.showToast({ title: '添加成功', icon: 'success' })
        } catch (error: any) {
          console.error('添加语录失败:', error)
          uni.showToast({ title: '添加失败', icon: 'none' })
        }
      }
    }
  })
}

const deleteQuote = (quoteId: string) => {
  uni.showModal({
    title: '确认删除',
    content: '确定要删除这条语录吗？',
    success: async (res) => {
      if (res.confirm) {
        try {
          await deleteIdolQuoteApi(quoteId)
          await loadIdolDetail()
          uni.showToast({ title: '删除成功', icon: 'success' })
        } catch (error: any) {
          console.error('删除语录失败:', error)
          uni.showToast({ title: '删除失败', icon: 'none' })
        }
      }
    }
  })
}

const addWork = () => {
  uni.showModal({
    title: '添加作品',
    editable: true,
    placeholderText: '输入作品标题',
    success: async (res) => {
      if (res.confirm && res.content) {
        try {
          await createIdolWorkApi(idolId.value, {
            title: res.content,
            work_type: 'other',
            file_url: ''
          })
          await loadIdolDetail()
          uni.showToast({ title: '添加成功', icon: 'success' })
        } catch (error: any) {
          console.error('添加作品失败:', error)
          uni.showToast({ title: '添加失败', icon: 'none' })
        }
      }
    }
  })
}

const deleteWork = (workId: string) => {
  uni.showModal({
    title: '确认删除',
    content: '确定要删除这个作品吗？',
    success: async (res) => {
      if (res.confirm) {
        try {
          await deleteIdolWorkApi(workId)
          await loadIdolDetail()
          uni.showToast({ title: '删除成功', icon: 'success' })
        } catch (error: any) {
          console.error('删除作品失败:', error)
          uni.showToast({ title: '删除失败', icon: 'none' })
        }
      }
    }
  })
}

const viewWork = async (workId: string) => {
  try {
    await viewWorkApi(workId)
    await loadIdolDetail()
  } catch (error) {
    console.error('浏览作品失败:', error)
  }
}

const toggleLike = async (workId: string) => {
  try {
    if (likedWorks.value.has(workId)) {
      await unlikeWorkApi(workId)
      likedWorks.value.delete(workId)
    } else {
      await likeWorkApi(workId)
      likedWorks.value.add(workId)
    }
    await loadIdolDetail()
  } catch (error) {
    console.error('点赞失败:', error)
  }
}

const formatDate = (dateStr: string) => {
  return dateStr ? new Date(dateStr).toLocaleDateString('zh-CN') : ''
}

const formatDuration = (seconds: number) => {
  const mins = Math.floor(seconds / 60)
  const secs = seconds % 60
  return `${mins}:${secs.toString().padStart(2, '0')}`
}

const getWorkTypeLabel = (type: string) => {
  const labels: Record<string, string> = {
    audio: '音频',
    video: '视频',
    image: '图片',
    other: '其他'
  }
  return labels[type] || '其他'
}

const getWorkTypeIcon = (type: string) => {
  const icons: Record<string, string> = {
    audio: '🎵',
    video: '🎬',
    image: '🖼️',
    other: '📄'
  }
  return icons[type] || '📄'
}

const gotoEdit = () => {
  navigateTo('/pages/idol/edit', { id: idolId.value })
}

const getFullUrl = (url: string | null | undefined) => {
  if (!url) return ''
  if (url.startsWith('http')) return url
  return `${BASE_API.replace('/api/v1', '')}${url}`
}
</script>

<style lang="scss" scoped>
.detail-page {
  min-height: 100vh;
  background: var(--theme-background);
}

.idol-header {
  position: relative;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  padding: 60rpx 30rpx 40rpx;

  .edit-header-btn {
    position: absolute;
    top: 30rpx;
    right: 30rpx;
    padding: 10rpx 20rpx;
    background: rgba(255, 255, 255, 0.9);
    border-radius: 30rpx;
    font-size: 24rpx;
    color: var(--theme-primary);
    font-weight: bold;
    z-index: 10;
    box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.1);
  }
  
  .idol-info {
    display: flex;
    flex-direction: column;
    align-items: center;
    color: #ffffff;
    
    .idol-avatar-wrapper {
      width: 180rpx;
      height: 180rpx;
      margin-bottom: 20rpx;
      
      .idol-avatar {
        width: 100%;
        height: 100%;
        border-radius: 90rpx;
        border: 4rpx solid rgba(255, 255, 255, 0.3);
      }
      
      &.idol-avatar-emoji {
        display: flex;
        align-items: center;
        justify-content: center;
        background: rgba(255, 255, 255, 0.2);
        border-radius: 90rpx;
        font-size: 100rpx;
      }
    }
    
    .idol-name {
      font-size: 40rpx;
      font-weight: bold;
      margin-bottom: 10rpx;
    }
    
    .idol-profession {
      font-size: 26rpx;
      opacity: 0.9;
      margin-bottom: 15rpx;
    }
    
    .idol-description {
      font-size: 24rpx;
      opacity: 0.8;
      text-align: center;
      line-height: 1.6;
      margin-bottom: 20rpx;
    }
    
    .idol-tags {
      display: flex;
      flex-wrap: wrap;
      gap: 10rpx;
      justify-content: center;
      
      .tag {
        padding: 8rpx 20rpx;
        background: rgba(255, 255, 255, 0.2);
        border-radius: 30rpx;
        font-size: 22rpx;
      }
    }
  }
}

.tabs {
  display: flex;
  background: var(--theme-surface);
  border-bottom: 1rpx solid #e0e0e0;
  
  .tab {
    flex: 1;
    text-align: center;
    padding: 30rpx 0;
    font-size: 28rpx;
    color: var(--theme-text-secondary);
    position: relative;
    
    &.active {
      color: var(--theme-primary);
      font-weight: bold;
      
      &::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 50%;
        transform: translateX(-50%);
        width: 60rpx;
        height: 4rpx;
        background: var(--theme-primary);
        border-radius: 2rpx;
      }
    }
  }
}

.content-section {
  padding: 30rpx;
  
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20rpx;
    
    .section-title {
      font-size: 32rpx;
      font-weight: bold;
      color: var(--theme-text);
    }
    
    .add-btn {
      padding: 10rpx 25rpx;
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      color: #ffffff;
      border-radius: 30rpx;
      font-size: 24rpx;
    }
  }
}

.quotes-list {
  .quote-item {
    background: var(--theme-surface);
    border-radius: 15rpx;
    padding: 25rpx;
    margin-bottom: 15rpx;
    display: flex;
    gap: 15rpx;
    box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.05);
    
    .quote-icon {
      font-size: 40rpx;
      flex-shrink: 0;
    }
    
    .quote-content {
      flex: 1;
      display: flex;
      flex-direction: column;
      gap: 8rpx;
      
      .quote-text {
        font-size: 28rpx;
        color: var(--theme-text);
        line-height: 1.6;
      }
      
      .quote-source {
        font-size: 24rpx;
        color: var(--theme-text-secondary);
        font-style: italic;
      }
      
      .quote-date {
        font-size: 22rpx;
        color: #cccccc;
      }
    }
    
    .quote-actions {
      display: flex;
      align-items: center;
      
      .action-icon {
        font-size: 32rpx;
        padding: 10rpx;
      }
    }
  }
}

.works-list {
  .work-item {
    background: var(--theme-surface);
    border-radius: 15rpx;
    padding: 20rpx;
    margin-bottom: 15rpx;
    display: flex;
    gap: 15rpx;
    box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.05);
    
    .work-thumbnail {
      width: 120rpx;
      height: 120rpx;
      border-radius: 10rpx;
      overflow: hidden;
      position: relative;
      flex-shrink: 0;
      
      image {
        width: 100%;
        height: 100%;
      }
      
      .work-type-badge {
        position: absolute;
        top: 5rpx;
        right: 5rpx;
        padding: 4rpx 10rpx;
        background: rgba(0, 0, 0, 0.6);
        color: #ffffff;
        font-size: 20rpx;
        border-radius: 10rpx;
      }
      
      &.work-thumbnail-empty {
        background: #f0f0f0;
        display: flex;
        align-items: center;
        justify-content: center;
        
        .work-type-icon {
          font-size: 60rpx;
        }
      }
    }
    
    .work-info {
      flex: 1;
      display: flex;
      flex-direction: column;
      gap: 8rpx;
      
      .work-title {
        font-size: 28rpx;
        font-weight: bold;
        color: var(--theme-text);
      }
      
      .work-desc {
        font-size: 24rpx;
        color: var(--theme-text-secondary);
        overflow: hidden;
        text-overflow: ellipsis;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
      }
      
      .work-meta {
        display: flex;
        gap: 20rpx;
        
        .meta-item {
          font-size: 22rpx;
          color: var(--theme-text-secondary);
        }
      }
    }
    
    .work-actions {
      display: flex;
      flex-direction: column;
      justify-content: center;
      gap: 10rpx;
      
      .action-icon {
        font-size: 32rpx;
        padding: 5rpx;
      }
    }
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 100rpx 30rpx;
  
  .empty-icon {
    font-size: 100rpx;
    margin-bottom: 20rpx;
  }
  
  .empty-text {
    font-size: 28rpx;
    color: var(--theme-text-secondary);
  }
}
</style>
