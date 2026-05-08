<template>
  <view class="page">
    <!-- 空状态 -->
    <view v-if="friends.length === 0" class="empty-state">
      <text class="empty-icon">👥</text>
      <text class="empty-text">暂无好友</text>
      <text class="empty-hint">请先添加好友后再写情书</text>
      <button class="btn-add-friend" @click="goToAddFriend">去添加好友</button>
    </view>

    <!-- 表单 -->
    <view v-else class="form">
      <view class="form-item">
        <text class="label">收信人 *</text>
        <picker :value="selectedFriendIndex" :range="friendNames" @change="onFriendChange">
          <view class="picker" :class="{ placeholder: !selectedFriend }">
            {{ selectedFriend ? (selectedFriend.nickname || selectedFriend.username || '未命名好友') : '请选择好友' }}
          </view>
        </picker>
      </view>

      <view class="form-item">
        <text class="label">寄信人</text>
        <view class="sender-info">{{ currentUserName }}</view>
      </view>

      <!-- 情书内容表单项，内部带 AI 生成功能 -->
      <view class="form-item">
        <view class="content-header">
          <text class="label">情书内容</text>
          <button class="btn-generate-inline" @click="generate">AI 生成</button>
        </view>
        <textarea
          class="result-text"
          v-model="result"
          placeholder="写下你想对TA说的话，或点击右侧 AI 生成"
          :auto-height="true"
        />
      </view>

      <view class="result">
        <button class="btn-send" @click="send">寄给心爱的他/她 💌</button>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { onShow, onLoad } from '@dcloudio/uni-app'
import { getFriendsApi, sendFriendMessageApi } from '@/api/friend'
import type { Friend } from '@/api/friend'
import { createLoveLetterApi } from '@/api/loveLetter'
import { useUserStore } from '@/store/user'

const userStore = useUserStore()
const friends = ref<Friend[]>([])
const selectedFriendIndex = ref(-1)
const result = ref('')
const isReply = ref(false)
const replyToUserId = ref('')

// 当前用户名
const currentUserName = computed(() => userStore.userInfo?.nickname || userStore.userInfo?.username || '我')

// 好友名称列表（供 picker 使用）
const friendNames = computed(() =>
  friends.value.map(f => f.nickname || f.username || '未命名好友')
)

// 选中的好友
const selectedFriend = computed(() => {
  if (selectedFriendIndex.value >= 0 && selectedFriendIndex.value < friends.value.length) {
    return friends.value[selectedFriendIndex.value]
  }
  return null
})

// 接收页面参数
onLoad((options: any) => {
  if (options.replyTo) {
    isReply.value = true
    replyToUserId.value = options.replyTo
  }
})

// 加载好友列表
const loadFriends = async () => {
  try {
    const result = await getFriendsApi()
    console.log('好友API返回:', result)
    
    // 处理可能的数据结构
    if (Array.isArray(result)) {
      friends.value = result
    } else if (result && (result as any).data && Array.isArray((result as any).data)) {
      friends.value = (result as any).data
    } else if (result && (result as any).friends && Array.isArray((result as any).friends)) {
      friends.value = (result as any).friends
    } else {
      friends.value = []
    }
    
    console.log('处理后的好友列表:', friends.value)
    console.log('好友名称列表:', friendNames.value)
    
    // 如果是回复，自动选择对应的好友
    if (isReply.value && replyToUserId.value) {
      const targetIndex = friends.value.findIndex(f => f.user_id === replyToUserId.value)
      if (targetIndex !== -1) {
        selectedFriendIndex.value = targetIndex
        uni.showToast({ title: '已自动选择收信人', icon: 'success', duration: 1500 })
      }
    }
    
    if (friends.value.length === 0) {
      console.log('好友列表为空')
    }
  } catch (error) {
    console.error('加载好友列表失败:', error)
    uni.showToast({ title: '加载好友列表失败', icon: 'none' })
  }
}

onMounted(() => {
  loadFriends()
})

onShow(() => {
  loadFriends()
})

const onFriendChange = (e: any) => {
  selectedFriendIndex.value = e.detail.value
}

// 情书模板（随机选择风格）
const loveLetterTemplates = [
  // 温馨浪漫
  (toName: string, fromName: string) => 
    `亲爱的${toName}：\n\n每一次看到你，我的心都会加速跳动。你的笑容就像阳光，温暖了我的整个世界。有你在身边的每一天，都是我最珍贵的回忆。\n\n我想告诉你，你是我生命中最美好的存在。无论何时何地，我都会陪伴在你身边，守护你、爱护你。\n\n永远爱你的${fromName}`,
  
  (toName: string, fromName: string) => 
    `致我最爱的${toName}：\n\n时光荏苒，但我对你的爱从未改变。你是我心中最柔软的部分，是我每天醒来最想见到的人。\n\n愿我们的爱情像星辰般永恒，像花朵般美丽。我会用一生的时间去爱你，去陪伴你。\n\n深爱你的${fromName}`,
  
  // 活泼可爱
  (toName: string, fromName: string) => 
    `Hey ${toName}! 🎈\n\n你知道吗？遇见你是我这辈子最幸运的事！你就像一个小太阳，每天都给我带来满满的正能量！\n\n和你在一起的时候，我感觉整个世界都变得可爱了起来。让我们一起加油鸭！永远开心地在一起！\n\n超级喜欢你的${fromName} 💕`,
  
  (toName: string, fromName: string) => 
    `亲爱的小${toName}：\n\n嘿嘿，今天也想你啦！你是我的开心果，我的小确幸。每次想到你，嘴角都会不自觉地上扬～\n\n希望我们能一直这样甜甜蜜蜜地在一起，做最可爱的一对！mua～\n\n永远喜欢你的${fromName} ❤️`,
  
  // 深情感人
  (toName: string, fromName: string) => 
    `${toName}：\n\n我想了很久，才决定写下这些话。你对我来说，不仅仅是爱人，更是我生命中不可或缺的一部分。\n\n感谢你一直以来的陪伴和理解，感谢你在我最需要的时候给予我力量。我会用我的一生去爱你，去珍惜你。\n\n此生不渝，${fromName}`,
  
  (toName: string, fromName: string) => 
    `我最珍爱的${toName}：\n\n爱你，是我做过最对的决定。在漫长的人生旅途中，能与你相遇、相知、相爱，是我最大的幸福。\n\n我愿意用我的所有，换你一世安稳。无论未来如何，我都会牢牢牵着你的手，不离不弃。\n\n永远的${fromName}`,
  
  // 文艺诗意
  (toName: string, fromName: string) => 
    `致我的${toName}：\n\n你是晨曦的第一缕光，是暮色中最亮的星。在你眼中，我看到了整个宇宙的温柔。\n\n岁月静好，因有你相伴。愿我们的故事，写满浪漫与诗意，直到时光尽头。\n\n你的${fromName}`
]

const generate = () => {
  if (!selectedFriend.value) {
    return uni.showToast({ title: '请选择收信人', icon: 'none' })
  }
  
  // 随机选择一个模板
  const template =
    loveLetterTemplates[Math.floor(Math.random() * loveLetterTemplates.length)]
  const toName = selectedFriend.value.nickname || selectedFriend.value.username || 'Ta'
  result.value = template(toName, currentUserName.value)
  
  uni.showToast({ title: '生成成功', icon: 'success' })
}

const send = async () => {
  if (!selectedFriend.value) {
    return uni.showToast({ title: '请选择收信人', icon: 'none' })
  }

  if (!result.value || !result.value.trim()) {
    return uni.showToast({ title: '请先填写情书内容', icon: 'none' })
  }

  try {
    uni.showLoading({ title: '发送中...', mask: true })
    const toUserId = selectedFriend.value.user_id
    const toName = selectedFriend.value.nickname || selectedFriend.value.username || 'Ta'
    const title = `给 ${toName} 的情书`

    // 1. 创建情书记录，用于首页的情书信封列表
    await createLoveLetterApi({
      title,
      content: result.value,
      to_user_id: toUserId
    })

    // 2. 发送一条好友消息通知（用于即时提醒 & 未读数）
    await sendFriendMessageApi({
      to_user_id: toUserId,
      content: '我给你写了一封情书，快来首页看看吧 💌',
      message_type: 'love_letter'
    })

    uni.showToast({ title: '已寄出给TA', icon: 'success' })
  } catch (error) {
    console.error('发送情书失败:', error)
    uni.showToast({ title: '发送失败，请稍后重试', icon: 'none' })
  } finally {
    uni.hideLoading()
  }
}

const goToAddFriend = () => {
  uni.navigateTo({ url: '/pages/friends/add-friend' })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.page {
  @include cyber-page-bg;
  min-height: 100vh;
  padding: 30rpx;
  position: relative;
  display: flex;
  flex-direction: column;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 30% 20%, rgba(255, 0, 214, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 70% 80%, rgba(250, 112, 154, 0.12) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 120rpx 60rpx;
  @include neon-card;
  border-radius: 30rpx;
  border: 2rpx solid rgba(255, 0, 214, 0.5);
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(255, 0, 214, 0.4),
    inset 0 0 50rpx rgba(255, 0, 214, 0.1);
  position: relative;
  z-index: 1;
  animation: fadeIn 0.6s ease-out;
  
  .empty-icon {
    font-size: 140rpx;
    margin-bottom: 40rpx;
    filter: drop-shadow(0 0 40rpx rgba(255, 0, 214, 0.8));
    animation: emptyFloat 3s ease-in-out infinite;
  }
  
  .empty-text {
    font-size: 36rpx;
    @include neon-title(#FF00D6);
    font-weight: bold;
    margin-bottom: 20rpx;
  }
  
  .empty-hint {
    font-size: 28rpx;
    @include neon-text(#ffffff);
    margin-bottom: 60rpx;
  }
  
  .btn-add-friend {
    width: 320rpx;
    height: 90rpx;
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(250, 112, 154, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(255, 0, 214, 0.7);
    border-radius: 50rpx;
    font-size: 30rpx;
    font-weight: bold;
    box-shadow: 
      0 10rpx 40rpx rgba(255, 0, 214, 0.5),
      0 0 60rpx rgba(255, 0, 214, 0.4),
      inset 0 0 40rpx rgba(255, 0, 214, 0.2);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
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
    }
    
    &::after {
      border: none;
    }
    
    &:active {
      transform: scale(0.95) translateY(2rpx);
      box-shadow: 
        0 6rpx 24rpx rgba(255, 0, 214, 0.7),
        0 0 80rpx rgba(255, 0, 214, 0.6),
        inset 0 0 50rpx rgba(255, 0, 214, 0.3);
    }
  }
}

.form {
  @include neon-card;
  border-radius: 30rpx;
  padding: 45rpx 35rpx;
  border: 2rpx solid rgba(255, 0, 214, 0.5);
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(255, 0, 214, 0.4),
    inset 0 0 50rpx rgba(255, 0, 214, 0.1);
  position: relative;
  z-index: 1;
  animation: fadeIn 0.6s ease-out 0.2s backwards;

  .form-item {
    margin-bottom: 35rpx;

    .label {
      display: block;
      font-size: 30rpx;
      @include neon-text(#FF00D6);
      font-weight: 500;
      margin-bottom: 18rpx;
    }

    .content-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 18rpx;
    }

    .input,
    .picker {
      @include neon-input;
      width: 100%;
      height: 80rpx;
      border-radius: 15rpx;
      padding: 0 25rpx;
      font-size: 28rpx;
      color: #ffffff;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      
      &:focus {
        border-color: rgba(255, 0, 214, 0.8);
        box-shadow: 
          0 0 40rpx rgba(255, 0, 214, 0.5),
          inset 0 0 30rpx rgba(255, 0, 214, 0.1);
        transform: translateY(-2rpx);
      }
    }

    .picker {
      line-height: 80rpx;
      cursor: pointer;
      
      &.placeholder {
        color: #6b7b93;
      }
      
      // 注意：picker 弹出的选择器是系统原生组件，样式由系统控制
      // 如需完全自定义样式，可考虑使用自定义弹窗组件替代
    }
    
    .sender-info {
      @include neon-input;
      width: 100%;
      height: 80rpx;
      border-radius: 15rpx;
      padding: 0 25rpx;
      font-size: 28rpx;
      line-height: 80rpx;
      color: #b8c5d6;
    }

    .btn-generate-inline {
      height: 65rpx;
      padding: 0 35rpx;
      background: linear-gradient(135deg, rgba(138, 92, 246, 0.9) 0%, rgba(240, 147, 251, 0.9) 100%);
      @include neon-text(#ffffff);
      border: 2rpx solid rgba(138, 92, 246, 0.7);
      border-radius: 33rpx;
      font-size: 26rpx;
      font-weight: bold;
      box-shadow: 
        0 6rpx 20rpx rgba(138, 92, 246, 0.4),
        0 0 30rpx rgba(138, 92, 246, 0.3),
        inset 0 0 20rpx rgba(138, 92, 246, 0.15);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
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
          rgba(255, 255, 255, 0.25) 90deg,
          transparent 180deg
        );
        animation: rotate 3s linear infinite;
      }
      
      &::after {
        border: none;
      }
      
      &:active {
        transform: scale(0.93) translateY(2rpx);
        box-shadow: 
          0 4rpx 12rpx rgba(138, 92, 246, 0.6),
          0 0 40rpx rgba(138, 92, 246, 0.5),
          inset 0 0 30rpx rgba(138, 92, 246, 0.25);
      }
    }

    // 情书内容深色霓虹样式
    .result-text {
      width: 100%;
      min-height: 420rpx;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.8) 0%, rgba(30, 36, 66, 0.6) 100%);
      backdrop-filter: blur(20rpx);
      border-radius: 20rpx;
      padding: 35rpx;
      font-size: 30rpx;
      line-height: 2;
      border: 3rpx solid rgba(250, 112, 154, 0.6);
      box-shadow: 
        0 12rpx 30rpx rgba(0, 0, 0, 0.4),
        0 0 40rpx rgba(250, 112, 154, 0.4),
        inset 0 0 30rpx rgba(250, 112, 154, 0.1);
      box-sizing: border-box;
      color: #ffffff;
      position: relative;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      
      &::placeholder {
        color: #6b7b93;
      }
      
      &::before {
        content: '';
        position: absolute;
        inset: -3rpx;
        border-radius: 20rpx;
        padding: 3rpx;
        background: linear-gradient(135deg, 
          rgba(250, 112, 154, 0.8) 0%,
          rgba(255, 0, 214, 0.8) 50%,
          rgba(250, 112, 154, 0.8) 100%
        );
        -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
        -webkit-mask-composite: xor;
        mask-composite: exclude;
        animation: borderPulse 3s ease-in-out infinite;
        z-index: -1;
      }
      
      &:focus {
        border-color: rgba(255, 0, 214, 0.9);
        box-shadow: 
          0 15rpx 35rpx rgba(250, 112, 154, 0.5),
          0 0 60rpx rgba(255, 0, 214, 0.5),
          inset 0 0 40rpx rgba(255, 0, 214, 0.15);
        transform: translateY(-2rpx);
      }
    }
  }

  .result {
    margin-top: 50rpx;
    
    .btn-send {
      width: 100%;
      height: 90rpx;
      background: linear-gradient(135deg, rgba(250, 112, 154, 0.95) 0%, rgba(255, 0, 214, 0.95) 100%);
      @include neon-text(#ffffff);
      border: 3rpx solid rgba(250, 112, 154, 0.8);
      border-radius: 50rpx;
      font-size: 32rpx;
      font-weight: bold;
      box-shadow: 
        0 15rpx 50rpx rgba(250, 112, 154, 0.6),
        0 0 70rpx rgba(250, 112, 154, 0.5),
        inset 0 0 50rpx rgba(250, 112, 154, 0.2);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      position: relative;
      overflow: hidden;
      animation: sendBtnPulse 3s ease-in-out infinite;
      
      &::before {
        content: '';
        position: absolute;
        inset: -3rpx;
        border-radius: 50rpx;
        padding: 3rpx;
        background: linear-gradient(45deg, 
          rgba(250, 112, 154, 0.8) 0%,
          rgba(255, 0, 214, 0.8) 25%,
          rgba(240, 147, 251, 0.8) 50%,
          rgba(255, 0, 214, 0.8) 75%,
          rgba(250, 112, 154, 0.8) 100%
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
        border: none;
      }
      
      &:active {
        transform: scale(0.96) translateY(2rpx);
        animation: none;
        box-shadow: 
          0 10rpx 35rpx rgba(250, 112, 154, 0.8),
          0 0 90rpx rgba(250, 112, 154, 0.7),
          inset 0 0 60rpx rgba(250, 112, 154, 0.3);
      }
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes emptyFloat {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-20rpx) rotate(5deg);
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes borderRotate {
  from {
    filter: hue-rotate(0deg);
  }
  to {
    filter: hue-rotate(360deg);
  }
}

@keyframes borderPulse {
  0%, 100% {
    opacity: 0.6;
  }
  50% {
    opacity: 1;
  }
}

@keyframes sendBtnPulse {
  0%, 100% {
    box-shadow: 
      0 15rpx 50rpx rgba(250, 112, 154, 0.6),
      0 0 70rpx rgba(250, 112, 154, 0.5),
      inset 0 0 50rpx rgba(250, 112, 154, 0.2);
  }
  50% {
    box-shadow: 
      0 20rpx 60rpx rgba(250, 112, 154, 0.8),
      0 0 90rpx rgba(250, 112, 154, 0.7),
      inset 0 0 60rpx rgba(250, 112, 154, 0.3);
  }
}
</style>

