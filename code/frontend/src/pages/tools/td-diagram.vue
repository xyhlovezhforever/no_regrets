<template>
  <view class="td-diagram-page">
    <view class="header">
      <text class="title">📊 TD流程图生成器</text>
      <text class="subtitle">快速生成技术文档流程图</text>
    </view>

    <view class="form-section">
      <view class="form-item">
        <text class="form-label">流程图标题</text>
        <input 
          v-model="diagramTitle" 
          class="form-input" 
          placeholder="请输入流程图标题"
          maxlength="50"
        />
      </view>

      <view class="form-item">
        <text class="form-label">流程类型</text>
        <picker :range="diagramTypes" :value="selectedTypeIndex" @change="onTypeChange">
          <view class="picker-value">{{ diagramTypes[selectedTypeIndex] }}</view>
        </picker>
      </view>

      <view class="form-item">
        <text class="form-label">流程步骤</text>
        <view class="steps-container">
          <view 
            v-for="(step, index) in steps" 
            :key="index" 
            class="step-item"
          >
            <input 
              v-model="step.text" 
              class="step-input" 
              :placeholder="`步骤 ${index + 1}`"
              maxlength="30"
            />
            <text class="step-delete" @click="removeStep(index)">×</text>
          </view>
          <button class="add-step-btn" @click="addStep">+ 添加步骤</button>
        </view>
      </view>

      <view class="form-item">
        <text class="form-label">样式主题</text>
        <view class="theme-grid">
          <view 
            v-for="(theme, index) in themes" 
            :key="index"
            class="theme-item"
            :class="{ active: selectedThemeIndex === index }"
            @click="selectedThemeIndex = index"
          >
            <view class="theme-preview" :style="{ background: theme.color }"></view>
            <text class="theme-name">{{ theme.name }}</text>
          </view>
        </view>
      </view>
    </view>

    <view class="action-section">
      <button class="action-btn primary" @click="generateDiagram" :disabled="generating">
        <text v-if="!generating">🎨 生成流程图</text>
        <text v-else>生成中...</text>
      </button>
    </view>

    <view v-if="diagramImage" class="result-section">
      <view class="result-header">
        <text class="result-title">✅ 生成完成</text>
      </view>
      <image :src="diagramImage" class="diagram-image" mode="widthFix" />
      <view class="result-actions">
        <button class="result-btn" @click="previewDiagram">预览</button>
        <button class="result-btn primary" @click="saveDiagram">保存图片</button>
        <button class="result-btn" @click="shareDiagram">分享</button>
      </view>
    </view>

    <view class="tips-section">
      <text class="tips-title">💡 使用提示</text>
      <view class="tips-list">
        <text class="tip-item">• TD流程图适用于技术文档、流程说明</text>
        <text class="tip-item">• 建议步骤控制在3-10个之间</text>
        <text class="tip-item">• 支持多种样式主题选择</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Step {
  text: string
}

const diagramTitle = ref('')
const selectedTypeIndex = ref(0)
const steps = ref<Step[]>([{ text: '' }, { text: '' }, { text: '' }])
const selectedThemeIndex = ref(0)
const generating = ref(false)
const diagramImage = ref('')

const diagramTypes = ['顺序流程', '分支流程', '循环流程', '并行流程']
const themes = [
  { name: '经典蓝', color: '#667eea' },
  { name: '清新绿', color: '#48bb78' },
  { name: '温暖橙', color: '#ed8936' },
  { name: '优雅紫', color: '#9f7aea' },
  { name: '活力红', color: '#f56565' },
  { name: '简约灰', color: '#718096' }
]

const onTypeChange = (e: any) => {
  selectedTypeIndex.value = e.detail.value
}

const addStep = () => {
  if (steps.value.length < 15) {
    steps.value.push({ text: '' })
  } else {
    uni.showToast({ title: '最多添加15个步骤', icon: 'none' })
  }
}

const removeStep = (index: number) => {
  if (steps.value.length > 1) {
    steps.value.splice(index, 1)
  } else {
    uni.showToast({ title: '至少保留1个步骤', icon: 'none' })
  }
}

const generateDiagram = async () => {
  // 验证输入
  if (!diagramTitle.value.trim()) {
    uni.showToast({ title: '请输入流程图标题', icon: 'none' })
    return
  }

  const validSteps = steps.value.filter(s => s.text.trim())
  if (validSteps.length < 2) {
    uni.showToast({ title: '至少需要2个步骤', icon: 'none' })
    return
  }

  generating.value = true
  uni.showLoading({ title: '生成中...', mask: true })

  try {
    // TODO: 调用后端API生成流程图
    // const res = await generateTdDiagramApi({
    //   title: diagramTitle.value,
    //   type: diagramTypes[selectedTypeIndex.value],
    //   steps: validSteps.map(s => s.text),
    //   theme: themes[selectedThemeIndex.value]
    // })
    
    // 模拟生成过程
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    // diagramImage.value = res.imageUrl
    // 临时使用占位图
    diagramImage.value = 'https://via.placeholder.com/600x400/667eea/ffffff?text=TD流程图'
    
    uni.hideLoading()
    uni.showToast({ title: '生成成功', icon: 'success' })
  } catch (error) {
    console.error('生成失败:', error)
    uni.hideLoading()
    uni.showToast({ title: '生成失败，请重试', icon: 'none' })
  } finally {
    generating.value = false
  }
}

const previewDiagram = () => {
  if (diagramImage.value) {
    uni.previewImage({
      urls: [diagramImage.value],
      current: diagramImage.value
    })
  }
}

const saveDiagram = () => {
  if (diagramImage.value) {
    uni.downloadFile({
      url: diagramImage.value,
      success: (res) => {
        uni.saveImageToPhotosAlbum({
          filePath: res.tempFilePath,
          success: () => {
            uni.showToast({ title: '保存成功', icon: 'success' })
          },
          fail: () => {
            uni.showToast({ title: '保存失败', icon: 'none' })
          }
        })
      },
      fail: () => {
        uni.showToast({ title: '下载失败', icon: 'none' })
      }
    })
  }
}

const shareDiagram = () => {
  if (diagramImage.value) {
    uni.share({
      provider: 'weixin',
      scene: 'WXSceneSession',
      type: 2,
      imageUrl: diagramImage.value,
      success: () => {
        uni.showToast({ title: '分享成功', icon: 'success' })
      },
      fail: () => {
        uni.showToast({ title: '分享失败', icon: 'none' })
      }
    })
  }
}
</script>

<style lang="scss" scoped>
.td-diagram-page {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  padding: 40rpx 30rpx;
}

.header {
  text-align: center;
  margin-bottom: 50rpx;

  .title {
    display: block;
    font-size: 48rpx;
    font-weight: bold;
    color: #ffffff;
    margin-bottom: 15rpx;
  }

  .subtitle {
    display: block;
    font-size: 26rpx;
    color: rgba(255, 255, 255, 0.8);
  }
}

.form-section {
  background: var(--theme-surface);
  border-radius: 20rpx;
  padding: 40rpx;
  margin-bottom: 40rpx;
  box-shadow: 0 8rpx 24rpx rgba(0, 0, 0, 0.15);

  .form-item {
    margin-bottom: 40rpx;

    &:last-child {
      margin-bottom: 0;
    }

    .form-label {
      display: block;
      font-size: 28rpx;
      color: var(--theme-text);
      font-weight: bold;
      margin-bottom: 20rpx;
    }

    .form-input {
      width: 100%;
      padding: 20rpx;
      background: #f5f7fa;
      border-radius: 10rpx;
      font-size: 28rpx;
      color: var(--theme-text);
    }

    .picker-value {
      padding: 20rpx;
      background: #f5f7fa;
      border-radius: 10rpx;
      font-size: 28rpx;
      color: var(--theme-text);
    }
  }
}

.steps-container {
  .step-item {
    display: flex;
    align-items: center;
    gap: 15rpx;
    margin-bottom: 15rpx;

    .step-input {
      flex: 1;
      padding: 20rpx;
      background: #f5f7fa;
      border-radius: 10rpx;
      font-size: 28rpx;
      color: var(--theme-text);
    }

    .step-delete {
      width: 60rpx;
      height: 60rpx;
      line-height: 60rpx;
      text-align: center;
      background: #fee;
      color: #f56565;
      border-radius: 50%;
      font-size: 40rpx;
      font-weight: bold;
    }
  }

  .add-step-btn {
    width: 100%;
    height: 80rpx;
    background: #f5f7fa;
    color: var(--theme-primary);
    border-radius: 15rpx;
    font-size: 28rpx;
    border: 2rpx dashed #667eea;

    &::after {
      border: none;
    }
  }
}

.theme-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20rpx;

  .theme-item {
    text-align: center;
    padding: 20rpx;
    background: #f5f7fa;
    border-radius: 15rpx;
    border: 3rpx solid transparent;
    transition: all 0.3s;

    &.active {
      border-color: var(--theme-primary);
      background: rgba(102, 126, 234, 0.1);
    }

    .theme-preview {
      width: 100%;
      height: 80rpx;
      border-radius: 10rpx;
      margin-bottom: 10rpx;
    }

    .theme-name {
      display: block;
      font-size: 24rpx;
      color: var(--theme-text);
    }
  }
}

.action-section {
  margin-bottom: 40rpx;

  .action-btn {
    width: 100%;
    height: 90rpx;
    background: var(--theme-surface);
    color: var(--theme-primary);
    border-radius: 50rpx;
    font-size: 32rpx;
    font-weight: bold;
    border: none;
    box-shadow: 0 8rpx 24rpx rgba(0, 0, 0, 0.15);

    &::after {
      border: none;
    }

    &[disabled] {
      background: rgba(255, 255, 255, 0.5);
      color: rgba(102, 126, 234, 0.5);
    }
  }
}

.result-section {
  background: var(--theme-surface);
  border-radius: 20rpx;
  padding: 40rpx;
  margin-bottom: 40rpx;
  box-shadow: 0 8rpx 24rpx rgba(0, 0, 0, 0.15);

  .result-header {
    text-align: center;
    margin-bottom: 30rpx;

    .result-title {
      font-size: 32rpx;
      color: var(--theme-text);
      font-weight: bold;
    }
  }

  .diagram-image {
    width: 100%;
    border-radius: 15rpx;
    margin-bottom: 30rpx;
  }

  .result-actions {
    display: flex;
    gap: 15rpx;

    .result-btn {
      flex: 1;
      height: 80rpx;
      background: #f5f7fa;
      color: var(--theme-text);
      border-radius: 15rpx;
      font-size: 28rpx;
      border: none;

      &::after {
        border: none;
      }

      &.primary {
        background: var(--theme-primary);
        color: #ffffff;
      }
    }
  }
}

.tips-section {
  background: rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10rpx);
  border-radius: 20rpx;
  padding: 30rpx;

  .tips-title {
    display: block;
    font-size: 28rpx;
    color: #ffffff;
    font-weight: bold;
    margin-bottom: 20rpx;
  }

  .tips-list {
    display: flex;
    flex-direction: column;
    gap: 10rpx;

    .tip-item {
      font-size: 24rpx;
      color: rgba(255, 255, 255, 0.9);
      line-height: 1.6;
    }
  }
}
</style>

