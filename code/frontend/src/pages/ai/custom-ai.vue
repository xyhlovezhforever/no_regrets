<template>
  <view class="custom-ai-page">
    <view class="header">
      <text class="title">创建专属AI好友</text>
      <text class="subtitle">定制一个只属于你的AI陪伴</text>
    </view>

    <scroll-view class="form-scroll" scroll-y>
      <!-- 参考好友 -->
      <view class="form-section">
        <view class="section-title">参考好友（可选）</view>
        <view class="form-item">
          <text class="label">选择一个好友作为参考</text>
          <view class="friend-selector">
            <view v-if="loadingFriends" class="loading-friends">
              <text>加载中...</text>
            </view>
            <view
              v-for="friend in friends"
              :key="friend.id"
              class="friend-option"
              :class="{ active: selectedFriendId === friend.id }"
              @click="selectFriend(friend)"
            >
              <text class="friend-avatar">{{ friend.avatar || '👤' }}</text>
              <text class="friend-name">{{ friend.name }}</text>
            </view>
            <view v-if="!loadingFriends && friends.length === 0" class="no-friends">
              <text>还没有好友，可以跳过此步骤</text>
            </view>
          </view>
        </view>
      </view>

      <!-- 基本信息 -->
      <view class="form-section">
        <view class="section-title">基本信息</view>
        <view class="form-item">
          <text class="label">AI名称</text>
          <input v-model="form.name" class="input" placeholder="给你的AI起个名字" maxlength="20" />
        </view>
        <view class="form-item">
          <text class="label">头像</text>
          <view class="avatar-selector">
            <view
              v-for="avatar in avatars"
              :key="avatar"
              class="avatar-option"
              :class="{ active: form.avatar === avatar }"
              @click="form.avatar = avatar"
            >
              <text class="avatar-emoji">{{ avatar }}</text>
            </view>
          </view>
        </view>
      </view>

      <!-- 性格设定 -->
      <view class="form-section">
        <view class="section-title">性格设定</view>
        <view class="form-item">
          <text class="label">性格类型</text>
          <view class="tag-list">
            <view
              v-for="personality in personalities"
              :key="personality"
              class="tag"
              :class="{ active: form.personality.includes(personality) }"
              @click="togglePersonality(personality)"
            >
              {{ personality }}
            </view>
          </view>
        </view>
        <view class="form-item">
          <text class="label">说话风格</text>
          <picker mode="selector" :range="styles" :value="styleIndex" @change="onStyleChange">
            <view class="picker-value">{{ styles[styleIndex] }}</view>
          </picker>
        </view>
      </view>

      <!-- 详细设定 -->
      <view class="form-section">
        <view class="section-title">详细设定</view>
        <view class="form-item">
          <text class="label">角色背景</text>
          <textarea
            v-model="form.background"
            class="textarea"
            placeholder="描述你的AI好友的背景故事..."
            maxlength="200"
          />
          <text class="char-count">{{ form.background.length }}/200</text>
        </view>
        <view class="form-item">
          <text class="label">专属称呼</text>
          <input v-model="form.nickname" class="input" placeholder="AI如何称呼你" maxlength="10" />
        </view>
        <view class="form-item">
          <text class="label">常用口头禅</text>
          <input v-model="form.catchphrase" class="input" placeholder="例如：加油哦～" maxlength="20" />
        </view>
      </view>

      <!-- 保存按钮 -->
      <view class="action-buttons">
        <button class="btn btn-primary" @click="saveCustomAI">保存并创建</button>
        <button class="btn btn-secondary" @click="previewAI">预览效果</button>
      </view>
    </scroll-view>

    <!-- 预览弹窗 -->
    <view v-if="showPreview" class="preview-modal" @click="showPreview = false">
      <view class="preview-content" @click.stop>
        <view class="preview-header">
          <text class="preview-title">AI预览</text>
          <text class="preview-close" @click="showPreview = false">✕</text>
        </view>
        <view class="preview-ai">
          <text class="preview-avatar">{{ form.avatar || '🤖' }}</text>
          <text class="preview-name">{{ form.name || '未命名' }}</text>
          <text class="preview-desc">{{ getPreviewDesc() }}</text>
        </view>
        <view class="preview-chat">
          <view class="preview-message">
            <text class="message-avatar">{{ form.avatar || '🤖' }}</text>
            <view class="message-bubble">
              <text>{{ getPreviewMessage() }}</text>
            </view>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { getStorage, setStorage } from '@/utils/storage'
import { generateId } from '@/utils'
import { getFriendsApi } from '@/api/friend'

interface CustomAI {
  id: string
  name: string
  avatar: string
  personality: string[]
  style: string
  background: string
  nickname: string
  catchphrase: string
  createdAt: number
}

interface Friend {
  id: string
  name: string
  avatar: string
}

const form = ref<CustomAI>({
  id: '',
  name: '',
  avatar: '🤖',
  personality: [],
  style: '温柔',
  background: '',
  nickname: '',
  catchphrase: '',
  createdAt: 0
})

const friends = ref<Friend[]>([])
const selectedFriendId = ref('')
const loadingFriends = ref(false)

onMounted(() => {
  loadFriends()
})

const loadFriends = async () => {
  try {
    loadingFriends.value = true
    const data = await getFriendsApi()
    
    // 转换后端数据格式为页面需要的格式
    friends.value = (data as any[]).map((f: any) => ({
      id: f.user_id || f.id,
      name: f.name || f.username || '未命名',
      avatar: f.avatar || '👤'
    }))
    
    console.log('已加载好友列表:', friends.value.length, '个好友')
  } catch (error) {
    console.error('加载好友列表失败:', error)
    friends.value = []
    uni.showToast({ 
      title: '加载好友列表失败', 
      icon: 'none',
      duration: 2000
    })
  } finally {
    loadingFriends.value = false
  }
}

const selectFriend = (friend: Friend) => {
  if (selectedFriendId.value === friend.id) {
    // 取消选择
    selectedFriendId.value = ''
    return
  }
  
  selectedFriendId.value = friend.id
  
  // 自动填充信息
  if (!form.value.name) {
    form.value.name = friend.name + '的AI'
  }
  if (friend.avatar) {
    form.value.avatar = friend.avatar
  }
  
  uni.showToast({ title: '已参考好友信息', icon: 'success' })
}

const avatars = ['🤖', '👨', '👩', '🧑', '👦', '👧', '🧒', '👴', '👵', '🧑‍💼', '🧑‍🎓', '🧑‍🎨']
const personalities = ['温柔', '活泼', '成熟', '幽默', '认真', '浪漫', '理性', '感性', '乐观', '沉稳']
const styles = ['温柔', '活泼', '成熟', '幽默', '正式', '随意', '文艺', '可爱']

const styleIndex = computed(() => styles.indexOf(form.value.style))

const showPreview = ref(false)

const togglePersonality = (personality: string) => {
  const index = form.value.personality.indexOf(personality)
  if (index >= 0) {
    form.value.personality.splice(index, 1)
  } else {
    if (form.value.personality.length < 3) {
      form.value.personality.push(personality)
    } else {
      uni.showToast({ title: '最多选择3个性格', icon: 'none' })
    }
  }
}

const onStyleChange = (e: any) => {
  form.value.style = styles[e.detail.value]
}

const getPreviewDesc = () => {
  if (form.value.personality.length === 0) return '性格待设定'
  return form.value.personality.join('、') + '的' + form.value.style + '风格'
}

const getPreviewMessage = () => {
  const greeting = form.value.nickname ? `你好，${form.value.nickname}！` : '你好！'
  const catchphrase = form.value.catchphrase ? ` ${form.value.catchphrase}` : ''
  return greeting + catchphrase + ' 我是' + (form.value.name || '你的AI好友') + '，很高兴认识你！'
}

const previewAI = () => {
  if (!form.value.name) {
    return uni.showToast({ title: '请先输入AI名称', icon: 'none' })
  }
  showPreview.value = true
}

const saveCustomAI = () => {
  if (!form.value.name.trim()) {
    return uni.showToast({ title: '请输入AI名称', icon: 'none' })
  }

  if (form.value.personality.length === 0) {
    return uni.showToast({ title: '请至少选择一个性格', icon: 'none' })
  }

  form.value.id = generateId()
  form.value.createdAt = Date.now()

  const customAIs = getStorage<CustomAI[]>('customAIs', [])
  customAIs.push({ ...form.value })
  setStorage('customAIs', customAIs)

  uni.showToast({ title: '创建成功', icon: 'success' })

  setTimeout(() => {
    uni.navigateBack()
  }, 1500)
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.custom-ai-page {
  min-height: 100vh;
  @include cyber-page-bg;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 20% 20%, rgba(255, 0, 214, 0.1) 0%, transparent 50%),
      radial-gradient(circle at 80% 80%, rgba(0, 217, 255, 0.1) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
  }
}

.header {
  padding: 50rpx 30rpx 40rpx;
  background: linear-gradient(180deg, rgba(30, 36, 66, 0.95) 0%, rgba(30, 36, 66, 0.85) 100%);
  backdrop-filter: blur(30rpx);
  border-bottom: 2rpx solid rgba(255, 0, 214, 0.4);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(255, 0, 214, 0.3),
    inset 0 -2rpx 0 rgba(255, 0, 214, 0.5);
  position: relative;
  z-index: 2;

  .title {
    display: block;
    font-size: 48rpx;
    font-weight: bold;
    @include neon-title(#FF00D6);
    margin-bottom: 15rpx;
    animation: titlePulse 3s ease-in-out infinite;
  }

  .subtitle {
    display: block;
    font-size: 28rpx;
    @include neon-text(#00D9FF);
    animation: subtitleBlink 2s ease-in-out infinite;
  }
}

.form-scroll {
  height: calc(100vh - 200rpx);
  padding: 30rpx;
  position: relative;
  z-index: 1;
}

.form-section {
  @include neon-card;
  border-radius: 25rpx;
  padding: 35rpx;
  margin-bottom: 30rpx;
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(138, 92, 246, 0.3),
    inset 0 0 40rpx rgba(138, 92, 246, 0.1);
  border: 2rpx solid rgba(138, 92, 246, 0.5);
  backdrop-filter: blur(20rpx);
  animation: sectionAppear 0.6s ease-out backwards;
  
  &:nth-child(1) { animation-delay: 0.1s; }
  &:nth-child(2) { animation-delay: 0.2s; }
  &:nth-child(3) { animation-delay: 0.3s; }
  &:nth-child(4) { animation-delay: 0.4s; }

  .section-title {
    display: block;
    font-size: 36rpx;
    font-weight: bold;
    @include neon-title(#8B5CF6);
    margin-bottom: 30rpx;
    padding-bottom: 20rpx;
    border-bottom: 2rpx solid rgba(138, 92, 246, 0.4);
    position: relative;
    
    &::after {
      content: '';
      position: absolute;
      bottom: -2rpx;
      left: 0;
      width: 80rpx;
      height: 2rpx;
      background: linear-gradient(90deg, rgba(138, 92, 246, 1) 0%, transparent 100%);
      box-shadow: 0 0 10rpx rgba(138, 92, 246, 0.8);
    }
  }
}

.form-item {
  margin-bottom: 30rpx;

  &:last-child {
    margin-bottom: 0;
  }

  .label {
    display: block;
    font-size: 28rpx;
    @include neon-text(#00D9FF);
    margin-bottom: 15rpx;
    font-weight: 500;
  }

  .input {
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

  .textarea {
    @include neon-input;
    width: 100%;
    min-height: 200rpx;
    border-radius: 15rpx;
    padding: 20rpx 25rpx;
    font-size: 28rpx;
    color: #ffffff;
    line-height: 1.8;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    
    &:focus {
      border-color: rgba(255, 0, 214, 0.8);
      box-shadow: 
        0 0 40rpx rgba(255, 0, 214, 0.5),
        inset 0 0 30rpx rgba(255, 0, 214, 0.1);
    }
  }

  .char-count {
    display: block;
    text-align: right;
    font-size: 22rpx;
    @include neon-text(#6b7b93);
    margin-top: 10rpx;
  }
}

.avatar-selector {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 15rpx;

  .avatar-option {
    aspect-ratio: 1;
    border-radius: 15rpx;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
    backdrop-filter: blur(10rpx);
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2rpx solid rgba(138, 92, 246, 0.3);
    box-shadow: 
      0 4rpx 16rpx rgba(0, 0, 0, 0.3),
      0 0 20rpx rgba(138, 92, 246, 0.2);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;

    &::before {
      content: '';
      position: absolute;
      inset: 0;
      background: conic-gradient(
        from 0deg,
        transparent 0deg,
        rgba(138, 92, 246, 0.3) 90deg,
        transparent 180deg
      );
      opacity: 0;
      transition: opacity 0.3s;
      animation: rotate 3s linear infinite;
    }

    &.active {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.6) 0%, rgba(138, 92, 246, 0.6) 100%);
      border-color: rgba(255, 0, 214, 0.8);
      box-shadow: 
        0 8rpx 24rpx rgba(255, 0, 214, 0.5),
        0 0 40rpx rgba(255, 0, 214, 0.6),
        inset 0 0 30rpx rgba(255, 0, 214, 0.2);
      transform: scale(1.15);
      
      &::before {
        opacity: 1;
      }
    }

    .avatar-emoji {
      font-size: 40rpx;
      position: relative;
      z-index: 1;
      filter: drop-shadow(0 0 10rpx rgba(255, 255, 255, 0.5));
    }
  }
}

  .friend-selector {
    display: flex;
    flex-direction: column;
    gap: 15rpx;
    max-height: 300rpx;
    overflow-y: auto;

    .friend-option {
      display: flex;
      align-items: center;
      padding: 25rpx;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
      backdrop-filter: blur(10rpx);
      border-radius: 15rpx;
      border: 2rpx solid rgba(0, 217, 255, 0.3);
      box-shadow: 
        0 4rpx 16rpx rgba(0, 0, 0, 0.3),
        0 0 20rpx rgba(0, 217, 255, 0.2);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

      &.active {
        background: linear-gradient(135deg, rgba(0, 217, 255, 0.6) 0%, rgba(138, 92, 246, 0.6) 100%);
        border-color: rgba(0, 217, 255, 0.8);
        box-shadow: 
          0 8rpx 24rpx rgba(0, 217, 255, 0.5),
          0 0 40rpx rgba(0, 217, 255, 0.6),
          inset 0 0 30rpx rgba(0, 217, 255, 0.2);
        transform: translateX(10rpx);

        .friend-name {
          @include neon-text(#ffffff);
        }
      }

      .friend-avatar {
        font-size: 40rpx;
        margin-right: 15rpx;
        filter: drop-shadow(0 0 10rpx rgba(255, 255, 255, 0.5));
      }

      .friend-name {
        font-size: 28rpx;
        color: #b8c5d6;
        font-weight: 500;
      }
    }

    .loading-friends {
      text-align: center;
      padding: 40rpx 0;
      @include neon-text(#00D9FF);
      font-size: 26rpx;
      animation: pulse 1.5s ease-in-out infinite;
    }

    .no-friends {
      text-align: center;
      padding: 40rpx 0;
      @include neon-text(#6b7b93);
      font-size: 26rpx;
    }
  }

  .tag-list {
    display: flex;
    flex-wrap: wrap;
    gap: 15rpx;

    .tag {
      padding: 18rpx 35rpx;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
      backdrop-filter: blur(10rpx);
      border-radius: 50rpx;
      font-size: 26rpx;
      color: #b8c5d6;
      border: 2rpx solid rgba(138, 92, 246, 0.3);
      box-shadow: 0 0 20rpx rgba(138, 92, 246, 0.2);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      font-weight: 500;

      &.active {
        background: linear-gradient(135deg, rgba(138, 92, 246, 0.8) 0%, rgba(240, 147, 251, 0.8) 100%);
        @include neon-text(#ffffff);
        border-color: rgba(138, 92, 246, 0.8);
        box-shadow: 
          0 4rpx 16rpx rgba(138, 92, 246, 0.5),
          0 0 30rpx rgba(138, 92, 246, 0.6),
          inset 0 0 20rpx rgba(138, 92, 246, 0.2);
        transform: scale(1.05);
      }
    }
  }

.picker-value {
  @include neon-input;
  width: 100%;
  height: 80rpx;
  border-radius: 15rpx;
  padding: 0 25rpx;
  display: flex;
  align-items: center;
  font-size: 28rpx;
  color: #ffffff;
  position: relative;
  
  &::after {
    content: '▼';
    position: absolute;
    right: 25rpx;
    font-size: 20rpx;
    @include neon-text(#00D9FF);
  }
}

.action-buttons {
  display: flex;
  gap: 20rpx;
  padding: 30rpx 0 50rpx;

  .btn {
    flex: 1;
    height: 90rpx;
    border-radius: 50rpx;
    border: none;
    font-size: 32rpx;
    font-weight: bold;
    line-height: 90rpx;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
    
    &::after {
      border: none;
    }

    &.btn-primary {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      border: 2rpx solid rgba(255, 0, 214, 0.7);
      @include neon-text(#ffffff);
      box-shadow: 
        0 10rpx 40rpx rgba(255, 0, 214, 0.5),
        0 0 60rpx rgba(255, 0, 214, 0.4),
        inset 0 0 40rpx rgba(255, 0, 214, 0.2);
      animation: primaryBtnPulse 3s ease-in-out infinite;
      
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
      
      &:active {
        transform: scale(0.96) translateY(2rpx);
        animation: none;
        box-shadow: 
          0 6rpx 24rpx rgba(255, 0, 214, 0.7),
          0 0 80rpx rgba(255, 0, 214, 0.6),
          inset 0 0 60rpx rgba(255, 0, 214, 0.3);
      }
    }

    &.btn-secondary {
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.7) 0%, rgba(30, 36, 66, 0.5) 100%);
      backdrop-filter: blur(20rpx);
      border: 2rpx solid rgba(0, 217, 255, 0.5);
      @include neon-text(#00D9FF);
      box-shadow: 
        0 4rpx 20rpx rgba(0, 0, 0, 0.3),
        0 0 30rpx rgba(0, 217, 255, 0.3);
      
      &:active {
        transform: scale(0.96) translateY(2rpx);
        border-color: rgba(0, 217, 255, 0.8);
        box-shadow: 
          0 4rpx 20rpx rgba(0, 217, 255, 0.5),
          0 0 40rpx rgba(0, 217, 255, 0.5);
      }
    }
  }
}

.preview-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(10rpx);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
  animation: modalFadeIn 0.3s ease-out;
}

.preview-content {
  width: 620rpx;
  @include neon-card;
  border-radius: 30rpx;
  padding: 45rpx;
  border: 2rpx solid rgba(255, 0, 214, 0.6);
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.6),
    0 0 80rpx rgba(255, 0, 214, 0.5),
    inset 0 0 60rpx rgba(255, 0, 214, 0.1);
  animation: modalSlideIn 0.4s cubic-bezier(0.4, 0, 0.2, 1);

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 35rpx;
    padding-bottom: 20rpx;
    border-bottom: 2rpx solid rgba(255, 0, 214, 0.3);

    .preview-title {
      font-size: 40rpx;
      font-weight: bold;
      @include neon-title(#FF00D6);
      animation: titlePulse 3s ease-in-out infinite;
    }

    .preview-close {
      font-size: 44rpx;
      @include neon-text(#6b7b93);
      padding: 10rpx;
      transition: all 0.3s ease;
      
      &:active {
        @include neon-text(#00D9FF);
        transform: rotate(90deg) scale(1.2);
      }
    }
  }

  .preview-ai {
    text-align: center;
    padding: 35rpx 0;
    border-bottom: 2rpx solid rgba(138, 92, 246, 0.3);
    margin-bottom: 35rpx;
    position: relative;

    .preview-avatar {
      display: block;
      font-size: 120rpx;
      margin-bottom: 25rpx;
      filter: drop-shadow(0 0 30rpx rgba(255, 0, 214, 0.8));
      animation: avatarBounce 2s ease-in-out infinite;
    }

    .preview-name {
      display: block;
      font-size: 40rpx;
      font-weight: bold;
      @include neon-title(#8B5CF6);
      margin-bottom: 15rpx;
    }

    .preview-desc {
      display: block;
      font-size: 26rpx;
      @include neon-text(#00D9FF);
    }
  }

  .preview-chat {
    .preview-message {
      display: flex;
      align-items: flex-start;
      gap: 20rpx;

      .message-avatar {
        font-size: 60rpx;
        flex-shrink: 0;
        filter: drop-shadow(0 0 15rpx rgba(138, 92, 246, 0.8));
      }

      .message-bubble {
        flex: 1;
        background: linear-gradient(135deg, rgba(138, 92, 246, 0.3) 0%, rgba(240, 147, 251, 0.3) 100%);
        backdrop-filter: blur(10rpx);
        border: 2rpx solid rgba(138, 92, 246, 0.5);
        border-radius: 20rpx;
        padding: 25rpx;
        font-size: 28rpx;
        color: #ffffff;
        line-height: 1.8;
        box-shadow: 
          0 4rpx 16rpx rgba(0, 0, 0, 0.3),
          0 0 30rpx rgba(138, 92, 246, 0.3),
          inset 0 0 20rpx rgba(138, 92, 246, 0.1);
        text-shadow: 0 0 10rpx rgba(255, 255, 255, 0.3);
        animation: bubbleAppear 0.5s ease-out 0.3s backwards;
      }
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.5; }
  50% { opacity: 0.8; }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 15rpx currentColor,
      0 0 30rpx currentColor;
  }
  50% {
    text-shadow: 
      0 0 25rpx currentColor,
      0 0 50rpx currentColor;
  }
}

@keyframes subtitleBlink {
  0%, 100% { opacity: 0.7; }
  50% { opacity: 1; }
}

@keyframes sectionAppear {
  from {
    opacity: 0;
    transform: translateY(30rpx);
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

@keyframes primaryBtnPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(255, 0, 214, 0.5),
      0 0 60rpx rgba(255, 0, 214, 0.4),
      inset 0 0 40rpx rgba(255, 0, 214, 0.2);
  }
  50% {
    box-shadow: 
      0 15rpx 50rpx rgba(255, 0, 214, 0.7),
      0 0 80rpx rgba(255, 0, 214, 0.6),
      inset 0 0 50rpx rgba(255, 0, 214, 0.3);
  }
}

@keyframes modalFadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes modalSlideIn {
  from {
    opacity: 0;
    transform: translateY(50rpx) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

@keyframes avatarBounce {
  0%, 100% {
    transform: translateY(0) scale(1);
  }
  50% {
    transform: translateY(-15rpx) scale(1.05);
  }
}

@keyframes bubbleAppear {
  from {
    opacity: 0;
    transform: translateX(-20rpx);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 0.6;
  }
  50% {
    opacity: 1;
  }
}
</style>

