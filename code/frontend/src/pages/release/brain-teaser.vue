<template>
  <view class="brain-teaser-page">
    <!-- 头部统计 -->
    <view class="header">
      <view class="stat-item">
        <text class="stat-value">{{ answeredCount }}</text>
        <text class="stat-label">已答对</text>
      </view>
      <view class="stat-item">
        <text class="stat-value">{{ totalCount }}</text>
        <text class="stat-label">总题数</text>
      </view>
    </view>

    <!-- 题目卡片 -->
    <view class="content">
      <view class="question-card">
        <view class="question-number">第 {{ currentIndex + 1 }} 题</view>
        <text class="question-text">{{ currentQuestion.question }}</text>
        
        <!-- 答案区域 -->
        <view v-if="showAnswer" class="answer-section">
          <view class="answer-divider"></view>
          <text class="answer-label">答案：</text>
          <text class="answer-text">{{ currentQuestion.answer }}</text>
        </view>
      </view>
    </view>

    <!-- 操作按钮 -->
    <view class="actions">
      <button v-if="!showAnswer" class="action-btn reveal-btn" @click="revealAnswer">
        <text class="btn-icon">👀</text>
        <text class="btn-text">查看答案</text>
      </button>
      <button v-else class="action-btn next-btn" @click="nextQuestion">
        <text class="btn-icon">➡️</text>
        <text class="btn-text">下一题</text>
      </button>
      <button class="action-btn add-btn" @click="showAddModal = true">
        <text class="btn-icon">➕</text>
        <text class="btn-text">添加题目</text>
      </button>
      <button class="action-btn share-btn" @click="shareQuestion">
        <text class="btn-icon">📤</text>
        <text class="btn-text">分享</text>
      </button>
    </view>

    <!-- 添加题目弹窗 -->
    <view v-if="showAddModal" class="modal-mask" @click="showAddModal = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">添加脑筋急转弯</text>
          <text class="modal-close" @click="showAddModal = false">✕</text>
        </view>
        <view class="modal-body">
          <view class="form-item">
            <text class="form-label">问题</text>
            <textarea
              v-model="newQuestion.question"
              class="form-textarea"
              placeholder="请输入问题内容"
              maxlength="200"
              :show-confirm-bar="false"
            />
          </view>
          <view class="form-item">
            <text class="form-label">答案</text>
            <textarea
              v-model="newQuestion.answer"
              class="form-textarea"
              placeholder="请输入答案"
              maxlength="100"
              :show-confirm-bar="false"
            />
          </view>
          <view class="form-item checkbox-item">
            <checkbox :checked="newQuestion.is_public" @click="newQuestion.is_public = !newQuestion.is_public" />
            <text class="checkbox-label">公开分享（允许其他用户看到）</text>
          </view>
        </view>
        <view class="modal-footer">
          <button class="modal-btn cancel-btn" @click="showAddModal = false">取消</button>
          <button class="modal-btn confirm-btn" @click="handleAddQuestion">确定</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getStorage, setStorage } from '@/utils/storage'
import { useUserStore } from '@/store/user'
import {
  createBrainTeaserApi,
  getUserBrainTeasersApi,
  getPublicBrainTeasersApi,
  type BrainTeaser as ApiBrainTeaser
} from '@/api/brainTeaser'

const userStore = useUserStore()

interface BrainTeaser {
  question: string
  answer: string
  isCustom?: boolean
  id?: string
}

const currentIndex = ref(0)
const showAnswer = ref(false)
const answeredCount = ref(0)
const totalCount = ref(0)
const showAddModal = ref(false)
const newQuestion = ref({
  question: '',
  answer: '',
  is_public: false
})

// 系统预设题目
const systemQuestions: BrainTeaser[] = [
  { question: '什么东西越洗越脏？', answer: '水' },
  { question: '什么车可以不受交通规则限制横冲直撞？', answer: '碰碰车' },
  { question: '什么书买不到？', answer: '遗书' },
  { question: '大象的左耳朵像什么？', answer: '右耳朵' },
  { question: '什么水永远用不完？', answer: '口水' },
  { question: '什么东西天气越热，它爬的越高？', answer: '温度计' },
  { question: '什么动物，大的像猫，小的像猫，但不是猫？', answer: '老虎' },
  { question: '什么东西有五个头，但人不觉得它怪？', answer: '手和脚' },
  { question: '什么书中没有一个字？', answer: '天书' },
  { question: '什么帽不能戴？', answer: '螺帽' },
  { question: '什么路不能走？', answer: '电路' },
  { question: '什么门永远关不上？', answer: '球门' },
  { question: '什么东西比天更高？', answer: '心比天高' },
  { question: '有一个人,他是你父母生的,但他不是你的兄弟姐妹,他是谁？', answer: '你自己' },
  { question: '打什么东西不需要花力气？', answer: '打瞌睡' }
]

// 用户自定义题目
const userQuestions = ref<BrainTeaser[]>([])

// 公开分享的题目
const publicQuestions = ref<BrainTeaser[]>([])

// 所有题目（系统 + 用户 + 公开）
const allQuestions = ref<BrainTeaser[]>([...systemQuestions])

const currentQuestion = ref<BrainTeaser>(allQuestions.value[0])

onMounted(async () => {
  await loadUserQuestions()
  totalCount.value = allQuestions.value.length
  loadProgress()
  loadRandomQuestion()
})

// 加载用户自定义题目和公开题目
const loadUserQuestions = async () => {
  try {
    // 加载用户自己的题目
    const userData = await getUserBrainTeasersApi()
    userQuestions.value = userData.map(item => ({
      question: item.question,
      answer: item.answer,
      isCustom: true,
      id: item.id
    }))
    
    // 加载所有公开分享的题目
    const publicData = await getPublicBrainTeasersApi()
    // 获取当前用户ID，过滤掉自己的题目（避免重复）
    const currentUserId = userStore.userId
    publicQuestions.value = publicData
      .filter(item => item.user_id !== currentUserId)
      .map(item => ({
        question: item.question,
        answer: item.answer,
        isCustom: true,
        id: item.id
      }))
    
    // 整合所有题目：系统 + 用户自己的 + 其他人公开的
    allQuestions.value = [
      ...systemQuestions, 
      ...userQuestions.value,
      ...publicQuestions.value
    ]
    
    console.log(`已加载题目：系统${systemQuestions.length}题，个人${userQuestions.value.length}题，公开${publicQuestions.value.length}题`)
  } catch (error) {
    console.error('加载题目失败:', error)
  }
}

const loadProgress = () => {
  const progress = getStorage<{ answeredCount: number }>('brainTeaserProgress', { answeredCount: 0 })
  answeredCount.value = progress.answeredCount
}

const saveProgress = () => {
  setStorage('brainTeaserProgress', { answeredCount: answeredCount.value })
}

const loadRandomQuestion = () => {
  if (allQuestions.value.length === 0) return
  currentIndex.value = Math.floor(Math.random() * allQuestions.value.length)
  currentQuestion.value = allQuestions.value[currentIndex.value]
  showAnswer.value = false
}

// 添加题目
const handleAddQuestion = async () => {
  if (!newQuestion.value.question.trim()) {
    uni.showToast({ title: '请输入问题', icon: 'none' })
    return
  }
  if (!newQuestion.value.answer.trim()) {
    uni.showToast({ title: '请输入答案', icon: 'none' })
    return
  }
  
  try {
    uni.showLoading({ title: '添加中...', mask: true })
    await createBrainTeaserApi(newQuestion.value)
    uni.showToast({ title: '添加成功', icon: 'success' })
    
    // 重新加载题目列表
    await loadUserQuestions()
    totalCount.value = allQuestions.value.length
    
    // 重置表单
    newQuestion.value = {
      question: '',
      answer: '',
      is_public: false
    }
    showAddModal.value = false
  } catch (error) {
    console.error('添加题目失败:', error)
    uni.showToast({ title: '添加失败', icon: 'none' })
  } finally {
    uni.hideLoading()
  }
}

const revealAnswer = () => {
  showAnswer.value = true
  answeredCount.value++
  saveProgress()
}

const nextQuestion = () => {
  loadRandomQuestion()
}

const shareQuestion = () => {
  const shareText = `脑筋急转弯：${currentQuestion.value.question}\n\n你知道答案吗？🤔`
  
  uni.showActionSheet({
    itemList: ['复制到剪贴板', '生成分享图片'],
    success: (res) => {
      if (res.tapIndex === 0) {
        uni.setClipboardData({
          data: shareText,
          success: () => {
            uni.showToast({ title: '已复制', icon: 'success' })
          }
        })
      } else {
        uni.showToast({ title: '生成分享图片功能开发中', icon: 'none' })
      }
    }
  })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.brain-teaser-page {
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
      radial-gradient(circle at 30% 25%, rgba(255, 0, 214, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 70% 75%, rgba(0, 217, 255, 0.15) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.header {
  display: flex;
  padding: 40rpx 30rpx;
  gap: 25rpx;
  position: relative;
  z-index: 1;

  .stat-item {
    flex: 1;
    @include neon-card;
    background: linear-gradient(135deg, rgba(20, 26, 56, 0.95) 0%, rgba(15, 20, 45, 0.98) 100%);
    backdrop-filter: blur(30rpx);
    border-radius: 30rpx;
    border: 3rpx solid rgba(255, 0, 214, 0.5);
    padding: 35rpx;
    text-align: center;
    box-shadow: 
      0 15rpx 50rpx rgba(0, 0, 0, 0.6),
      0 0 60rpx rgba(255, 0, 214, 0.4),
      inset 0 0 50rpx rgba(255, 0, 214, 0.08);
    animation: statFloat 3s ease-in-out infinite;

    &:first-child {
      animation-delay: 0.2s;
    }

    .stat-value {
      display: block;
      font-size: 56rpx;
      font-weight: bold;
      @include neon-title(#FFD600);
      margin-bottom: 12rpx;
      filter: brightness(1.2);
      text-shadow: 
        0 0 20rpx rgba(255, 214, 0, 1),
        0 0 40rpx rgba(255, 214, 0, 0.6),
        0 4rpx 8rpx rgba(0, 0, 0, 0.3);
    }

    .stat-label {
      font-size: 26rpx;
      @include neon-text(#00D9FF);
      font-weight: 600;
    }
  }
}

.content {
  flex: 1;
  padding: 0 30rpx 30rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  z-index: 1;
}

.question-card {
  width: 100%;
  @include neon-card;
  background: linear-gradient(135deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 1) 100%);
  backdrop-filter: blur(30rpx);
  border-radius: 35rpx;
  border: 4rpx solid rgba(138, 92, 246, 0.6);
  padding: 55rpx 45rpx;
  box-shadow: 
    0 25rpx 80rpx rgba(0, 0, 0, 0.7),
    0 0 100rpx rgba(138, 92, 246, 0.5),
    inset 0 0 80rpx rgba(138, 92, 246, 0.12);
  animation: cardPulse 4s ease-in-out infinite;

  .question-number {
    font-size: 30rpx;
    @include neon-text(#8B5CF6);
    margin-bottom: 35rpx;
    font-weight: 600;
    display: inline-block;
    padding: 8rpx 20rpx;
    background: linear-gradient(135deg, rgba(138, 92, 246, 0.2) 0%, rgba(138, 92, 246, 0.1) 100%);
    border-radius: 20rpx;
    border: 2rpx solid rgba(138, 92, 246, 0.4);
  }

  .question-text {
    display: block;
    font-size: 40rpx;
    @include neon-title(#ffffff);
    line-height: 1.9;
    margin-bottom: 25rpx;
    filter: brightness(1.15);
    text-shadow: 
      0 2rpx 8rpx rgba(0, 0, 0, 0.3),
      0 0 20rpx rgba(255, 255, 255, 0.2);
  }

  .answer-section {
    margin-top: 45rpx;

    .answer-divider {
      height: 3rpx;
      background: linear-gradient(90deg, 
        transparent 0%, 
        rgba(0, 217, 255, 0.8) 50%, 
        transparent 100%);
      margin-bottom: 35rpx;
      box-shadow: 0 0 15rpx rgba(0, 217, 255, 0.6);
      animation: dividerGlow 2s ease-in-out infinite;
    }

    .answer-label {
      font-size: 32rpx;
      @include neon-text(#00D9FF);
      font-weight: bold;
    }

    .answer-text {
      display: block;
      font-size: 36rpx;
      @include neon-text(#FFD600);
      margin-top: 20rpx;
      padding: 28rpx;
      background: linear-gradient(135deg, rgba(255, 214, 0, 0.15) 0%, rgba(0, 217, 255, 0.15) 100%);
      backdrop-filter: blur(10rpx);
      border-radius: 25rpx;
      border: 2rpx solid rgba(255, 214, 0, 0.4);
      box-shadow: 
        0 8rpx 24rpx rgba(0, 0, 0, 0.4),
        0 0 40rpx rgba(255, 214, 0, 0.3),
        inset 0 0 30rpx rgba(255, 214, 0, 0.1);
      font-weight: 600;
      text-shadow: 
        0 2rpx 6rpx rgba(0, 0, 0, 0.3),
        0 0 15rpx rgba(255, 214, 0, 0.4);
      animation: answerReveal 0.5s ease-out;
    }
  }
}

.actions {
  padding: 30rpx;
  display: flex;
  gap: 25rpx;
  position: relative;
  z-index: 1;

  .action-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 35rpx 22rpx;
    border-radius: 30rpx;
    border: none;
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;

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
      opacity: 0;
      transition: opacity 0.3s;
    }

    &::after {
      border: none;
    }

    &:active {
      transform: scale(0.92);

      &::before {
        opacity: 1;
      }
    }

    .btn-icon {
      font-size: 56rpx;
      margin-bottom: 12rpx;
      filter: drop-shadow(0 0 15rpx rgba(255, 255, 255, 0.8));
      position: relative;
      z-index: 1;
    }

    .btn-text {
      font-size: 26rpx;
      @include neon-text(#ffffff);
      font-weight: 600;
      position: relative;
      z-index: 1;
      text-shadow: 
        0 0 10rpx rgba(255, 255, 255, 0.6),
        0 2rpx 5rpx rgba(0, 0, 0, 0.3);
    }
  }

  .reveal-btn,
  .next-btn {
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    border: 2rpx solid rgba(0, 217, 255, 0.7);
    box-shadow: 
      0 10rpx 40rpx rgba(0, 217, 255, 0.5),
      0 0 60rpx rgba(0, 217, 255, 0.4);
    animation: btnPulse 3s ease-in-out infinite;

    &:active {
      box-shadow: 
        0 12rpx 50rpx rgba(0, 217, 255, 0.7),
        0 0 80rpx rgba(0, 217, 255, 0.6);
    }
  }

  .add-btn {
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    border: 2rpx solid rgba(255, 0, 214, 0.7);
    box-shadow: 
      0 10rpx 40rpx rgba(255, 0, 214, 0.5),
      0 0 60rpx rgba(255, 0, 214, 0.4);

    &:active {
      box-shadow: 
        0 12rpx 50rpx rgba(255, 0, 214, 0.7),
        0 0 80rpx rgba(255, 0, 214, 0.6);
    }
  }

  .share-btn {
    background: linear-gradient(135deg, rgba(255, 214, 0, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    border: 2rpx solid rgba(255, 214, 0, 0.7);
    box-shadow: 
      0 10rpx 40rpx rgba(255, 214, 0, 0.5),
      0 0 60rpx rgba(255, 214, 0, 0.4);

    &:active {
      box-shadow: 
        0 12rpx 50rpx rgba(255, 214, 0, 0.7),
        0 0 80rpx rgba(255, 214, 0, 0.6);
    }
  }
}

/* 弹窗样式 */
.modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.75);
  backdrop-filter: blur(10rpx);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
  animation: fadeIn 0.3s ease-out;
}

.modal-content {
  width: 85%;
  max-width: 600rpx;
  @include neon-card;
  background: linear-gradient(135deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 1) 100%);
  backdrop-filter: blur(30rpx);
  border-radius: 35rpx;
  border: 3rpx solid rgba(138, 92, 246, 0.6);
  box-shadow: 
    0 25rpx 80rpx rgba(0, 0, 0, 0.7),
    0 0 100rpx rgba(138, 92, 246, 0.5),
    inset 0 0 80rpx rgba(138, 92, 246, 0.12);
  overflow: hidden;
  animation: scaleIn 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 35rpx 45rpx;
  border-bottom: 2rpx solid rgba(138, 92, 246, 0.3);

  .modal-title {
    font-size: 36rpx;
    font-weight: bold;
    @include neon-title(#00D9FF);
  }

  .modal-close {
    font-size: 42rpx;
    @include neon-text(#FF00D6);
    padding: 8rpx;
    transition: all 0.3s;

    &:active {
      transform: rotate(90deg) scale(0.88);
      text-shadow: 
        0 0 30rpx rgba(255, 0, 214, 1),
        0 0 50rpx rgba(255, 0, 214, 0.8);
    }
  }
}

.modal-body {
  padding: 45rpx;

  .form-item {
    margin-bottom: 35rpx;

    .form-label {
      display: block;
      font-size: 30rpx;
      @include neon-text(#8B5CF6);
      margin-bottom: 18rpx;
      font-weight: 600;
    }

    .form-textarea {
      @include neon-input;
      width: 100%;
      min-height: 180rpx;
      padding: 25rpx;
      border-radius: 25rpx;
      font-size: 30rpx;
      @include neon-text(#ffffff);
      box-sizing: border-box;
      transition: all 0.4s;

      &:focus {
        border-color: rgba(138, 92, 246, 0.8);
        box-shadow: 
          0 0 50rpx rgba(138, 92, 246, 0.5),
          inset 0 0 30rpx rgba(138, 92, 246, 0.1);
      }
    }

    &.checkbox-item {
      display: flex;
      align-items: center;
      gap: 18rpx;

      checkbox {
        transform: scale(1.3);
      }

      .checkbox-label {
        font-size: 28rpx;
        @include neon-text(#00D9FF);
        font-weight: 600;
      }
    }
  }
}

.modal-footer {
  display: flex;
  padding: 35rpx 45rpx;
  gap: 25rpx;
  border-top: 2rpx solid rgba(138, 92, 246, 0.3);

  .modal-btn {
    flex: 1;
    padding: 28rpx;
    border-radius: 25rpx;
    font-size: 30rpx;
    font-weight: bold;
    border: none;
    transition: all 0.3s;

    &::after {
      border: none;
    }
  }

  .cancel-btn {
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
    backdrop-filter: blur(10rpx);
    @include neon-text(#b8c5d6);
    border: 2rpx solid rgba(138, 92, 246, 0.5);
    box-shadow: 0 6rpx 18rpx rgba(0, 0, 0, 0.4);

    &:active {
      transform: scale(0.95);
    }
  }

  .confirm-btn {
    background: linear-gradient(135deg, rgba(138, 92, 246, 0.9) 0%, rgba(0, 217, 255, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(138, 92, 246, 0.7);
    box-shadow: 
      0 8rpx 24rpx rgba(138, 92, 246, 0.5),
      0 0 40rpx rgba(138, 92, 246, 0.4);
    text-shadow: 
      0 0 12rpx rgba(255, 255, 255, 0.8),
      0 2rpx 5rpx rgba(0, 0, 0, 0.3);

    &:active {
      transform: scale(0.95);
      box-shadow: 
        0 10rpx 30rpx rgba(138, 92, 246, 0.7),
        0 0 60rpx rgba(138, 92, 246, 0.6);
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes statFloat {
  0%, 100% {
    box-shadow: 
      0 15rpx 50rpx rgba(0, 0, 0, 0.6),
      0 0 60rpx rgba(255, 0, 214, 0.4),
      inset 0 0 50rpx rgba(255, 0, 214, 0.08);
  }
  50% {
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.7),
      0 0 80rpx rgba(255, 0, 214, 0.6),
      inset 0 0 60rpx rgba(255, 0, 214, 0.15);
  }
}

@keyframes cardPulse {
  0%, 100% {
    box-shadow: 
      0 25rpx 80rpx rgba(0, 0, 0, 0.7),
      0 0 100rpx rgba(138, 92, 246, 0.5),
      inset 0 0 80rpx rgba(138, 92, 246, 0.12);
  }
  50% {
    box-shadow: 
      0 30rpx 90rpx rgba(0, 0, 0, 0.8),
      0 0 120rpx rgba(138, 92, 246, 0.7),
      inset 0 0 100rpx rgba(138, 92, 246, 0.18);
  }
}

@keyframes dividerGlow {
  0%, 100% {
    box-shadow: 0 0 15rpx rgba(0, 217, 255, 0.6);
  }
  50% {
    box-shadow: 0 0 25rpx rgba(0, 217, 255, 1);
  }
}

@keyframes answerReveal {
  from {
    opacity: 0;
    transform: translateY(-20rpx) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(0, 217, 255, 0.5),
      0 0 60rpx rgba(0, 217, 255, 0.4);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(0, 217, 255, 0.7),
      0 0 80rpx rgba(0, 217, 255, 0.6);
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes scaleIn {
  from {
    transform: scale(0.85);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}
</style>

