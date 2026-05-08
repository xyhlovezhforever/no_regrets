<template>
  <view class="watermark-page">
    <view class="header">
      <text class="title">💧 去文档水印</text>
      <text class="subtitle">自动识别并去除文档中的水印</text>
    </view>

    <view class="upload-section">
      <view class="upload-area" @click="chooseFile">
        <view v-if="!filePath" class="upload-placeholder">
          <text class="upload-icon">📄</text>
          <text class="upload-text">点击选择文档</text>
          <text class="upload-hint">支持 PDF、Word、图片格式</text>
        </view>
        <view v-else class="file-info">
          <text class="file-icon">📎</text>
          <text class="file-name">{{ fileName }}</text>
          <text class="file-size">{{ fileSize }}</text>
        </view>
      </view>
    </view>

    <view v-if="filePath" class="options-section">
      <view class="option-item">
        <text class="option-label">水印类型</text>
        <picker :range="watermarkTypes" :value="selectedTypeIndex" @change="onTypeChange">
          <view class="picker-value">{{ watermarkTypes[selectedTypeIndex] }}</view>
        </picker>
      </view>

      <view class="option-item">
        <text class="option-label">处理强度</text>
        <slider 
          :value="intensity" 
          min="1" 
          max="10" 
          step="1"
          show-value
          activeColor="#667eea"
          @change="onIntensityChange"
        />
      </view>
    </view>

    <view class="action-section">
      <button 
        class="action-btn primary" 
        :disabled="!filePath || processing"
        @click="processWatermark"
      >
        <text v-if="!processing">🚀 开始去除水印</text>
        <text v-else>处理中...</text>
      </button>
    </view>

    <view v-if="resultPath" class="result-section">
      <view class="result-header">
        <text class="result-title">✅ 处理完成</text>
      </view>
      <view class="result-actions">
        <button class="result-btn" @click="previewResult">预览</button>
        <button class="result-btn primary" @click="saveResult">保存文件</button>
      </view>
    </view>

    <view class="tips-section">
      <text class="tips-title">💡 使用提示</text>
      <view class="tips-list">
        <text class="tip-item">• 支持去除文字水印、图片水印</text>
        <text class="tip-item">• 处理时间取决于文件大小</text>
        <text class="tip-item">• 建议选择清晰的原文件</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { removeWatermarkApi } from '@/api/watermark'

const filePath = ref('')
const fileName = ref('')
const fileSize = ref('')
const selectedTypeIndex = ref(0)
const intensity = ref(5)
const processing = ref(false)
const resultPath = ref('')
const selectedFile = ref<File | null>(null)

const watermarkTypes = ['自动识别', '文字水印', '图片水印', '背景水印']
const watermarkTypeMap: Record<number, string> = {
  0: 'auto',
  1: 'text',
  2: 'image',
  3: 'background'
}

const chooseFile = () => {
  // #ifdef H5
  // H5端使用input标签
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = 'image/*'
  input.onchange = (e: any) => {
    const file = e.target.files[0]
    if (file) {
      selectedFile.value = file
      fileName.value = file.name
      fileSize.value = formatFileSize(file.size)
      filePath.value = URL.createObjectURL(file)
    }
  }
  input.click()
  // #endif
  
  // #ifndef H5
  // 小程序端使用chooseImage
  uni.chooseImage({
    count: 1,
    sizeType: ['original'],
    sourceType: ['album', 'camera'],
    success: (res) => {
      const file = res.tempFiles?.[0] || { path: res.tempFilePaths[0], size: 0 }
      filePath.value = file.path || res.tempFilePaths[0]
      fileName.value = `图片_${Date.now()}.jpg`
      fileSize.value = file.size ? formatFileSize(file.size) : '未知大小'
    },
    fail: (err) => {
      console.error('选择文件失败:', err)
      uni.showToast({ title: '选择文件失败', icon: 'none' })
    }
  })
  // #endif
}

const formatFileSize = (bytes: number): string => {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB'
  return (bytes / (1024 * 1024)).toFixed(2) + ' MB'
}

const onTypeChange = (e: any) => {
  selectedTypeIndex.value = e.detail.value
}

const onIntensityChange = (e: any) => {
  intensity.value = e.detail.value
}

const processWatermark = async () => {
  if (!filePath.value) {
    uni.showToast({ title: '请先选择文件', icon: 'none' })
    return
  }

  processing.value = true
  uni.showLoading({ title: '处理中...', mask: true })

  try {
    // #ifdef H5
    if (!selectedFile.value) {
      throw new Error('未选择文件')
    }
    const res = await removeWatermarkApi(selectedFile.value, {
      type: watermarkTypeMap[selectedTypeIndex.value],
      intensity: intensity.value
    })
    // #endif
    
    // #ifndef H5
    const res = await removeWatermarkApi(filePath.value, {
      type: watermarkTypeMap[selectedTypeIndex.value],
      intensity: intensity.value
    })
    // #endif
    
    if (res.success && res.file_url) {
      resultPath.value = res.file_url
      uni.hideLoading()
      uni.showToast({ title: '处理完成', icon: 'success' })
    } else {
      throw new Error(res.message || '处理失败')
    }
  } catch (error: any) {
    console.error('处理失败:', error)
    uni.hideLoading()
    uni.showToast({ 
      title: error.message || '处理失败，请重试', 
      icon: 'none' 
    })
  } finally {
    processing.value = false
  }
}

const previewResult = () => {
  if (resultPath.value) {
    uni.previewImage({
      urls: [resultPath.value],
      fail: () => {
        uni.showToast({ title: '预览失败', icon: 'none' })
      }
    })
  }
}

const saveResult = () => {
  if (resultPath.value) {
    uni.saveFile({
      tempFilePath: resultPath.value,
      success: () => {
        uni.showToast({ title: '保存成功', icon: 'success' })
      },
      fail: () => {
        uni.showToast({ title: '保存失败', icon: 'none' })
      }
    })
  }
}
</script>

<style lang="scss" scoped>
/* 霓虹赛博朋克主题 - 纯CSS版本 */
.watermark-page {
  min-height: 100vh;
  background: #0a0e27 !important; /* 深色背景 */
  background-image: 
    radial-gradient(circle at 10% 20%, rgba(0, 217, 255, 0.15) 0%, transparent 40%),
    radial-gradient(circle at 90% 80%, rgba(255, 0, 214, 0.15) 0%, transparent 40%),
    radial-gradient(circle at 50% 50%, rgba(255, 214, 0, 0.1) 0%, transparent 50%) !important;
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

.upload-section {
  margin-bottom: 40rpx;

  .upload-area {
    background: var(--theme-surface);
    border-radius: 20rpx;
    padding: 60rpx 40rpx;
    text-align: center;
    box-shadow: 0 8rpx 24rpx rgba(0, 0, 0, 0.15);

    .upload-placeholder {
      .upload-icon {
        display: block;
        font-size: 100rpx;
        margin-bottom: 20rpx;
      }

      .upload-text {
        display: block;
        font-size: 32rpx;
        color: var(--theme-text);
        font-weight: bold;
        margin-bottom: 10rpx;
      }

      .upload-hint {
        display: block;
        font-size: 24rpx;
        color: var(--theme-text-secondary);
      }
    }

    .file-info {
      .file-icon {
        display: block;
        font-size: 80rpx;
        margin-bottom: 15rpx;
      }

      .file-name {
        display: block;
        font-size: 28rpx;
        color: var(--theme-text);
        font-weight: bold;
        margin-bottom: 8rpx;
        word-break: break-all;
      }

      .file-size {
        display: block;
        font-size: 24rpx;
        color: var(--theme-text-secondary);
      }
    }
  }
}

.options-section {
  background: var(--theme-surface);
  border-radius: 20rpx;
  padding: 40rpx;
  margin-bottom: 40rpx;
  box-shadow: 0 8rpx 24rpx rgba(0, 0, 0, 0.15);

  .option-item {
    margin-bottom: 40rpx;

    &:last-child {
      margin-bottom: 0;
    }

    .option-label {
      display: block;
      font-size: 28rpx;
      color: var(--theme-text);
      font-weight: bold;
      margin-bottom: 20rpx;
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

    &.primary {
      background: var(--theme-surface);
      color: var(--theme-primary);
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

  .result-actions {
    display: flex;
    gap: 20rpx;

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

