<template>
  <view class="inspiration-page">
    <!-- 顶部切换标签 -->
    <view class="tabs">
      <view
        class="tab-item"
        :class="{ active: activeTab === 'encourage' }"
        @click="switchTab('encourage')"
      >
        <text class="tab-icon">💪</text>
        <text class="tab-text">每日鼓励</text>
      </view>
      <view
        class="tab-item"
        :class="{ active: activeTab === 'philosophy' }"
        @click="switchTab('philosophy')"
      >
        <text class="tab-icon">📚</text>
        <text class="tab-text">人生哲理</text>
      </view>
    </view>

    <!-- 卡片内容区 -->
    <view class="content-wrapper">
      <!-- 鼓励卡片 -->
      <view v-if="activeTab === 'encourage'" class="card-container">
        <view class="card" :class="{ flipped: isFlipped }">
          <view class="card-front">
            <view class="card-icon">💪</view>
            <text class="card-title">{{ encourageCard.title }}</text>
            <text class="card-content">{{ encourageCard.content }}</text>
            <view class="card-footer">
              <text class="card-date">{{ formatDate(new Date()) }}</text>
            </view>
          </view>
          <view class="card-back">
            <text class="card-message">点击翻回卡片</text>
          </view>
        </view>
      </view>

      <!-- 哲理卡片 -->
      <view v-else class="card-container">
        <view class="card philosophy-card" :class="{ flipped: isFlipped }">
          <view class="card-front">
            <view class="card-icon">📚</view>
            <text class="card-title">{{ philosophyCard.title }}</text>
            <text class="card-content">{{ philosophyCard.content }}</text>
            <text v-if="philosophyCard.author" class="card-author">— {{ philosophyCard.author }}</text>
            <view class="card-footer">
              <text class="card-date">{{ formatDate(new Date()) }}</text>
            </view>
          </view>
          <view class="card-back">
            <text class="card-message">点击翻回卡片</text>
          </view>
        </view>
      </view>
    </view>

    <!-- 底部操作按钮 -->
    <view class="actions">
      <button class="action-btn flip-btn" @click="flipCard">
        <text class="btn-icon">🔄</text>
        <text class="btn-text">翻转</text>
      </button>
      <button class="action-btn refresh-btn" @click="refreshCard">
        <text class="btn-icon">🎲</text>
        <text class="btn-text">换一张</text>
      </button>
      <button class="action-btn create-btn" @click="createCustomCard">
        <text class="btn-icon">✏️</text>
        <text class="btn-text">自定义</text>
      </button>
      <button class="action-btn share-btn" @click="shareCard">
        <text class="btn-icon">📤</text>
        <text class="btn-text">分享</text>
      </button>
    </view>

    <!-- 自定义卡片弹窗 -->
    <view v-if="showCustomModal" class="custom-modal-mask" @click="showCustomModal = false">
      <view class="custom-modal" @click.stop>
        <view class="modal-header">
          <text class="modal-title">创建{{ activeTab === 'encourage' ? '鼓励' : '哲理' }}卡片</text>
          <text class="modal-close" @click="showCustomModal = false">✕</text>
        </view>
        <view class="modal-body">
          <view class="form-item">
            <text class="form-label">标题</text>
            <input v-model="customForm.title" class="form-input" placeholder="输入卡片标题" maxlength="20" />
          </view>
          <view class="form-item">
            <text class="form-label">内容</text>
            <textarea
              v-model="customForm.content"
              class="form-textarea"
              placeholder="输入卡片内容..."
              maxlength="200"
            />
            <text class="char-count">{{ customForm.content.length }}/200</text>
          </view>
          <view v-if="activeTab === 'philosophy'" class="form-item">
            <text class="form-label">作者（可选）</text>
            <input v-model="customForm.author" class="form-input" placeholder="输入作者名称" maxlength="20" />
          </view>
        </view>
        <view class="modal-footer">
          <button class="modal-btn cancel-btn" @click="showCustomModal = false">取消</button>
          <button class="modal-btn confirm-btn" @click="saveCustomCard">保存并分享</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getStorage, setStorage } from '@/utils/storage'
import { shareContent } from '@/utils/share'

interface EncourageCard {
  title: string
  content: string
  isCustom?: boolean
}

interface PhilosophyCard {
  title: string
  content: string
  author?: string
  isCustom?: boolean
}

const activeTab = ref<'encourage' | 'philosophy'>('encourage')
const isFlipped = ref(false)
const showCustomModal = ref(false)

const customForm = ref({
  title: '',
  content: '',
  author: ''
})

const encourageCard = ref<EncourageCard>({
  title: '你是最棒的',
  content: '每一次努力都不会白费，相信自己，你一定可以的！'
})

const philosophyCard = ref<PhilosophyCard>({
  title: '人生的意义',
  content: '人生的意义不在于我们拥有什么，而在于我们成为了什么样的人。',
  author: '佚名'
})

const encourageCards: EncourageCard[] = [
  { title: '你是最棒的', content: '每一次努力都不会白费，相信自己，你一定可以的！' },
  { title: '坚持就是胜利', content: '成功的道路充满挑战，但只要坚持，终会迎来曙光。' },
  { title: '拥抱每一天', content: '生活总会有起伏，但每一天都值得我们用心去感受。' },
  { title: '勇敢前行', content: '不要害怕失败，每一次尝试都是成长的机会。' },
  { title: '珍惜当下', content: '过去已成为历史，未来还未到来，只有当下才是最真实的。' },
  { title: '相信自己', content: '你拥有无限的潜能，只要相信自己，就能创造奇迹。' },
  { title: '永不放弃', content: '困难只是暂时的，只要不放弃，就一定能看到希望。' },
  { title: '保持乐观', content: '积极的心态能让你在任何困境中都能找到出路。' }
]

const philosophyCards: PhilosophyCard[] = [
  { title: '人生的意义', content: '人生的意义不在于我们拥有什么，而在于我们成为了什么样的人。', author: '佚名' },
  { title: '时间的价值', content: '时间是最公平的资源，每个人每天都拥有24小时，关键在于如何使用它。', author: '佚名' },
  { title: '选择与改变', content: '我们无法改变过去，但可以选择如何面对未来。', author: '佚名' },
  { title: '知行合一', content: '知道和做到之间，隔着的是行动。真正的智慧在于知行合一。', author: '王阳明' },
  { title: '内心的平静', content: '真正的强大不是征服别人，而是能够驾驭自己的内心。', author: '佚名' },
  { title: '人生如旅', content: '人生就像一场旅行，重要的不是目的地，而是沿途的风景和看风景的心情。', author: '佚名' },
  { title: '简单生活', content: '简单是一种美德，学会放下不必要的执着，生活会更轻松。', author: '佚名' },
  { title: '自我成长', content: '每个人都是一座待开发的矿山，关键在于你愿意挖掘多深。', author: '佚名' }
]

onMounted(() => {
  loadTodayCard()
})

const switchTab = (tab: 'encourage' | 'philosophy') => {
  activeTab.value = tab
  isFlipped.value = false
  loadTodayCard()
}

const flipCard = () => {
  isFlipped.value = !isFlipped.value
}

const loadTodayCard = () => {
  const today = new Date().toDateString()
  
  if (activeTab.value === 'encourage') {
    const cached = getStorage<{ date: string; card: EncourageCard }>('dailyEncourage', null)
    if (cached && cached.date === today) {
      encourageCard.value = cached.card
    } else {
      const randomCard = encourageCards[Math.floor(Math.random() * encourageCards.length)]
      encourageCard.value = randomCard
      setStorage('dailyEncourage', { date: today, card: randomCard })
    }
  } else {
    const cached = getStorage<{ date: string; card: PhilosophyCard }>('dailyPhilosophy', null)
    if (cached && cached.date === today) {
      philosophyCard.value = cached.card
    } else {
      const randomCard = philosophyCards[Math.floor(Math.random() * philosophyCards.length)]
      philosophyCard.value = randomCard
      setStorage('dailyPhilosophy', { date: today, card: randomCard })
    }
  }
}

const refreshCard = () => {
  if (activeTab.value === 'encourage') {
    const randomCard = encourageCards[Math.floor(Math.random() * encourageCards.length)]
    encourageCard.value = randomCard
  } else {
    const randomCard = philosophyCards[Math.floor(Math.random() * philosophyCards.length)]
    philosophyCard.value = randomCard
  }
  isFlipped.value = false
  uni.showToast({ title: '已刷新', icon: 'success' })
}

const createCustomCard = () => {
  customForm.value = { title: '', content: '', author: '' }
  showCustomModal.value = true
}

const saveCustomCard = () => {
  if (!customForm.value.title || !customForm.value.content) {
    return uni.showToast({ title: '请填写标题和内容', icon: 'none' })
  }

  // 保存自定义卡片
  if (activeTab.value === 'encourage') {
    encourageCard.value = {
      title: customForm.value.title,
      content: customForm.value.content,
      isCustom: true
    }
    
    // 保存到本地存储
    const customCards = getStorage<EncourageCard[]>('customEncourageCards', [])
    customCards.unshift(encourageCard.value)
    setStorage('customEncourageCards', customCards.slice(0, 20)) // 最多保存20张
  } else {
    philosophyCard.value = {
      title: customForm.value.title,
      content: customForm.value.content,
      author: customForm.value.author || '佚名',
      isCustom: true
    }
    
    // 保存到本地存储
    const customCards = getStorage<PhilosophyCard[]>('customPhilosophyCards', [])
    customCards.unshift(philosophyCard.value)
    setStorage('customPhilosophyCards', customCards.slice(0, 20))
  }

  showCustomModal.value = false
  uni.showToast({ title: '创建成功', icon: 'success' })
  
  // 自动分享
  setTimeout(() => {
    shareCard()
  }, 500)
}

const shareCard = () => {
  const card = activeTab.value === 'encourage' ? encourageCard.value : philosophyCard.value
  const author = 'author' in card && card.author ? `\n\n— ${card.author}` : ''
  
  shareContent({
    title: card.title,
    content: card.content + author,
    link: 'https://your-app-link.com'
  })
}

const formatDate = (date: Date): string => {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}.${month}.${day}`
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.inspiration-page {
  min-height: 100vh;
  @include cyber-page-bg;
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
      radial-gradient(circle at 20% 30%, rgba(255, 214, 0, 0.1) 0%, transparent 50%),
      radial-gradient(circle at 80% 70%, rgba(138, 92, 246, 0.1) 0%, transparent 50%);
    pointer-events: none;
    animation: pulseGlow 8s ease-in-out infinite;
  }
}

@keyframes pulseGlow {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 0.8; }
}

.tabs {
  display: flex;
  padding: 30rpx 30rpx 0;
  gap: 20rpx;
  position: relative;
  z-index: 1;

  .tab-item {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 25rpx;
    background: rgba(30, 36, 66, 0.6);
    backdrop-filter: blur(10rpx);
    border: 2rpx solid rgba(0, 217, 255, 0.3);
    border-radius: 20rpx;
    box-shadow: 0 0 20rpx rgba(0, 217, 255, 0.2);
    transition: all 0.3s;

    &.active {
      @include neon-card;
      border: 2rpx solid rgba(255, 214, 0, 0.8);
      box-shadow: 
        0 0 40rpx rgba(255, 214, 0, 0.5),
        inset 0 0 40rpx rgba(255, 214, 0, 0.1);

      .tab-icon {
        transform: scale(1.2);
        filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 0.8));
      }

      .tab-text {
        @include neon-text(#FFD600);
        font-weight: bold;
      }
    }

    .tab-icon {
      font-size: 48rpx;
      margin-bottom: 10rpx;
      transition: all 0.3s;
    }

    .tab-text {
      font-size: 26rpx;
      @include neon-text(#ffffff);
      transition: all 0.3s;
    }
  }
}

.content-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40rpx 30rpx;
  position: relative;
  z-index: 1;
}

.card-container {
  width: 100%;
  perspective: 2000rpx;
  animation: fadeIn 0.8s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.card {
  width: 100%;
  min-height: 600rpx;
  position: relative;
  transform-style: preserve-3d;
  transition: transform 0.8s cubic-bezier(0.4, 0, 0.2, 1);
  animation: cardEnter 1s ease-out, cardFloat 6s ease-in-out infinite 1s;

  &.flipped {
    transform: rotateY(180deg);
  }

  .card-front,
  .card-back {
    position: absolute;
    width: 100%;
    min-height: 600rpx;
    backface-visibility: hidden;
    border-radius: 30rpx;
    padding: 50rpx 40rpx;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    box-shadow: 0 20rpx 60rpx rgba(0, 0, 0, 0.3);
  }

  .card-front {
    @include neon-card;
    backdrop-filter: blur(10rpx);
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.5),
      0 0 60rpx rgba(245, 87, 108, 0.4),
      inset 0 0 80rpx rgba(245, 87, 108, 0.1);
    border: 2rpx solid rgba(245, 87, 108, 0.6);

    .card-icon {
      font-size: 100rpx;
      margin-bottom: 30rpx;
      filter: drop-shadow(0 0 30rpx rgba(245, 87, 108, 0.8));
      animation: iconPulse 3s ease-in-out infinite;
    }

    .card-title {
      font-size: 44rpx;
      font-weight: bold;
      @include neon-title(#F5576C);
      margin-bottom: 30rpx;
      text-align: center;
      animation: titlePulse 3s ease-in-out infinite;
    }

    .card-content {
      font-size: 32rpx;
      color: #b8c5d6;
      line-height: 1.8;
      text-align: center;
      margin-bottom: 20rpx;
      text-shadow: 0 0 10rpx rgba(184, 197, 214, 0.3);
    }

    .card-author {
      font-size: 28rpx;
      @include neon-text(#FFD600);
      font-style: italic;
      margin-top: 20rpx;
    }

    .card-footer {
      margin-top: auto;
      padding-top: 30rpx;

      .card-date {
        font-size: 24rpx;
        @include neon-text(#6b7b93);
      }
    }
  }

  .card-back {
    background: linear-gradient(135deg, rgba(245, 87, 108, 0.9) 0%, rgba(240, 147, 251, 0.9) 100%);
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.5),
      0 0 60rpx rgba(240, 147, 251, 0.6),
      inset 0 0 80rpx rgba(240, 147, 251, 0.2);
    border: 2rpx solid rgba(240, 147, 251, 0.8);
    transform: rotateY(180deg);

    .card-message {
      font-size: 36rpx;
      @include neon-text(#ffffff);
      text-align: center;
      animation: blink 2s ease-in-out infinite;
    }
  }
}

@keyframes cardEnter {
  from {
    opacity: 0;
    transform: perspective(2000rpx) rotateY(-30deg) scale(0.8);
  }
  to {
    opacity: 1;
    transform: perspective(2000rpx) rotateY(0deg) scale(1);
  }
}

@keyframes cardFloat {
  0%, 100% {
    transform: perspective(2000rpx) rotateY(0deg) translateY(0);
  }
  50% {
    transform: perspective(2000rpx) rotateY(0deg) translateY(-10rpx);
  }
}

@keyframes iconPulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.1);
  }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 15rpx rgba(245, 87, 108, 0.8),
      0 0 30rpx rgba(245, 87, 108, 0.5),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
  50% {
    text-shadow: 
      0 0 25rpx rgba(245, 87, 108, 1),
      0 0 50rpx rgba(245, 87, 108, 0.8),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
}

@keyframes blink {
  0%, 100% { opacity: 0.7; }
  50% { opacity: 1; }
}

.philosophy-card {
  .card-front {
    background: linear-gradient(135deg, rgba(79, 172, 254, 0.3) 0%, rgba(0, 242, 254, 0.3) 100%) !important;
    border: 2rpx solid rgba(0, 242, 254, 0.6) !important;
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.5),
      0 0 60rpx rgba(0, 242, 254, 0.4),
      inset 0 0 80rpx rgba(0, 242, 254, 0.1) !important;

    .card-icon {
      filter: drop-shadow(0 0 30rpx rgba(0, 242, 254, 0.8)) !important;
    }

    .card-title {
      @include neon-title(#00F2FE);
      animation: philoTitlePulse 3s ease-in-out infinite !important;
    }

    .card-content {
      @include neon-text(#ffffff);
    }

    .card-author {
      @include neon-text(#FFD600);
    }

    .card-date {
      @include neon-text(#6b7b93);
    }
  }
}

@keyframes philoTitlePulse {
  0%, 100% {
    text-shadow: 
      0 0 15rpx rgba(0, 242, 254, 0.8),
      0 0 30rpx rgba(0, 242, 254, 0.5),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
  50% {
    text-shadow: 
      0 0 25rpx rgba(0, 242, 254, 1),
      0 0 50rpx rgba(0, 242, 254, 0.8),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
}

.actions {
  padding: 30rpx;
  display: flex;
  gap: 20rpx;
  background: rgba(30, 36, 66, 0.8);
  backdrop-filter: blur(20rpx);
  border-top: 2rpx solid rgba(0, 217, 255, 0.3);
  box-shadow: 0 -8rpx 30rpx rgba(0, 0, 0, 0.3);
  position: relative;
  z-index: 1;

  .action-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 25rpx 20rpx;
    border-radius: 20rpx;
    border: none;
    transition: all 0.3s;

    &::after {
      border: none;
    }

    &:active {
      transform: scale(0.95);
    }

    .btn-icon {
      font-size: 40rpx;
      margin-bottom: 8rpx;
    }

    .btn-text {
      font-size: 24rpx;
    }
  }

  .flip-btn {
    @include glow-button(#00D9FF);
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.8) 0%, rgba(79, 172, 254, 0.8) 100%);

    .btn-text {
      @include neon-text(#ffffff);
    }
  }

  .refresh-btn {
    @include glow-button(#8B5CF6);
    background: linear-gradient(135deg, rgba(138, 92, 246, 0.8) 0%, rgba(102, 126, 234, 0.8) 100%);

    .btn-text {
      @include neon-text(#ffffff);
    }
  }

  .create-btn {
    @include glow-button(#FFD600);
    background: linear-gradient(135deg, rgba(255, 214, 0, 0.8) 0%, rgba(254, 225, 64, 0.8) 100%);

    .btn-text {
      @include neon-text(#ffffff);
    }
  }

  .share-btn {
    @include glow-button(#FF00D6);
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.8) 0%, rgba(245, 87, 108, 0.8) 100%);

    .btn-text {
      @include neon-text(#ffffff);
    }
  }
}

.custom-modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(10rpx);
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: maskFadeIn 0.3s ease-out;
}

@keyframes maskFadeIn {
  from {
    opacity: 0;
    backdrop-filter: blur(0);
  }
  to {
    opacity: 1;
    backdrop-filter: blur(10rpx);
  }
}

.custom-modal {
  width: 90%;
  max-width: 600rpx;
  @include neon-card;
  border-radius: 30rpx;
  overflow: hidden;
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(255, 214, 0, 0.5),
    inset 0 0 80rpx rgba(255, 214, 0, 0.1);
  border: 2rpx solid rgba(255, 214, 0, 0.6);
  animation: modalSlideUp 0.4s cubic-bezier(0.4, 0, 0.2, 1);

  .modal-header {
    padding: 40rpx 30rpx 20rpx;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 2rpx solid rgba(255, 214, 0, 0.3);
    background: linear-gradient(135deg, rgba(255, 214, 0, 0.1) 0%, transparent 100%);

    .modal-title {
      font-size: 36rpx;
      font-weight: bold;
      @include neon-title(#FFD600);
      animation: titlePulse2 3s ease-in-out infinite;
    }

    .modal-close {
      font-size: 40rpx;
      @include neon-text(#6b7b93);
      padding: 10rpx;
      transition: all 0.3s ease;
      
      &:active {
        @include neon-text(#FF00D6);
        transform: rotate(90deg) scale(1.2);
      }
    }
  }

  .modal-body {
    padding: 30rpx;
    max-height: 600rpx;
    overflow-y: auto;

    .form-item {
      margin-bottom: 30rpx;

      &:last-child {
        margin-bottom: 0;
      }

      .form-label {
        display: block;
        font-size: 28rpx;
        @include neon-text(#00D9FF);
        margin-bottom: 15rpx;
      }

      .form-input,
      .form-textarea {
        @include neon-input;
        width: 100%;
        font-size: 28rpx;
        color: #ffffff;
        
        &:focus {
          border-color: rgba(255, 214, 0, 0.8);
          box-shadow: 0 0 30rpx rgba(255, 214, 0, 0.5);
        }
      }

      .form-textarea {
        min-height: 200rpx;
        line-height: 1.6;
      }

      .char-count {
        display: block;
        text-align: right;
        font-size: 22rpx;
        @include neon-text(#6b7b93);
        margin-top: 10rpx;
      }
    }
  }

  .modal-footer {
    padding: 20rpx 30rpx 30rpx;
    display: flex;
    gap: 20rpx;

    .modal-btn {
      flex: 1;
      height: 80rpx;
      border-radius: 50rpx;
      font-size: 30rpx;
      border: none;
      line-height: 80rpx;
      transition: all 0.3s ease;

      &::after {
        border: none;
      }
    }

    .cancel-btn {
      background: rgba(30, 36, 66, 0.6);
      backdrop-filter: blur(10rpx);
      @include neon-text(#ffffff);
      border: 2rpx solid rgba(0, 217, 255, 0.4);
      box-shadow: 0 0 20rpx rgba(0, 217, 255, 0.3);
      
      &:active {
        transform: scale(0.95);
        border-color: rgba(0, 217, 255, 0.8);
        box-shadow: 0 0 30rpx rgba(0, 217, 255, 0.6);
      }
    }

    .confirm-btn {
      @include glow-button(#FFD600);
      background: linear-gradient(135deg, rgba(255, 214, 0, 0.8) 0%, rgba(254, 225, 64, 0.8) 100%);
      animation: btnPulse4 3s ease-in-out infinite;
      
      &:active {
        transform: scale(0.95);
        animation: none;
      }
    }
  }
}

@keyframes modalSlideUp {
  from {
    opacity: 0;
    transform: translateY(100rpx) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

@keyframes titlePulse2 {
  0%, 100% {
    text-shadow: 
      0 0 10rpx rgba(255, 214, 0, 0.8),
      0 0 20rpx rgba(255, 214, 0, 0.5);
  }
  50% {
    text-shadow: 
      0 0 15rpx rgba(255, 214, 0, 1),
      0 0 30rpx rgba(255, 214, 0, 0.8);
  }
}

@keyframes btnPulse4 {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(255, 214, 0, 0.5),
      0 8rpx 24rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(255, 214, 0, 0.2);
  }
  50% {
    box-shadow: 
      0 0 30rpx rgba(255, 214, 0, 0.8),
      0 12rpx 32rpx rgba(0, 0, 0, 0.4),
      inset 0 0 30rpx rgba(255, 214, 0, 0.3);
  }
}
</style>

