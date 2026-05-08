<template>
  <view class="page">
    <!-- 调试区 -->
    <view class="debug-box">
      <text>偶像数量: {{ idols.length }}</text>
      <button @click="loadIdols(true)">刷新</button>
      <button @click="testData">测试数据</button>
    </view>

    <!-- 偶像列表 -->
    <view class="idol-container">
      <view v-for="idol in idols" :key="idol.id" class="idol-item">
        <text class="name">{{ idol.name }}</text>
        <text class="desc">{{ idol.description }}</text>
      </view>
      <view v-if="idols.length === 0" class="empty">
        没有偶像数据
      </view>
    </view>

    <!-- 添加按钮 -->
    <view class="add-btn" @click="addIdol">
      <text>+ 添加偶像</text>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { Idol } from '@/types/idol'
import { getMyIdolsApi, createIdolApi } from '@/api/idol'

const idols = ref<Idol[]>([])
const loading = ref(false)

onMounted(() => {
  loadIdols(true)
})

const loadIdols = async (refresh = false) => {
  if (loading.value) return
  
  try {
    loading.value = true
    console.log('开始加载')
    
    const response = await getMyIdolsApi({ page: 1, page_size: 20 })
    console.log('响应:', response)
    
    const items = response?.items || response?.data?.items || []
    idols.value = Array.isArray(items) ? items : []
    
    console.log('加载完成，数量:', idols.value.length)
    uni.showToast({ title: `加载${idols.value.length}个偶像`, icon: 'none' })
  } catch (error: any) {
    console.error('加载失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  } finally {
    loading.value = false
  }
}

const testData = () => {
  idols.value = [
    { id: '1', name: '测试1', description: '描述1', user_id: '', is_public: true, created_at: '', updated_at: '', avatar_url: null, birth_date: null, nationality: null, profession: null, tags: null },
    { id: '2', name: '测试2', description: '描述2', user_id: '', is_public: true, created_at: '', updated_at: '', avatar_url: null, birth_date: null, nationality: null, profession: null, tags: null }
  ]
  console.log('测试数据:', idols.value)
}

const addIdol = () => {
  uni.showModal({
    title: '添加偶像',
    editable: true,
    placeholderText: '输入偶像姓名',
    success: async (res) => {
      if (res.confirm && res.content) {
        try {
          await createIdolApi({ name: res.content, is_public: true })
          await loadIdols(true)
          uni.showToast({ title: '添加成功', icon: 'success' })
        } catch (error: any) {
          uni.showToast({ title: '添加失败', icon: 'none' })
        }
      }
    }
  })
}
</script>

<style scoped>
.page {
  min-height: 100vh;
  background: var(--theme-background);
  padding: 20rpx;
}

.debug-box {
  background: white;
  padding: 20rpx;
  margin-bottom: 20rpx;
  border-radius: 10rpx;
}

.debug-box text {
  display: block;
  margin-bottom: 10rpx;
}

.debug-box button {
  margin-right: 10rpx;
  margin-top: 10rpx;
}

.idol-container {
  background: white;
  padding: 20rpx;
  border-radius: 10rpx;
  margin-bottom: 20rpx;
}

.idol-item {
  padding: 20rpx;
  margin-bottom: 15rpx;
  background: #f9f9f9;
  border-radius: 8rpx;
}

.name {
  display: block;
  font-size: 32rpx;
  font-weight: bold;
  margin-bottom: 10rpx;
}

.desc {
  display: block;
  font-size: 28rpx;
  color: var(--theme-text-secondary);
}

.empty {
  padding: 60rpx;
  text-align: center;
  color: var(--theme-text-secondary);
}

.add-btn {
  position: fixed;
  bottom: 60rpx;
  right: 60rpx;
  width: 120rpx;
  height: 120rpx;
  background: #1890ff;
  color: white;
  border-radius: 60rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.15);
}

.add-btn text {
  font-size: 28rpx;
}
</style>
