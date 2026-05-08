<template>
  <view class="home-page">
    <!-- 欢迎弹窗 -->
    <WelcomeModal v-model:visible="showWelcomeModal" @confirm="handleWelcomeConfirm" />

    <!-- 顶部横幅 -->
    <view class="hero-section">
      <view class="hero-content">
        <view class="greeting-wrapper">
          <text class="greeting">{{ greeting }}</text>
          <text class="username">{{ userStore.nickname || '朋友' }}</text>
        </view>
        <view class="daily-quote-mini">
          <text class="quote-icon">✨</text>
          <text class="quote-text">{{ dailyQuote }}</text>
        </view>
      </view>
      <view class="hero-decoration">
        <view class="decoration-circle circle-1"></view>
        <view class="decoration-circle circle-2"></view>
        <view class="decoration-circle circle-3"></view>
      </view>
    </view>

    <!-- 情书信封 -->
    <view v-if="unreadLetters.length > 0" class="love-letter-section">
      <view 
        v-for="letter in unreadLetters" 
        :key="letter.id"
        class="mini-envelope" 
        @click="openLetter(letter.id)"
      >
        <view class="envelope-flap"></view>
        <view class="envelope-body">
          <view class="envelope-icon">💌</view>
          <view class="envelope-info">
            <text class="envelope-from">来自 {{ letter.from_name }}</text>
            <text class="envelope-title">{{ letter.title }}</text>
          </view>
          <view class="new-badge">NEW</view>
        </view>
      </view>
    </view>

    <!-- 功能分类 -->
    <scroll-view class="content-scroll" scroll-y>
      <!-- 情感陪伴 -->
      <view v-show="activeTab === 0" class="section tab-content" :class="{ 'content-active': activeTab === 0 }">
        <view class="section-header">
          <view class="section-icon">💝</view>
          <text class="section-title">情感陪伴</text>
          <text class="section-subtitle">温暖你的每一天</text>
        </view>
        <view class="section-cards">
          <view class="feature-card card-ai" @click="handleNavigate('/pages/ai/chat-list')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">🤖</text>
              <text class="card-title">AI 陪伴</text>
              <text class="card-desc">24小时倾听</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <view class="feature-card card-friends" @click="handleNavigate('/pages/friends/list')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">👥</text>
              <text class="card-title">好友聊天</text>
              <text class="card-desc">与好友畅聊</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <view class="feature-card card-warmth" @click="handleNavigate('/pages/warmth/index')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">💌</text>
              <text class="card-title">温情篇</text>
              <text class="card-desc">情书、抱抱</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <view class="feature-card card-idol" @click="handleNavigate('/pages/idol/index')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">🌟</text>
              <text class="card-title">我的偶像</text>
              <text class="card-desc">榜样的力量</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <!-- 暂时注释：私人空间功能
          <view class="feature-card card-private" @click="handleNavigate('/pages/private/space')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">🎵</text>
              <text class="card-title">私人空间</text>
              <text class="card-desc">音乐、光芒、独处</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          -->
        </view>
      </view>

      <!-- 我的日常 -->
      <view v-show="activeTab === 2" class="section tab-content" :class="{ 'content-active': activeTab === 2 }">
        <view class="section-header">
          <view class="section-icon">📝</view>
          <text class="section-title">我的日常</text>
          <text class="section-subtitle">记录美好生活</text>
        </view>
        <view class="section-cards">
          <view class="feature-card card-account" @click="handleNavigate('/pages/self/account')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">💰</text>
              <text class="card-title">记账</text>
              <text class="card-desc">收支管理</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <view class="feature-card card-note" @click="handleNavigate('/pages/self/note')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">📒</text>
              <text class="card-title">便签</text>
              <text class="card-desc">随时记录</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <view class="feature-card card-mirror" @click="handleNavigate('/pages/self/mirror')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">🪞</text>
              <text class="card-title">镜子</text>
              <text class="card-desc">照见真我</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <!-- 暂时注释：去水印功能
          <view class="feature-card card-watermark" @click="handleNavigate('/pages/tools/watermark-remover')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">💧</text>
              <text class="card-title">去水印</text>
              <text class="card-desc">图片处理</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          -->
          <view class="feature-card card-md" @click="handleNavigate('/pages/tools/md-editor')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">📝</text>
              <text class="card-title">Markdown编辑器</text>
              <text class="card-desc">文档编辑</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <!-- 暂时注释：数羊助眠功能
          <view class="feature-card card-sheep" @click="handleNavigate('/pages/release/count-sheep')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">🐑</text>
              <text class="card-title">数羊助眠</text>
              <text class="card-desc">安然入睡</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          -->
        </view>
      </view>

      <!-- 休闲娱乐 -->
      <view v-show="activeTab === 1" class="section tab-content" :class="{ 'content-active': activeTab === 1 }">
        <view class="section-header">
          <view class="section-icon">🎮</view>
          <text class="section-title">休闲娱乐</text>
          <text class="section-subtitle">轻松娱乐，放松心情</text>
        </view>
        <view class="section-cards">
          <view class="feature-card card-forum" @click="handleNavigate('/pages/forum/index')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">💬</text>
              <text class="card-title">论坛</text>
              <text class="card-desc">灵感、话题、创作</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <view class="feature-card card-brain" @click="handleNavigate('/pages/release/brain-teaser')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">💡</text>
              <text class="card-title">脑筋急转弯</text>
              <text class="card-desc">挑战智慧</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
          <view class="feature-card card-game" @click="handleNavigate('/pages/games/index')">
            <view class="card-bg"></view>
            <view class="card-content">
              <text class="card-icon">🎮</text>
              <text class="card-title">游戏中心</text>
              <text class="card-desc">多款小游戏</text>
            </view>
            <view class="card-arrow">→</view>
          </view>
        </view>
      </view>

      <!-- 底部间距 -->
      <view class="bottom-space"></view>
    </scroll-view>

    <!-- 超炫底部Tab导航栏 -->
    <view class="ultra-tab-bar">
      <view class="tab-bg-glow"></view>
      <view class="tab-container">
        <view 
          v-for="(tab, index) in tabs" 
          :key="tab.id"
          class="tab-item"
          :class="{ active: activeTab === index }"
          @click="switchTab(index)"
        >
          <view class="tab-icon">{{ tab.icon }}</view>
          <text class="tab-name">{{ tab.name }}</text>
          <view class="tab-glow" :style="{ background: tab.color }"></view>
        </view>
        <view class="tab-indicator" :style="{ transform: `translateX(${activeTab * 100}%)` }"></view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { onPullDownRefresh, onShow } from '@dcloudio/uni-app'
import { useAppStore, useUserStore } from '@/store'
import { navigateTo } from '@/utils'
import type { LoveLetter } from '@/types'
import { getReceivedLettersApi } from '@/api/loveLetter'
import { getWebSocketClient, type WsMessage } from '@/utils/websocket'
import WelcomeModal from '@/components/WelcomeModal.vue'

const appStore = useAppStore()
const userStore = useUserStore()
const wsClient = getWebSocketClient()

const showWelcomeModal = ref(false)
const dailyQuote = ref('平凡的人也要为自己的梦想而努力')
const unreadLetters = ref<LoveLetter[]>([])

// Tab系统
const activeTab = ref(0) // 0: 情感陪伴, 1: 休闲娱乐, 2: 我的日常
const tabs = [
  { id: 0, name: '情感陪伴', icon: '💝', color: '#FF00D6' },
  { id: 1, name: '休闲娱乐', icon: '🎮', color: '#00D9FF' },
  { id: 2, name: '我的日常', icon: '📝', color: '#FFD600' }
]

// 切换Tab
const switchTab = (index: number) => {
  if (index === activeTab.value) return
  activeTab.value = index
}

// 问候语
const greeting = computed(() => {
  const hour = new Date().getHours()
  if (hour < 6) return '夜深了'
  if (hour < 9) return '早上好'
  if (hour < 12) return '上午好'
  if (hour < 14) return '中午好'
  if (hour < 18) return '下午好'
  if (hour < 22) return '晚上好'
  return '夜深了'
})

// 页面加载
onMounted(() => {
  // 检查是否首次访问
  // 暂时禁用欢迎弹窗，避免初始化问题
  // if (appStore.isFirstVisit && appStore.showWelcomeModal) {
  //   showWelcomeModal.value = true
  // }
  
  // 注册 WebSocket 消息处理器，用于实时情书提醒
  wsClient.onMessage(handleWsMessage)
  if (!wsClient.isConnected()) {
    wsClient.connect()
  }
})

// 确认欢迎弹窗
const handleWelcomeConfirm = (dontShowToday: boolean) => {
  if (dontShowToday) {
    appStore.setShowWelcomeModal(false)
  }
  appStore.setFirstVisit(false)
}

// 页面跳转
const handleNavigate = (url: string) => {
  navigateTo(url)
}

// 加载未读情书
const loadUnreadLetters = async () => {
  try {
    const data = await getReceivedLettersApi()
    // 过滤未读情书
    unreadLetters.value = data.filter(letter => !letter.is_read)
  } catch (error) {
    console.error('加载情书失败:', error)
  }
}

// 打开情书
const openLetter = (id: string) => {
  navigateTo(`/pages/loveLetter/detail?id=${id}`)
}

// WebSocket 消息处理：收到情书消息时刷新未读情书列表
const handleWsMessage = (message: WsMessage) => {
  if (message.type === 'message') {
    const msg = message as any
    const msgToId = String(msg.to_user_id || '')
    const currentUserId = String(userStore.userId || '')

    // 只处理发给当前用户且类型为情书的消息
    if (msgToId === currentUserId && msg.message_type === 'love_letter') {
      loadUnreadLetters()
    }
  }
}

// 页面显示时加载数据
onShow(() => {
  loadUnreadLetters()
})

onUnmounted(() => {
  wsClient.offMessage(handleWsMessage)
})

// 下拉刷新
onPullDownRefresh(() => {
  // 刷新数据
  setTimeout(() => {
    uni.stopPullDownRefresh()
  }, 1000)
})
</script>

<style lang="scss" scoped>
.home-page {
  min-height: 100vh;
  background: #0a0e27;
  background-image: 
    radial-gradient(circle at 10% 20%, rgba(0, 217, 255, 0.15) 0%, transparent 40%),
    radial-gradient(circle at 90% 80%, rgba(255, 0, 214, 0.15) 0%, transparent 40%),
    radial-gradient(circle at 50% 50%, rgba(255, 214, 0, 0.1) 0%, transparent 50%);
  display: flex;
  flex-direction: column;
  position: relative;
  overflow: hidden;
}

.home-page::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    linear-gradient(135deg, transparent 0%, rgba(0, 217, 255, 0.05) 50%, transparent 100%),
    linear-gradient(45deg, transparent 0%, rgba(255, 0, 214, 0.05) 50%, transparent 100%);
  background-size: 400% 400%;
  animation: gradientShift 15s ease infinite;
  pointer-events: none;
}

@keyframes gradientShift {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

/* 顶部横幅 */
.hero-section {
  position: relative;
  padding: 60rpx 30rpx 40rpx;
  background: rgba(30, 36, 66, 0.8);
  -webkit-backdrop-filter: blur(20rpx);
  backdrop-filter: blur(20rpx);
  border-radius: 0 0 50rpx 50rpx;
  overflow: hidden;
  z-index: 1;
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(0, 217, 255, 0.3),
    inset 0 -2rpx 20rpx rgba(0, 217, 255, 0.2);
  border: 1rpx solid rgba(0, 217, 255, 0.3);

  .hero-content {
    position: relative;
    z-index: 2;
  }

  .greeting-wrapper {
    margin-bottom: 30rpx;

    .greeting {
      display: block;
      font-size: 28rpx;
      color: #00D9FF;
      margin-bottom: 10rpx;
      text-shadow: 0 0 10rpx rgba(0, 217, 255, 0.8);
    }

    .username {
      display: block;
      font-size: 56rpx;
      font-weight: bold;
      color: #ffffff;
      text-shadow: 
        0 0 20rpx rgba(0, 217, 255, 0.8),
        0 0 40rpx rgba(0, 217, 255, 0.5),
        0 4rpx 8rpx rgba(0, 0, 0, 0.5);
    }
  }

  .daily-quote-mini {
    display: flex;
    align-items: center;
    background: rgba(30, 36, 66, 0.6);
    -webkit-backdrop-filter: blur(10rpx);
    backdrop-filter: blur(10rpx);
    border-radius: 20rpx;
    padding: 20rpx 25rpx;
    border: 1rpx solid rgba(255, 0, 214, 0.4);
    box-shadow: 
      0 0 20rpx rgba(255, 0, 214, 0.3),
      inset 0 0 20rpx rgba(255, 0, 214, 0.1);

    .quote-icon {
      font-size: 32rpx;
      margin-right: 15rpx;
    }

    .quote-text {
      flex: 1;
      font-size: 26rpx;
      color: #ffffff;
      line-height: 1.6;
    }
  }

  .hero-decoration {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    z-index: 1;

    .decoration-circle {
      position: absolute;
      border-radius: 50%;
      background: rgba(255, 255, 255, 0.1);
      animation: float 6s ease-in-out infinite;

      &.circle-1 {
        width: 200rpx;
        height: 200rpx;
        top: -100rpx;
        right: -50rpx;
        animation-delay: 0s;
      }

      &.circle-2 {
        width: 150rpx;
        height: 150rpx;
        bottom: -50rpx;
        left: 50rpx;
        animation-delay: 2s;
      }

      &.circle-3 {
        width: 100rpx;
        height: 100rpx;
        top: 50%;
        right: 100rpx;
        animation-delay: 4s;
      }
    }
  }
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) scale(1);
    opacity: 0.5;
  }
  50% {
    transform: translateY(-30rpx) scale(1.1);
    opacity: 0.8;
  }
}

/* 内容滚动区域 */
.content-scroll {
  flex: 1;
  padding: 30rpx;
  position: relative;
  z-index: 2;
}

/* 功能分类区域 */
.section {
  margin-bottom: 50rpx;
  animation: fadeInUp 0.6s ease-out;

  .section-header {
    display: flex;
    align-items: center;
    margin-bottom: 30rpx;
    background: rgba(30, 36, 66, 0.7);
    -webkit-backdrop-filter: blur(10rpx);
    backdrop-filter: blur(10rpx);
    padding: 20rpx 25rpx;
    border-radius: 20rpx;
    box-shadow: 
      0 0 30rpx rgba(255, 214, 0, 0.3),
      0 4rpx 15rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(255, 214, 0, 0.1);
    border: 1rpx solid rgba(255, 214, 0, 0.4);

    .section-icon {
      font-size: 44rpx;
      margin-right: 15rpx;
      filter: drop-shadow(0 0 10rpx rgba(255, 214, 0, 0.8));
      animation: iconBounce 2s ease-in-out infinite;
    }

    .section-title {
      font-size: 38rpx;
      font-weight: bold;
      color: #FFD600;
      margin-right: 15rpx;
      text-shadow: 
        0 0 15rpx rgba(255, 214, 0, 0.8),
        0 0 30rpx rgba(255, 214, 0, 0.5);
    }

    .section-subtitle {
      font-size: 24rpx;
      color: #ffffff;
      text-shadow: 0 0 10rpx rgba(255, 255, 255, 0.5);
    }
  }

  .section-cards {
    display: flex;
    flex-direction: column;
    gap: 25rpx;
  }
}

@keyframes iconBounce {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
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

/* 情书信封 */
.love-letter-section {
  padding: 0 30rpx 30rpx;
  position: relative;
  z-index: 2;
  
  .mini-envelope {
    position: relative;
    width: 100%;
    height: 180rpx;
    margin-bottom: 20rpx;
    cursor: pointer;
    filter: drop-shadow(0 10rpx 30rpx rgba(255, 107, 157, 0.4));
    transition: all 0.3s ease;
    animation: envelopeFloat 3s ease-in-out infinite;

    &:active {
      transform: scale(0.95) translateY(-5rpx);
      filter: drop-shadow(0 15rpx 40rpx rgba(255, 107, 157, 0.6));
    }
    
    .envelope-flap {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      width: 0;
      height: 0;
      border-left: 340rpx solid transparent;
      border-right: 340rpx solid transparent;
      border-top: 100rpx solid #ff6b9d;
      transform-origin: center bottom;
      z-index: 2;
    }
    
    .envelope-body {
      position: absolute;
      top: 90rpx;
      left: 0;
      right: 0;
      width: 680rpx;
      height: 120rpx;
      background: linear-gradient(135deg, #ff8fab 0%, #ffa8bd 100%);
      border-radius: 0 0 20rpx 20rpx;
      box-shadow: 0 8rpx 20rpx rgba(255, 107, 157, 0.3);
      display: flex;
      align-items: center;
      padding: 0 30rpx;
      gap: 20rpx;
      
      .envelope-icon {
        font-size: 50rpx;
        flex-shrink: 0;
      }
      
      .envelope-info {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: 10rpx;
        overflow: hidden;
        
        .envelope-from {
          font-size: 24rpx;
          color: rgba(255, 255, 255, 0.9);
        }
        
        .envelope-title {
          font-size: 28rpx;
          color: #ffffff;
          font-weight: bold;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
      }
      
      .new-badge {
        position: absolute;
        top: -10rpx;
        right: -10rpx;
        background: #ff4757;
        color: #ffffff;
        font-size: 20rpx;
        font-weight: bold;
        padding: 4rpx 12rpx;
        border-radius: 20rpx;
        box-shadow: 0 4rpx 12rpx rgba(255, 71, 87, 0.4);
        z-index: 3;
      }
    }
  }
}

@keyframes envelopeFloat {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-10rpx) rotate(2deg);
  }
}

/* ============= 超炫Tab导航栏 ============= */
.ultra-tab-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 15rpx 20rpx;
  padding-bottom: calc(15rpx + env(safe-area-inset-bottom));
  background: linear-gradient(135deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 1) 100%);
  backdrop-filter: blur(40rpx);
  overflow: visible;
  box-shadow: 
    0 -20rpx 60rpx rgba(0, 0, 0, 0.7),
    0 0 80rpx rgba(0, 217, 255, 0.5),
    0 0 120rpx rgba(255, 0, 214, 0.4),
    inset 0 2rpx 50rpx rgba(0, 217, 255, 0.15);
  border-top: 3rpx solid transparent;
  background-clip: padding-box;
  z-index: 100;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 3rpx;
    background: linear-gradient(90deg, #00D9FF, #FF00D6, #FFD600, #8B5CF6, #00D9FF);
    background-size: 400% 100%;
    animation: tabBorderFlow 3s linear infinite;
    z-index: -1;
  }
  
  .tab-bg-glow {
    position: absolute;
    top: -100%;
    left: 50%;
    transform: translateX(-50%);
    width: 200%;
    height: 200%;
    background: radial-gradient(ellipse at center, rgba(0, 217, 255, 0.3) 0%, transparent 60%);
    animation: glowPulse 3s ease-in-out infinite;
    pointer-events: none;
  }
}

.tab-container {
  position: relative;
  display: flex;
  gap: 10rpx;
  background: transparent;
  padding: 0;
}

.tab-item {
  flex: 1;
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 18rpx 10rpx 22rpx;
  border-radius: 20rpx;
  cursor: pointer;
  transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);
  overflow: visible;
  z-index: 1;
  transform: scale(1) translateY(0) translateZ(0);
  transform-style: preserve-3d;
  
  .tab-icon {
    font-size: 48rpx;
    margin-bottom: 8rpx;
    transition: all 0.5s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    filter: grayscale(0.6) brightness(0.8);
    transform: scale(1) rotate(0deg);
  }
  
  .tab-name {
    font-size: 24rpx;
    color: rgba(255, 255, 255, 0.5);
    font-weight: 600;
    transition: all 0.4s ease;
    letter-spacing: 0.5rpx;
  }
  
  .tab-glow {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border-radius: 32rpx;
    opacity: 0;
    filter: blur(20rpx);
    transition: all 0.4s ease;
    z-index: -1;
  }
  
  &.active {
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.7) 0%, rgba(20, 26, 56, 0.8) 100%);
    transform: scale(1.08) translateY(-10rpx) translateZ(20rpx);
    box-shadow: 
      0 -10rpx 40rpx rgba(0, 0, 0, 0.6),
      0 0 50rpx rgba(0, 217, 255, 0.7),
      0 0 80rpx rgba(255, 0, 214, 0.5),
      inset 0 0 40rpx rgba(0, 217, 255, 0.2);
    
    .tab-icon {
      filter: grayscale(0) brightness(1.3) drop-shadow(0 0 20rpx currentColor);
      transform: scale(1.2) rotate(360deg);
      animation: iconBounceActive 0.6s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    }
    
    .tab-name {
      color: #ffffff;
      text-shadow: 
        0 0 15rpx rgba(255, 255, 255, 0.8),
        0 0 30rpx rgba(0, 217, 255, 0.6);
      letter-spacing: 1.5rpx;
      transform: scale(1.1);
    }
    
    .tab-glow {
      opacity: 0.6;
      animation: tabGlowPulse 2s ease-in-out infinite;
    }
  }
  
  &:active {
    transform: scale(0.95) translateY(0);
  }
}

.tab-indicator {
  position: absolute;
  top: 0;
  left: 0;
  width: calc(33.333% - 6.667rpx);
  height: 8rpx;
  background: linear-gradient(90deg, #00D9FF, #FF00D6, #FFD600);
  border-radius: 0 0 4rpx 4rpx;
  transition: transform 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
  box-shadow: 
    0 5rpx 25rpx rgba(0, 217, 255, 1),
    0 10rpx 50rpx rgba(255, 0, 214, 0.8),
    0 0 30rpx rgba(255, 214, 0, 0.6);
  animation: indicatorGlow 2s ease-in-out infinite;
  z-index: 2;
}

/* Tab内容动画 */
.tab-content {
  animation: contentFadeIn 0.6s ease-out;
  
  &.content-active {
    animation: contentSlideIn 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
  }
}

/* ============= Tab动画 ============= */
@keyframes tabBorderFlow {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

@keyframes iconBounceActive {
  0% {
    transform: scale(1) rotate(0deg);
  }
  50% {
    transform: scale(1.3) rotate(180deg);
  }
  100% {
    transform: scale(1.2) rotate(360deg);
  }
}

@keyframes tabGlowPulse {
  0%, 100% {
    opacity: 0.6;
    filter: blur(20rpx);
  }
  50% {
    opacity: 0.9;
    filter: blur(30rpx);
  }
}

@keyframes indicatorGlow {
  0%, 100% {
    box-shadow: 
      0 5rpx 25rpx rgba(0, 217, 255, 1),
      0 10rpx 50rpx rgba(255, 0, 214, 0.8),
      0 0 30rpx rgba(255, 214, 0, 0.6);
  }
  50% {
    box-shadow: 
      0 8rpx 35rpx rgba(0, 217, 255, 1),
      0 15rpx 70rpx rgba(255, 0, 214, 1),
      0 0 50rpx rgba(255, 214, 0, 0.8),
      0 20rpx 90rpx rgba(138, 92, 246, 0.6);
  }
}

/* 底部间距 - 为Tab导航栏留出空间 */
.bottom-space {
  height: 180rpx;
}

@keyframes contentFadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes contentSlideIn {
  0% {
    opacity: 0;
    transform: translateX(-50rpx);
  }
  100% {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes glowPulse {
  0%, 100% {
    opacity: 0.3;
    transform: translate(-50%, -50%) scale(1);
  }
  50% {
    opacity: 0.6;
    transform: translate(-50%, -50%) scale(1.2);
  }
}

/* 功能卡片 */
.feature-card {
  position: relative;
  border-radius: 32rpx;
  padding: 50rpx 40rpx;
  overflow: visible;
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.6),
    0 0 40rpx rgba(0, 217, 255, 0.5),
    0 0 80rpx rgba(255, 0, 214, 0.3),
    inset 0 0 40rpx rgba(0, 217, 255, 0.15),
    0 30rpx 90rpx rgba(0, 0, 0, 0.3);
  transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
  display: flex;
  align-items: center;
  justify-content: space-between;
  -webkit-backdrop-filter: blur(15rpx);
  backdrop-filter: blur(15rpx);
  transform: translateY(0) scale(1) perspective(1500px) rotateX(0deg) rotateY(0deg) rotateZ(0deg) translateZ(0);
  transform-style: preserve-3d;
  animation: cardFloat 3s ease-in-out infinite, auroraShift 8s ease-in-out infinite;
  border: 3rpx solid transparent;
  background-clip: padding-box;
  cursor: pointer;
  min-height: 150rpx;
  
  /* 彩虹边框层 */
  &::before {
    content: '';
    position: absolute;
    top: -3rpx;
    left: -3rpx;
    right: -3rpx;
    bottom: -3rpx;
    border-radius: 32rpx;
    background: linear-gradient(135deg, #00D9FF, #FF00D6, #FFD600, #8B5CF6, #00D9FF);
    background-size: 400% 400%;
    animation: ultraGradient 2s linear infinite;
    z-index: -1;
    opacity: 0.9;
    transition: all 0.3s ease;
    filter: blur(2rpx) brightness(1.2);
  }

  /* 能量涟漪层 */
  &::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 0;
    height: 0;
    border-radius: 50%;
    background: radial-gradient(circle, 
      rgba(0, 217, 255, 0.8) 0%,
      rgba(255, 0, 214, 0.6) 30%,
      rgba(255, 214, 0, 0.4) 60%,
      transparent 100%
    );
    transform: translate(-50%, -50%);
    opacity: 0;
    transition: all 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
    pointer-events: none;
    box-shadow: 
      0 0 60rpx rgba(0, 217, 255, 1),
      0 0 120rpx rgba(255, 0, 214, 0.8);
  }

  &:active {
    transform: translateY(-25rpx) scale(0.96) perspective(1500px) rotateX(8deg) rotateY(15deg) rotateZ(720deg) translateZ(50rpx);
    box-shadow: 
      0 40rpx 100rpx rgba(0, 0, 0, 0.8),
      0 0 120rpx rgba(0, 217, 255, 1),
      0 0 180rpx rgba(255, 0, 214, 1),
      0 0 240rpx rgba(255, 214, 0, 0.8),
      inset 0 0 80rpx rgba(0, 217, 255, 0.5),
      inset 0 0 120rpx rgba(255, 0, 214, 0.3);
    animation: 
      megaFlip 0.8s cubic-bezier(0.68, -0.55, 0.265, 1.55),
      glowPulseUltra 0.8s ease-in-out;
    
    &::before {
      opacity: 1;
      animation: ultraGradient 0.2s linear infinite, rainbowBlast 0.8s ease-out;
      filter: blur(0) brightness(2);
      transform: scale(1.1);
    }

    &::after {
      width: 800rpx;
      height: 800rpx;
      opacity: 1;
      animation: superRipple 0.8s ease-out;
    }
  }

/* 超级3D翻转动画 */
@keyframes megaFlip {
  0% {
    transform: translateY(0) scale(1) perspective(1500px) rotateX(0deg) rotateY(0deg) rotateZ(0deg) translateZ(0);
  }
  25% {
    transform: translateY(-30rpx) scale(0.9) perspective(1500px) rotateX(15deg) rotateY(10deg) rotateZ(180deg) translateZ(30rpx);
  }
  50% {
    transform: translateY(-35rpx) scale(0.92) perspective(1500px) rotateX(10deg) rotateY(20deg) rotateZ(360deg) translateZ(60rpx);
  }
  75% {
    transform: translateY(-30rpx) scale(0.94) perspective(1500px) rotateX(12deg) rotateY(15deg) rotateZ(540deg) translateZ(40rpx);
  }
  100% {
    transform: translateY(-25rpx) scale(0.96) perspective(1500px) rotateX(8deg) rotateY(15deg) rotateZ(720deg) translateZ(50rpx);
  }
}

/* 超级光晕脉冲 */
@keyframes glowPulseUltra {
  0%, 100% {
    filter: brightness(1) saturate(1);
  }
  25% {
    filter: brightness(1.5) saturate(1.5);
  }
  50% {
    filter: brightness(2) saturate(2);
  }
  75% {
    filter: brightness(1.5) saturate(1.5);
  }
}

/* 彩虹爆发 */
@keyframes rainbowBlast {
  0% {
    transform: scale(1);
    filter: blur(2rpx) brightness(1.2) hue-rotate(0deg);
  }
  50% {
    transform: scale(1.15);
    filter: blur(0) brightness(2.5) hue-rotate(180deg);
  }
  100% {
    transform: scale(1.1);
    filter: blur(0) brightness(2) hue-rotate(360deg);
  }
}

/* 超级涟漪扩散 */
@keyframes superRipple {
  0% {
    width: 0;
    height: 0;
    opacity: 1;
    transform: translate(-50%, -50%) scale(0) rotate(0deg);
  }
  50% {
    opacity: 0.8;
    transform: translate(-50%, -50%) scale(1.2) rotate(180deg);
  }
  100% {
    width: 800rpx;
    height: 800rpx;
    opacity: 0;
    transform: translate(-50%, -50%) scale(1) rotate(360deg);
  }
}

/* 超级渐变动画 */
@keyframes ultraGradient {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

/* 极光变换 */
@keyframes auroraShift {
  0%, 100% {
    filter: hue-rotate(0deg) brightness(1);
  }
  50% {
    filter: hue-rotate(30deg) brightness(1.2);
  }
}

  .card-bg {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    opacity: 0.95;
    transition: all 0.4s;
  }

  &:active .card-bg {
    opacity: 1;
    transform: scale(1.05);
  }

  .card-content {
    flex: 1;
    display: flex;
    align-items: center;
    position: relative;
    z-index: 1;

    .card-icon {
      font-size: 80rpx;
      margin-right: 25rpx;
      filter: drop-shadow(0 4rpx 8rpx rgba(0, 0, 0, 0.3));
      transition: all 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);
      animation: iconPulse 2s ease-in-out infinite;
      transform-origin: center;
    }

    .card-title {
      display: block;
      font-size: 38rpx;
      font-weight: bold;
      color: #ffffff;
      margin-bottom: 10rpx;
      text-shadow: 
        0 0 15rpx rgba(255, 255, 255, 0.8),
        0 0 30rpx rgba(0, 217, 255, 0.6),
        0 4rpx 8rpx rgba(0, 0, 0, 0.5);
      letter-spacing: 1.5rpx;
      transition: all 0.3s ease;
    }

    .card-desc {
      display: block;
      font-size: 26rpx;
      color: #00D9FF;
      text-shadow: 
        0 0 10rpx rgba(0, 217, 255, 0.8),
        0 2rpx 4rpx rgba(0, 0, 0, 0.3);
      transition: all 0.3s ease;
    }
  }

  &:active .card-content {
    .card-icon {
      transform: scale(1.6) rotate(720deg) translateY(-15rpx) translateZ(30rpx);
      filter: 
        drop-shadow(0 0 30rpx rgba(0, 217, 255, 1)) 
        drop-shadow(0 0 60rpx rgba(255, 0, 214, 1))
        drop-shadow(0 0 90rpx rgba(255, 214, 0, 0.8))
        brightness(1.5);
      animation: ultraIconExplosion 0.8s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    }

    .card-title {
      text-shadow: 
        0 0 40rpx rgba(255, 255, 255, 1),
        0 0 80rpx rgba(0, 217, 255, 1),
        0 0 120rpx rgba(255, 0, 214, 1),
        0 0 160rpx rgba(255, 214, 0, 0.8),
        0 4rpx 8rpx rgba(0, 0, 0, 0.5);
      letter-spacing: 3rpx;
      transform: translateZ(20rpx);
    }

    .card-desc {
      text-shadow: 
        0 0 30rpx rgba(0, 217, 255, 1),
        0 0 60rpx rgba(0, 217, 255, 1),
        0 0 90rpx rgba(255, 0, 214, 0.8),
        0 2rpx 4rpx rgba(0, 0, 0, 0.3);
      transform: translateZ(15rpx);
    }
  }

  .card-arrow {
    font-size: 50rpx;
    color: rgba(255, 255, 255, 0.9);
    position: relative;
    z-index: 1;
    transition: all 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    filter: drop-shadow(0 2rpx 4rpx rgba(0, 0, 0, 0.2));
    
    &::before {
      content: '→→→';
      position: absolute;
      left: 0;
      opacity: 0;
      transform: translateX(-20rpx);
      transition: all 0.3s ease;
      letter-spacing: -10rpx;
    }
  }

  &:active .card-arrow {
    transform: translateX(35rpx) scale(1.6) rotate(720deg) translateZ(40rpx);
    filter: 
      drop-shadow(0 0 25rpx rgba(0, 217, 255, 1)) 
      drop-shadow(0 0 50rpx rgba(255, 0, 214, 1))
      drop-shadow(0 0 75rpx rgba(255, 214, 0, 0.8))
      brightness(1.8);
    animation: megaArrowBlast 0.8s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    
    &::before {
      opacity: 0.7;
      transform: translateX(-60rpx) scale(0.8);
    }
  }
}

/* 超级图标爆炸动画 */
@keyframes ultraIconExplosion {
  0% {
    transform: scale(1) rotate(0deg) translateY(0) translateZ(0);
  }
  15% {
    transform: scale(1.4) rotate(180deg) translateY(-20rpx) translateZ(20rpx);
  }
  30% {
    transform: scale(1.8) rotate(360deg) translateY(-25rpx) translateZ(40rpx);
  }
  50% {
    transform: scale(2) rotate(540deg) translateY(-30rpx) translateZ(50rpx);
  }
  70% {
    transform: scale(1.7) rotate(630deg) translateY(-20rpx) translateZ(35rpx);
  }
  100% {
    transform: scale(1.6) rotate(720deg) translateY(-15rpx) translateZ(30rpx);
  }
}

/* 超级箭头爆炸动画 */
@keyframes megaArrowBlast {
  0% {
    transform: translateX(0) scale(1) rotate(0deg) translateZ(0);
  }
  20% {
    transform: translateX(40rpx) scale(1.8) rotate(180deg) translateZ(30rpx);
  }
  40% {
    transform: translateX(45rpx) scale(2) rotate(360deg) translateZ(50rpx);
  }
  60% {
    transform: translateX(50rpx) scale(1.9) rotate(540deg) translateZ(45rpx);
  }
  80% {
    transform: translateX(40rpx) scale(1.7) rotate(630deg) translateZ(42rpx);
  }
  100% {
    transform: translateX(35rpx) scale(1.6) rotate(720deg) translateZ(40rpx);
  }
}

@keyframes cardFloat {
  0%, 100% {
    transform: translateY(0) scale(1);
  }
  50% {
    transform: translateY(-8rpx) scale(1.01);
  }
}

@keyframes iconPulse {
  0%, 100% {
    transform: scale(1);
    filter: 
      drop-shadow(0 4rpx 8rpx rgba(0, 0, 0, 0.3))
      drop-shadow(0 0 10rpx rgba(0, 217, 255, 0.3));
  }
  25% {
    transform: scale(1.06);
    filter: 
      drop-shadow(0 0 15rpx rgba(0, 217, 255, 0.8))
      drop-shadow(0 0 25rpx rgba(255, 0, 214, 0.4));
  }
  50% {
    transform: scale(1.12);
    filter: 
      drop-shadow(0 0 20rpx rgba(0, 217, 255, 1))
      drop-shadow(0 0 35rpx rgba(255, 0, 214, 0.6))
      drop-shadow(0 0 50rpx rgba(255, 214, 0, 0.4));
  }
  75% {
    transform: scale(1.06);
    filter: 
      drop-shadow(0 0 15rpx rgba(0, 217, 255, 0.8))
      drop-shadow(0 0 25rpx rgba(255, 0, 214, 0.4));
  }
}

/* 不同卡片的交错动画延迟 */
.card-ai {
  animation-delay: 0s;
}

.card-friends {
  animation-delay: 0.15s;
}

.card-warmth {
  animation-delay: 0.3s;
}

.card-idol {
  animation-delay: 0.45s;
}

.card-private {
  animation-delay: 0.6s;
}

.card-release {
  animation-delay: 0.75s;
}

.card-writing {
  animation-delay: 0.9s;
}

.card-tools {
  animation-delay: 1.05s;
}

.card-game {
  animation-delay: 1.2s;
}

/* 卡片渐变背景 */
.card-ai .card-bg {
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.9) 0%, rgba(118, 75, 162, 0.9) 100%);
  box-shadow: inset 0 0 40rpx rgba(102, 126, 234, 0.3);
}

.card-friends .card-bg {
  background: linear-gradient(135deg, rgba(67, 233, 123, 0.9) 0%, rgba(56, 249, 215, 0.9) 100%);
  box-shadow: inset 0 0 40rpx rgba(67, 233, 123, 0.3);
}

.card-warmth .card-bg {
  background: linear-gradient(135deg, rgba(250, 112, 154, 0.9) 0%, rgba(254, 225, 64, 0.9) 100%);
  box-shadow: inset 0 0 40rpx rgba(250, 112, 154, 0.3);
}

.card-idol .card-bg {
  background: linear-gradient(135deg, rgba(240, 147, 251, 0.9) 0%, rgba(245, 87, 108, 0.9) 100%);
  box-shadow: inset 0 0 40rpx rgba(240, 147, 251, 0.3);
}

.card-private .card-bg {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  position: relative;
  overflow: hidden;
}

.card-private .card-bg::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: conic-gradient(from 0deg, rgba(255, 255, 255, 0) 0deg, rgba(255, 255, 255, 0.3) 90deg, rgba(255, 255, 255, 0) 180deg, rgba(255, 255, 255, 0.3) 270deg, rgba(255, 255, 255, 0) 360deg);
  animation: rotate 8s linear infinite;
}

.card-private .card-bg::after {
  content: '';
  position: absolute;
  inset: 20rpx;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 16rpx;
  z-index: 1;
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.card-note .card-bg {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
}

.card-mirror .card-bg {
  background: linear-gradient(135deg, #a8edea 0%, #fed6e3 100%);
}

.card-watermark .card-bg {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.card-md .card-bg {
  background: linear-gradient(135deg, #30cfd0 0%, #330867 100%);
}

.card-sheep .card-bg {
  background: linear-gradient(135deg, #0f2027 0%, #2c5364 100%);
}

.card-forum .card-bg {
  background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
}

.card-brain .card-bg {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.card-game .card-bg {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.card-account .card-bg {
  background: linear-gradient(135deg, #ffecd2 0%, #fcb69f 100%);
}

.bottom-space {
  height: 40rpx;
}
</style>
