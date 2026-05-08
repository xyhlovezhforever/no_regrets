<template>
  <view class="idol-page">
    <!-- 顶部操作栏 -->
    <view class="top-bar" v-if="isSelectMode">
      <view class="select-info">
        <text>已选择 {{ selectedIds.length }} 个</text>
      </view>
      <view class="top-actions">
        <button class="action-btn cancel-btn" @click="cancelSelectMode">取消</button>
        <button class="action-btn delete-btn" @click="batchDelete" v-if="selectedIds.length > 0">删除</button>
      </view>
    </view>

    <view class="idol-list" v-if="idols.length > 0">
      <view
        v-for="idol in idols"
        :key="idol.id"
        class="idol-card"
        :class="{ selected: selectedIds.includes(idol.id) }"
        @click="handleCardClick(idol.id)"
        @longpress="enableSelectMode(idol.id)"
      >
        <!-- 选中标记 -->
        <view class="select-badge" v-if="isSelectMode">
          <view class="checkbox" :class="{ checked: selectedIds.includes(idol.id) }">
            <text v-if="selectedIds.includes(idol.id)">✔</text>
          </view>
        </view>

        <!-- 操作按钮组 -->
        <view class="action-btns" v-if="!isSelectMode">
          <view class="action-btn edit" @click.stop="gotoEdit(idol.id)">
            <text>✏️</text>
          </view>
          <view class="action-btn delete" @click.stop="deleteSingleIdol(idol.id)">
            <text>🗑️</text>
          </view>
        </view>
        
        <view v-if="idol.avatar_url" class="idol-avatar-wrapper">
          <image class="idol-avatar" :src="getFullUrl(idol.avatar_url)" mode="aspectFill" />
        </view>
        <view v-else class="idol-avatar-wrapper idol-avatar-emoji">
          <text>⭐</text>
        </view>
        <text class="idol-name">{{ idol.name }}</text>
        <text class="idol-category" v-if="idol.profession">{{ idol.profession }}</text>
        <text class="idol-motto" v-if="idol.description">{{ idol.description }}</text>
      </view>

      <!-- 添加偶像卡片 -->
      <view class="idol-card add-card" @click="gotoEdit" v-if="!isSelectMode">
        <text class="add-icon">+</text>
        <text class="add-text">添加偶像</text>
      </view>
    </view>

    <!-- 空状态 -->
    <view v-else class="empty-state">
      <text class="empty-icon">⭐</text>
      <text class="empty-text">还没有添加偶像</text>
      <text class="empty-hint">偶像可以给你力量和鼓励</text>
      <button class="add-btn" @click="gotoEdit">添加偶像</button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { navigateTo } from '@/utils'
import type { Idol } from '@/types/idol'
import { getMyIdolsApi, createIdolApi, deleteIdolApi } from '@/api/idol'
import { BASE_API } from '@/config'

const idols = ref<Idol[]>([])
const isSelectMode = ref(false)
const selectedIds = ref<string[]>([])
const page = ref(1)
const pageSize = ref(20)
const total = ref(0)
const loading = ref(false)

onMounted(() => {
  loadIdols(true)
})

onShow(() => {
  // 每次显示都刷新数据，确保编辑后能看到最新内容
  loadIdols(true)
})

const loadIdols = async (refresh = false) => {
  if (loading.value) return
  
  try {
    loading.value = true
    if (refresh) {
      page.value = 1
      idols.value = []
    }
    
    const response = await getMyIdolsApi({
      page: page.value,
      page_size: pageSize.value
    })
    
    if (!response || !response.items || !Array.isArray(response.items)) {
      console.error('响应数据格式错误:', response)
      return
    }
    
    // 转换后端数据格式，确保ID是字符串
    const convertedItems = response.items.map((idol: any) => ({
      ...idol,
      id: String(idol.id || idol._id || idol.idol_id || '')
    }))
    
    if (refresh) {
      idols.value = convertedItems
    } else {
      idols.value = [...idols.value, ...convertedItems]
    }
    total.value = response.total || 0
  } catch (error: any) {
    console.error('加载偶像列表失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  } finally {
    loading.value = false
  }
}

const viewDetail = (id: string) => {
  navigateTo('/pages/idol/detail', { id })
}

// 处理卡片点击
const handleCardClick = (id: string) => {
  if (isSelectMode.value) {
    toggleSelect(id)
  } else {
    viewDetail(id)
  }
}

// 切换选中状态
const toggleSelect = (id: string) => {
  const index = selectedIds.value.indexOf(id)
  if (index > -1) {
    selectedIds.value.splice(index, 1)
  } else {
    selectedIds.value.push(id)
  }
}

// 启用选择模式（长按）
const enableSelectMode = (id: string) => {
  if (!isSelectMode.value) {
    isSelectMode.value = true
    selectedIds.value = [id]
  }
}

// 取消选择模式
const cancelSelectMode = () => {
  isSelectMode.value = false
  selectedIds.value = []
}

// 批量删除
const batchDelete = () => {
  uni.showModal({
    title: '确认删除',
    content: `确定要删除这 ${selectedIds.value.length} 个偶像吗？`,
    success: async (res) => {
      if (res.confirm) {
        try {
          let successCount = 0
          for (const id of selectedIds.value) {
            try {
              await deleteIdolApi(id)
              successCount++
            } catch (e) {
              console.error('删除失败:', e)
            }
          }
          await loadIdols(true)
          cancelSelectMode()
          uni.showToast({ title: `已删除 ${successCount} 个偶像`, icon: 'success' })
        } catch (error: any) {
          console.error('批量删除失败:', error)
          uni.showToast({ title: '删除失败', icon: 'none' })
        }
      }
    }
  })
}

const gotoEdit = (id?: string) => {
  if (id) {
    navigateTo('/pages/idol/edit', { id })
  } else {
    navigateTo('/pages/idol/edit')
  }
}

const deleteSingleIdol = (id: string) => {
  uni.showModal({
    title: '确认删除',
    content: '确定要删除这个偶像吗？',
    success: async (res) => {
      if (res.confirm) {
        try {
          await deleteIdolApi(id)
          await loadIdols(true)
          uni.showToast({ title: '删除成功', icon: 'success' })
        } catch (error: any) {
          console.error('删除偶像失败:', error)
          uni.showToast({ title: '删除失败', icon: 'none' })
        }
      }
    }
  })
}

const getFullUrl = (url: string | null | undefined) => {
  if (!url) return ''
  if (url.startsWith('http')) return url
  return `${BASE_API.replace('/api/v1', '')}${url}`
}

</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.idol-page {
  @include cyber-page-bg;
  min-height: 100vh;
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
      radial-gradient(circle at 20% 30%, rgba(255, 214, 0, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 80% 70%, rgba(138, 92, 246, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 50% 50%, rgba(0, 217, 255, 0.08) 0%, transparent 60%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
  
  &::after {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: conic-gradient(
      from 0deg,
      transparent 0deg,
      rgba(255, 214, 0, 0.05) 90deg,
      transparent 180deg,
      rgba(138, 92, 246, 0.05) 270deg,
      transparent 360deg
    );
    pointer-events: none;
    animation: bgRotate 20s linear infinite;
    z-index: 0;
  }
}

.top-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 25rpx 30rpx;
  @include neon-card;
  border-radius: 20rpx;
  margin-bottom: 25rpx;
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 50rpx rgba(0, 217, 255, 0.4),
    inset 0 0 40rpx rgba(0, 217, 255, 0.1);
  border: 2rpx solid rgba(0, 217, 255, 0.6);
  position: relative;
  z-index: 10;
  animation: slideDown 0.5s ease-out;

  .select-info {
    font-size: 28rpx;
    @include neon-text(#00D9FF);
    font-weight: bold;
  }

  .top-actions {
    display: flex;
    gap: 15rpx;

    .action-btn {
      padding: 15rpx 30rpx;
      border-radius: 50rpx;
      font-size: 26rpx;
      font-weight: bold;
      line-height: normal;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      position: relative;
      overflow: hidden;

      &::after {
        border: none;
      }

      &.cancel-btn {
        background: linear-gradient(135deg, rgba(30, 36, 66, 0.8) 0%, rgba(30, 36, 66, 0.6) 100%);
        @include neon-text(#b8c5d6);
        border: 2rpx solid rgba(138, 92, 246, 0.5);
        box-shadow: 
          0 4rpx 16rpx rgba(0, 0, 0, 0.3),
          0 0 20rpx rgba(138, 92, 246, 0.2);
        
        &:active {
          transform: scale(0.93);
          @include neon-text(#ffffff);
        }
      }

      &.delete-btn {
        background: linear-gradient(135deg, rgba(255, 0, 79, 0.9) 0%, rgba(238, 90, 111, 0.9) 100%);
        @include neon-text(#ffffff);
        border: 2rpx solid rgba(255, 0, 79, 0.7);
        box-shadow: 
          0 6rpx 20rpx rgba(255, 0, 79, 0.4),
          0 0 30rpx rgba(255, 0, 79, 0.3),
          inset 0 0 20rpx rgba(255, 0, 79, 0.15);
        
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
            rgba(255, 255, 255, 0.25) 90deg,
            transparent 180deg
          );
          animation: rotate 3s linear infinite;
        }
        
        &:active {
          transform: scale(0.93);
          box-shadow: 
            0 4rpx 12rpx rgba(255, 0, 79, 0.6),
            0 0 40rpx rgba(255, 0, 79, 0.5),
            inset 0 0 30rpx rgba(255, 0, 79, 0.25);
        }
      }
    }
  }
}

.idol-list {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 25rpx;
  position: relative;
  z-index: 1;
}

.idol-card {
  @include neon-card;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  position: relative;
  min-height: 280rpx;
  height: 100%;
  animation: cardFloat 3s ease-in-out infinite;
  @include rainbow-border;

  &:active {
    transform: translateY(-8rpx) scale(0.98);
  }

  &.selected {
    background: rgba(102, 126, 234, 0.3);
    border-color: #667eea;
    box-shadow: 
      0 0 40rpx rgba(102, 126, 234, 0.6),
      inset 0 0 40rpx rgba(102, 126, 234, 0.2);
  }

@keyframes cardFloat {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-5rpx); }
}

  .action-btns {
    position: absolute;
    top: 15rpx;
    right: 15rpx;
    z-index: 10;
    display: flex;
    gap: 12rpx;

    .action-btn {
      width: 56rpx;
      height: 56rpx;
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 32rpx;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      backdrop-filter: blur(10rpx);
      position: relative;
      overflow: hidden;

      &.edit {
        background: linear-gradient(135deg, rgba(0, 217, 255, 0.8) 0%, rgba(138, 92, 246, 0.8) 100%);
        border: 2rpx solid rgba(0, 217, 255, 0.6);
        box-shadow: 
          0 4rpx 12rpx rgba(0, 217, 255, 0.4),
          0 0 20rpx rgba(0, 217, 255, 0.3);
        
        &:active {
          transform: scale(0.88);
          box-shadow: 
            0 2rpx 8rpx rgba(0, 217, 255, 0.6),
            0 0 30rpx rgba(0, 217, 255, 0.5);
        }
      }

      &.delete {
        background: linear-gradient(135deg, rgba(255, 0, 79, 0.8) 0%, rgba(238, 90, 111, 0.8) 100%);
        border: 2rpx solid rgba(255, 0, 79, 0.6);
        box-shadow: 
          0 4rpx 12rpx rgba(255, 0, 79, 0.4),
          0 0 20rpx rgba(255, 0, 79, 0.3);
        
        &:active {
          transform: scale(0.88);
          box-shadow: 
            0 2rpx 8rpx rgba(255, 0, 79, 0.6),
            0 0 30rpx rgba(255, 0, 79, 0.5);
        }
      }
    }
  }

  .select-badge {
    position: absolute;
    top: 15rpx;
    right: 15rpx;
    z-index: 10;

    .checkbox {
      width: 48rpx;
      height: 48rpx;
      border-radius: 50%;
      border: 3rpx solid rgba(138, 92, 246, 0.4);
      background: rgba(30, 36, 66, 0.8);
      backdrop-filter: blur(10rpx);
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 26rpx;
      color: #ffffff;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      box-shadow: 
        0 4rpx 12rpx rgba(0, 0, 0, 0.3),
        0 0 20rpx rgba(138, 92, 246, 0.2);

      &.checked {
        background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
        border-color: rgba(0, 217, 255, 0.8);
        box-shadow: 
          0 6rpx 16rpx rgba(0, 217, 255, 0.5),
          0 0 30rpx rgba(0, 217, 255, 0.6),
          inset 0 0 20rpx rgba(0, 217, 255, 0.2);
        transform: scale(1.1);
        animation: checkPulse 0.3s ease-out;
      }
    }
  }

  .idol-avatar-wrapper {
    width: 150rpx;
    height: 150rpx;
    margin-bottom: 15rpx;

    .idol-avatar {
      width: 100%;
      height: 100%;
      border-radius: 75rpx;
      border: 3rpx solid rgba(0, 217, 255, 0.6);
      box-shadow: 
        0 0 20rpx rgba(0, 217, 255, 0.5),
        inset 0 0 20rpx rgba(0, 217, 255, 0.1);
    }

    &.idol-avatar-emoji {
      display: flex;
      align-items: center;
      justify-content: center;
      background: rgba(30, 36, 66, 0.6);
      border-radius: 75rpx;
      font-size: 80rpx;
      border: 3rpx solid rgba(255, 214, 0, 0.6);
      box-shadow: 0 0 20rpx rgba(255, 214, 0, 0.5);
    }
  }

  .idol-name {
    font-size: 30rpx;
    font-weight: bold;
    @include neon-title(#ffffff);
    margin-bottom: 8rpx;
  }

  .idol-category {
    font-size: 24rpx;
    @include neon-text(#FFD600);
    margin-bottom: 10rpx;
  }

  .idol-motto {
    font-size: 22rpx;
    color: #b8c5d6;
    line-height: 1.4;
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    text-shadow: 0 0 8rpx rgba(184, 197, 214, 0.5);
  }

  &.add-card {
    background: linear-gradient(135deg, rgba(255, 214, 0, 0.2) 0%, rgba(138, 92, 246, 0.2) 100%);
    backdrop-filter: blur(15rpx);
    justify-content: center;
    min-height: 280rpx;
    border: 3rpx dashed rgba(255, 214, 0, 0.7);
    box-shadow: 
      0 10rpx 40rpx rgba(0, 0, 0, 0.4),
      0 0 50rpx rgba(255, 214, 0, 0.5),
      inset 0 0 40rpx rgba(255, 214, 0, 0.15);
    position: relative;
    overflow: hidden;
    animation: cardFloat 3s ease-in-out infinite, addCardPulse 3s ease-in-out infinite;
    
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
        rgba(255, 214, 0, 0.15) 90deg,
        transparent 180deg,
        rgba(138, 92, 246, 0.15) 270deg,
        transparent 360deg
      );
      animation: rotate 8s linear infinite;
      z-index: 0;
    }

    .add-icon {
      font-size: 90rpx;
      @include neon-text(#FFD600);
      margin-bottom: 18rpx;
      position: relative;
      z-index: 1;
      filter: drop-shadow(0 0 30rpx rgba(255, 214, 0, 0.9));
      animation: iconBounce 2s ease-in-out infinite;
    }

    .add-text {
      font-size: 30rpx;
      font-weight: bold;
      @include neon-text(#FFD600);
      position: relative;
      z-index: 1;
    }
    
    &:active {
      transform: translateY(-8rpx) scale(0.96);
      border-style: solid;
      box-shadow: 
        0 8rpx 30rpx rgba(0, 0, 0, 0.5),
        0 0 70rpx rgba(255, 214, 0, 0.8),
        inset 0 0 50rpx rgba(255, 214, 0, 0.25);
    }
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 150rpx 30rpx;
  position: relative;
  z-index: 1;

  .empty-icon {
    font-size: 150rpx;
    margin-bottom: 40rpx;
    filter: drop-shadow(0 0 40rpx rgba(255, 214, 0, 1));
    animation: starFloat 3s ease-in-out infinite, starGlow 2s ease-in-out infinite;
  }

  .empty-text {
    font-size: 38rpx;
    @include neon-title(#FFD600);
    margin-bottom: 15rpx;
    animation: textPulse 2s ease-in-out infinite;
  }

  .empty-hint {
    font-size: 28rpx;
    @include neon-text(#00D9FF);
    margin-bottom: 60rpx;
  }

  .add-btn {
    width: 320rpx;
    height: 90rpx;
    background: linear-gradient(135deg, rgba(255, 214, 0, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 3rpx solid rgba(255, 214, 0, 0.7);
    border-radius: 50rpx;
    font-size: 32rpx;
    font-weight: bold;
    line-height: 90rpx;
    box-shadow: 
      0 12rpx 40rpx rgba(255, 214, 0, 0.5),
      0 0 60rpx rgba(255, 214, 0, 0.4),
      inset 0 0 40rpx rgba(255, 214, 0, 0.2);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
    animation: btnPulse 3s ease-in-out infinite;
    
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
      transform: scale(0.95);
      box-shadow: 
        0 8rpx 30rpx rgba(255, 214, 0, 0.7),
        0 0 80rpx rgba(255, 214, 0, 0.6),
        inset 0 0 50rpx rgba(255, 214, 0, 0.3);
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes bgRotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes checkPulse {
  0% {
    transform: scale(0.8);
  }
  50% {
    transform: scale(1.2);
  }
  100% {
    transform: scale(1.1);
  }
}

@keyframes starFloat {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-25rpx) rotate(10deg);
  }
}

@keyframes starGlow {
  0%, 100% {
    filter: drop-shadow(0 0 30rpx rgba(255, 214, 0, 0.8));
  }
  50% {
    filter: drop-shadow(0 0 60rpx rgba(255, 214, 0, 1)) drop-shadow(0 0 80rpx rgba(255, 214, 0, 0.6));
  }
}

@keyframes textPulse {
  0%, 100% {
    text-shadow: 
      0 0 20rpx rgba(255, 214, 0, 1),
      0 0 40rpx rgba(255, 214, 0, 0.8);
  }
  50% {
    text-shadow: 
      0 0 30rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(255, 214, 0, 1),
      0 0 90rpx rgba(255, 214, 0, 0.6);
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 12rpx 40rpx rgba(255, 214, 0, 0.5),
      0 0 60rpx rgba(255, 214, 0, 0.4),
      inset 0 0 40rpx rgba(255, 214, 0, 0.2);
  }
  50% {
    box-shadow: 
      0 15rpx 50rpx rgba(255, 214, 0, 0.7),
      0 0 80rpx rgba(255, 214, 0, 0.6),
      inset 0 0 50rpx rgba(255, 214, 0, 0.3);
  }
}

@keyframes addCardPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(0, 0, 0, 0.4),
      0 0 50rpx rgba(255, 214, 0, 0.5),
      inset 0 0 40rpx rgba(255, 214, 0, 0.15);
  }
  50% {
    box-shadow: 
      0 12rpx 45rpx rgba(0, 0, 0, 0.5),
      0 0 70rpx rgba(255, 214, 0, 0.7),
      inset 0 0 50rpx rgba(255, 214, 0, 0.2);
  }
}

@keyframes iconBounce {
  0%, 100% {
    transform: translateY(0) scale(1);
    filter: drop-shadow(0 0 30rpx rgba(255, 214, 0, 0.9));
  }
  50% {
    transform: translateY(-10rpx) scale(1.1);
    filter: drop-shadow(0 0 45rpx rgba(255, 214, 0, 1)) drop-shadow(0 0 60rpx rgba(255, 214, 0, 0.6));
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>

