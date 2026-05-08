<template>
  <view class="chat-page">
    <!-- 消息列表 -->
    <scroll-view class="message-list" scroll-y :scroll-into-view="scrollToView">
      <view
        v-for="message in messages"
        :key="message.id"
        :id="`msg-${message.id}`"
        class="message-item"
        :class="{ 'message-user': message.role === 'user' }"
      >
        <view class="message-avatar">
          <text>{{ message.role === 'user' ? '👤' : '🤖' }}</text>
        </view>
        <view class="message-content">
          <text class="message-text">{{ message.content }}</text>
        </view>
      </view>

      <!-- 正在输入 -->
      <view v-if="isTyping" class="message-item">
        <view class="message-avatar">
          <text>🤖</text>
        </view>
        <view class="message-content">
          <text class="typing-indicator">正在思考...</text>
        </view>
      </view>
    </scroll-view>

    <!-- 输入框 -->
    <view class="input-bar">
      <input
        class="message-input"
        v-model="inputText"
        placeholder="说说你的想法..."
        confirm-type="send"
        @confirm="sendMessage"
      />
      <view class="send-btn" @click="sendMessage">
        <text>发送</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'
import { onLoad, onShow } from '@dcloudio/uni-app'
import { getStorage, setStorage } from '@/utils/storage'

interface Message {
  id: string
  role: 'user' | 'assistant'
  content: string
  timestamp: number
}

const chatId = ref('')
const messages = ref<Message[]>([])
const inputText = ref('')
const isTyping = ref(false)
const scrollToView = ref('')

// AI 智能回复库
const aiResponses = {
  // 情绪相关
  伤心: ['我能感受到你的难过，想和我说说发生了什么吗？', '别难过，我会一直陪着你的。有什么想倾诉的吗？'],
  开心: ['看到你开心我也很高兴！是发生了什么好事吗？', '能分享一下你的快乐吗？我也想一起开心！'],
  累: ['辛苦了！要不要休息一下？我可以陪你聊聊轻松的话题。', '工作确实很累，记得好好照顾自己哦。'],
  焦虑: ['别太焦虑了，我们一起想办法解决问题吧。', '深呼吸，放松一下。焦虑的时候可以试着做点让自己开心的事情。'],
  孤独: ['你不是一个人，我会一直陪伴着你。', '想聊什么都可以，我会认真倾听的。'],
  
  // 问候
  你好: ['你好呀！很高兴见到你！今天过得怎么样？', '嗨！有什么想聊的吗？'],
  早: ['早上好！新的一天开始了，今天要加油哦！', '早安！祝你有美好的一天！'],
  晚: ['晚上好！今天过得怎么样？', '晚安，祝你做个好梦！'],
  
  // 鼓励
  加油: ['加油！你一定可以的！', '相信自己，你比想象中更强大！'],
  谢谢: ['不客气！很高兴能帮到你。', '这是我应该做的，随时都可以找我聊天哦！'],
  
  // 默认回复
  default: [
    '我在认真听你说话，请继续。',
    '听起来很有意思，能详细说说吗？',
    '我理解你的感受。',
    '嗯，然后呢？',
    '这确实是个值得思考的问题。',
    '你的想法很棒！',
  ]
}

onLoad((options: any) => {
  if (options.id) {
    chatId.value = options.id
    loadChat()
  } else {
    // 新聊天
    chatId.value = Date.now().toString()
    messages.value = [{
      id: '1',
      role: 'assistant',
      content: '你好！我是你的 AI 陪伴，有什么想聊的吗？😊',
      timestamp: Date.now()
    }]
  }
})

onShow(() => {
  // 每次显示时滚动到底部
  if (messages.value.length > 0) {
    const lastMsg = messages.value[messages.value.length - 1]
    nextTick(() => {
      scrollToView.value = `msg-${lastMsg.id}`
    })
  }
})

const loadChat = () => {
  const chats = getStorage<any>('aiChats', {})
  if (chats[chatId.value]) {
    messages.value = chats[chatId.value].messages || []
  }
}

const saveChat = () => {
  const chats = getStorage<any>('aiChats', {})
  chats[chatId.value] = {
    id: chatId.value,
    messages: messages.value,
    updatedAt: Date.now()
  }
  setStorage('aiChats', chats)
  
  // 更新聊天列表
  const chatList = getStorage<any[]>('aiChatList', [])
  const existingIndex = chatList.findIndex(c => c.id === chatId.value)
  
  const chatItem = {
    id: chatId.value,
    lastMessage: messages.value[messages.value.length - 1].content,
    updatedAt: Date.now()
  }
  
  if (existingIndex >= 0) {
    chatList[existingIndex] = chatItem
  } else {
    chatList.unshift(chatItem)
  }
  
  setStorage('aiChatList', chatList)
}

const getAIResponse = (userInput: string): string => {
  // 根据关键词匹配回复
  for (const [keyword, responses] of Object.entries(aiResponses)) {
    if (keyword !== 'default' && userInput.includes(keyword)) {
      return responses[Math.floor(Math.random() * responses.length)]
    }
  }
  
  // 默认回复
  const defaultResponses = aiResponses.default
  return defaultResponses[Math.floor(Math.random() * defaultResponses.length)]
}

const sendMessage = async () => {
  if (!inputText.value.trim()) return

  const userMessage: Message = {
    id: Date.now().toString(),
    role: 'user',
    content: inputText.value.trim(),
    timestamp: Date.now()
  }

  const userInput = inputText.value.trim()
  messages.value.push(userMessage)
  inputText.value = ''

  // 保存聊天记录
  saveChat()

  // 滚动到底部
  await nextTick()
  scrollToView.value = `msg-${userMessage.id}`

  // AI 回复
  isTyping.value = true
  setTimeout(() => {
    const aiMessage: Message = {
      id: (Date.now() + 1).toString(),
      role: 'assistant',
      content: getAIResponse(userInput),
      timestamp: Date.now()
    }
    messages.value.push(aiMessage)
    isTyping.value = false
    saveChat()
    nextTick(() => {
      scrollToView.value = `msg-${aiMessage.id}`
    })
  }, 1000 + Math.random() * 1000) // 1-2秒随机延迟
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.chat-page {
  height: 100vh;
  display: flex;
  flex-direction: column;
  @include cyber-page-bg;
  position: relative;
}

.message-list {
  flex: 1;
  padding: 20rpx 30rpx;
  overflow-y: auto;
  position: relative;
  z-index: 1;
}

.message-item {
  display: flex;
  margin-bottom: 30rpx;
  animation: messageSlideIn 0.4s ease-out;

  &.message-user {
    flex-direction: row-reverse;

    .message-avatar {
      background: linear-gradient(135deg, rgba(0, 217, 255, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
      border: 2rpx solid rgba(0, 217, 255, 0.6);
      box-shadow: 0 0 20rpx rgba(0, 217, 255, 0.5);
    }

    .message-content {
      background: linear-gradient(135deg, rgba(0, 217, 255, 0.8) 0%, rgba(79, 172, 254, 0.8) 100%);
      border: 2rpx solid rgba(0, 217, 255, 0.6);
      box-shadow: 
        0 8rpx 24rpx rgba(0, 0, 0, 0.3),
        0 0 20rpx rgba(0, 217, 255, 0.4),
        inset 0 0 20rpx rgba(0, 217, 255, 0.1);
      @include neon-text(#ffffff);
    }
  }

  .message-avatar {
    width: 70rpx;
    height: 70rpx;
    border-radius: 35rpx;
    background: linear-gradient(135deg, rgba(138, 92, 246, 0.3) 0%, rgba(240, 147, 251, 0.3) 100%);
    border: 2rpx solid rgba(138, 92, 246, 0.6);
    box-shadow: 0 0 20rpx rgba(138, 92, 246, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 36rpx;
    flex-shrink: 0;
    animation: avatarPulse 2s ease-in-out infinite;
  }
}

@keyframes messageSlideIn {
  from {
    opacity: 0;
    transform: translateY(20rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes avatarPulse {
  0%, 100% {
    box-shadow: 0 0 20rpx currentColor;
  }
  50% {
    box-shadow: 0 0 30rpx currentColor;
  }
}

.message-content {
  max-width: 500rpx;
  background: rgba(30, 36, 66, 0.7);
  backdrop-filter: blur(10rpx);
  border: 2rpx solid rgba(138, 92, 246, 0.4);
  border-radius: 15rpx;
  padding: 20rpx;
  margin: 0 15rpx;
  box-shadow: 
    0 8rpx 24rpx rgba(0, 0, 0, 0.3),
    0 0 20rpx rgba(138, 92, 246, 0.3),
    inset 0 0 20rpx rgba(138, 92, 246, 0.05);

  .message-text {
    font-size: 28rpx;
    line-height: 1.6;
    color: #ffffff;
    text-shadow: 0 0 8rpx rgba(255, 255, 255, 0.3);
  }

  .typing-indicator {
    font-size: 28rpx;
    @include neon-text(#8B5CF6);
    animation: blink 1.5s ease-in-out infinite;
  }
}

@keyframes blink {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 1; }
}

.input-bar {
  display: flex;
  align-items: center;
  padding: 25rpx 30rpx;
  background: linear-gradient(180deg, rgba(30, 36, 66, 0.95) 0%, rgba(30, 36, 66, 0.9) 100%);
  backdrop-filter: blur(30rpx);
  border-top: 2rpx solid rgba(0, 217, 255, 0.4);
  box-shadow: 
    0 -10rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 30rpx rgba(0, 217, 255, 0.15),
    inset 0 2rpx 0 rgba(255, 255, 255, 0.1),
    inset 0 4rpx 0 rgba(0, 217, 255, 0.2);
  position: sticky;
  bottom: 0;
  z-index: 10;

  .message-input {
    flex: 1;
    height: 70rpx;
    border-radius: 35rpx;
    padding: 0 30rpx;
    font-size: 28rpx;
    color: #ffffff;
    background: rgba(30, 36, 66, 0.6);
    backdrop-filter: blur(10rpx);
    border: 2rpx solid rgba(138, 92, 246, 0.4);
    box-shadow: 
      0 4rpx 16rpx rgba(0, 0, 0, 0.2),
      0 0 20rpx rgba(138, 92, 246, 0.2),
      inset 0 0 20rpx rgba(138, 92, 246, 0.05);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    
    &:focus {
      border-color: rgba(0, 217, 255, 0.8);
      background: rgba(30, 36, 66, 0.7);
      box-shadow: 
        0 6rpx 20rpx rgba(0, 217, 255, 0.3),
        0 0 40rpx rgba(0, 217, 255, 0.4),
        inset 0 0 30rpx rgba(0, 217, 255, 0.1);
      transform: translateY(-2rpx);
    }
    
    &::placeholder {
      color: #6b7b93;
      text-shadow: 0 0 5rpx rgba(107, 123, 147, 0.5);
    }
  }

  .send-btn {
    width: 130rpx;
    height: 70rpx;
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.95) 0%, rgba(138, 92, 246, 0.95) 100%);
    border: 2rpx solid rgba(0, 217, 255, 0.7);
    border-radius: 35rpx;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: 15rpx;
    font-size: 28rpx;
    font-weight: bold;
    box-shadow: 
      0 8rpx 24rpx rgba(0, 217, 255, 0.5),
      0 0 40rpx rgba(0, 217, 255, 0.4),
      inset 0 0 30rpx rgba(0, 217, 255, 0.15);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
    animation: sendBtnFloat 3s ease-in-out infinite;
    
    text {
      position: relative;
      z-index: 2;
      @include neon-text(#ffffff);
      text-shadow: 
        0 0 15rpx rgba(0, 217, 255, 1),
        0 0 30rpx rgba(0, 217, 255, 0.6),
        0 2rpx 6rpx rgba(0, 0, 0, 0.6);
      letter-spacing: 2rpx;
    }
    
    &::before {
      content: '✨';
      position: absolute;
      left: 15rpx;
      font-size: 24rpx;
      z-index: 2;
      animation: sparkle 2s ease-in-out infinite;
      filter: drop-shadow(0 0 8rpx rgba(255, 255, 255, 0.8));
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
        rgba(255, 255, 255, 0.4) 60deg,
        transparent 120deg,
        rgba(255, 255, 255, 0.4) 240deg,
        transparent 300deg
      );
      animation: rotateShine 3s linear infinite;
      z-index: 1;
    }

    &:active {
      transform: scale(0.92) translateY(2rpx);
      box-shadow: 
        0 4rpx 16rpx rgba(0, 217, 255, 0.7),
        0 0 30rpx rgba(0, 217, 255, 0.6),
        inset 0 0 40rpx rgba(0, 217, 255, 0.25);
      animation: none;
      
      &::before {
        animation: none;
        transform: scale(1.3);
      }
    }
  }
}

@keyframes sendBtnFloat {
  0%, 100% {
    transform: translateY(0);
    box-shadow: 
      0 8rpx 24rpx rgba(0, 217, 255, 0.5),
      0 0 40rpx rgba(0, 217, 255, 0.4),
      inset 0 0 30rpx rgba(0, 217, 255, 0.15);
  }
  50% {
    transform: translateY(-3rpx);
    box-shadow: 
      0 12rpx 32rpx rgba(0, 217, 255, 0.6),
      0 0 50rpx rgba(0, 217, 255, 0.5),
      inset 0 0 40rpx rgba(0, 217, 255, 0.2);
  }
}

@keyframes rotateShine {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes sparkle {
  0%, 100% {
    opacity: 0.6;
    transform: scale(1) rotate(0deg);
  }
  50% {
    opacity: 1;
    transform: scale(1.2) rotate(180deg);
  }
}
</style>

