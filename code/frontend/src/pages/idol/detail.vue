<template>
  <view class="detail-page">
    <view class="idol-header">
      <view class="header-bg"></view>
      <view class="idol-info">
        <view v-if="idol.avatar" class="idol-avatar-wrapper" @click="uploadAvatar">
          <image class="idol-avatar" :src="getFullUrl(idol.avatar)" mode="aspectFill" />
          <view class="avatar-upload-hint">
            <text>📷</text>
          </view>
        </view>
        <view v-else class="idol-avatar-wrapper idol-avatar-emoji" @click="uploadAvatar">
          <text>⭐</text>
          <view class="avatar-upload-hint">
            <text>📷</text>
          </view>
        </view>
        <text class="idol-name">{{ idol.name }}</text>
        <text class="idol-category">{{ idol.category }}</text>
        <text v-if="idol.motto" class="idol-motto">{{ idol.motto }}</text>
      </view>
      <view class="header-actions">
        <view class="action-btn" @click="shareIdol">
          <text>📤</text>
          <text>分享</text>
        </view>
      </view>
    </view>

    <view class="tabs">
      <view class="tab" :class="{ active: activeTab === 'quotes' }" @click="activeTab = 'quotes'">
        <text>💬 语录</text>
      </view>
      <view class="tab" :class="{ active: activeTab === 'works' }" @click="activeTab = 'works'">
        <text>📚 作品</text>
      </view>
    </view>

    <!-- 语录列表 -->
    <view v-if="activeTab === 'quotes'" class="content-section">
      <view class="section-header">
        <text class="section-title">经典语录</text>
        <view class="add-btn" @click="addQuote">
          <text>+ 添加</text>
        </view>
      </view>
      <view v-if="quotes.length > 0" class="quotes-list">
        <view
          v-for="(quote, index) in quotes"
          :key="quote.id"
          class="quote-item"
          @click="editQuote(index)"
          @longpress="deleteQuote(index)"
        >
          <text class="quote-icon">💬</text>
          <text class="quote-text">{{ quote.content }}</text>
          <view class="quote-actions">
            <text class="action-icon" @click.stop="shareQuote(quote.content)">📤</text>
          </view>
        </view>
      </view>
      <view v-else class="empty-state">
        <text class="empty-icon">💬</text>
        <text class="empty-text">还没有语录</text>
        <text class="empty-hint">点击上方"添加"按钮添加语录</text>
      </view>
    </view>

    <!-- 作品列表 -->
    <view v-if="activeTab === 'works'" class="content-section">
      <view class="section-header">
        <text class="section-title">代表作品</text>
        <view class="add-btn" @click="addWork">
          <text>+ 添加</text>
        </view>
      </view>
      <view v-if="works.length > 0" class="works-list">
        <view
          v-for="(work, index) in works"
          :key="index"
          class="work-item"
          :class="`work-type-${work.work_type}`"
        >
          <!-- 文本作品 -->
          <template v-if="work.work_type === 'text'">
            <view class="work-icon">📚</view>
            <view class="work-info" @click="editWork(index)">
              <text class="work-title">{{ work.title }}</text>
              <text class="work-desc">{{ work.description || '暂无描述' }}</text>
            </view>
          </template>

          <!-- 图片作品 -->
          <template v-if="work.work_type === 'image' && work.file_url">
            <view class="work-image-container" @click="previewImage(work.file_url)">
              <image class="work-image" :src="getFullUrl(work.file_url)" mode="widthFix" />
              <view class="work-image-overlay">
                <text class="work-title">{{ work.title }}</text>
              </view>
            </view>
          </template>

          <!-- 音频作品 -->
          <template v-if="work.work_type === 'audio' && work.file_url">
            <view class="audio-work">
              <view class="audio-header">
                <view class="audio-icon">🎵</view>
                <view class="audio-info">
                  <text class="work-title">{{ work.title }}</text>
                  <text class="work-desc" v-if="work.description">{{ work.description }}</text>
                </view>
              </view>
              <view class="audio-player">
                <view class="player-controls">
                  <view 
                    class="play-btn" 
                    @click="toggleAudio(index)"
                  >
                    <text class="control-icon">{{ currentPlayingAudio === index ? '⏸️' : '▶️' }}</text>
                  </view>
                  <view class="time-display">
                    <text class="current-time">{{ formatDuration(audioCurrentTime[index] || 0) }}</text>
                    <text class="time-separator">/</text>
                    <text class="total-time">{{ formatDuration(work.duration || 0) }}</text>
                  </view>
                </view>
                <view class="audio-progress-bar" @click="seekAudio($event, index)">
                  <view class="progress-track">
                    <view 
                      class="progress-fill" 
                      :style="{ width: audioProgress[index] || '0%' }"
                    ></view>
                    <view 
                      class="progress-thumb" 
                      :style="{ left: audioProgress[index] || '0%' }"
                    ></view>
                  </view>
                </view>
                <view class="audio-controls">
                  <view class="speed-control">
                    <text class="control-label">速度:</text>
                    <text class="speed-value" @click="changeAudioSpeed(index)">{{ audioSpeed[index] || 1.0 }}x</text>
                  </view>
                  <view class="volume-control">
                    <text class="control-icon">🔊</text>
                    <slider 
                      class="volume-slider" 
                      :value="audioVolume[index] || 100" 
                      @change="changeAudioVolume($event, index)"
                      min="0" 
                      max="100"
                      activeColor="#667eea"
                      backgroundColor="#e0e0e0"
                      block-size="12"
                    />
                  </view>
                </view>
              </view>
            </view>
          </template>

          <!-- 视频作品 -->
          <template v-if="work.work_type === 'video' && work.file_url">
            <view class="video-work">
              <video 
                :id="`video-${index}`"
                class="video-player" 
                :src="getFullUrl(work.file_url)"
                :poster="getFullUrl(work.cover_url)"
                :controls="true"
                :show-center-play-btn="true"
                :show-play-btn="true"
                :show-fullscreen-btn="true"
                :show-progress="true"
                :enable-progress-gesture="true"
                :page-gesture="false"
                :object-fit="'contain'"
                :play-btn-position="'center'"
                :enable-play-gesture="true"
                @play="onVideoPlay(index)"
                @pause="onVideoPause(index)"
                @timeupdate="onVideoTimeUpdate($event, index)"
                @ended="onVideoEnded(index)"
                @error="onVideoError($event, index)"
                @fullscreenchange="onVideoFullscreenChange($event, index)"
              >
              </video>
              <view class="video-info">
                <view class="video-meta">
                  <text class="work-title">{{ work.title }}</text>
                  <text class="work-desc" v-if="work.description">{{ work.description }}</text>
                </view>
                <view class="video-stats" v-if="work.duration">
                  <text class="stat-item">
                    <text class="stat-icon">⏱️</text>
                    <text class="stat-value">{{ formatDuration(work.duration) }}</text>
                  </text>
                  <text class="stat-item" v-if="videoProgress[index]">
                    <text class="stat-icon">📊</text>
                    <text class="stat-value">{{ videoProgress[index] }}%</text>
                  </text>
                </view>
              </view>
            </view>
          </template>

          <!-- 操作按钮 -->
          <view class="work-actions">
            <text class="action-icon" @click.stop="editWork(index)">✏️</text>
            <text class="action-icon" @click.stop="deleteWork(index)">🗑️</text>
            <text class="action-icon" @click.stop="shareWork(work)">📤</text>
          </view>
        </view>
      </view>
      <view v-else class="empty-state">
        <text class="empty-icon">📚</text>
        <text class="empty-text">还没有作品</text>
        <text class="empty-hint">点击上方"添加"按钮添加作品</text>
      </view>
    </view>

    <!-- 添加/编辑语录弹窗 -->
    <view v-if="showQuoteModal" class="modal-mask" @click="showQuoteModal = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">{{ editingQuoteIndex >= 0 ? '编辑语录' : '添加语录' }}</text>
          <text class="modal-close" @click="showQuoteModal = false">✕</text>
        </view>
        <textarea
          v-model="quoteText"
          class="modal-textarea"
          placeholder="输入语录内容..."
          maxlength="200"
        />
        <text class="char-count">{{ quoteText.length }}/200</text>
        <view class="modal-actions">
          <button class="modal-btn" @click="saveQuote">保存</button>
        </view>
      </view>
    </view>

    <!-- 添加/编辑作品弹窗 -->
    <view v-if="showWorkModal" class="modal-mask" @click="showWorkModal = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">{{ editingWorkIndex >= 0 ? '编辑作品' : '添加作品' }}</text>
          <text class="modal-close" @click="showWorkModal = false">✕</text>
        </view>
        <view class="modal-form">
          <view class="form-item">
            <text class="form-label">作品名称</text>
            <input v-model="workForm.title" class="form-input" placeholder="输入作品名称" maxlength="50" />
          </view>
          <view class="form-item">
            <text class="form-label">作品类型</text>
            <view class="work-type-selector">
              <view 
                v-for="type in workTypes" 
                :key="type.value"
                class="type-option"
                :class="{ active: workForm.work_type === type.value }"
                @click="workForm.work_type = type.value"
              >
                <text>{{ type.icon }} {{ type.label }}</text>
              </view>
            </view>
          </view>
          <view class="form-item">
            <text class="form-label">作品描述</text>
            <textarea
              v-model="workForm.description"
              class="form-textarea"
              placeholder="输入作品描述..."
              maxlength="200"
            />
          </view>
          
          <!-- 图片上传 -->
          <view v-if="workForm.work_type === 'image'" class="form-item">
            <text class="form-label">上传图片</text>
            <view class="upload-area" @click="uploadImage">
              <image v-if="workForm.file_url" class="preview-image" :src="getFullUrl(workForm.file_url)" mode="widthFix" />
              <view v-else class="upload-placeholder">
                <text class="upload-icon">🖼️</text>
                <text class="upload-text">点击上传图片</text>
              </view>
            </view>
          </view>

          <!-- 音频上传 -->
          <view v-if="workForm.work_type === 'audio'" class="form-item">
            <text class="form-label">上传音频</text>
            <view class="upload-area" @click="uploadAudio">
              <view v-if="workForm.file_url" class="file-info">
                <text class="file-icon">🎵</text>
                <text class="file-name">已上传音频</text>
              </view>
              <view v-else class="upload-placeholder">
                <text class="upload-icon">🎵</text>
                <text class="upload-text">点击上传音频</text>
              </view>
            </view>
          </view>

          <!-- 视频上传 -->
          <view v-if="workForm.work_type === 'video'" class="form-item">
            <text class="form-label">上传视频</text>
            <view class="upload-area" @click="uploadVideo">
              <view v-if="workForm.file_url" class="file-info">
                <text class="file-icon">🎥</text>
                <text class="file-name">已上传视频</text>
              </view>
              <view v-else class="upload-placeholder">
                <text class="upload-icon">🎥</text>
                <text class="upload-text">点击上传视频</text>
              </view>
            </view>
            <!-- 视频封面 -->
            <view v-if="workForm.file_url" class="form-subitem">
              <text class="form-label">视频封面（可选）</text>
              <view class="upload-area small" @click="uploadVideoCover">
                <image v-if="workForm.cover_url" class="preview-image" :src="getFullUrl(workForm.cover_url)" mode="widthFix" />
                <view v-else class="upload-placeholder small">
                  <text class="upload-text">上传封面</text>
                </view>
              </view>
            </view>
          </view>
        </view>
        <view class="modal-actions">
          <button class="modal-btn" @click="saveWork">保存</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { navigateTo } from '@/utils'
import { BASE_API } from '@/config'
import type { IdolWork, IdolQuote, IdolInfo } from '@/types'
import { uploadImageApi, uploadVideoApi, uploadAudioApi } from '@/api/upload'
import { 
  getIdolApi, 
  updateIdolApi,
  createIdolQuoteApi, 
  deleteIdolQuoteApi,
  createIdolWorkApi,
  deleteIdolWorkApi 
} from '@/api/idol'

const idolId = ref('')
const idol = ref<IdolInfo>({ id: '', name: '', avatar: '', description: '', category: '' })
const activeTab = ref<'quotes' | 'works'>('quotes')
const quotes = ref<IdolQuote[]>([])
const works = ref<IdolWork[]>([])

// 将相对路径转换为完整URL
const getFullUrl = (url: string | null | undefined) => {
  if (!url) return ''
  if (url.startsWith('http')) return url
  return `${BASE_API.replace('/api/v1', '')}${url}`
}

const showQuoteModal = ref(false)
const showWorkModal = ref(false)
const quoteText = ref('')
const editingQuoteIndex = ref(-1)

// 作品类型选项
const workTypes = [
  { value: 'text', label: '文本', icon: '📚' },
  { value: 'image', label: '图片', icon: '🖼️' },
  { value: 'audio', label: '音频', icon: '🎵' },
  { value: 'video', label: '视频', icon: '🎥' },
]

const workForm = ref<{
  title: string
  description: string
  work_type: 'text' | 'image' | 'audio' | 'video'
  file_url?: string
  cover_url?: string
  duration?: number
}>({
  title: '',
  description: '',
  work_type: 'text'
})

const editingWorkIndex = ref(-1)

// 音频播放相关
const currentPlayingAudio = ref<number | null>(null)
const audioContext = ref<any>(null)
const audioProgress = ref<Record<number, string>>({})
const audioCurrentTime = ref<Record<number, number>>({})
const audioSpeed = ref<Record<number, number>>({})
const audioVolume = ref<Record<number, number>>({})

// 视频播放相关
const currentPlayingVideo = ref<number | null>(null)
const videoProgress = ref<Record<number, number>>({})

onLoad((options: any) => {
  if (options.id) {
    idolId.value = options.id
    loadIdolDetail()
  }
})

onUnmounted(() => {
  // 清理音频播放
  if (audioContext.value) {
    audioContext.value.stop()
    audioContext.value.destroy()
  }
})

const loadIdolDetail = async () => {
  try {
    const data = await getIdolApi(idolId.value)
    // 新的 API 返回包含 idol, quotes, works 的完整数据
    if (data.idol) {
      idol.value = data.idol as any
    } else {
      idol.value = data as any
    }
    quotes.value = data.quotes || []
    works.value = data.works || []
  } catch (error) {
    console.error('加载偶像详情失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  }
}

const addQuote = () => {
  quoteText.value = ''
  editingQuoteIndex.value = -1
  showQuoteModal.value = true
}

const editQuote = (index: number) => {
  quoteText.value = quotes.value[index].content
  editingQuoteIndex.value = index
  showQuoteModal.value = true
}

const saveQuote = async () => {
  if (!quoteText.value.trim()) {
    return uni.showToast({ title: '请输入语录内容', icon: 'none' })
  }
  
  try {
    if (editingQuoteIndex.value >= 0) {
      // 编辑功能暂不支持，可以先删除再添加
      const quote = quotes.value[editingQuoteIndex.value]
      console.log('编辑语录-先删除:', { quoteId: quote.id })
      await deleteIdolQuoteApi(quote.id)
      await createIdolQuoteApi(idolId.value, {
        idol_id: idolId.value,
        content: quoteText.value.trim()
      })
    } else {
      await createIdolQuoteApi(idolId.value, {
        idol_id: idolId.value,
        content: quoteText.value.trim()
      })
    }
    
    await loadIdolDetail()
    showQuoteModal.value = false
    uni.showToast({ title: '保存成功', icon: 'success' })
  } catch (error: any) {
    console.error('保存语录失败:', error)
    uni.showToast({ 
      title: `保存失败: ${error.message || '未知错误'}`, 
      icon: 'none',
      duration: 3000
    })
  }
}

const deleteQuote = async (index: number) => {
  uni.showModal({
    title: '确认删除',
    content: '确定要删除这条语录吗？',
    success: async (res) => {
      if (res.confirm) {
        try {
          const quote = quotes.value[index]
          console.log('删除语录:', { quoteId: quote.id, idolId: idolId.value })
          await deleteIdolQuoteApi(quote.id)
          await loadIdolDetail()
          uni.showToast({ title: '已删除', icon: 'success' })
        } catch (error: any) {
          console.error('删除语录失败:', error)
          uni.showToast({ 
            title: `删除失败: ${error.message || '未知错误'}`, 
            icon: 'none',
            duration: 3000
          })
        }
      }
    }
  })
}

const addWork = () => {
  workForm.value = { 
    title: '', 
    description: '',
    work_type: 'text'
  }
  editingWorkIndex.value = -1
  showWorkModal.value = true
}

const editWork = (index: number) => {
  const work = works.value[index]
  workForm.value = { 
    title: work.title,
    description: work.description || '',
    work_type: work.work_type,
    file_url: work.file_url,
    cover_url: work.cover_url,
    duration: work.duration
  }
  editingWorkIndex.value = index
  showWorkModal.value = true
}

const saveWork = async () => {
  if (!workForm.value.title.trim()) {
    return uni.showToast({ title: '请输入作品名称', icon: 'none' })
  }
  
  // 检查是否需要文件
  if (['image', 'audio', 'video'].includes(workForm.value.work_type) && !workForm.value.file_url) {
    return uni.showToast({ title: '请上传文件', icon: 'none' })
  }
  
  try {
    if (editingWorkIndex.value >= 0) {
      // 编辑功能暂不支持，先删除再添加
      const work = works.value[editingWorkIndex.value]
      console.log('编辑作品-先删除:', { workId: work.id })
      await deleteIdolWorkApi(work.id)
    }
    
    await createIdolWorkApi(idolId.value, {
      idol_id: idolId.value,
      title: workForm.value.title.trim(),
      description: workForm.value.description.trim(),
      work_type: workForm.value.work_type,
      file_url: workForm.value.file_url,
      cover_url: workForm.value.cover_url,
      duration: workForm.value.duration
    })
    
    await loadIdolDetail()
    showWorkModal.value = false
    uni.showToast({ title: '保存成功', icon: 'success' })
  } catch (error) {
    console.error('保存作品失败:', error)
    uni.showToast({ title: '保存失败', icon: 'none' })
  }
}

const deleteWork = async (index: number) => {
  uni.showModal({
    title: '确认删除',
    content: '确定要删除这个作品吗？',
    success: async (res) => {
      if (res.confirm) {
        try {
          const work = works.value[index]
          console.log('删除作品:', { workId: work.id, idolId: idolId.value })
          await deleteIdolWorkApi(work.id)
          await loadIdolDetail()
          uni.showToast({ title: '已删除', icon: 'success' })
        } catch (error: any) {
          console.error('删除作品失败:', error)
          uni.showToast({ 
            title: `删除失败: ${error.message || '未知错误'}`, 
            icon: 'none',
            duration: 3000
          })
        }
      }
    }
  })
}

const shareIdol = () => {
  const quoteContents = quotes.value.map(q => q.content).slice(0, 3)
  const shareText = `${idol.value.name} - ${idol.value.category}

经典语录：
${quoteContents.join('\n')}`
  shareContent(shareText, `分享${idol.value.name}`)
}

const shareQuote = (quote: string) => {
  shareContent(`${quote}\n\n—— ${idol.value.name}`, '分享语录')
}

const shareWork = (work: any) => {
  shareContent(`${work.title}\n${work.description || ''}\n\n—— ${idol.value.name}`, '分享作品')
}

const shareContent = (content: string, title: string) => {
  uni.showModal({
    title: title,
    content: content,
    confirmText: '复制',
    success: (res) => {
      if (res.confirm) {
        uni.setClipboardData({
          data: content,
          success: () => {
            uni.showToast({ title: '已复制到剪贴板', icon: 'success' })
          }
        })
      }
    }
  })
}

// 格式化时长
const formatDuration = (seconds: number) => {
  const mins = Math.floor(seconds / 60)
  const secs = Math.floor(seconds % 60)
  return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`
}

// 图片预览
const previewImage = (url: string) => {
  uni.previewImage({
    urls: [url],
    current: url
  })
}

// 音频播放控制
const toggleAudio = (index: number) => {
  const work = works.value[index]
  if (!work.file_url) return

  if (currentPlayingAudio.value === index) {
    // 暂停当前音频
    if (audioContext.value) {
      audioContext.value.pause()
      currentPlayingAudio.value = null
    }
  } else {
    // 停止之前的音频
    if (audioContext.value) {
      audioContext.value.stop()
    }
    
    // 播放新音频
    audioContext.value = uni.createInnerAudioContext()
    audioContext.value.src = getFullUrl(work.file_url)
    
    // 设置初始音量和速度
    audioVolume.value[index] = audioVolume.value[index] || 100
    audioSpeed.value[index] = audioSpeed.value[index] || 1.0
    audioContext.value.volume = audioVolume.value[index] / 100
    // 注意：uniapp 的 InnerAudioContext 不支持 playbackRate，这是平台限制
    
    audioContext.value.onPlay(() => {
      currentPlayingAudio.value = index
    })
    
    audioContext.value.onTimeUpdate(() => {
      if (audioContext.value && work.duration) {
        const currentTime = audioContext.value.currentTime
        const progress = (currentTime / work.duration) * 100
        audioProgress.value[index] = `${progress}%`
        audioCurrentTime.value[index] = currentTime
      }
    })
    
    audioContext.value.onEnded(() => {
      currentPlayingAudio.value = null
      audioProgress.value[index] = '0%'
      audioCurrentTime.value[index] = 0
    })
    
    audioContext.value.onError((error: any) => {
      console.error('音频播放错误:', error)
      uni.showToast({ title: '音频播放失败', icon: 'none' })
      currentPlayingAudio.value = null
    })
    
    audioContext.value.play()
  }
}

// 音频进度条点击跳转
const seekAudio = (event: any, index: number) => {
  const work = works.value[index]
  if (!audioContext.value || currentPlayingAudio.value !== index || !work.duration) return
  
  // 获取点击位置
  const touch = event.detail || event.touches?.[0] || event.changedTouches?.[0]
  if (!touch) return
  
  // 计算进度（需要根据元素宽度计算）
  uni.createSelectorQuery()
    .select('.audio-progress-bar')
    .boundingClientRect((rect: any) => {
      if (rect) {
        const clickX = touch.x || touch.clientX
        const percent = (clickX - rect.left) / rect.width
        const seekTime = percent * work.duration
        audioContext.value.seek(seekTime)
      }
    })
    .exec()
}

// 改变音频速度
const changeAudioSpeed = (index: number) => {
  const speeds = [0.5, 0.75, 1.0, 1.25, 1.5, 2.0]
  const currentSpeed = audioSpeed.value[index] || 1.0
  const currentIndex = speeds.indexOf(currentSpeed)
  const nextIndex = (currentIndex + 1) % speeds.length
  audioSpeed.value[index] = speeds[nextIndex]
  
  uni.showToast({ 
    title: `播放速度: ${audioSpeed.value[index]}x`, 
    icon: 'none',
    duration: 1000
  })
  
  // 注意：uniapp 的 InnerAudioContext 不支持动态调整播放速度
  // 这是平台限制，仅显示提示
}

// 改变音量
const changeAudioVolume = (event: any, index: number) => {
  const volume = event.detail.value
  audioVolume.value[index] = volume
  
  if (audioContext.value && currentPlayingAudio.value === index) {
    audioContext.value.volume = volume / 100
  }
}

// 视频播放控制
const onVideoPlay = (index: number) => {
  // 暂停其他视频
  if (currentPlayingVideo.value !== null && currentPlayingVideo.value !== index) {
    const videoContext = uni.createVideoContext(`video-${currentPlayingVideo.value}`)
    videoContext.pause()
  }
  currentPlayingVideo.value = index
  
  // 暂停音频
  if (audioContext.value) {
    audioContext.value.pause()
    currentPlayingAudio.value = null
  }
}

const onVideoPause = (index: number) => {
  if (currentPlayingVideo.value === index) {
    currentPlayingVideo.value = null
  }
}

// 视频时间更新
const onVideoTimeUpdate = (event: any, index: number) => {
  const work = works.value[index]
  if (work.duration && event.detail) {
    const currentTime = event.detail.currentTime
    const progress = Math.round((currentTime / work.duration) * 100)
    videoProgress.value[index] = progress
  }
}

// 视频播放结束
const onVideoEnded = (index: number) => {
  currentPlayingVideo.value = null
  videoProgress.value[index] = 0
}

// 视频错误
const onVideoError = (event: any, index: number) => {
  const work = works.value[index]
  console.error('视频播放错误:', {
    event,
    index,
    videoUrl: work.file_url,
    fullUrl: getFullUrl(work.file_url),
    errorDetail: event.detail
  })
  uni.showToast({ 
    title: '视频加载失败，请检查网络或视频格式', 
    icon: 'none',
    duration: 3000
  })
  currentPlayingVideo.value = null
}

// 视频全屏变化
const onVideoFullscreenChange = (event: any, index: number) => {
  console.log('全屏状态变化:', event.detail)
}

// 图片上传
const uploadImage = () => {
  uni.chooseImage({
    count: 1,
    sizeType: ['original', 'compressed'],  // 优先使用原图，保证图片完整性
    sourceType: ['album', 'camera'],
    success: async (res) => {
      try {
        const tempFilePath = res.tempFilePaths[0]
        console.log('图片选择成功:', tempFilePath)
        
        // 显示上传中提示
        uni.showLoading({ title: '上传中...' })
        
        // 上传到服务器
        const uploadResult = await uploadImageApi(tempFilePath)
        console.log('图片上传成功:', uploadResult.url)
        
        // 使用服务器返回的URL
        workForm.value.file_url = uploadResult.url
        
        uni.hideLoading()
        uni.showToast({ title: '图片上传成功', icon: 'success' })
      } catch (error: any) {
        uni.hideLoading()
        console.error('图片上传失败:', error)
        uni.showToast({ title: '上传失败: ' + (error.message || '未知错误'), icon: 'none' })
      }
    },
    fail: (error) => {
      console.error('图片选择失败:', error)
      uni.showToast({ title: '图片选择失败', icon: 'none' })
    }
  })
}

// 音频上传
const uploadAudio = () => {
  // #ifdef MP-WEIXIN
  uni.chooseMessageFile({
    count: 1,
    type: 'file',
    extension: ['.mp3', '.wav', '.m4a'],
    success: async (res) => {
      try {
        const file = res.tempFiles[0]
        console.log('音频文件信息:', {
          path: file.path,
          name: file.name,
          size: file.size,
          type: file.type
        })
        
        // 检查文件大小 (限制为50MB)
        const maxSize = 50 * 1024 * 1024 // 50MB
        if (file.size > maxSize) {
          uni.showToast({ 
            title: `文件过大，请选择小于50MB的音频`, 
            icon: 'none',
            duration: 3000
          })
          return
        }
        
        // 显示上传中提示
        uni.showLoading({ title: '上传音频中...' })
        
        // 上传音频到服务器
        const audioUploadResult = await uploadAudioApi(file.path)
        console.log('音频上传成功:', audioUploadResult.url)
        
        // 使用服务器返回的URL
        workForm.value.file_url = audioUploadResult.url
        
        // 获取音频时长
        const audio = uni.createInnerAudioContext()
        audio.src = file.path
        audio.onCanplay(() => {
          // 将浮点数时长转换为整数（秒）
          workForm.value.duration = Math.round(audio.duration)
          audio.destroy()
        })
        
        uni.hideLoading()
        uni.showToast({ title: '音频上传成功', icon: 'success' })
      } catch (error: any) {
        uni.hideLoading()
        console.error('音频上传失败:', error)
        const errorMsg = error.errMsg || error.message || '未知错误'
        uni.showToast({ 
          title: `上传失败: ${errorMsg}`, 
          icon: 'none',
          duration: 3000
        })
      }
    },
    fail: (error) => {
      console.error('选择音频失败:', error)
      uni.showToast({ title: '选择音频失败', icon: 'none' })
    }
  })
  // #endif
  
  // #ifndef MP-WEIXIN
  uni.showToast({ title: '当前平台不支持音频上传', icon: 'none' })
  // #endif
}

// 视频上传
const uploadVideo = () => {
  console.log('开始选择视频...')
  uni.chooseVideo({
    count: 1,
    sourceType: ['album', 'camera'],
    maxDuration: 60, // 微信小程序限制最长60秒
    success: async (res) => {
      try {
        console.log('视频选择成功:', res)
        const tempFilePath = res.tempFilePath
        
        // 显示上传中提示
        uni.showLoading({ title: '上传视频中...' })
        
        // 上传视频到服务器
        const videoUploadResult = await uploadVideoApi(tempFilePath)
        console.log('视频上传成功:', videoUploadResult.url)
        
        // 使用服务器返回的URL
        workForm.value.file_url = videoUploadResult.url
        
        // 将浮点数时长转换为整数（秒）
        workForm.value.duration = Math.round(res.duration)
        console.log('视频时长:', workForm.value.duration)
        
        // 自动设置封面为视频缩略图
        if (res.thumbTempFilePath) {
          // 上传封面图片
          const coverUploadResult = await uploadImageApi(res.thumbTempFilePath)
          workForm.value.cover_url = coverUploadResult.url
          console.log('视频封面上传成功:', workForm.value.cover_url)
        }
        
        uni.hideLoading()
        uni.showToast({ title: '视频上传成功', icon: 'success' })
      } catch (error: any) {
        uni.hideLoading()
        console.error('视频上传失败:', error)
        uni.showToast({ title: '上传失败: ' + (error.message || '未知错误'), icon: 'none' })
      }
    },
    fail: (error) => {
      console.error('视频选择失败:', error)
      uni.showToast({ title: '视频选择失败', icon: 'none' })
    },
    complete: () => {
      console.log('视频选择完成')
    }
  })
}

// 视频封面上传
const uploadVideoCover = () => {
  uni.chooseImage({
    count: 1,
    sizeType: ['compressed'],
    sourceType: ['album', 'camera'],
    success: async (res) => {
      try {
        const tempFilePath = res.tempFilePaths[0]
        console.log('封面选择成功:', tempFilePath)
        
        // 显示上传中提示
        uni.showLoading({ title: '上传中...' })
        
        // 上传到服务器
        const uploadResult = await uploadImageApi(tempFilePath)
        console.log('封面上传成功:', uploadResult.url)
        
        // 使用服务器返回的URL
        workForm.value.cover_url = uploadResult.url
        
        uni.hideLoading()
        uni.showToast({ title: '封面上传成功', icon: 'success' })
      } catch (error: any) {
        uni.hideLoading()
        console.error('封面上传失败:', error)
        uni.showToast({ title: '上传失败: ' + (error.message || '未知错误'), icon: 'none' })
      }
    }
  })
}

// 上传偶像头像
const uploadAvatar = () => {
  uni.chooseImage({
    count: 1,
    sizeType: ['original', 'compressed'],  // 优先使用原图
    sourceType: ['album', 'camera'],
    success: async (res) => {
      try {
        const tempFilePath = res.tempFilePaths[0]
        console.log('选择头像成功:', tempFilePath)
        
        // 显示上传中提示
        uni.showLoading({ title: '上传中...' })
        
        // 上传到服务器
        const uploadResult = await uploadImageApi(tempFilePath)
        console.log('头像上传成功:', uploadResult.url)
        
        // 更新偶像信息
        await updateIdolApi(idolId.value, {
          name: idol.value.name,
          avatar: uploadResult.url,
          description: idol.value.description,
          category: idol.value.category
        })
        
        // 隐藏加载提示
        uni.hideLoading()
        
        // 重新加载偶像信息以刷新显示
        await loadIdolDetail()
        uni.showToast({ title: '头像更新成功', icon: 'success' })
      } catch (error: any) {
        uni.hideLoading()
        console.error('更新头像失败:', error)
        uni.showToast({ title: '更新失败: ' + (error.message || '未知错误'), icon: 'none' })
      }
    },
    fail: (error) => {
      console.error('选择头像失败:', error)
      uni.showToast({ title: '选择头像失败', icon: 'none' })
    }
  })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.detail-page {
  @include cyber-page-bg;
  min-height: 100vh;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 30% 20%, rgba(255, 214, 0, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 70% 80%, rgba(138, 92, 246, 0.12) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.idol-header {
  position: relative;
  padding: 60rpx 30rpx 50rpx;
  background: linear-gradient(135deg, rgba(255, 214, 0, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
  backdrop-filter: blur(30rpx);
  overflow: hidden;
  z-index: 1;
  box-shadow: 
    0 0 60rpx rgba(255, 214, 0, 0.4),
    inset 0 0 80rpx rgba(255, 214, 0, 0.15);
  border-bottom: 3rpx solid rgba(255, 214, 0, 0.6);

  .header-bg {
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: conic-gradient(
      from 0deg,
      transparent 0deg,
      rgba(255, 214, 0, 0.15) 90deg,
      transparent 180deg,
      rgba(138, 92, 246, 0.15) 270deg,
      transparent 360deg
    );
    animation: bgRotate 20s linear infinite;
    z-index: 0;
  }

  .idol-info {
    position: relative;
    z-index: 1;
    text-align: center;

    .idol-avatar-wrapper {
      width: 180rpx;
      height: 180rpx;
      margin: 0 auto 25rpx;
      position: relative;
      cursor: pointer;
      animation: avatarFloat 3s ease-in-out infinite;

      .idol-avatar {
        width: 100%;
        height: 100%;
        border-radius: 90rpx;
        border: 4rpx solid rgba(255, 214, 0, 0.8);
        box-shadow: 
          0 0 40rpx rgba(255, 214, 0, 0.8),
          inset 0 0 40rpx rgba(255, 214, 0, 0.2);
        animation: avatarPulse 2s ease-in-out infinite;
      }

      .avatar-upload-hint {
        position: absolute;
        bottom: 5rpx;
        right: 5rpx;
        width: 50rpx;
        height: 50rpx;
        background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 24rpx;
        border: 3rpx solid rgba(255, 214, 0, 0.8);
        box-shadow: 
          0 4rpx 12rpx rgba(0, 217, 255, 0.5),
          0 0 30rpx rgba(0, 217, 255, 0.4);
        animation: iconPulse 2s ease-in-out infinite;
      }

      &.idol-avatar-emoji {
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(135deg, rgba(255, 214, 0, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
        border-radius: 90rpx;
        border: 4rpx solid rgba(255, 214, 0, 0.8);
        font-size: 90rpx;
        box-shadow: 
          0 0 40rpx rgba(255, 214, 0, 0.8),
          inset 0 0 40rpx rgba(255, 214, 0, 0.2);
        animation: avatarPulse 2s ease-in-out infinite;
        filter: drop-shadow(0 0 30rpx rgba(255, 214, 0, 0.8));
      }
    }

    .idol-name {
      display: block;
      font-size: 48rpx;
      font-weight: bold;
      @include neon-title(#FFD600);
      margin-bottom: 15rpx;
      animation: titlePulse 3s ease-in-out infinite;
    }

    .idol-category {
      display: block;
      font-size: 30rpx;
      @include neon-text(#00D9FF);
      margin-bottom: 15rpx;
    }

    .idol-motto {
      display: block;
      font-size: 26rpx;
      @include neon-text(#ffffff);
      font-style: italic;
      padding: 0 40rpx;
      line-height: 1.8;
      opacity: 0.9;
    }
  }

  .header-actions {
    position: absolute;
    top: 30rpx;
    right: 30rpx;
    z-index: 2;

    .action-btn {
      background: linear-gradient(135deg, rgba(0, 217, 255, 0.8) 0%, rgba(138, 92, 246, 0.8) 100%);
      backdrop-filter: blur(15rpx);
      border-radius: 50rpx;
      padding: 18rpx 30rpx;
      display: flex;
      align-items: center;
      gap: 10rpx;
      font-size: 26rpx;
      @include neon-text(#ffffff);
      border: 2rpx solid rgba(0, 217, 255, 0.6);
      box-shadow: 
        0 6rpx 20rpx rgba(0, 217, 255, 0.4),
        0 0 30rpx rgba(0, 217, 255, 0.3);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      
      &:active {
        transform: scale(0.93);
        box-shadow: 
          0 4rpx 12rpx rgba(0, 217, 255, 0.6),
          0 0 40rpx rgba(0, 217, 255, 0.5);
      }
    }
  }
}

.tabs {
  display: flex;
  @include neon-card;
  border-bottom: 3rpx solid rgba(255, 214, 0, 0.3);
  position: sticky;
  top: 0;
  z-index: 10;
  backdrop-filter: blur(30rpx);

  .tab {
    flex: 1;
    padding: 35rpx 0;
    text-align: center;
    font-size: 30rpx;
    color: #b8c5d6;
    position: relative;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    cursor: pointer;

    &.active {
      @include neon-text(#FFD600);
      font-weight: bold;

      &::after {
        content: '';
        position: absolute;
        bottom: 0;
        left: 50%;
        transform: translateX(-50%);
        width: 80rpx;
        height: 6rpx;
        background: linear-gradient(90deg, transparent, #FFD600, transparent);
        border-radius: 3rpx;
        box-shadow: 
          0 0 20rpx rgba(255, 214, 0, 0.8),
          0 -4rpx 10rpx rgba(255, 214, 0, 0.6);
        animation: tabGlow 2s ease-in-out infinite;
      }
    }
    
    &:active {
      transform: scale(0.96);
    }
  }
}

.content-section {
  padding: 30rpx;
  position: relative;
  z-index: 1;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30rpx;

  .section-title {
    font-size: 36rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
  }

  .add-btn {
    background: linear-gradient(135deg, rgba(255, 214, 0, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    @include neon-text(#ffffff);
    padding: 15rpx 30rpx;
    border-radius: 50rpx;
    font-size: 26rpx;
    font-weight: bold;
    border: 2rpx solid rgba(255, 214, 0, 0.7);
    box-shadow: 
      0 6rpx 20rpx rgba(255, 214, 0, 0.4),
      0 0 30rpx rgba(255, 214, 0, 0.3),
      inset 0 0 20rpx rgba(255, 214, 0, 0.15);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    
    &:active {
      transform: scale(0.93);
      box-shadow: 
        0 4rpx 12rpx rgba(255, 214, 0, 0.6),
        0 0 40rpx rgba(255, 214, 0, 0.5),
        inset 0 0 30rpx rgba(255, 214, 0, 0.25);
    }
  }
}

.quotes-list,
.works-list {
  display: flex;
  flex-direction: column;
  gap: 20rpx;
}

.quote-item,
.work-item {
  @include neon-card;
  border-radius: 25rpx;
  padding: 35rpx;
  display: flex;
  flex-direction: column;
  border: 2rpx solid rgba(0, 217, 255, 0.5);
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 40rpx rgba(0, 217, 255, 0.3),
    inset 0 0 40rpx rgba(0, 217, 255, 0.1);
  position: relative;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  animation: itemSlideIn 0.5s ease-out backwards;
  
  &:hover {
    border-color: rgba(255, 214, 0, 0.6);
    box-shadow: 
      0 12rpx 45rpx rgba(0, 0, 0, 0.6),
      0 0 50rpx rgba(255, 214, 0, 0.4),
      inset 0 0 45rpx rgba(255, 214, 0, 0.15);
  }

  &.work-type-text {
    flex-direction: row;
    align-items: flex-start;
  }

  .quote-icon,
  .work-icon {
    font-size: 48rpx;
    margin-right: 20rpx;
    flex-shrink: 0;
    filter: drop-shadow(0 0 20rpx rgba(0, 217, 255, 0.8));
    animation: iconFloat 3s ease-in-out infinite;
  }

  .quote-text,
  .work-info {
    flex: 1;
    min-width: 0;

    .quote-text {
      font-size: 28rpx;
      color: var(--theme-text-secondary);
      line-height: 1.8;
    }

    .work-title {
      display: block;
      font-size: 30rpx;
      font-weight: bold;
      color: var(--theme-text);
      margin-bottom: 10rpx;
    }

    .work-desc {
      display: block;
      font-size: 26rpx;
      color: var(--theme-text-secondary);
      line-height: 1.6;
    }
  }

  .quote-actions,
  .work-actions {
    display: flex;
    gap: 15rpx;
    margin-top: 20rpx;

    .action-icon {
      font-size: 36rpx;
      padding: 12rpx;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      filter: drop-shadow(0 0 15rpx rgba(255, 214, 0, 0.6));
      
      &:active {
        transform: scale(1.2) rotate(10deg);
        filter: drop-shadow(0 0 25rpx rgba(255, 214, 0, 1));
      }
    }
  }

  // 图片作品
  .work-image-container {
    width: 100%;
    border-radius: 15rpx;
    overflow: hidden;
    position: relative;
    margin-bottom: 15rpx;

    .work-image {
      width: 100%;
      height: auto;
      display: block;
    }

    .work-image-overlay {
      position: absolute;
      bottom: 0;
      left: 0;
      right: 0;
      padding: 20rpx 30rpx;
      background: linear-gradient(to top, rgba(0,0,0,0.7), transparent);

      .work-title {
        color: #ffffff;
        font-size: 30rpx;
        font-weight: bold;
      }
    }
  }

  // 音频作品
  .audio-work {
    width: 100%;

    .audio-header {
      display: flex;
      align-items: flex-start;
      margin-bottom: 20rpx;

      .audio-icon {
        font-size: 40rpx;
        margin-right: 20rpx;
      }

      .audio-info {
        flex: 1;

        .work-title {
          font-size: 30rpx;
          font-weight: bold;
          color: var(--theme-text);
          margin-bottom: 8rpx;
        }

        .work-desc {
          display: block;
          font-size: 24rpx;
          color: var(--theme-text-secondary);
          margin-bottom: 8rpx;
          line-height: 1.4;
        }

        .audio-duration {
          display: block;
          font-size: 22rpx;
          color: var(--theme-primary);
        }
      }
    }

    .audio-player {
      background: var(--theme-background);
      padding: 20rpx;
      border-radius: 15rpx;

      .player-controls {
        display: flex;
        align-items: center;
        gap: 15rpx;
        margin-bottom: 15rpx;

        .play-btn {
          width: 60rpx;
          height: 60rpx;
          background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
          border-radius: 50%;
          display: flex;
          align-items: center;
          justify-content: center;
          flex-shrink: 0;
          box-shadow: 0 4rpx 12rpx rgba(102, 126, 234, 0.3);
          transition: all 0.3s;

          &:active {
            transform: scale(0.95);
          }

          .control-icon {
            font-size: 28rpx;
            color: #ffffff;
          }
        }

        .time-display {
          flex: 1;
          display: flex;
          align-items: center;
          gap: 8rpx;
          font-size: 22rpx;
          color: var(--theme-text-secondary);

          .current-time {
            color: var(--theme-primary);
            font-weight: bold;
          }

          .time-separator {
            color: #cccccc;
          }

          .total-time {
            color: var(--theme-text-secondary);
          }
        }
      }

      .audio-progress-bar {
        width: 100%;
        padding: 10rpx 0;
        margin-bottom: 15rpx;

        .progress-track {
          position: relative;
          width: 100%;
          height: 8rpx;
          background: #e0e0e0;
          border-radius: 4rpx;
          overflow: visible;

          .progress-fill {
            height: 100%;
            background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
            border-radius: 4rpx;
            transition: width 0.1s;
          }

          .progress-thumb {
            position: absolute;
            top: 50%;
            transform: translate(-50%, -50%);
            width: 16rpx;
            height: 16rpx;
            background: var(--theme-surface);
            border: 3rpx solid #667eea;
            border-radius: 50%;
            box-shadow: 0 2rpx 8rpx rgba(102, 126, 234, 0.4);
            transition: left 0.1s;
          }
        }
      }

      .audio-controls {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 20rpx;

        .speed-control {
          display: flex;
          align-items: center;
          gap: 8rpx;
          padding: 8rpx 15rpx;
          background: var(--theme-surface);
          border-radius: 20rpx;
          border: 1rpx solid #e0e0e0;

          .control-label {
            font-size: 22rpx;
            color: var(--theme-text-secondary);
          }

          .speed-value {
            font-size: 22rpx;
            color: var(--theme-primary);
            font-weight: bold;
            min-width: 50rpx;
            text-align: center;
          }
        }

        .volume-control {
          flex: 1;
          display: flex;
          align-items: center;
          gap: 10rpx;
          background: var(--theme-surface);
          border-radius: 20rpx;
          padding: 8rpx 15rpx;
          border: 1rpx solid #e0e0e0;

          .control-icon {
            font-size: 20rpx;
            flex-shrink: 0;
          }

          .volume-slider {
            flex: 1;
            margin: 0;
          }
        }
      }
    }
  }

  // 视频作品
  .video-work {
    width: 100%;

    .video-player {
      width: 100%;
      height: 400rpx;
      border-radius: 15rpx;
      overflow: hidden;
      background: #000000;
      box-shadow: 0 4rpx 16rpx rgba(0, 0, 0, 0.15);
    }

    .video-info {
      margin-top: 15rpx;
      padding: 15rpx 20rpx;
      background: var(--theme-background);
      border-radius: 15rpx;

      .video-meta {
        margin-bottom: 12rpx;

        .work-title {
          display: block;
          font-size: 30rpx;
          font-weight: bold;
          color: var(--theme-text);
          margin-bottom: 8rpx;
        }

        .work-desc {
          display: block;
          font-size: 24rpx;
          color: var(--theme-text-secondary);
          line-height: 1.4;
        }
      }

      .video-stats {
        display: flex;
        gap: 20rpx;
        padding-top: 12rpx;
        border-top: 1rpx solid #e0e0e0;

        .stat-item {
          display: flex;
          align-items: center;
          gap: 6rpx;
          font-size: 22rpx;

          .stat-icon {
            font-size: 20rpx;
          }

          .stat-value {
            color: var(--theme-primary);
            font-weight: bold;
          }
        }
      }
    }
  }
}

.empty-state {
  text-align: center;
  padding: 120rpx 30rpx;

  .empty-icon {
    font-size: 140rpx;
    margin-bottom: 35rpx;
    filter: drop-shadow(0 0 40rpx rgba(138, 92, 246, 0.8));
    animation: emptyFloat 3s ease-in-out infinite;
  }

  .empty-text {
    display: block;
    font-size: 36rpx;
    @include neon-title(#8B5CF6);
    margin-bottom: 15rpx;
  }

  .empty-hint {
    display: block;
    font-size: 28rpx;
    @include neon-text(#00D9FF);
  }
}

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
  width: 620rpx;
  max-height: 85vh;
  overflow-y: auto;
  @include neon-card;
  border-radius: 30rpx;
  padding: 45rpx;
  border: 3rpx solid rgba(255, 214, 0, 0.6);
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.7),
    0 0 80rpx rgba(255, 214, 0, 0.5),
    inset 0 0 60rpx rgba(255, 214, 0, 0.15);
  animation: scaleIn 0.4s cubic-bezier(0.4, 0, 0.2, 1);

  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 35rpx;

    .modal-title {
      font-size: 40rpx;
      font-weight: bold;
      @include neon-title(#FFD600);
    }

    .modal-close {
      font-size: 44rpx;
      @include neon-text(#FF00D6);
      cursor: pointer;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      
      &:active {
        transform: rotate(90deg) scale(1.2);
        filter: drop-shadow(0 0 20rpx rgba(255, 0, 214, 1));
      }
    }
  }

  .modal-textarea {
    @include neon-input;
    width: 100%;
    min-height: 220rpx;
    border-radius: 18rpx;
    padding: 25rpx;
    font-size: 28rpx;
    line-height: 1.8;
    color: #ffffff;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    
    &:focus {
      border-color: rgba(255, 214, 0, 0.8);
      box-shadow: 
        0 0 40rpx rgba(255, 214, 0, 0.5),
        inset 0 0 30rpx rgba(255, 214, 0, 0.1);
      transform: translateY(-2rpx);
    }
  }

  .char-count {
    display: block;
    text-align: right;
    font-size: 22rpx;
    color: var(--theme-text-secondary);
    margin-top: 10rpx;
  }

  .modal-form {
    max-height: 60vh;
    overflow-y: auto;

    .form-item {
      margin-bottom: 25rpx;

      .form-label {
        display: block;
        font-size: 26rpx;
        color: var(--theme-text-secondary);
        margin-bottom: 10rpx;
      }

      .form-input {
        @include neon-input;
        width: 100%;
        height: 80rpx;
        border-radius: 18rpx;
        padding: 0 25rpx;
        font-size: 28rpx;
        color: #ffffff;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        
        &:focus {
          border-color: rgba(255, 214, 0, 0.8);
          box-shadow: 
            0 0 40rpx rgba(255, 214, 0, 0.5),
            inset 0 0 30rpx rgba(255, 214, 0, 0.1);
        }
      }

      .form-textarea {
        @include neon-input;
        width: 100%;
        min-height: 160rpx;
        border-radius: 18rpx;
        padding: 25rpx;
        font-size: 28rpx;
        line-height: 1.8;
        color: #ffffff;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        
        &:focus {
          border-color: rgba(255, 214, 0, 0.8);
          box-shadow: 
            0 0 40rpx rgba(255, 214, 0, 0.5),
            inset 0 0 30rpx rgba(255, 214, 0, 0.1);
        }
      }

      // 作品类型选择
      .work-type-selector {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 15rpx;

        .type-option {
          padding: 22rpx;
          background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
          backdrop-filter: blur(10rpx);
          border-radius: 18rpx;
          text-align: center;
          font-size: 28rpx;
          color: #b8c5d6;
          border: 2rpx solid rgba(138, 92, 246, 0.4);
          transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
          cursor: pointer;

          &.active {
            background: linear-gradient(135deg, rgba(255, 214, 0, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
            @include neon-text(#ffffff);
            border-color: rgba(255, 214, 0, 0.8);
            box-shadow: 
              0 6rpx 20rpx rgba(255, 214, 0, 0.5),
              0 0 30rpx rgba(255, 214, 0, 0.4);
            transform: scale(1.05);
          }
        }
      }

      // 上传区域
      .upload-area {
        width: 100%;
        min-height: 200rpx;
        background: var(--theme-background);
        border-radius: 15rpx;
        border: 2rpx dashed #d0d0d0;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        overflow: hidden;
        position: relative;

        &.small {
          min-height: 150rpx;
        }

        .upload-placeholder {
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          padding: 30rpx;

          &.small {
            padding: 20rpx;
          }

          .upload-icon {
            font-size: 60rpx;
            margin-bottom: 10rpx;
          }

          .upload-text {
            font-size: 24rpx;
            color: var(--theme-text-secondary);
          }
        }

        .preview-image {
          width: 100%;
          max-width: 100%;
          height: auto;
          min-height: 150rpx;
          display: block;
        }

        .file-info {
          display: flex;
          align-items: center;
          gap: 15rpx;
          padding: 20rpx;

          .file-icon {
            font-size: 50rpx;
          }

          .file-name {
            font-size: 26rpx;
            color: var(--theme-text);
          }
        }
      }

      .form-subitem {
        margin-top: 20rpx;
      }
    }
  }

  .modal-actions {
    margin-top: 40rpx;

    .modal-btn {
      width: 100%;
      height: 95rpx;
      background: linear-gradient(135deg, rgba(255, 214, 0, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border-radius: 50rpx;
      font-size: 34rpx;
      font-weight: bold;
      line-height: 95rpx;
      border: 3rpx solid rgba(255, 214, 0, 0.7);
      box-shadow: 
        0 10rpx 40rpx rgba(255, 214, 0, 0.5),
        0 0 60rpx rgba(255, 214, 0, 0.4),
        inset 0 0 40rpx rgba(255, 214, 0, 0.2);
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
        transform: scale(0.96);
        box-shadow: 
          0 8rpx 30rpx rgba(255, 214, 0, 0.7),
          0 0 80rpx rgba(255, 214, 0, 0.6),
          inset 0 0 50rpx rgba(255, 214, 0, 0.3);
      }
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes bgRotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes avatarFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10rpx);
  }
}

@keyframes avatarPulse {
  0%, 100% {
    box-shadow: 
      0 0 30rpx rgba(255, 214, 0, 0.6),
      inset 0 0 30rpx rgba(255, 214, 0, 0.15);
  }
  50% {
    box-shadow: 
      0 0 50rpx rgba(255, 214, 0, 1),
      inset 0 0 50rpx rgba(255, 214, 0, 0.25);
  }
}

@keyframes iconPulse {
  0%, 100% {
    box-shadow: 
      0 4rpx 12rpx rgba(0, 217, 255, 0.4),
      0 0 20rpx rgba(0, 217, 255, 0.3);
  }
  50% {
    box-shadow: 
      0 6rpx 16rpx rgba(0, 217, 255, 0.6),
      0 0 40rpx rgba(0, 217, 255, 0.5);
  }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 20rpx rgba(255, 214, 0, 1),
      0 0 40rpx rgba(255, 214, 0, 0.8);
  }
  50% {
    text-shadow: 
      0 0 30rpx rgba(255, 214, 0, 1),
      0 0 60rpx rgba(255, 214, 0, 1),
      0 0 90rpx rgba(255, 214, 0, 0.6);
  }
}

@keyframes tabGlow {
  0%, 100% {
    box-shadow: 
      0 0 15rpx rgba(255, 214, 0, 0.6),
      0 -4rpx 8rpx rgba(255, 214, 0, 0.4);
  }
  50% {
    box-shadow: 
      0 0 25rpx rgba(255, 214, 0, 1),
      0 -6rpx 12rpx rgba(255, 214, 0, 0.7);
  }
}

@keyframes itemSlideIn {
  from {
    opacity: 0;
    transform: translateX(-30rpx);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes iconFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-8rpx);
  }
}

@keyframes emptyFloat {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-25rpx) rotate(10deg);
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
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

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
