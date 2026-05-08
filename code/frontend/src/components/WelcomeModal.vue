<template>
  <view v-if="visible" class="modal-overlay" @click="handleClose">
    <view class="modal-content" @click.stop>
      <view class="modal-header">
        <text class="modal-title">🌟 欢迎来到无悔青春</text>
      </view>
      
      <view class="modal-body">
        <view class="quote-item">
          <text class="quote-icon">💪</text>
          <text class="quote-text">平凡的人也要为自己的梦想而努力</text>
        </view>
        <view class="quote-item">
          <text class="quote-icon">🌈</text>
          <text class="quote-text">每一次努力都不会白费</text>
        </view>
        <view class="quote-item">
          <text class="quote-icon">✨</text>
          <text class="quote-text">相信自己，你一定可以的</text>
        </view>
      </view>

      <view class="modal-footer">
        <button class="btn-primary" @click="handleConfirm">开启旅程</button>
        <view class="checkbox-group">
          <checkbox-group @change="handleCheckboxChange">
            <label class="checkbox-label">
              <checkbox value="dontShow" />
              <text class="checkbox-text">今天不再显示</text>
            </label>
          </checkbox-group>
        </view>
      </view>

      <view class="close-btn" @click="handleClose">
        <text>✕</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

interface Props {
  visible: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'update:visible', value: boolean): void
  (e: 'confirm', dontShowToday: boolean): void
}>()

const dontShowToday = ref(false)

const handleCheckboxChange = (e: any) => {
  dontShowToday.value = e.detail.value.includes('dontShow')
}

const handleConfirm = () => {
  emit('confirm', dontShowToday.value)
  emit('update:visible', false)
}

const handleClose = () => {
  emit('update:visible', false)
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(15rpx);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  padding: 30rpx;
  animation: overlayFadeIn 0.4s ease-out;
}

@keyframes overlayFadeIn {
  from {
    opacity: 0;
    backdrop-filter: blur(0);
  }
  to {
    opacity: 1;
    backdrop-filter: blur(15rpx);
  }
}

.modal-content {
  position: relative;
  @include neon-card;
  @include rainbow-border;
  border-radius: 30rpx;
  width: 100%;
  max-width: 600rpx;
  padding: 60rpx 40rpx 40rpx;
  box-shadow: 
    0 30rpx 80rpx rgba(0, 0, 0, 0.7),
    0 0 80rpx rgba(138, 92, 246, 0.6),
    inset 0 0 100rpx rgba(138, 92, 246, 0.1);
  animation: modalSlideUp 0.5s cubic-bezier(0.4, 0, 0.2, 1);
  
  &::before {
    content: '';
    position: absolute;
    top: -2rpx;
    left: 50%;
    transform: translateX(-50%);
    width: 80%;
    height: 3rpx;
    background: linear-gradient(90deg, transparent, rgba(138, 92, 246, 0.8), transparent);
    box-shadow: 0 0 30rpx rgba(138, 92, 246, 0.8);
  }
}

@keyframes modalSlideUp {
  from {
    transform: translateY(150rpx) scale(0.8);
    opacity: 0;
  }
  to {
    transform: translateY(0) scale(1);
    opacity: 1;
  }
}

.modal-header {
  text-align: center;
  margin-bottom: 40rpx;
  animation: fadeInDown 0.6s ease-out 0.2s backwards;

  .modal-title {
    font-size: 40rpx;
    font-weight: bold;
    @include neon-title(#8B5CF6);
    animation: titleGlow 3s ease-in-out infinite;
  }
}

@keyframes fadeInDown {
  from {
    opacity: 0;
    transform: translateY(-20rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes titleGlow {
  0%, 100% {
    text-shadow: 
      0 0 20rpx rgba(138, 92, 246, 0.8),
      0 0 40rpx rgba(138, 92, 246, 0.5),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
  50% {
    text-shadow: 
      0 0 30rpx rgba(138, 92, 246, 1),
      0 0 60rpx rgba(138, 92, 246, 0.8),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
}

.modal-body {
  margin-bottom: 40rpx;

  .quote-item {
    display: flex;
    align-items: flex-start;
    margin-bottom: 30rpx;
    padding: 20rpx;
    background: rgba(30, 36, 66, 0.5);
    backdrop-filter: blur(10rpx);
    border: 2rpx solid rgba(0, 217, 255, 0.3);
    border-radius: 15rpx;
    box-shadow: 0 0 20rpx rgba(0, 217, 255, 0.2);
    animation: fadeInUp 0.6s ease-out backwards;
    transition: all 0.3s ease;
    
    &:nth-child(1) { animation-delay: 0.3s; }
    &:nth-child(2) { animation-delay: 0.4s; }
    &:nth-child(3) { animation-delay: 0.5s; }
    
    &:active {
      transform: translateX(5rpx);
      border-color: rgba(0, 217, 255, 0.6);
      box-shadow: 0 0 30rpx rgba(0, 217, 255, 0.4);
    }

    .quote-icon {
      font-size: 36rpx;
      margin-right: 20rpx;
      filter: drop-shadow(0 0 15rpx rgba(0, 217, 255, 0.6));
      animation: iconPulse 3s ease-in-out infinite;
    }

    .quote-text {
      flex: 1;
      font-size: 28rpx;
      @include neon-text(#ffffff);
      line-height: 1.6;
    }
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes iconPulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

.modal-footer {
  animation: fadeIn 0.6s ease-out 0.6s backwards;
  
  .btn-primary {
    width: 100%;
    height: 80rpx;
    @include glow-button(#8B5CF6);
    background: linear-gradient(135deg, rgba(138, 92, 246, 0.8) 0%, rgba(102, 126, 234, 0.8) 100%);
    font-size: 30rpx;
    font-weight: bold;
    border-radius: 50rpx;
    border: none;
    margin-bottom: 20rpx;
    display: flex;
    align-items: center;
    justify-content: center;
    animation: btnPulse 3s ease-in-out infinite;
    transition: all 0.3s ease;

    &:active {
      transform: scale(0.95);
      animation: none;
    }
  }

  .checkbox-group {
    text-align: center;

    .checkbox-label {
      display: inline-flex;
      align-items: center;
      font-size: 24rpx;
      @include neon-text(#6b7b93);
      transition: all 0.3s ease;

      checkbox {
        margin-right: 10rpx;
      }
      
      &:active {
        @include neon-text(#00D9FF);
      }
    }
  }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 0 25rpx rgba(138, 92, 246, 0.5),
      0 10rpx 30rpx rgba(0, 0, 0, 0.3),
      inset 0 0 25rpx rgba(138, 92, 246, 0.2);
  }
  50% {
    box-shadow: 
      0 0 40rpx rgba(138, 92, 246, 0.8),
      0 15rpx 40rpx rgba(0, 0, 0, 0.4),
      inset 0 0 40rpx rgba(138, 92, 246, 0.3);
  }
}

.close-btn {
  position: absolute;
  top: 20rpx;
  right: 20rpx;
  width: 50rpx;
  height: 50rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 36rpx;
  @include neon-text(#6b7b93);
  cursor: pointer;
  border-radius: 25rpx;
  transition: all 0.3s ease;
  animation: fadeIn 0.6s ease-out 0.7s backwards;
  
  &:hover {
    background: rgba(255, 0, 214, 0.1);
  }

  &:active {
    @include neon-text(#FF00D6);
    transform: rotate(90deg) scale(1.2);
    background: rgba(255, 0, 214, 0.2);
  }
}
</style>

