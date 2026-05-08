<template>
  <view class="login-page">
    <view class="login-container">
      <!-- Logo -->
      <view class="logo-section">
        <text class="app-name">你没有遗憾</text>
        <text class="app-slogan">平凡的人也要为自己的梦想而努力</text>
      </view>

      <!-- 登录表单 -->
      <view class="form-section">
        <view class="input-group">
          <view class="input-wrapper">
            <text class="input-icon">👤</text>
            <input
              class="input-field"
              v-model="formData.username"
              placeholder="请输入用户名"
              :maxlength="50"
            />
          </view>
        </view>

        <view class="input-group">
          <view class="input-wrapper">
            <text class="input-icon">🔒</text>
            <input
              class="input-field"
              v-model="formData.password"
              placeholder="请输入密码"
              :password="!showPassword"
              :maxlength="20"
            />
            <text class="eye-icon" @click="togglePassword">
              {{ showPassword ? '👁️' : '🙈' }}
            </text>
          </view>
        </view>

        <!-- 登录按钮 -->
        <button class="login-btn" @click="handleLogin" :loading="loading">
          {{ loading ? '登录中...' : '登录' }}
        </button>

        <!-- 注册链接 -->
        <view class="register-link">
          <text class="link-text">还没有账号？</text>
          <text class="link-btn" @click="goToRegister">立即注册</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useUserStore } from '@/store'
import { showToast } from '@/utils'

const userStore = useUserStore()

const formData = ref({
  username: '',
  password: '',
})

const showPassword = ref(false)
const loading = ref(false)

// 页面加载时，如果有传递用户名参数，自动填充
onMounted(() => {
  const pages = getCurrentPages()
  const currentPage = pages[pages.length - 1] as any
  const options = currentPage.options || {}
  
  if (options.username) {
    formData.value.username = decodeURIComponent(options.username)
  }
})

// 切换密码显示
const togglePassword = () => {
  showPassword.value = !showPassword.value
}

// 登录
const handleLogin = async () => {
  if (!formData.value.username) {
    return showToast('请输入用户名', 'none')
  }
  if (!formData.value.password) {
    return showToast('请输入密码', 'none')
  }

  loading.value = true
  try {
    const success = await userStore.login(formData.value.username, formData.value.password)
    if (success) {
      showToast('登录成功', 'success')
      setTimeout(() => {
        uni.reLaunch({
          url: '/pages/emotion/index',
        })
      }, 1000)
    }
  } catch (error: any) {
    showToast(error.message || '登录失败', 'none')
  } finally {
    loading.value = false
  }
}

// 跳转注册
const goToRegister = () => {
  uni.navigateTo({
    url: '/pages/auth/register',
  })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.login-page {
  min-height: 100vh;
  @include cyber-page-bg;
  padding: 0 30rpx;
  display: flex;
  align-items: center;
  justify-content: center;
}

.login-container {
  width: 100%;
  max-width: 600rpx;
}

.logo-section {
  text-align: center;
  margin-bottom: 80rpx;
  animation: fadeInDown 0.8s ease-out;
  padding: 40rpx 0;

  .app-name {
    display: block;
    font-size: 72rpx;
    font-weight: 800;
    background: linear-gradient(135deg, #00D9FF, #FF00D6, #FFD600);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 20rpx;
    letter-spacing: 4rpx;
    filter: drop-shadow(0 0 30rpx rgba(0, 217, 255, 0.8));
    animation: titleGlow 3s ease-in-out infinite, titleFloat 4s ease-in-out infinite;
  }

  .app-slogan {
    display: block;
    font-size: 28rpx;
    color: #FFD600;
    text-shadow: 
      0 0 20rpx rgba(255, 214, 0, 1),
      0 0 40rpx rgba(255, 214, 0, 0.6);
    letter-spacing: 2rpx;
  }
}

@keyframes fadeInDown {
  from { opacity: 0; transform: translateY(-30rpx); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes logoFloat {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-15rpx); }
}

@keyframes titleGlow {
  0%, 100% {
    filter: drop-shadow(0 0 30rpx rgba(0, 217, 255, 0.8));
  }
  50% {
    filter: drop-shadow(0 0 50rpx rgba(255, 0, 214, 1)) drop-shadow(0 0 70rpx rgba(0, 217, 255, 0.8));
  }
}

@keyframes titleFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10rpx);
  }
}

.form-section {
  @include neon-card;
  border-radius: 20rpx;
  padding: 60rpx 40rpx;
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(0, 217, 255, 0.4),
    inset 0 0 80rpx rgba(0, 217, 255, 0.05);
  border: 2rpx solid rgba(0, 217, 255, 0.5);
  animation: fadeInUp 0.8s ease-out 0.2s backwards;
}

@keyframes fadeInUp {
  from { opacity: 0; transform: translateY(30rpx); }
  to { opacity: 1; transform: translateY(0); }
}

.input-group {
  margin-bottom: 30rpx;

  .input-wrapper {
    @include neon-input;
    display: flex;
    align-items: center;
    border-radius: 50rpx;
    padding: 0 30rpx;
    height: 90rpx;
    transition: all 0.3s;

    &:focus-within {
      border-color: rgba(0, 217, 255, 0.8);
      box-shadow: 0 0 30rpx rgba(0, 217, 255, 0.5);
    }

    .input-icon {
      font-size: 36rpx;
      margin-right: 20rpx;
      filter: drop-shadow(0 0 10rpx rgba(0, 217, 255, 0.5));
    }

    .input-field {
      flex: 1;
      font-size: 28rpx;
      color: #ffffff;
    }

    .eye-icon {
      font-size: 32rpx;
      cursor: pointer;
      filter: drop-shadow(0 0 10rpx rgba(138, 92, 246, 0.5));
    }
  }
}

.login-btn {
  width: 100%;
  height: 90rpx;
  @include glow-button(#00D9FF);
  background: linear-gradient(135deg, rgba(0, 217, 255, 0.8) 0%, rgba(138, 92, 246, 0.8) 100%);
  font-size: 32rpx;
  font-weight: bold;
  border-radius: 50rpx;
  border: none;
  margin-top: 20rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: btnPulse 3s ease-in-out infinite;

  &:active {
    transform: scale(0.95);
    animation: none;
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 0 25rpx rgba(0, 217, 255, 0.5),
      0 10rpx 30rpx rgba(0, 0, 0, 0.3);
  }
  50% {
    box-shadow: 
      0 0 40rpx rgba(0, 217, 255, 0.8),
      0 15rpx 40rpx rgba(0, 0, 0, 0.4);
  }
}

.register-link {
  text-align: center;
  margin-top: 40rpx;
  font-size: 26rpx;

  .link-text {
    color: #b8c5d6;
  }

  .link-btn {
    @include neon-text(#8B5CF6);
    font-weight: bold;
    margin-left: 10rpx;
  }
}
</style>

