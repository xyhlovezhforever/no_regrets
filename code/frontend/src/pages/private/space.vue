<template>
  <view class="private-space">
    <!-- 背景光芒效果 -->
    <view class="light-effects">
      <view
        v-for="(light, index) in lights"
        :key="index"
        class="light"
        :style="{
          left: light.x + '%',
          top: light.y + '%',
          width: light.size + 'rpx',
          height: light.size + 'rpx',
          background: light.color,
          animationDuration: light.duration + 's',
          animationDelay: light.delay + 's'
        }"
      ></view>
    </view>

    <!-- 内容区域 -->
    <view class="content-area">
      <!-- 标题 -->
      <view class="title-section">
        <text class="title">我的私人空间</text>
        <text class="subtitle">只属于你的小天地</text>
      </view>

      <!-- 音乐播放器 -->
      <view class="music-player">
        <view class="player-header">
          <text class="player-title">🎵 我的音乐</text>
          <view class="add-music-btn" @click="showMusicModal = true">
            <text>+ 添加</text>
          </view>
        </view>

        <view v-if="musicList.length > 0" class="music-list">
          <view
            v-for="(music, index) in musicList"
            :key="index"
            class="music-item"
            :class="{ active: currentMusicIndex === index }"
            @click="playMusic(index)"
          >
            <view class="music-info">
              <text class="music-name">{{ music.name }}</text>
              <text class="music-artist">{{ music.artist || '未知艺术家' }}</text>
            </view>
            <view class="music-controls">
              <text v-if="currentMusicIndex === index && isPlaying" class="control-icon">⏸</text>
              <text v-else class="control-icon">▶</text>
            </view>
          </view>
        </view>

        <view v-else class="empty-music">
          <text class="empty-icon">🎵</text>
          <text class="empty-text">还没有音乐</text>
          <text class="empty-hint">点击"添加"添加你喜欢的音乐</text>
        </view>
      </view>

      <!-- 光芒设置 -->
      <view class="light-settings">
        <view class="settings-header">
          <text class="settings-title">✨ 光芒设置</text>
        </view>
        <view class="color-picker">
          <text class="picker-label">选择光芒颜色</text>
          <view class="color-list">
            <view
              v-for="color in colors"
              :key="color.value"
              class="color-item"
              :class="{ active: selectedColor === color.value }"
              :style="{ background: color.value }"
              @click="selectedColor = color.value; updateLights()"
            >
              <text v-if="selectedColor === color.value" class="check-icon">✓</text>
            </view>
          </view>
        </view>
        <view class="intensity-control">
          <text class="control-label">光芒强度</text>
          <slider
            :value="lightIntensity"
            min="1"
            max="10"
            step="1"
            activeColor="#667eea"
            @change="onIntensityChange"
          />
          <text class="intensity-value">{{ lightIntensity }}</text>
        </view>
      </view>

      <!-- 养宠物入口 -->
      <view class="feature-section">
        <view class="feature-card" @click="goToPet">
          <text class="feature-icon">🐱</text>
          <view class="feature-info">
            <text class="feature-title">我的宠物</text>
            <text class="feature-desc">养一只专属宠物陪伴你</text>
          </view>
          <text class="feature-arrow">›</text>
        </view>
      </view>

      <!-- 分享按钮 -->
      <view class="share-section">
        <button class="share-btn" @click="shareSpace">
          <text>📤 分享我的空间</text>
        </button>
      </view>
    </view>

    <!-- 添加音乐弹窗 -->
    <view v-if="showMusicModal" class="modal-mask" @click="showMusicModal = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">添加音乐</text>
          <text class="modal-close" @click="showMusicModal = false">✕</text>
        </view>
        <view class="modal-form">
          <view class="form-item">
            <text class="form-label">歌曲名称</text>
            <input v-model="musicForm.name" class="form-input" placeholder="输入歌曲名称" maxlength="50" />
          </view>
          <view class="form-item">
            <text class="form-label">艺术家</text>
            <input v-model="musicForm.artist" class="form-input" placeholder="输入艺术家名称" maxlength="50" />
          </view>
          <view class="form-item">
            <text class="form-label">音乐链接（可选）</text>
            <input v-model="musicForm.url" class="form-input" placeholder="输入音乐链接" />
          </view>
        </view>
        <view class="modal-actions">
          <button class="modal-btn" @click="saveMusic">保存</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getStorage, setStorage } from '@/utils/storage'
import { navigateTo } from '@/utils'
import { shareToFriend } from '@/utils'

interface Music {
  name: string
  artist: string
  url?: string
}

interface Light {
  x: number
  y: number
  size: number
  color: string
  duration: number
  delay: number
}

const musicList = ref<Music[]>([])
const currentMusicIndex = ref(-1)
const isPlaying = ref(false)
const showMusicModal = ref(false)
const musicForm = ref<Music>({ name: '', artist: '', url: '' })

const selectedColor = ref('#667eea')
const lightIntensity = ref(5)
const lights = ref<Light[]>([])

const colors = [
  { name: '紫色', value: '#667eea' },
  { name: '蓝色', value: '#4facfe' },
  { name: '绿色', value: '#43e97b' },
  { name: '粉色', value: '#f093fb' },
  { name: '橙色', value: '#fa709a' },
  { name: '黄色', value: '#fee140' },
  { name: '青色', value: '#30cfd0' },
  { name: '红色', value: '#ff4d4f' }
]

let audioContext: any = null

onMounted(() => {
  loadMusicList()
  loadSettings()
  generateLights()
})

onUnmounted(() => {
  if (audioContext) {
    audioContext.destroy()
  }
})

const loadMusicList = () => {
  musicList.value = getStorage<Music[]>('privateSpaceMusic', [])
}

const loadSettings = () => {
  const settings = getStorage<any>('privateSpaceSettings', {})
  selectedColor.value = settings.color || '#667eea'
  lightIntensity.value = settings.intensity || 5
}

const saveSettings = () => {
  setStorage('privateSpaceSettings', {
    color: selectedColor.value,
    intensity: lightIntensity.value
  })
}

const generateLights = () => {
  lights.value = []
  const count = 8 + lightIntensity.value * 2
  
  for (let i = 0; i < count; i++) {
    lights.value.push({
      x: Math.random() * 100,
      y: Math.random() * 100,
      size: 100 + Math.random() * 200,
      color: selectedColor.value,
      duration: 3 + Math.random() * 4,
      delay: Math.random() * 2
    })
  }
}

const updateLights = () => {
  generateLights()
  saveSettings()
}

const onIntensityChange = (e: any) => {
  lightIntensity.value = e.detail.value
  updateLights()
}

const playMusic = (index: number) => {
  if (currentMusicIndex.value === index && isPlaying.value) {
    // 暂停
    isPlaying.value = false
    if (audioContext) {
      audioContext.pause()
    }
  } else {
    // 播放
    currentMusicIndex.value = index
    isPlaying.value = true
    
    const music = musicList.value[index]
    if (music.url) {
      // 如果有链接，尝试播放
      audioContext = uni.createInnerAudioContext()
      audioContext.src = music.url
      audioContext.play()
      audioContext.onEnded(() => {
        isPlaying.value = false
      })
    } else {
      // 没有链接，模拟播放
      uni.showToast({ title: `正在播放：${music.name}`, icon: 'none' })
      setTimeout(() => {
        isPlaying.value = false
      }, 3000)
    }
  }
}

const saveMusic = () => {
  if (!musicForm.value.name.trim()) {
    return uni.showToast({ title: '请输入歌曲名称', icon: 'none' })
  }

  musicList.value.push({
    name: musicForm.value.name.trim(),
    artist: musicForm.value.artist.trim(),
    url: musicForm.value.url.trim()
  })

  setStorage('privateSpaceMusic', musicList.value)
  showMusicModal.value = false
  musicForm.value = { name: '', artist: '', url: '' }
  uni.showToast({ title: '添加成功', icon: 'success' })
}

const goToPet = () => {
  navigateTo('/pages/private/pet')
}

const shareSpace = () => {
  const musicText = musicList.value.length > 0
    ? `我的音乐：\n${musicList.value.map(m => `🎵 ${m.name} - ${m.artist || '未知'}`).join('\n')}`
    : '我的私人空间'
  
  shareToFriend(musicText, '分享我的私人空间')
}
</script>

<style lang="scss" scoped>
.private-space {
  min-height: 100vh;
  background: #000000;
  position: relative;
  overflow: hidden;
}

.light-effects {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  z-index: 1;

  .light {
    position: absolute;
    border-radius: 50%;
    opacity: 0.3;
    filter: blur(40rpx);
    animation: pulse infinite ease-in-out;
  }
}

@keyframes pulse {
  0%, 100% {
    transform: scale(1);
    opacity: 0.2;
  }
  50% {
    transform: scale(1.2);
    opacity: 0.4;
  }
}

.content-area {
  position: relative;
  z-index: 2;
  padding: 60rpx 30rpx 100rpx;
  color: #ffffff;
}

.title-section {
  text-align: center;
  margin-bottom: 60rpx;

  .title {
    display: block;
    font-size: 56rpx;
    font-weight: bold;
    margin-bottom: 15rpx;
    text-shadow: 0 0 20rpx rgba(255, 255, 255, 0.5);
  }

  .subtitle {
    display: block;
    font-size: 28rpx;
    color: rgba(255, 255, 255, 0.8);
  }
}

.music-player,
.light-settings {
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20rpx);
  border-radius: 30rpx;
  padding: 40rpx;
  margin-bottom: 40rpx;
  border: 1rpx solid rgba(255, 255, 255, 0.2);
}

.player-header,
.settings-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30rpx;

  .player-title,
  .settings-title {
    font-size: 32rpx;
    font-weight: bold;
  }

  .add-music-btn {
    background: rgba(255, 255, 255, 0.2);
    padding: 12rpx 25rpx;
    border-radius: 50rpx;
    font-size: 24rpx;
  }
}

.music-list {
  display: flex;
  flex-direction: column;
  gap: 15rpx;
}

.music-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 25rpx;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 20rpx;
  border: 1rpx solid transparent;
  transition: all 0.3s;

  &.active {
    background: rgba(102, 126, 234, 0.3);
    border-color: rgba(102, 126, 234, 0.5);
  }

  .music-info {
    flex: 1;
    min-width: 0;

    .music-name {
      display: block;
      font-size: 30rpx;
      font-weight: bold;
      margin-bottom: 8rpx;
    }

    .music-artist {
      display: block;
      font-size: 24rpx;
      color: rgba(255, 255, 255, 0.7);
    }
  }

  .music-controls {
    .control-icon {
      font-size: 40rpx;
    }
  }
}

.empty-music {
  text-align: center;
  padding: 60rpx 0;

  .empty-icon {
    font-size: 80rpx;
    margin-bottom: 20rpx;
  }

  .empty-text {
    display: block;
    font-size: 28rpx;
    margin-bottom: 10rpx;
  }

  .empty-hint {
    display: block;
    font-size: 24rpx;
    color: rgba(255, 255, 255, 0.6);
  }
}

.color-picker {
  margin-bottom: 40rpx;

  .picker-label {
    display: block;
    font-size: 26rpx;
    margin-bottom: 20rpx;
    color: rgba(255, 255, 255, 0.9);
  }

  .color-list {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 20rpx;

    .color-item {
      aspect-ratio: 1;
      border-radius: 15rpx;
      display: flex;
      align-items: center;
      justify-content: center;
      border: 3rpx solid transparent;
      transition: all 0.3s;

      &.active {
        border-color: #ffffff;
        transform: scale(1.1);
        box-shadow: 0 0 20rpx rgba(255, 255, 255, 0.5);
      }

      .check-icon {
        color: #ffffff;
        font-size: 32rpx;
        font-weight: bold;
        text-shadow: 0 0 10rpx rgba(0, 0, 0, 0.5);
      }
    }
  }
}

.intensity-control {
  .control-label {
    display: block;
    font-size: 26rpx;
    margin-bottom: 20rpx;
    color: rgba(255, 255, 255, 0.9);
  }

  .intensity-value {
    display: block;
    text-align: center;
    font-size: 32rpx;
    font-weight: bold;
    margin-top: 15rpx;
  }
}

.feature-section {
  margin-top: 40rpx;

  .feature-card {
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10rpx);
    border-radius: 20rpx;
    padding: 30rpx;
    display: flex;
    align-items: center;
    transition: all 0.3s;

    &:active {
      background: rgba(255, 255, 255, 0.15);
      transform: scale(0.98);
    }

    .feature-icon {
      font-size: 60rpx;
      margin-right: 20rpx;
    }

    .feature-info {
      flex: 1;

      .feature-title {
        display: block;
        font-size: 32rpx;
        color: #ffffff;
        font-weight: bold;
        margin-bottom: 8rpx;
      }

      .feature-desc {
        display: block;
        font-size: 24rpx;
        color: rgba(255, 255, 255, 0.7);
      }
    }

    .feature-arrow {
      font-size: 48rpx;
      color: rgba(255, 255, 255, 0.5);
    }
  }
}

.share-section {
  margin-top: 40rpx;

  .share-btn {
    width: 100%;
    height: 90rpx;
    background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
    color: #ffffff;
    border-radius: 50rpx;
    border: none;
    font-size: 32rpx;
    font-weight: bold;
    line-height: 90rpx;
    
    &::after {
      border: none;
    }
  }
}

.modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}

.modal-content {
  width: 600rpx;
  background: var(--theme-surface);
  border-radius: 30rpx;
  padding: 40rpx;
  animation: scaleIn 0.3s;

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 30rpx;

    .modal-title {
      font-size: 36rpx;
      font-weight: bold;
      color: var(--theme-text);
    }

    .modal-close {
      font-size: 40rpx;
      color: var(--theme-text-secondary);
    }
  }

  .modal-form {
    .form-item {
      margin-bottom: 25rpx;

      .form-label {
        display: block;
        font-size: 26rpx;
        color: var(--theme-text-secondary);
        margin-bottom: 10rpx;
      }

      .form-input {
        width: 100%;
        height: 80rpx;
        background: var(--theme-background);
        border-radius: 15rpx;
        padding: 0 20rpx;
        font-size: 28rpx;
      }
    }
  }

  .modal-actions {
    margin-top: 40rpx;

    .modal-btn {
      width: 100%;
      height: 90rpx;
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      color: #ffffff;
      border-radius: 50rpx;
      border: none;
      font-size: 32rpx;
      font-weight: bold;
      line-height: 90rpx;
      
      &::after {
        border: none;
      }
    }
  }
}

@keyframes scaleIn {
  from {
    transform: scale(0.8);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}
</style>

