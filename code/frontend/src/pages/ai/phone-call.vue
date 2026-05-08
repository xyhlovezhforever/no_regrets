<template>
  <view class="phone-call-page">
    <!-- 背景动画 -->
    <view class="call-background">
      <view class="ripple" :class="{ active: isConnected || isRinging }"></view>
      <view class="ripple" :class="{ active: isConnected || isRinging }" style="animation-delay: 0.3s"></view>
      <view class="ripple" :class="{ active: isConnected || isRinging }" style="animation-delay: 0.6s"></view>
    </view>

    <!-- AI 头像 -->
    <view class="ai-avatar">
      <text class="avatar-emoji">🤖</text>
    </view>

    <!-- 状态信息 -->
    <view class="call-info">
      <text class="ai-name">AI 小助手</text>
      <text class="call-status">{{ statusText }}</text>
      <text v-if="isConnected" class="call-duration">{{ formatDuration(callDuration) }}</text>
    </view>

    <!-- 语音波形（通话中显示） -->
    <view v-if="isConnected && (isSpeaking || isListening)" class="voice-wave">
      <view v-for="i in 5" :key="i" class="wave-bar" :style="{ animationDelay: `${i * 0.1}s` }"></view>
    </view>

    <!-- 识别的文字 -->
    <view v-if="recognizedText" class="recognized-text">
      <text>{{ recognizedText }}</text>
    </view>

    <!-- 操作按钮 -->
    <view class="call-actions">
      <!-- 未接通时：拨打/取消 -->
      <view v-if="!isConnected && !isRinging" class="action-group">
        <view class="call-btn-wrapper">
          <view class="glow-particle" style="top: 10%; left: 10%;"></view>
          <view class="glow-particle" style="top: 10%; right: 10%; animation-delay: 0.5s;"></view>
          <view class="glow-particle" style="bottom: 10%; left: 10%; animation-delay: 1s;"></view>
          <view class="glow-particle" style="bottom: 10%; right: 10%; animation-delay: 1.5s;"></view>
          <button class="action-btn call-btn" @click="startCall">
            <text class="btn-icon">📞</text>
            <text class="btn-text">拨打电话</text>
          </button>
        </view>
      </view>

      <!-- 拨号中：取消 -->
      <view v-else-if="isRinging" class="action-group">
        <button class="action-btn cancel-btn" @click="cancelCall">
          <text class="btn-icon">✕</text>
          <text class="btn-text">取消</text>
        </button>
      </view>

      <!-- 通话中：静音、免提、挂断、说话 -->
      <view v-else class="action-group connected">
        <button class="action-btn small-btn" :class="{ active: isMuted }" @click="toggleMute">
          <text class="btn-icon">{{ isMuted ? '🔇' : '🔊' }}</text>
          <text class="btn-text">{{ isMuted ? '已静音' : '静音' }}</text>
        </button>

        <button class="action-btn small-btn" :class="{ active: isSpeakerOn }" @click="toggleSpeaker">
          <text class="btn-icon">{{ isSpeakerOn ? '📢' : '📱' }}</text>
          <text class="btn-text">{{ isSpeakerOn ? '免提' : '听筒' }}</text>
        </button>

        <button class="action-btn hangup-btn" @click="endCall">
          <text class="btn-icon">📵</text>
        </button>

        <button 
          class="action-btn speak-btn"
          :class="{ active: isListening }"
          @touchstart="startSpeaking"
          @touchend="stopSpeaking"
          @touchcancel="stopSpeaking"
        >
          <text class="btn-icon">🎤</text>
          <text class="btn-text">{{ isListening ? '松开发送' : '按住说话' }}</text>
        </button>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

const isRinging = ref(false)
const isConnected = ref(false)
const isMuted = ref(false)
const isSpeakerOn = ref(false)
const isSpeaking = ref(false)
const isListening = ref(false)
const callDuration = ref(0)
const recognizedText = ref('')

let callTimer: any = null
let durationTimer: any = null
let speakingTimeout: any = null

const statusText = computed(() => {
  if (isConnected.value) return '通话中...'
  if (isRinging.value) return '正在呼叫...'
  return '准备拨打'
})

onMounted(() => {
  // 初始化
})

onUnmounted(() => {
  if (callTimer) clearTimeout(callTimer)
  if (durationTimer) clearInterval(durationTimer)
  if (speakingTimeout) clearTimeout(speakingTimeout)
})

const startCall = () => {
  isRinging.value = true
  uni.vibrateShort({ type: 'heavy' })
  
  // 模拟接通（2秒后）
  callTimer = setTimeout(() => {
    isRinging.value = false
    isConnected.value = true
    
    uni.showToast({
      title: 'AI已接听',
      icon: 'success'
    })
    
    // 开始计时
    durationTimer = setInterval(() => {
      callDuration.value++
    }, 1000)
    
    // AI自动问候
    setTimeout(() => {
      speakAIResponse('你好！我是AI小助手，有什么可以帮你的吗？')
    }, 500)
  }, 2000)
}

const cancelCall = () => {
  isRinging.value = false
  if (callTimer) {
    clearTimeout(callTimer)
    callTimer = null
  }
  uni.showToast({
    title: '已取消',
    icon: 'none'
  })
}

const endCall = () => {
  isConnected.value = false
  callDuration.value = 0
  
  if (durationTimer) {
    clearInterval(durationTimer)
    durationTimer = null
  }
  
  uni.showToast({
    title: '通话已结束',
    icon: 'none'
  })
  
  // 1秒后返回
  setTimeout(() => {
    uni.navigateBack()
  }, 1000)
}

const toggleMute = () => {
  isMuted.value = !isMuted.value
  uni.showToast({
    title: isMuted.value ? '已静音' : '取消静音',
    icon: 'none'
  })
}

const toggleSpeaker = () => {
  isSpeakerOn.value = !isSpeakerOn.value
  uni.showToast({
    title: isSpeakerOn.value ? '免提已开启' : '免提已关闭',
    icon: 'none'
  })
}

const startSpeaking = () => {
  if (!isConnected.value || isMuted.value) return
  
  isListening.value = true
  recognizedText.value = ''
  uni.vibrateShort({ type: 'light' })
  
  // 开始语音识别（实际应调用 uni.startRecord 或其他录音API）
  // 这里模拟识别过程
}

const stopSpeaking = () => {
  if (!isListening.value) return
  
  isListening.value = false
  uni.vibrateShort({ type: 'light' })
  
  // 停止语音识别
  // 模拟识别结果
  speakingTimeout = setTimeout(() => {
    const mockTexts = [
      '今天天气怎么样？',
      '我想听首歌',
      '给我讲个笑话',
      '我有点累了',
      '帮我查一下明天的日程'
    ]
    recognizedText.value = mockTexts[Math.floor(Math.random() * mockTexts.length)]
    
    // 显示识别结果2秒后，AI回应
    setTimeout(() => {
      const response = generateAIResponse(recognizedText.value)
      speakAIResponse(response)
    }, 1500)
  }, 500)
}

const generateAIResponse = (userText: string): string => {
  if (userText.includes('天气')) {
    return '今天天气不错，阳光明媚，适合出门走走哦~'
  } else if (userText.includes('歌') || userText.includes('音乐')) {
    return '好的，为你推荐一首轻松的音乐，希望你喜欢~'
  } else if (userText.includes('笑话')) {
    return '为什么程序员喜欢黑夜？因为黑夜有很多Bug！哈哈~'
  } else if (userText.includes('累')) {
    return '听起来你有点累了，要不要休息一下？我可以给你放点轻音乐。'
  } else if (userText.includes('日程')) {
    return '明天你有三个重要事项：早上9点开会，下午2点项目讨论，晚上7点健身。'
  } else {
    return '我明白了，让我想想该怎么帮你...'
  }
}

const speakAIResponse = (text: string) => {
  isSpeaking.value = true
  recognizedText.value = `AI: ${text}`
  
  // 模拟AI说话（根据文字长度计算时间）
  const duration = text.length * 200
  
  setTimeout(() => {
    isSpeaking.value = false
    
    // 再过1秒清除文字
    setTimeout(() => {
      recognizedText.value = ''
    }, 1000)
  }, duration)
}

const formatDuration = (seconds: number): string => {
  const mins = Math.floor(seconds / 60)
  const secs = seconds % 60
  return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.phone-call-page {
  min-height: 100vh;
  @include cyber-page-bg;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  padding: 100rpx 30rpx 50rpx;
  position: relative;
  overflow: hidden;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 50% 30%, rgba(255, 0, 214, 0.15) 0%, transparent 60%),
      radial-gradient(circle at 50% 70%, rgba(0, 217, 255, 0.15) 0%, transparent 60%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
  }
}

.call-background {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 600rpx;
  height: 600rpx;
  pointer-events: none;
  z-index: 0;

  .ripple {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 100%;
    height: 100%;
    border: 3rpx solid rgba(255, 0, 214, 0.6);
    border-radius: 50%;
    opacity: 0;
    box-shadow: 
      0 0 40rpx rgba(255, 0, 214, 0.8),
      inset 0 0 40rpx rgba(255, 0, 214, 0.3);

    &.active {
      animation: neonRipple 2s ease-out infinite;
    }
    
    &:nth-child(2) {
      border-color: rgba(0, 217, 255, 0.6);
      box-shadow: 
        0 0 40rpx rgba(0, 217, 255, 0.8),
        inset 0 0 40rpx rgba(0, 217, 255, 0.3);
    }
    
    &:nth-child(3) {
      border-color: rgba(138, 92, 246, 0.6);
      box-shadow: 
        0 0 40rpx rgba(138, 92, 246, 0.8),
        inset 0 0 40rpx rgba(138, 92, 246, 0.3);
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 0.8; }
}

@keyframes neonRipple {
  0% {
    width: 200rpx;
    height: 200rpx;
    opacity: 1;
  }
  100% {
    width: 600rpx;
    height: 600rpx;
    opacity: 0;
  }
}

.ai-avatar {
  width: 220rpx;
  height: 220rpx;
  background: linear-gradient(135deg, rgba(255, 0, 214, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
  border: 3rpx solid rgba(255, 0, 214, 0.8);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(255, 0, 214, 0.6),
    inset 0 0 60rpx rgba(255, 0, 214, 0.2);
  backdrop-filter: blur(20rpx);
  position: relative;
  z-index: 1;
  animation: avatarGlow 3s ease-in-out infinite;
  
  &::before {
    content: '';
    position: absolute;
    top: -10rpx;
    left: -10rpx;
    right: -10rpx;
    bottom: -10rpx;
    background: conic-gradient(
      from 0deg,
      transparent 0deg,
      rgba(255, 0, 214, 0.4) 90deg,
      transparent 180deg,
      rgba(0, 217, 255, 0.4) 270deg,
      transparent 360deg
    );
    border-radius: 50%;
    animation: rotate 4s linear infinite;
    z-index: -1;
  }

  .avatar-emoji {
    font-size: 120rpx;
    filter: drop-shadow(0 0 20rpx rgba(255, 0, 214, 0.8));
    animation: emojiFloat 3s ease-in-out infinite;
  }
}

.call-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-top: 60rpx;
  position: relative;
  z-index: 1;

  .ai-name {
    font-size: 48rpx;
    font-weight: bold;
    @include neon-title(#FF00D6);
    margin-bottom: 20rpx;
    animation: titlePulse 3s ease-in-out infinite;
  }

  .call-status {
    font-size: 28rpx;
    @include neon-text(#00D9FF);
    animation: statusBlink 2s ease-in-out infinite;
  }

  .call-duration {
    font-size: 40rpx;
    font-weight: bold;
    @include neon-text(#ffffff);
    margin-top: 20rpx;
    font-family: 'Courier New', monospace;
    letter-spacing: 4rpx;
    text-shadow: 
      0 0 20rpx rgba(255, 0, 214, 1),
      0 0 40rpx rgba(255, 0, 214, 0.8);
  }
}

.voice-wave {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12rpx;
  margin-top: 60rpx;
  position: relative;
  z-index: 1;

  .wave-bar {
    width: 12rpx;
    height: 50rpx;
    background: linear-gradient(180deg, rgba(0, 217, 255, 1) 0%, rgba(138, 92, 246, 1) 100%);
    border-radius: 6rpx;
    box-shadow: 
      0 0 20rpx rgba(0, 217, 255, 0.8),
      inset 0 0 10rpx rgba(0, 217, 255, 0.5);
    animation: neonWave 1s ease-in-out infinite;
    
    &:nth-child(2) {
      animation-delay: 0.1s;
    }
    &:nth-child(3) {
      animation-delay: 0.2s;
    }
    &:nth-child(4) {
      animation-delay: 0.3s;
    }
    &:nth-child(5) {
      animation-delay: 0.4s;
    }
  }
}

@keyframes neonWave {
  0%, 100% {
    transform: scaleY(1);
    box-shadow: 
      0 0 20rpx rgba(0, 217, 255, 0.8),
      inset 0 0 10rpx rgba(0, 217, 255, 0.5);
  }
  50% {
    transform: scaleY(2.5);
    box-shadow: 
      0 0 40rpx rgba(0, 217, 255, 1),
      inset 0 0 20rpx rgba(0, 217, 255, 0.8);
  }
}

.recognized-text {
  margin-top: 40rpx;
  padding: 30rpx 40rpx;
  background: linear-gradient(135deg, rgba(30, 36, 66, 0.8) 0%, rgba(30, 36, 66, 0.6) 100%);
  backdrop-filter: blur(20rpx);
  border-radius: 25rpx;
  max-width: 600rpx;
  border: 2rpx solid rgba(138, 92, 246, 0.6);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(138, 92, 246, 0.4),
    inset 0 0 40rpx rgba(138, 92, 246, 0.1);
  position: relative;
  z-index: 1;
  animation: textAppear 0.5s ease-out;

  text {
    font-size: 30rpx;
    @include neon-text(#ffffff);
    line-height: 1.8;
    text-shadow: 0 0 10rpx rgba(255, 255, 255, 0.5);
  }
}

.call-actions {
  width: 100%;
  margin-top: auto;
  position: relative;
  z-index: 1;

  .action-group {
    display: flex;
    flex-direction: column;
    gap: 20rpx;

    &.connected {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 20rpx;

      .hangup-btn {
        grid-column: 1 / -1;
      }

      .speak-btn {
        grid-column: 1 / -1;
      }
    }
  }

  .call-btn-wrapper {
    position: relative;
    width: 100%;
    
    .glow-particle {
      position: absolute;
      width: 12rpx;
      height: 12rpx;
      background: radial-gradient(circle, rgba(0, 217, 255, 1) 0%, transparent 70%);
      border-radius: 50%;
      box-shadow: 
        0 0 20rpx rgba(0, 217, 255, 1),
        0 0 40rpx rgba(0, 217, 255, 0.8);
      animation: particleFloat 3s ease-in-out infinite;
      pointer-events: none;
      z-index: 1;
      
      &:nth-child(2) {
        background: radial-gradient(circle, rgba(138, 92, 246, 1) 0%, transparent 70%);
        box-shadow: 
          0 0 20rpx rgba(138, 92, 246, 1),
          0 0 40rpx rgba(138, 92, 246, 0.8);
      }
      
      &:nth-child(3) {
        background: radial-gradient(circle, rgba(255, 0, 214, 1) 0%, transparent 70%);
        box-shadow: 
          0 0 20rpx rgba(255, 0, 214, 1),
          0 0 40rpx rgba(255, 0, 214, 0.8);
      }
      
      &:nth-child(4) {
        background: radial-gradient(circle, rgba(0, 217, 255, 1) 0%, transparent 70%);
        box-shadow: 
          0 0 20rpx rgba(0, 217, 255, 1),
          0 0 40rpx rgba(0, 217, 255, 0.8);
      }
    }
  }

  .action-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 30rpx;
    border-radius: 25rpx;
    border: none;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;

    &::after {
      border: none;
    }

    &:active {
      transform: scale(0.95) translateY(2rpx);
    }

    .btn-icon {
      font-size: 60rpx;
      margin-bottom: 10rpx;
      filter: drop-shadow(0 0 15rpx rgba(255, 255, 255, 0.8));
      position: relative;
      z-index: 2;
    }

    .btn-text {
      font-size: 26rpx;
      font-weight: bold;
      @include neon-text(#ffffff);
      letter-spacing: 1rpx;
      position: relative;
      z-index: 2;
    }
  }

  .call-btn {
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.95) 0%, rgba(138, 92, 246, 0.95) 50%, rgba(255, 0, 214, 0.95) 100%);
    border: 3rpx solid transparent;
    background-clip: padding-box;
    position: relative;
    padding: 40rpx 30rpx;
    min-height: 140rpx;
    box-shadow: 
      0 20rpx 60rpx rgba(0, 217, 255, 0.6),
      0 0 80rpx rgba(0, 217, 255, 0.5),
      0 0 120rpx rgba(138, 92, 246, 0.4),
      inset 0 0 60rpx rgba(0, 217, 255, 0.2);
    animation: callBtnFloat 3s ease-in-out infinite;
    
    &::before {
      content: '';
      position: absolute;
      inset: -3rpx;
      border-radius: 25rpx;
      padding: 3rpx;
      background: linear-gradient(45deg, 
        rgba(0, 217, 255, 0.8) 0%,
        rgba(138, 92, 246, 0.8) 25%,
        rgba(255, 0, 214, 0.8) 50%,
        rgba(138, 92, 246, 0.8) 75%,
        rgba(0, 217, 255, 0.8) 100%
      );
      -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
      -webkit-mask-composite: xor;
      mask-composite: exclude;
      animation: borderRotate 4s linear infinite;
      z-index: -1;
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
      animation: rotate 3s linear infinite;
      z-index: 0;
    }
    
    .btn-icon {
      animation: callIconPulse 2s ease-in-out infinite;
      filter: drop-shadow(0 0 20rpx rgba(255, 255, 255, 1));
      font-size: 72rpx;
    }
    
    .btn-text {
      font-size: 32rpx;
      letter-spacing: 3rpx;
      text-shadow: 
        0 0 20rpx rgba(0, 217, 255, 1),
        0 0 40rpx rgba(0, 217, 255, 0.8),
        0 4rpx 8rpx rgba(0, 0, 0, 0.6);
    }
    
    &:active {
      transform: scale(0.96) translateY(4rpx);
      animation: none;
      box-shadow: 
        0 10rpx 40rpx rgba(0, 217, 255, 0.8),
        0 0 100rpx rgba(0, 217, 255, 0.7),
        0 0 150rpx rgba(138, 92, 246, 0.6),
        inset 0 0 80rpx rgba(0, 217, 255, 0.3);
      
      .btn-icon {
        transform: scale(1.1);
        filter: drop-shadow(0 0 30rpx rgba(255, 255, 255, 1));
      }
    }
  }

  .cancel-btn {
    background: linear-gradient(135deg, rgba(255, 77, 79, 0.9) 0%, rgba(245, 87, 108, 0.9) 100%);
    border: 2rpx solid rgba(255, 77, 79, 0.7);
    box-shadow: 
      0 15rpx 50rpx rgba(255, 77, 79, 0.5),
      0 0 60rpx rgba(255, 77, 79, 0.4),
      inset 0 0 40rpx rgba(255, 77, 79, 0.2);
  }

  .hangup-btn {
    background: linear-gradient(135deg, rgba(255, 77, 79, 0.95) 0%, rgba(245, 87, 108, 0.95) 100%);
    border: 3rpx solid rgba(255, 77, 79, 0.8);
    box-shadow: 
      0 20rpx 60rpx rgba(255, 77, 79, 0.6),
      0 0 80rpx rgba(255, 77, 79, 0.5),
      inset 0 0 50rpx rgba(255, 77, 79, 0.3);
    border-radius: 50%;
    width: 130rpx;
    height: 130rpx;
    padding: 0;
    margin: 0 auto;
    animation: hangupPulse 2s ease-in-out infinite;
    
    &::before {
      content: '';
      position: absolute;
      inset: -8rpx;
      border-radius: 50%;
      background: conic-gradient(
        from 0deg,
        transparent 0deg,
        rgba(255, 255, 255, 0.4) 60deg,
        transparent 120deg,
        rgba(255, 255, 255, 0.4) 240deg,
        transparent 300deg
      );
      animation: rotate 3s linear infinite;
      z-index: -1;
    }

    .btn-icon {
      font-size: 64rpx;
      margin: 0;
    }
  }

  .small-btn {
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.7) 0%, rgba(30, 36, 66, 0.5) 100%);
    backdrop-filter: blur(20rpx);
    border: 2rpx solid rgba(138, 92, 246, 0.5);
    box-shadow: 
      0 8rpx 24rpx rgba(0, 0, 0, 0.3),
      0 0 30rpx rgba(138, 92, 246, 0.3),
      inset 0 0 20rpx rgba(138, 92, 246, 0.1);

    &.active {
      background: linear-gradient(135deg, rgba(138, 92, 246, 0.6) 0%, rgba(240, 147, 251, 0.6) 100%);
      border-color: rgba(138, 92, 246, 0.8);
      box-shadow: 
        0 10rpx 30rpx rgba(138, 92, 246, 0.5),
        0 0 50rpx rgba(138, 92, 246, 0.6),
        inset 0 0 40rpx rgba(138, 92, 246, 0.2);
    }
  }

  .speak-btn {
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(240, 147, 251, 0.9) 100%);
    border: 2rpx solid rgba(255, 0, 214, 0.7);
    box-shadow: 
      0 20rpx 60rpx rgba(255, 0, 214, 0.5),
      0 0 70rpx rgba(255, 0, 214, 0.4),
      inset 0 0 50rpx rgba(255, 0, 214, 0.2);
    min-height: 130rpx;
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
        transparent 180deg,
        rgba(255, 255, 255, 0.3) 270deg,
        transparent 360deg
      );
      animation: rotate 4s linear infinite;
    }

    &.active {
      background: linear-gradient(135deg, rgba(245, 87, 108, 0.95) 0%, rgba(255, 0, 214, 0.95) 100%);
      box-shadow: 
        0 25rpx 70rpx rgba(245, 87, 108, 0.7),
        0 0 90rpx rgba(245, 87, 108, 0.6),
        inset 0 0 60rpx rgba(245, 87, 108, 0.3);
      animation: speakPulse 0.5s ease infinite;

      .btn-icon {
        animation: micPulse 0.5s ease infinite;
      }
      
      &::before {
        animation: rotate 2s linear infinite;
      }
    }
  }
}

@keyframes micPulse {
  0%, 100% {
    transform: scale(1);
    filter: drop-shadow(0 0 15rpx rgba(255, 255, 255, 0.8));
  }
  50% {
    transform: scale(1.15);
    filter: drop-shadow(0 0 30rpx rgba(255, 255, 255, 1));
  }
}

@keyframes speakPulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.02);
  }
}

@keyframes avatarGlow {
  0%, 100% {
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.5),
      0 0 60rpx rgba(255, 0, 214, 0.6),
      inset 0 0 60rpx rgba(255, 0, 214, 0.2);
  }
  50% {
    box-shadow: 
      0 25rpx 70rpx rgba(0, 0, 0, 0.6),
      0 0 80rpx rgba(255, 0, 214, 0.8),
      inset 0 0 80rpx rgba(255, 0, 214, 0.3);
  }
}

@keyframes emojiFloat {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-10rpx) rotate(5deg);
  }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 15rpx rgba(255, 0, 214, 0.8),
      0 0 30rpx rgba(255, 0, 214, 0.5);
  }
  50% {
    text-shadow: 
      0 0 25rpx rgba(255, 0, 214, 1),
      0 0 50rpx rgba(255, 0, 214, 0.8);
  }
}

@keyframes statusBlink {
  0%, 100% { opacity: 0.7; }
  50% { opacity: 1; }
}

@keyframes textAppear {
  from {
    opacity: 0;
    transform: translateY(20rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes callBtnFloat {
  0%, 100% {
    transform: translateY(0);
    box-shadow: 
      0 20rpx 60rpx rgba(0, 217, 255, 0.6),
      0 0 80rpx rgba(0, 217, 255, 0.5),
      0 0 120rpx rgba(138, 92, 246, 0.4),
      inset 0 0 60rpx rgba(0, 217, 255, 0.2);
  }
  50% {
    transform: translateY(-8rpx);
    box-shadow: 
      0 28rpx 80rpx rgba(0, 217, 255, 0.8),
      0 0 100rpx rgba(0, 217, 255, 0.7),
      0 0 150rpx rgba(138, 92, 246, 0.6),
      inset 0 0 80rpx rgba(0, 217, 255, 0.3);
  }
}

@keyframes borderRotate {
  from {
    filter: hue-rotate(0deg);
  }
  to {
    filter: hue-rotate(360deg);
  }
}

@keyframes callIconPulse {
  0%, 100% {
    transform: scale(1) rotate(0deg);
    filter: drop-shadow(0 0 20rpx rgba(255, 255, 255, 1));
  }
  25% {
    transform: scale(1.1) rotate(-5deg);
  }
  50% {
    transform: scale(1.15) rotate(0deg);
    filter: drop-shadow(0 0 35rpx rgba(255, 255, 255, 1));
  }
  75% {
    transform: scale(1.1) rotate(5deg);
  }
}

@keyframes hangupPulse {
  0%, 100% {
    box-shadow: 
      0 20rpx 60rpx rgba(255, 77, 79, 0.6),
      0 0 80rpx rgba(255, 77, 79, 0.5),
      inset 0 0 50rpx rgba(255, 77, 79, 0.3);
  }
  50% {
    box-shadow: 
      0 25rpx 70rpx rgba(255, 77, 79, 0.8),
      0 0 100rpx rgba(255, 77, 79, 0.7),
      inset 0 0 60rpx rgba(255, 77, 79, 0.4);
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes particleFloat {
  0%, 100% {
    transform: translateY(0) scale(1);
    opacity: 0.6;
  }
  50% {
    transform: translateY(-15rpx) scale(1.3);
    opacity: 1;
  }
}
</style>

