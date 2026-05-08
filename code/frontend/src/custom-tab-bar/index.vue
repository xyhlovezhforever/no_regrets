<template>
  <view class="tabbar">
    <view 
      v-for="(item, index) in list" 
      :key="index"
      class="tabbar-item"
      :class="{ active: selected === index }"
      @tap="switchTab(index)"
    >
      <text class="icon">{{ item.icon }}</text>
      <text class="text">{{ item.text }}</text>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

const list = [
  { pagePath: '/pages/emotion/index', text: '温暖', icon: '❤️' },
  { pagePath: '/pages/leisure/index', text: '玩乐', icon: '⚡' },
  { pagePath: '/pages/daily/index', text: '生活', icon: '⭐' },
  { pagePath: '/pages/binding/index', text: '绑定', icon: '🔗' },
  { pagePath: '/pages/user/center', text: '空间', icon: '🔮' }
]

const getSelected = () => {
  const pages = getCurrentPages()
  if (!pages.length) {
    console.log('[TabBar] 页面栈为空')
    return 0
  }
  const currentPage = pages[pages.length - 1]
  const route = '/' + (currentPage.route || '')
  console.log('[TabBar] 当前路由:', route)
  console.log('[TabBar] 所有路径:', list.map(item => item.pagePath))
  const index = list.findIndex(item => item.pagePath === route)
  console.log('[TabBar] 匹配索引:', index)
  return index !== -1 ? index : 0
}

const selected = ref(getSelected())

watch(() => getCurrentPages().length, () => {
  console.log('[TabBar] 路由变化，重新检测')
  const index = getSelected()
  if (index !== -1) {
    console.log('[TabBar] 更新选中索引:', selected.value, '->', index)
    selected.value = index
  }
}, { flush: 'post' })

const switchTab = (index: number) => {
  console.log('[TabBar] 点击切换:', index, list[index].text, list[index].pagePath)
  selected.value = index
  uni.switchTab({ url: list[index].pagePath })
}
</script>

<style lang="scss" scoped>
.tabbar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  display: flex;
  height: 100rpx;
  padding-bottom: env(safe-area-inset-bottom);
  background: linear-gradient(135deg, rgba(15, 20, 45, 0.98), rgba(20, 26, 56, 0.95));
  backdrop-filter: blur(20rpx);
  box-shadow: 0 -4rpx 20rpx rgba(0, 0, 0, 0.3);
  border-top: 1rpx solid rgba(0, 217, 255, 0.2);
  z-index: 1000;
}

.tabbar-item {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4rpx;
  transition: all 0.3s ease;
  
  .icon {
    font-size: 48rpx;
    transition: all 0.3s ease;
    filter: grayscale(0.6) brightness(0.7);
  }
  
  .text {
    font-size: 20rpx;
    color: rgba(255, 255, 255, 0.5);
    font-weight: 500;
    transition: all 0.3s ease;
  }
  
  &.active {
    .icon {
      filter: grayscale(0) brightness(1.2);
      transform: scale(1.1);
      text-shadow: 0 0 20rpx rgba(0, 217, 255, 0.8);
    }
    
    .text {
      color: #00D9FF;
      font-weight: 600;
      text-shadow: 0 0 10rpx rgba(0, 217, 255, 0.5);
    }
  }
  
  &:active {
    opacity: 0.7;
  }
}
</style>
