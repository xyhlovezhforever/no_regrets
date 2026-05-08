<template>
  <view class="create-page">
    <!-- 类型选择 -->
    <view class="type-selector">
      <view
        v-for="type in postTypes"
        :key="type.value"
        class="type-item"
        :class="{ active: form.post_type === type.value }"
        @click="form.post_type = type.value"
      >
        <text class="type-icon">{{ type.icon }}</text>
        <text class="type-label">{{ type.label }}</text>
      </view>
    </view>

    <!-- 表单内容 -->
    <scroll-view class="form-content" scroll-y :scroll-top="scrollTop" @scroll="handleScroll">
      <!-- 灵感卡片模板 -->
      <view v-if="form.post_type === 'inspiration'" class="form-section">
        <view class="form-item">
          <text class="label">卡片类型</text>
          <view class="radio-group">
            <view
              class="radio-item"
              :class="{ active: form.card_category === 'encourage' }"
              @click="form.card_category = 'encourage'"
            >
              <text>💪 鼓励卡片</text>
            </view>
            <view
              class="radio-item"
              :class="{ active: form.card_category === 'philosophy' }"
              @click="form.card_category = 'philosophy'"
            >
              <text>📚 哲理卡片</text>
            </view>
          </view>
        </view>

        <view class="form-item">
          <text class="label">标题 <text class="required">*</text></text>
          <input
            v-model="form.title"
            class="input input-large"
            placeholder="输入卡片标题"
            maxlength="50"
          />
        </view>

        <view class="form-item">
          <text class="label">内容 <text class="required">*</text></text>
          <textarea
            v-model="form.content"
            class="textarea textarea-large"
            placeholder="输入卡片内容..."
            maxlength="500"
          />
          <text class="char-count">{{ form.content.length }}/500</text>
        </view>

        <view v-if="form.card_category === 'philosophy'" class="form-item">
          <text class="label">作者</text>
          <input
            v-model="form.author"
            class="input input-large"
            placeholder="输入作者名称（可选）"
            maxlength="50"
          />
        </view>
      </view>

      <!-- 话题模板 -->
      <view v-if="form.post_type === 'topic'" class="form-section">
        <view class="form-item">
          <text class="label">标题 <text class="required">*</text></text>
          <input
            v-model="form.title"
            class="input input-large"
            placeholder="给话题起个标题"
            maxlength="50"
          />
        </view>

        <view class="form-item">
          <text class="label">话题内容 <text class="required">*</text></text>
          <textarea
            v-model="form.content"
            class="textarea textarea-large"
            placeholder="输入你想讨论的话题内容..."
            maxlength="2000"
          />
          <text class="char-count">{{ form.content.length }}/2000</text>
        </view>

        <view class="form-item">
          <text class="label">我的想法（可选）</text>
          <textarea
            v-model="form.topic_answer"
            class="textarea textarea-large"
            placeholder="分享你的想法..."
            maxlength="500"
          />
          <text class="char-count">{{ form.topic_answer?.length || 0 }}/500</text>
        </view>
      </view>

      <!-- 创作模板 -->
      <view v-if="form.post_type === 'creation'" class="form-section">
        <view class="form-item">
          <text class="label">标题 <text class="required">*</text></text>
          <input
            v-model="form.title"
            class="input input-large"
            placeholder="给作品起个标题"
            maxlength="50"
          />
        </view>

        <view class="form-item">
          <text class="label">分类 <text class="required">*</text></text>
          <view class="category-list">
            <view
              v-for="cat in creationCategories"
              :key="cat"
              class="category-tag"
              :class="{ active: form.creation_category === cat }"
              @click="form.creation_category = cat"
            >
              {{ cat }}
            </view>
          </view>
        </view>

        <view class="form-item">
          <text class="label">内容 <text class="required">*</text></text>
          <textarea
            v-model="form.content"
            class="textarea textarea-large"
            placeholder="开始你的创作吧..."
            maxlength="5000"
          />
          <text class="char-count">{{ form.content.length }}/5000</text>
        </view>

        <!-- 常用标签 -->
        <view class="form-item">
          <text class="label">常用标签（可选）</text>
          <view class="preset-tags">
            <view class="preset-tag-list">
              <view
                v-for="tag in presetTags"
                :key="tag"
                class="preset-tag"
                :class="{ active: form.creation_tags?.includes(tag) }"
                @click="togglePresetTag(tag)"
              >
                #{{ tag }}
              </view>
            </view>
          </view>
        </view>

        <!-- 已添加的标签 -->
        <view v-if="form.creation_tags && form.creation_tags.length > 0" class="form-item">
          <text class="label">已添加的标签</text>
          <view class="selected-tags">
            <view class="selected-tags-list">
              <view
                v-for="(tag, index) in form.creation_tags"
                :key="index"
                class="selected-tag-item"
              >
                <text class="tag-text">#{{ tag }}</text>
                <text class="tag-remove" @click="removeTag(index)">×</text>
              </view>
            </view>
          </view>
        </view>
        
        <!-- 自定义标签输入 -->
        <view class="form-item">
          <text class="label">自定义标签</text>
          <view class="tag-input-wrapper">
            <input
              v-model="tagInput"
              class="tag-input"
              type="text"
              placeholder="输入自定义标签"
              placeholder-class="input-placeholder"
              @confirm="addTag"
              @focus="handleInputFocus"
              @blur="handleInputBlur"
              confirm-type="done"
              maxlength="10"
              cursor-spacing="50"
              adjust-position
            />
            <view 
              class="add-tag-btn" 
              :class="{ disabled: !tagInput.trim() }"
              @click="addTag"
            >
              <text>+ 添加</text>
            </view>
          </view>
        </view>
      </view>

      <!-- 底部安全区域 -->
      <view class="bottom-safe-area"></view>
    </scroll-view>

    <!-- 底部操作栏 -->
    <view class="action-bar">
      <button class="btn cancel-btn" @click="handleCancel">取消</button>
      <button class="btn publish-btn" @click="handlePublish">发布</button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { createPostApi } from '@/api/forum'

const postTypes = [
  { value: 'inspiration', icon: '✨', label: '灵感' },
  { value: 'topic', icon: '💭', label: '话题' },
  { value: 'creation', icon: '✍️', label: '创作' }
]

const creationCategories = ['随笔', '日记', '诗歌', '小说', '感悟', '其他']

const presetTags = ['心情', '生活', '感悟', '思考', '成长', '梦想', '回忆', '未来']

const form = ref({
  post_type: 'inspiration' as 'inspiration' | 'topic' | 'creation',
  title: '',
  content: '',
  card_category: 'encourage' as 'encourage' | 'philosophy' | undefined,
  author: '',
  topic_answer: '',
  creation_category: '随笔',
  creation_tags: [] as string[],
  image_url: ''
})

const tagInput = ref('')
const scrollTop = ref(0)

onMounted(() => {
  const pages = getCurrentPages()
  const currentPage = pages[pages.length - 1]
  const type = (currentPage as any).options?.type
  if (type && ['inspiration', 'topic', 'creation'].includes(type)) {
    form.value.post_type = type as any
  }
})

const handleScroll = (e: any) => {
  scrollTop.value = e.detail.scrollTop
}

const togglePresetTag = (tag: string) => {
  if (!form.value.creation_tags) {
    form.value.creation_tags = []
  }
  const index = form.value.creation_tags.indexOf(tag)
  if (index > -1) {
    form.value.creation_tags.splice(index, 1)
  } else {
    form.value.creation_tags.push(tag)
  }
}

const addTag = () => {
  const tag = tagInput.value.trim()
  if (!tag) return
  
  if (!form.value.creation_tags) {
    form.value.creation_tags = []
  }
  
  // 检查标签是否已存在
  if (!form.value.creation_tags.includes(tag)) {
    form.value.creation_tags.push(tag)
    tagInput.value = ''
    uni.showToast({ title: '标签已添加', icon: 'success', duration: 1000 })
  } else {
    uni.showToast({ title: '标签已存在', icon: 'none', duration: 1000 })
    tagInput.value = ''
  }
}

const removeTag = (index: number) => {
  form.value.creation_tags?.splice(index, 1)
}

const handleInputFocus = () => {
  console.log('输入框获得焦点')
}

const handleInputBlur = () => {
  console.log('输入框失去焦点')
}

const validateForm = (): boolean => {
  if (!form.value.title || !form.value.content) {
    uni.showToast({ title: '请填写标题和内容', icon: 'none' })
    return false
  }

  if (form.value.post_type === 'creation' && !form.value.creation_category) {
    uni.showToast({ title: '请选择分类', icon: 'none' })
    return false
  }

  return true
}

const handlePublish = async () => {
  if (!validateForm()) return

  try {
    const data: any = {
      post_type: form.value.post_type,
      title: form.value.title,
      content: form.value.content
    }

    if (form.value.post_type === 'inspiration') {
      data.card_category = form.value.card_category
      if (form.value.card_category === 'philosophy' && form.value.author) {
        data.author = form.value.author
      }
    }

    if (form.value.post_type === 'topic') {
      // 话题类型只需要标题、内容和可选的回答
      if (form.value.topic_answer) {
        data.topic_answer = form.value.topic_answer
      }
    }

    if (form.value.post_type === 'creation') {
      data.creation_category = form.value.creation_category
      if (form.value.creation_tags && form.value.creation_tags.length > 0) {
        data.creation_tags = form.value.creation_tags
      }
    }

    await createPostApi(data)
    uni.showToast({ title: '发布成功', icon: 'success' })
    
    // 触发列表刷新事件
    uni.$emit('refreshForumList', {
      post_type: form.value.post_type
    })
    
    setTimeout(() => {
      uni.navigateBack()
    }, 1500)
  } catch (error: any) {
    console.error('发布失败:', error)
    uni.showToast({
      title: error?.message || '发布失败',
      icon: 'none'
    })
  }
}

const handleCancel = () => {
  uni.navigateBack()
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.create-page {
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
      radial-gradient(circle at 30% 20%, rgba(255, 0, 214, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 70% 80%, rgba(0, 217, 255, 0.15) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.type-selector {
  position: sticky;
  top: 0;
  z-index: 100;
  @include neon-card;
  background: linear-gradient(180deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 0.95) 100%);
  backdrop-filter: blur(30rpx);
  display: flex;
  padding: 25rpx 20rpx;
  gap: 18rpx;
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(255, 0, 214, 0.4),
    inset 0 2rpx 0 rgba(255, 0, 214, 0.3);
  border-bottom: 3rpx solid rgba(255, 0, 214, 0.5);

  .type-item {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 28rpx 20rpx;
    border-radius: 25rpx;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
    backdrop-filter: blur(10rpx);
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    border: 2rpx solid rgba(138, 92, 246, 0.4);
    box-shadow: 0 6rpx 18rpx rgba(0, 0, 0, 0.3);

    .type-icon {
      font-size: 52rpx;
      margin-bottom: 14rpx;
      filter: drop-shadow(0 0 10rpx rgba(255, 255, 255, 0.3));
      transition: all 0.3s;
    }

    .type-label {
      font-size: 26rpx;
      @include neon-text(#b8c5d6);
      font-weight: 500;
    }

    &.active {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      box-shadow: 
        0 10rpx 30rpx rgba(255, 0, 214, 0.5),
        0 0 50rpx rgba(255, 0, 214, 0.6);
      border-color: rgba(255, 0, 214, 0.8);
      transform: translateY(-4rpx) scale(1.05);
      animation: typeGlow 2s ease-in-out infinite;

      .type-icon {
        filter: drop-shadow(0 0 20rpx rgba(255, 255, 255, 1));
        animation: iconBounce 2s ease-in-out infinite;
      }

      .type-label {
        @include neon-text(#ffffff);
        font-weight: bold;
        text-shadow: 
          0 0 15rpx rgba(255, 255, 255, 0.8),
          0 2rpx 5rpx rgba(0, 0, 0, 0.3);
      }
    }
    
    &:active {
      transform: scale(0.95);
    }
  }
}

.form-content {
  flex: 1;
  padding: 30rpx;
  padding-bottom: 200rpx;
  position: relative;
  z-index: 1;
}

.form-section {
  .form-item {
    margin-bottom: 50rpx;

    .label {
      display: block;
      font-size: 32rpx;
      @include neon-title(#00D9FF);
      margin-bottom: 22rpx;
      font-weight: bold;

      .required {
        @include neon-text(#FF004F);
        margin-left: 6rpx;
      }
    }

    .input,
    .textarea {
      @include neon-input;
      width: 100%;
      padding: 30rpx;
      border-radius: 25rpx;
      font-size: 30rpx;
      @include neon-text(#ffffff);
      box-sizing: border-box;
      transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
      text-shadow: 
        0 1rpx 3rpx rgba(0, 0, 0, 0.3),
        0 0 10rpx rgba(255, 255, 255, 0.1);
    }

    .input:focus,
    .textarea:focus {
      border-color: rgba(0, 217, 255, 0.8);
      box-shadow: 
        0 0 50rpx rgba(0, 217, 255, 0.5),
        inset 0 0 30rpx rgba(0, 217, 255, 0.1);
    }

    .input-large {
      height: 90rpx;
      font-size: 32rpx;
      padding: 28rpx 30rpx;
    }

    .textarea {
      min-height: 250rpx;
      line-height: 1.85;
    }

    .textarea-large {
      min-height: 300rpx;
      font-size: 30rpx;
      line-height: 1.85;
    }

    .char-count {
      display: block;
      text-align: right;
      font-size: 26rpx;
      @include neon-text(#FFD600);
      margin-top: 15rpx;
      font-weight: 600;
    }
  }
}

.radio-group {
  display: flex;
  gap: 22rpx;

  .radio-item {
    flex: 1;
    padding: 30rpx 22rpx;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
    backdrop-filter: blur(10rpx);
    border-radius: 25rpx;
    text-align: center;
    font-size: 30rpx;
    @include neon-text(#b8c5d6);
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    border: 2rpx solid rgba(138, 92, 246, 0.4);
    font-weight: 600;
    box-shadow: 0 6rpx 18rpx rgba(0, 0, 0, 0.3);

    &.active {
      background: linear-gradient(135deg, rgba(138, 92, 246, 0.9) 0%, rgba(0, 217, 255, 0.9) 100%);
      @include neon-text(#ffffff);
      border-color: rgba(138, 92, 246, 0.8);
      box-shadow: 
        0 10rpx 30rpx rgba(138, 92, 246, 0.5),
        0 0 50rpx rgba(138, 92, 246, 0.6);
      text-shadow: 
        0 0 15rpx rgba(255, 255, 255, 0.8),
        0 2rpx 5rpx rgba(0, 0, 0, 0.3);
      transform: scale(1.05);
    }
    
    &:active {
      transform: scale(0.95);
    }
  }
}

.preset-tags {
  .preset-tag-list {
    display: flex;
    flex-wrap: wrap;
    gap: 18rpx;
  }

  .preset-tag {
    padding: 20rpx 35rpx;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.5) 0%, rgba(30, 36, 66, 0.3) 100%);
    backdrop-filter: blur(10rpx);
    border-radius: 35rpx;
    font-size: 28rpx;
    @include neon-text(#b8c5d6);
    border: 2rpx solid rgba(0, 217, 255, 0.4);
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    font-weight: 600;
    box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.3);

    &.active {
      background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border-color: rgba(0, 217, 255, 0.8);
      box-shadow: 
        0 8rpx 24rpx rgba(0, 217, 255, 0.5),
        0 0 40rpx rgba(0, 217, 255, 0.6);
      text-shadow: 
        0 0 12rpx rgba(255, 255, 255, 0.8),
        0 2rpx 5rpx rgba(0, 0, 0, 0.3);
      transform: scale(1.08);
    }

    &:active {
      transform: scale(0.92);
    }
  }
}

.options-list {
  .option-item {
    display: flex;
    align-items: center;
    margin-bottom: 20rpx;
    gap: 15rpx;

    .option-input {
      flex: 1;
      padding: 28rpx;
      background: var(--theme-surface);
      border-radius: 20rpx;
      font-size: 30rpx;
      border: 2rpx solid #e8e8f0;
      transition: all 0.3s;
    }

    .option-input-large {
      height: 90rpx;
      font-size: 32rpx;
      padding: 28rpx 30rpx;
    }

    .option-input:focus {
      border-color: var(--theme-primary);
      box-shadow: 0 0 0 4rpx rgba(102, 126, 234, 0.1);
    }

    .remove-btn {
      width: 60rpx;
      height: 60rpx;
      display: flex;
      align-items: center;
      justify-content: center;
      background: #ff4444;
      color: #ffffff;
      border-radius: 50%;
      font-size: 36rpx;
      font-weight: bold;
      box-shadow: 0 4rpx 12rpx rgba(255, 68, 68, 0.3);
    }
  }

  .add-option-btn {
    padding: 28rpx;
    background: var(--theme-surface);
    border-radius: 20rpx;
    text-align: center;
    font-size: 30rpx;
    color: var(--theme-primary);
    border: 2rpx dashed #667eea;
    font-weight: 500;
    transition: all 0.3s;

    &:active {
      background: #f8f9ff;
    }
  }
}

.category-list {
  display: flex;
  flex-wrap: wrap;
  gap: 18rpx;

  .category-tag {
    padding: 22rpx 38rpx;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.5) 0%, rgba(30, 36, 66, 0.3) 100%);
    backdrop-filter: blur(10rpx);
    border-radius: 50rpx;
    font-size: 28rpx;
    @include neon-text(#b8c5d6);
    border: 2rpx solid rgba(255, 0, 214, 0.4);
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    font-weight: 600;
    box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.3);

    &.active {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border-color: rgba(255, 0, 214, 0.8);
      box-shadow: 
        0 8rpx 24rpx rgba(255, 0, 214, 0.5),
        0 0 40rpx rgba(255, 0, 214, 0.6);
      text-shadow: 
        0 0 12rpx rgba(255, 255, 255, 0.8),
        0 2rpx 5rpx rgba(0, 0, 0, 0.3);
      transform: scale(1.08);
    }
    
    &:active {
      transform: scale(0.92);
    }
  }
}

// 已添加的标签样式
.selected-tags {
  padding: 28rpx;
  background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(20, 26, 56, 0.4) 100%);
  backdrop-filter: blur(15rpx);
  border-radius: 25rpx;
  border: 2rpx solid rgba(0, 217, 255, 0.4);
  box-shadow: 
    0 8rpx 24rpx rgba(0, 0, 0, 0.4),
    0 0 40rpx rgba(0, 217, 255, 0.2);

  .selected-tags-list {
    display: flex;
    flex-wrap: wrap;
    gap: 18rpx;

    .selected-tag-item {
      display: flex;
      align-items: center;
      gap: 14rpx;
      padding: 16rpx 26rpx;
      background: linear-gradient(135deg, rgba(0, 217, 255, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
      backdrop-filter: blur(10rpx);
      border-radius: 35rpx;
      border: 2rpx solid rgba(0, 217, 255, 0.6);
      box-shadow: 
        0 6rpx 18rpx rgba(0, 0, 0, 0.3),
        0 0 30rpx rgba(0, 217, 255, 0.3);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      animation: tagFadeIn 0.4s ease-out;

      .tag-text {
        font-size: 28rpx;
        @include neon-text(#ffffff);
        font-weight: 600;
      }

      .tag-remove {
        width: 38rpx;
        height: 38rpx;
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(135deg, rgba(255, 0, 79, 0.8) 0%, rgba(255, 0, 214, 0.8) 100%);
        @include neon-text(#ffffff);
        border-radius: 50%;
        font-size: 26rpx;
        font-weight: bold;
        transition: all 0.3s;
        box-shadow: 0 0 20rpx rgba(255, 0, 79, 0.6);

        &:active {
          transform: scale(0.85);
          box-shadow: 0 0 30rpx rgba(255, 0, 79, 1);
        }
      }

      &:active {
        transform: translateY(-3rpx);
        box-shadow: 
          0 8rpx 24rpx rgba(0, 0, 0, 0.4),
          0 0 50rpx rgba(0, 217, 255, 0.5);
      }
    }
  }
}

.tag-input-wrapper {
  display: flex;
  gap: 18rpx;
  align-items: stretch;
  padding: 18rpx;
  background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(20, 26, 56, 0.4) 100%);
  backdrop-filter: blur(15rpx);
  border-radius: 25rpx;
  border: 2rpx solid rgba(138, 92, 246, 0.4);
  box-shadow: 
    0 8rpx 24rpx rgba(0, 0, 0, 0.4),
    0 0 40rpx rgba(138, 92, 246, 0.2);

  .tag-input {
    flex: 1;
    height: 80rpx;
    line-height: 80rpx;
    padding: 0 28rpx;
    @include neon-input;
    border-radius: 20rpx;
    font-size: 30rpx;
    @include neon-text(#ffffff);
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    box-sizing: border-box;

    &:focus {
      border-color: rgba(138, 92, 246, 0.8);
      box-shadow: 
        0 0 50rpx rgba(138, 92, 246, 0.5),
        inset 0 0 30rpx rgba(138, 92, 246, 0.1);
    }
  }

  .input-placeholder {
    color: rgba(255, 255, 255, 0.3);
    font-size: 28rpx;
  }

  .add-tag-btn {
    height: 80rpx;
    padding: 0 48rpx;
    background: linear-gradient(135deg, rgba(138, 92, 246, 0.9) 0%, rgba(0, 217, 255, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(138, 92, 246, 0.7);
    border-radius: 20rpx;
    font-size: 30rpx;
    font-weight: bold;
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 
      0 8rpx 24rpx rgba(138, 92, 246, 0.5),
      0 0 40rpx rgba(138, 92, 246, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    text-shadow: 
      0 0 12rpx rgba(255, 255, 255, 0.8),
      0 2rpx 5rpx rgba(0, 0, 0, 0.3);

    &:active {
      transform: scale(0.95);
      box-shadow: 
        0 6rpx 18rpx rgba(138, 92, 246, 0.6),
        0 0 60rpx rgba(138, 92, 246, 0.6);
    }

    &.disabled {
      background: linear-gradient(135deg, rgba(100, 100, 100, 0.4) 0%, rgba(80, 80, 80, 0.4) 100%);
      @include neon-text(#6b7b93);
      border-color: rgba(100, 100, 100, 0.4);
      box-shadow: none;
      text-shadow: none;
      
      &:active {
        transform: none;
      }
    }
  }
}

.bottom-safe-area {
  height: 250rpx;
}

.action-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 28rpx 30rpx;
  padding-bottom: calc(28rpx + env(safe-area-inset-bottom));
  @include neon-card;
  background: linear-gradient(180deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 1) 100%);
  backdrop-filter: blur(30rpx);
  box-shadow: 
    0 -15rpx 50rpx rgba(0, 0, 0, 0.7),
    0 0 80rpx rgba(255, 0, 214, 0.4),
    inset 0 2rpx 0 rgba(255, 0, 214, 0.3);
  border-top: 3rpx solid rgba(255, 0, 214, 0.6);
  display: flex;
  gap: 25rpx;
  z-index: 99;

  .btn {
    flex: 1;
    height: 95rpx;
    border-radius: 50rpx;
    font-size: 34rpx;
    font-weight: bold;
    border: none;
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);

    &.cancel-btn {
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
      backdrop-filter: blur(10rpx);
      @include neon-text(#b8c5d6);
      border: 2rpx solid rgba(138, 92, 246, 0.5);
      box-shadow: 0 6rpx 18rpx rgba(0, 0, 0, 0.4);

      &:active {
        transform: scale(0.95);
        box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.5);
      }
    }

    &.publish-btn {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border: 2rpx solid rgba(255, 0, 214, 0.7);
      box-shadow: 
        0 10rpx 40rpx rgba(255, 0, 214, 0.6),
        0 0 60rpx rgba(255, 0, 214, 0.5);
      text-shadow: 
        0 0 15rpx rgba(255, 255, 255, 0.8),
        0 2rpx 5rpx rgba(0, 0, 0, 0.3);
      animation: btnPulse 3s ease-in-out infinite;

      &:active {
        animation: none;
        transform: scale(0.95);
        box-shadow: 
          0 12rpx 45rpx rgba(255, 0, 214, 0.8),
          0 0 80rpx rgba(255, 0, 214, 0.7);
      }
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes typeGlow {
  0%, 100% {
    box-shadow: 
      0 10rpx 30rpx rgba(255, 0, 214, 0.5),
      0 0 50rpx rgba(255, 0, 214, 0.6);
  }
  50% {
    box-shadow: 
      0 12rpx 35rpx rgba(255, 0, 214, 0.7),
      0 0 70rpx rgba(255, 0, 214, 0.8);
  }
}

@keyframes iconBounce {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

@keyframes tagFadeIn {
  from {
    opacity: 0;
    transform: scale(0.8);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(255, 0, 214, 0.6),
      0 0 60rpx rgba(255, 0, 214, 0.5);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(255, 0, 214, 0.8),
      0 0 80rpx rgba(255, 0, 214, 0.7);
  }
}
</style>
