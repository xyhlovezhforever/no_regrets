<template>
  <view class="edit-page">
    <!-- 头像上传 -->
    <view class="avatar-section" @click="chooseAvatar">
      <view v-if="form.avatar_url" class="avatar-wrapper">
        <image :src="getFullUrl(form.avatar_url)" mode="aspectFill" class="avatar" />
        <view class="avatar-mask">
          <text>点击更换</text>
        </view>
      </view>
      <view v-else class="avatar-placeholder">
        <text class="icon">📷</text>
      </view>
    </view>

    <!-- 表单 -->
    <view class="form">
      <view class="form-item">
        <text class="label">姓名 *</text>
        <input v-model="form.name" class="input" placeholder="请输入偶像姓名" />
      </view>

      <view class="form-item">
        <text class="label">职业</text>
        <input v-model="form.profession" class="input" placeholder="如：演员、歌手" />
      </view>

      <view class="form-item">
        <text class="label">国籍</text>
        <input v-model="form.nationality" class="input" placeholder="请输入国籍" />
      </view>

      <view class="form-item">
        <text class="label">生日</text>
        <picker mode="date" :value="form.birth_date" @change="onDateChange">
          <view class="picker">
            {{ form.birth_date || '选择生日' }}
          </view>
        </picker>
      </view>

      <view class="form-item">
        <text class="label">简介</text>
        <textarea 
          v-model="form.description" 
          class="textarea" 
          placeholder="请输入简介" 
          maxlength="500"
        />
      </view>

      <view class="form-item">
        <text class="label">标签</text>
        <view class="tags-input">
          <view v-for="(tag, index) in (form.tags || [])" :key="index" class="tag">
            <text>{{ tag }}</text>
            <text class="remove" @click="removeTag(index)">×</text>
          </view>
          <input 
            v-model="newTag" 
            class="tag-input" 
            placeholder="输入标签后按回车"
            @confirm="addTag"
          />
        </view>
      </view>

      <view class="form-item">
        <view class="switch-row">
          <text class="label">公开显示</text>
          <switch :checked="form.is_public" @change="onSwitchChange" color="#667eea" />
        </view>
      </view>
    </view>

    <!-- 操作按钮 -->
    <view class="actions">
      <button class="btn btn-cancel" @click="goBack">取消</button>
      <button class="btn btn-save" @click="save" :disabled="saving">
        {{ saving ? '保存中...' : '保存' }}
      </button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import type { UpdateIdolRequest } from '@/types/idol'
import { getIdolApi, updateIdolApi, createIdolApi } from '@/api/idol'
import { uploadImageApi } from '@/api/upload'
import { BASE_API } from '@/config'

const idolId = ref('')
const isEdit = ref(false)
const saving = ref(false)
const newTag = ref('')

const form = ref<UpdateIdolRequest & { avatar_url?: string }>({
  name: '',
  description: '',
  profession: '',
  nationality: '',
  birth_date: '',
  avatar_url: '',
  tags: null as any,  // 初始化为 null 而不是空数组
  is_public: true
})

onLoad((options: any) => {
  if (options.id) {
    // uni-app 已经自动解码参数，直接使用即可
    idolId.value = String(options.id).trim()
    console.log('接收到的ID:', idolId.value)
    console.log('ID类型:', typeof idolId.value)
    console.log('ID长度:', idolId.value.length)
    isEdit.value = true
    loadIdol()
  }
})

const loadIdol = async () => {
  try {
    const data = await getIdolApi(idolId.value)
    const idol = data.idol || data
    form.value = {
      name: idol.name,
      description: idol.description,
      profession: idol.profession,
      nationality: idol.nationality,
      birth_date: idol.birth_date,
      avatar_url: idol.avatar_url,
      tags: idol.tags && idol.tags.length > 0 ? idol.tags : null as any,  // 空数组转为 null
      is_public: idol.is_public
    }
  } catch (error) {
    console.error('加载偶像失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  }
}

const getFullUrl = (url: string | null | undefined) => {
  if (!url) return ''
  if (url.startsWith('http')) return url
  return `${BASE_API.replace('/api/v1', '')}${url}`
}

const chooseAvatar = () => {
  uni.chooseImage({
    count: 1,
    sizeType: ['compressed'],
    sourceType: ['album', 'camera'],
    success: async (res) => {
      try {
        uni.showLoading({ title: '上传中...' })
        const tempFilePath = res.tempFilePaths[0]
        const result = await uploadImageApi(tempFilePath)
        form.value.avatar_url = result.url
        uni.hideLoading()
        uni.showToast({ title: '上传成功', icon: 'success' })
      } catch (error: any) {
        uni.hideLoading()
        console.error('上传失败:', error)
        uni.showToast({ title: '上传失败', icon: 'none' })
      }
    }
  })
}

const onDateChange = (e: any) => {
  form.value.birth_date = e.detail.value
}

const onSwitchChange = (e: any) => {
  form.value.is_public = e.detail.value
}

const addTag = () => {
  if (newTag.value.trim()) {
    if (!form.value.tags || !Array.isArray(form.value.tags)) {
      form.value.tags = []
    }
    form.value.tags.push(newTag.value.trim())
    newTag.value = ''
  }
}

const removeTag = (index: number) => {
  if (form.value.tags && Array.isArray(form.value.tags)) {
    form.value.tags.splice(index, 1)
    // 如果删除后为空数组，设置为 null
    if (form.value.tags.length === 0) {
      form.value.tags = null as any
    }
  }
}

const save = async () => {
  if (!form.value.name?.trim()) {
    uni.showToast({ title: '请输入偶像姓名', icon: 'none' })
    return
  }

  try {
    saving.value = true
    
    // 准备数据：清理空值
    const data = {
      name: form.value.name?.trim(),
      description: form.value.description?.trim() || null,
      avatar_url: form.value.avatar_url?.trim() || null,
      birth_date: form.value.birth_date?.trim() || null,
      nationality: form.value.nationality?.trim() || null,
      profession: form.value.profession?.trim() || null,
      tags: form.value.tags && form.value.tags.length > 0 ? form.value.tags : null,
      is_public: form.value.is_public
    }
    
    console.log('保存数据:', data)
    
    if (isEdit.value) {
      await updateIdolApi(idolId.value, data)
      uni.showToast({ title: '更新成功', icon: 'success' })
    } else {
      await createIdolApi(data as any)
      uni.showToast({ title: '添加成功', icon: 'success' })
    }
    setTimeout(() => {
      uni.navigateBack()
    }, 1500)
  } catch (error: any) {
    console.error('保存失败:', error)
    uni.showToast({ title: '保存失败', icon: 'none' })
  } finally {
    saving.value = false
  }
}

const goBack = () => {
  uni.navigateBack()
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.edit-page {
  @include cyber-page-bg;
  min-height: 100vh;
  padding-bottom: 140rpx;
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

.avatar-section {
  @include neon-card;
  padding: 70rpx 30rpx;
  display: flex;
  justify-content: center;
  align-items: center;
  margin: 0 30rpx 30rpx;
  border-radius: 30rpx;
  border: 2rpx solid rgba(255, 214, 0, 0.6);
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(255, 214, 0, 0.4),
    inset 0 0 50rpx rgba(255, 214, 0, 0.15);
  position: relative;
  z-index: 1;
  animation: slideDown 0.6s ease-out;
  
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
      rgba(255, 214, 0, 0.08) 90deg,
      transparent 180deg
    );
    animation: bgRotate 15s linear infinite;
    z-index: -1;
  }
}

.avatar-wrapper {
  position: relative;
  width: 220rpx;
  height: 220rpx;
  border-radius: 110rpx;
  overflow: hidden;
  border: 4rpx solid rgba(255, 214, 0, 0.8);
  box-shadow: 
    0 0 40rpx rgba(255, 214, 0, 0.8),
    inset 0 0 40rpx rgba(255, 214, 0, 0.2);
  animation: avatarPulse 2s ease-in-out infinite;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

  .avatar {
    width: 100%;
    height: 100%;
  }

  .avatar-mask {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(135deg, rgba(255, 214, 0, 0.7) 0%, rgba(138, 92, 246, 0.7) 100%);
    backdrop-filter: blur(10rpx);
    display: flex;
    align-items: center;
    justify-content: center;
    @include neon-text(#ffffff);
    font-size: 28rpx;
    font-weight: bold;
    opacity: 0;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  }
  
  &:hover .avatar-mask,
  &:active .avatar-mask {
    opacity: 1;
  }
  
  &:active {
    transform: scale(0.96);
    box-shadow: 
      0 0 50rpx rgba(255, 214, 0, 1),
      inset 0 0 50rpx rgba(255, 214, 0, 0.3);
  }
}

.avatar-placeholder {
  width: 220rpx;
  height: 220rpx;
  border-radius: 110rpx;
  background: linear-gradient(135deg, rgba(255, 214, 0, 0.2) 0%, rgba(138, 92, 246, 0.2) 100%);
  backdrop-filter: blur(10rpx);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  border: 4rpx dashed rgba(255, 214, 0, 0.6);
  box-shadow: 
    0 0 40rpx rgba(255, 214, 0, 0.5),
    inset 0 0 40rpx rgba(255, 214, 0, 0.15);
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  animation: placeholderPulse 3s ease-in-out infinite;

  .icon {
    font-size: 100rpx;
    filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 0.8));
    animation: iconFloat 3s ease-in-out infinite;
  }
  
  &:active {
    transform: scale(0.96);
    border-style: solid;
    box-shadow: 
      0 0 60rpx rgba(255, 214, 0, 0.8),
      inset 0 0 50rpx rgba(255, 214, 0, 0.25);
  }
}

.form {
  @include neon-card;
  margin: 0 30rpx 30rpx;
  padding: 40rpx 35rpx;
  border-radius: 30rpx;
  border: 2rpx solid rgba(0, 217, 255, 0.5);
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.5),
    0 0 60rpx rgba(0, 217, 255, 0.3),
    inset 0 0 50rpx rgba(0, 217, 255, 0.1);
  position: relative;
  z-index: 1;
  animation: slideUp 0.6s ease-out 0.2s backwards;
}

.form-item {
  margin-bottom: 35rpx;

  .label {
    display: block;
    font-size: 30rpx;
    @include neon-text(#FFD600);
    font-weight: 500;
    margin-bottom: 18rpx;
  }

  .input {
    @include neon-input;
    width: 100%;
    height: 80rpx;
    padding: 0 25rpx;
    border-radius: 18rpx;
    font-size: 28rpx;
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

  .picker {
    @include neon-input;
    height: 80rpx;
    padding: 0 25rpx;
    border-radius: 18rpx;
    font-size: 28rpx;
    display: flex;
    align-items: center;
    color: #ffffff;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    
    &:active {
      border-color: rgba(255, 214, 0, 0.8);
      box-shadow: 
        0 0 40rpx rgba(255, 214, 0, 0.5),
        inset 0 0 30rpx rgba(255, 214, 0, 0.1);
    }
  }

  .textarea {
    @include neon-input;
    width: 100%;
    min-height: 180rpx;
    padding: 25rpx;
    border-radius: 18rpx;
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

  .tags-input {
    display: flex;
    flex-wrap: wrap;
    gap: 18rpx;
    padding: 25rpx;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
    backdrop-filter: blur(10rpx);
    border-radius: 18rpx;
    min-height: 100rpx;
    border: 2rpx solid rgba(0, 217, 255, 0.5);
    box-shadow: 
      0 4rpx 16rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(0, 217, 255, 0.1);

    .tag {
      display: flex;
      align-items: center;
      gap: 10rpx;
      padding: 10rpx 20rpx;
      background: linear-gradient(135deg, rgba(255, 214, 0, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border-radius: 30rpx;
      font-size: 26rpx;
      border: 2rpx solid rgba(255, 214, 0, 0.6);
      box-shadow: 
        0 4rpx 12rpx rgba(255, 214, 0, 0.4),
        0 0 20rpx rgba(255, 214, 0, 0.3);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      animation: tagSlideIn 0.4s ease-out;

      .remove {
        font-size: 36rpx;
        font-weight: bold;
        cursor: pointer;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        
        &:active {
          transform: rotate(90deg) scale(1.3);
          color: #FF00D6;
        }
      }
      
      &:active {
        transform: scale(0.95);
      }
    }

    .tag-input {
      flex: 1;
      min-width: 180rpx;
      font-size: 28rpx;
      background: transparent;
      color: #ffffff;
      
      &::placeholder {
        color: #6b7b93;
      }
    }
  }

  .switch-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
}

.actions {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 25rpx 30rpx;
  padding-bottom: calc(25rpx + env(safe-area-inset-bottom));
  @include neon-card;
  border-top: 3rpx solid rgba(255, 214, 0, 0.5);
  display: flex;
  gap: 25rpx;
  box-shadow: 
    0 -15rpx 50rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(255, 214, 0, 0.3),
    inset 0 0 50rpx rgba(255, 214, 0, 0.1);
  backdrop-filter: blur(30rpx);
  z-index: 100;
  animation: slideUp 0.6s ease-out 0.4s backwards;

  .btn {
    flex: 1;
    height: 90rpx;
    border-radius: 50rpx;
    font-size: 32rpx;
    font-weight: bold;
    line-height: 90rpx;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;

    &::after {
      border: none;
    }

    &.btn-cancel {
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.8) 0%, rgba(30, 36, 66, 0.6) 100%);
      @include neon-text(#b8c5d6);
      border: 2rpx solid rgba(138, 92, 246, 0.5);
      box-shadow: 
        0 6rpx 20rpx rgba(0, 0, 0, 0.4),
        0 0 30rpx rgba(138, 92, 246, 0.2);
      
      &:active {
        transform: scale(0.96);
        @include neon-text(#ffffff);
        box-shadow: 
          0 4rpx 12rpx rgba(0, 0, 0, 0.5),
          0 0 40rpx rgba(138, 92, 246, 0.4);
      }
    }

    &.btn-save {
      background: linear-gradient(135deg, rgba(255, 214, 0, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border: 3rpx solid rgba(255, 214, 0, 0.7);
      box-shadow: 
        0 10rpx 40rpx rgba(255, 214, 0, 0.5),
        0 0 60rpx rgba(255, 214, 0, 0.4),
        inset 0 0 40rpx rgba(255, 214, 0, 0.2);
      animation: btnPulse 3s ease-in-out infinite;
      
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
        animation: none;
        transform: scale(0.96);
        box-shadow: 
          0 8rpx 30rpx rgba(255, 214, 0, 0.7),
          0 0 80rpx rgba(255, 214, 0, 0.6),
          inset 0 0 50rpx rgba(255, 214, 0, 0.3);
      }
      
      &:disabled {
        opacity: 0.5;
        animation: none;
        cursor: not-allowed;
        
        &::before {
          animation: none;
        }
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

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-40rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(40rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
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

@keyframes placeholderPulse {
  0%, 100% {
    box-shadow: 
      0 0 30rpx rgba(255, 214, 0, 0.4),
      inset 0 0 30rpx rgba(255, 214, 0, 0.1);
  }
  50% {
    box-shadow: 
      0 0 50rpx rgba(255, 214, 0, 0.6),
      inset 0 0 50rpx rgba(255, 214, 0, 0.2);
  }
}

@keyframes iconFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-12rpx);
  }
}

@keyframes tagSlideIn {
  from {
    opacity: 0;
    transform: scale(0.8) translateX(-20rpx);
  }
  to {
    opacity: 1;
    transform: scale(1) translateX(0);
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(255, 214, 0, 0.5),
      0 0 60rpx rgba(255, 214, 0, 0.4),
      inset 0 0 40rpx rgba(255, 214, 0, 0.2);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(255, 214, 0, 0.7),
      0 0 80rpx rgba(255, 214, 0, 0.6),
      inset 0 0 50rpx rgba(255, 214, 0, 0.3);
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
