<template>
  <view class="register-page">
    <!-- 动态背景粒子 -->
    <view class="particles-bg">
      <view v-for="i in 20" :key="i" class="particle" :style="getParticleStyle(i)"></view>
    </view>

    <!-- 浮动星星 -->
    <view class="floating-stars">
      <view v-for="i in 12" :key="i" class="star" :style="getStarStyle(i)">✨</view>
    </view>

    <!-- 光晕效果 -->
    <view class="glow-orb glow-orb-1"></view>
    <view class="glow-orb glow-orb-2"></view>
    <view class="glow-orb glow-orb-3"></view>

    <view class="register-container">
      <!-- 返回按钮 -->
      <view class="back-btn" @click="goBack">
        <text class="back-icon">←</text>
        <text class="back-text">返回</text>
      </view>

      <!-- Logo -->
      <view class="logo-section">
        <view class="logo-glow"></view>
        <text class="title">加入我们</text>
        <text class="subtitle">✨ 你没有遗憾 · 开启温暖之旅 ✨</text>
        <view class="welcome-badge">
          <text class="badge-icon">🎉</text>
          <text class="badge-text">欢迎新朋友</text>
        </view>
      </view>

      <!-- 注册表单 -->
      <view class="form-section">
        <view class="form-header">
          <text class="form-title">创建账号</text>
          <text class="form-subtitle">填写信息，开启精彩旅程</text>
        </view>

        <view class="input-group" :class="{ 'input-focused': focusedField === 'username' }">
          <view class="input-wrapper">
            <view class="input-icon-wrapper">
              <text class="input-icon">👤</text>
            </view>
            <input
              class="input-field"
              v-model="formData.username"
              placeholder="用户名（3-50个字符）"
              :maxlength="50"
              @focus="focusedField = 'username'"
              @blur="focusedField = ''"
            />
            <view v-if="formData.username.length >= 3" class="input-check">✓</view>
          </view>
        </view>

        <view class="input-group" :class="{ 'input-focused': focusedField === 'password' }">
          <view class="input-wrapper">
            <view class="input-icon-wrapper">
              <text class="input-icon">🔒</text>
            </view>
            <input
              class="input-field"
              v-model="formData.password"
              placeholder="密码（至少6个字符）"
              :password="!showPassword"
              :maxlength="20"
              @focus="focusedField = 'password'"
              @blur="focusedField = ''"
            />
            <text class="eye-icon" @click="togglePassword">
              {{ showPassword ? '👁️' : '🙈' }}
            </text>
          </view>
          <view v-if="formData.password" class="password-strength">
            <view class="strength-bar" :class="getPasswordStrength()"></view>
            <text class="strength-text">{{ getPasswordStrengthText() }}</text>
          </view>
        </view>

        <view class="input-group" :class="{ 'input-focused': focusedField === 'confirmPassword' }">
          <view class="input-wrapper">
            <view class="input-icon-wrapper">
              <text class="input-icon">🔐</text>
            </view>
            <input
              class="input-field"
              v-model="formData.confirmPassword"
              placeholder="确认密码"
              :password="!showPassword"
              :maxlength="20"
              @focus="focusedField = 'confirmPassword'"
              @blur="focusedField = ''"
            />
            <view v-if="formData.confirmPassword && formData.password === formData.confirmPassword" class="input-check">✓</view>
          </view>
        </view>

        <view class="input-group" :class="{ 'input-focused': focusedField === 'email' }">
          <view class="input-wrapper">
            <view class="input-icon-wrapper">
              <text class="input-icon">📧</text>
            </view>
            <input
              class="input-field"
              v-model="formData.email"
              placeholder="邮箱（选填）"
              type="email"
              @focus="focusedField = 'email'"
              @blur="focusedField = ''"
            />
          </view>
        </view>

        <!-- 协议 -->
        <view class="agreement">
          <checkbox-group @change="handleAgreeChange">
            <label class="agreement-label">
              <checkbox value="agree" :checked="agreed" class="agreement-checkbox" />
              <text class="agreement-text">我已阅读并同意<text class="link">《用户协议》</text>和<text class="link">《隐私政策》</text></text>
            </label>
          </checkbox-group>
        </view>

        <!-- 注册按钮 -->
        <button class="register-btn" @click="handleRegister" :loading="loading" :disabled="loading">
          <view v-if="!loading" class="btn-content">
            <text class="btn-icon">🚀</text>
            <text class="btn-text">立即注册</text>
          </view>
          <view v-else class="btn-content">
            <text class="btn-loading">⏳</text>
            <text class="btn-text">注册中...</text>
          </view>
        </button>

        <!-- 底部提示 -->
        <view class="form-footer">
          <text class="footer-text">已有账号？</text>
          <text class="footer-link" @click="goToLogin">立即登录</text>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useUserStore } from '@/store'
import { showToast } from '@/utils'

const userStore = useUserStore()

const formData = ref({
  username: '',
  password: '',
  confirmPassword: '',
  email: '',
})

const showPassword = ref(false)
const loading = ref(false)
const agreed = ref(false)
const focusedField = ref('')

// 生成粒子样式
const getParticleStyle = (index: number) => {
  const size = Math.random() * 4 + 2
  const left = Math.random() * 100
  const duration = Math.random() * 15 + 10
  const delay = Math.random() * 5
  
  return {
    width: `${size}rpx`,
    height: `${size}rpx`,
    left: `${left}%`,
    animationDuration: `${duration}s`,
    animationDelay: `${delay}s`,
  }
}

// 生成星星样式
const getStarStyle = (index: number) => {
  const left = Math.random() * 100
  const top = Math.random() * 100
  const duration = Math.random() * 3 + 2
  const delay = Math.random() * 2
  
  return {
    left: `${left}%`,
    top: `${top}%`,
    animationDuration: `${duration}s`,
    animationDelay: `${delay}s`,
  }
}

// 密码强度检测
const getPasswordStrength = () => {
  const pwd = formData.value.password
  if (!pwd) return ''
  
  let strength = 0
  if (pwd.length >= 6) strength++
  if (pwd.length >= 10) strength++
  if (/[a-z]/.test(pwd) && /[A-Z]/.test(pwd)) strength++
  if (/\d/.test(pwd)) strength++
  if (/[^a-zA-Z0-9]/.test(pwd)) strength++
  
  if (strength <= 1) return 'weak'
  if (strength <= 3) return 'medium'
  return 'strong'
}

const getPasswordStrengthText = () => {
  const strength = getPasswordStrength()
  if (strength === 'weak') return '弱'
  if (strength === 'medium') return '中'
  if (strength === 'strong') return '强'
  return ''
}

// 切换密码显示
const togglePassword = () => {
  showPassword.value = !showPassword.value
}

// 同意协议
const handleAgreeChange = (e: any) => {
  agreed.value = e.detail.value.includes('agree')
}

// 表单验证
const validate = () => {
  if (!formData.value.username) {
    showToast('请输入用户名', 'none')
    return false
  }
  if (formData.value.username.length < 3) {
    showToast('用户名至少3个字符', 'none')
    return false
  }
  if (!formData.value.password) {
    showToast('请输入密码', 'none')
    return false
  }
  if (formData.value.password.length < 6) {
    showToast('密码至少6个字符', 'none')
    return false
  }
  if (formData.value.password !== formData.value.confirmPassword) {
    showToast('两次密码不一致', 'none')
    return false
  }
  if (!agreed.value) {
    showToast('请阅读并同意用户协议', 'none')
    return false
  }
  return true
}

// 注册
const handleRegister = async () => {
  if (!validate()) return

  loading.value = true
  try {
    // 调用后端注册接口
    const success = await userStore.register(
      formData.value.username,
      formData.value.password,
      formData.value.email || undefined
    )
    
    if (success) {
      // 直接跳转到登录页，不显示提示
      uni.redirectTo({
        url: `/pages/auth/login?username=${encodeURIComponent(formData.value.username)}`,
      })
    }
  } catch (error: any) {
    showToast(error.message || '注册失败', 'none')
  } finally {
    loading.value = false
  }
}

// 返回
const goBack = () => {
  uni.navigateBack()
}

// 跳转到登录页
const goToLogin = () => {
  uni.redirectTo({
    url: '/pages/auth/login',
  })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.register-page {
  min-height: 100vh;
  position: relative;
  overflow: hidden;
  background: linear-gradient(135deg, #0a0e27 0%, #1a1f3a 50%, #0f1428 100%);
  padding: 0 30rpx;
  padding-top: 80rpx;
}

// 动态粒子背景
.particles-bg {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 1;

  .particle {
    position: absolute;
    background: radial-gradient(circle, rgba(100, 200, 255, 0.8), transparent);
    border-radius: 50%;
    animation: particleFloat linear infinite;
  }
}

@keyframes particleFloat {
  0% {
    transform: translateY(100vh) translateX(0) scale(0);
    opacity: 0;
  }
  10% {
    opacity: 1;
  }
  90% {
    opacity: 1;
  }
  100% {
    transform: translateY(-100rpx) translateX(100rpx) scale(1);
    opacity: 0;
  }
}

// 浮动星星
.floating-stars {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
  z-index: 1;

  .star {
    position: absolute;
    font-size: 32rpx;
    animation: starTwinkle ease-in-out infinite;
  }
}

@keyframes starTwinkle {
  0%, 100% {
    opacity: 0.3;
    transform: scale(1);
  }
  50% {
    opacity: 1;
    transform: scale(1.2);
  }
}

// 光晕效果
.glow-orb {
  position: fixed;
  border-radius: 50%;
  filter: blur(100rpx);
  pointer-events: none;
  z-index: 0;
  animation: orbFloat 20s ease-in-out infinite;
}

.glow-orb-1 {
  width: 600rpx;
  height: 600rpx;
  background: radial-gradient(circle, rgba(255, 0, 214, 0.3), transparent);
  top: -200rpx;
  right: -200rpx;
}

.glow-orb-2 {
  width: 500rpx;
  height: 500rpx;
  background: radial-gradient(circle, rgba(0, 217, 255, 0.25), transparent);
  bottom: -150rpx;
  left: -150rpx;
  animation-delay: -7s;
}

.glow-orb-3 {
  width: 400rpx;
  height: 400rpx;
  background: radial-gradient(circle, rgba(255, 214, 0, 0.2), transparent);
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  animation-delay: -14s;
}

@keyframes orbFloat {
  0%, 100% {
    transform: translate(0, 0) scale(1);
  }
  33% {
    transform: translate(50rpx, -50rpx) scale(1.1);
  }
  66% {
    transform: translate(-50rpx, 50rpx) scale(0.9);
  }
}

.back-btn {
  position: fixed;
  top: 40rpx;
  left: 30rpx;
  z-index: 100;
  display: flex;
  align-items: center;
  gap: 10rpx;
  padding: 16rpx 28rpx;
  background: rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(20rpx);
  border-radius: 50rpx;
  border: 1px solid rgba(255, 255, 255, 0.2);
  box-shadow: 0 8rpx 32rpx rgba(0, 0, 0, 0.3);
  transition: all 0.3s ease;

  .back-icon {
    font-size: 32rpx;
    color: #ffffff;
  }

  .back-text {
    font-size: 28rpx;
    color: #ffffff;
    font-weight: 500;
  }

  &:active {
    transform: scale(0.95);
    background: rgba(255, 255, 255, 0.15);
  }
}

.register-container {
  position: relative;
  z-index: 2;
  width: 100%;
  max-width: 600rpx;
  margin: 0 auto;
}

.logo-section {
  position: relative;
  text-align: center;
  margin-bottom: 60rpx;
  padding: 60rpx 0 40rpx;
  animation: fadeInDown 0.8s ease-out;

  .logo-glow {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 300rpx;
    height: 300rpx;
    background: radial-gradient(circle, rgba(255, 0, 214, 0.3), transparent);
    filter: blur(60rpx);
    animation: logoGlowPulse 3s ease-in-out infinite;
  }

  .title {
    position: relative;
    display: block;
    font-size: 64rpx;
    font-weight: 900;
    background: linear-gradient(135deg, #FF00D6 0%, #00D9FF 50%, #FFD600 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 20rpx;
    letter-spacing: 4rpx;
    filter: drop-shadow(0 0 30rpx rgba(255, 0, 214, 0.6));
    animation: titleGlow 3s ease-in-out infinite;
  }

  .subtitle {
    position: relative;
    display: block;
    font-size: 28rpx;
    color: #00D9FF;
    text-shadow: 
      0 0 20rpx rgba(0, 217, 255, 0.8),
      0 0 40rpx rgba(0, 217, 255, 0.4);
    letter-spacing: 2rpx;
    margin-bottom: 30rpx;
  }

  .welcome-badge {
    display: inline-flex;
    align-items: center;
    gap: 12rpx;
    padding: 12rpx 28rpx;
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.2), rgba(0, 217, 255, 0.2));
    backdrop-filter: blur(10rpx);
    border-radius: 50rpx;
    border: 1px solid rgba(255, 255, 255, 0.2);
    animation: badgeBounce 2s ease-in-out infinite;

    .badge-icon {
      font-size: 32rpx;
    }

    .badge-text {
      font-size: 24rpx;
      color: #ffffff;
      font-weight: 600;
    }
  }
}

@keyframes fadeInDown {
  from {
    opacity: 0;
    transform: translateY(-50rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes titleGlow {
  0%, 100% {
    filter: drop-shadow(0 0 30rpx rgba(255, 0, 214, 0.6));
  }
  50% {
    filter: drop-shadow(0 0 50rpx rgba(0, 217, 255, 0.8)) drop-shadow(0 0 70rpx rgba(255, 0, 214, 0.6));
  }
}

@keyframes logoGlowPulse {
  0%, 100% {
    transform: translate(-50%, -50%) scale(1);
    opacity: 0.3;
  }
  50% {
    transform: translate(-50%, -50%) scale(1.2);
    opacity: 0.5;
  }
}

@keyframes badgeBounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10rpx);
  }
}

.form-section {
  position: relative;
  background: rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(30rpx);
  border-radius: 32rpx;
  padding: 50rpx 40rpx;
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.3),
    inset 0 1rpx 0 rgba(255, 255, 255, 0.1);
  animation: formSlideUp 0.8s ease-out 0.2s both;
}

@keyframes formSlideUp {
  from {
    opacity: 0;
    transform: translateY(50rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.form-header {
  text-align: center;
  margin-bottom: 40rpx;

  .form-title {
    display: block;
    font-size: 40rpx;
    font-weight: 700;
    color: #ffffff;
    margin-bottom: 12rpx;
    text-shadow: 0 0 20rpx rgba(0, 217, 255, 0.5);
  }

  .form-subtitle {
    display: block;
    font-size: 24rpx;
    color: rgba(255, 255, 255, 0.6);
  }
}

.input-group {
  margin-bottom: 28rpx;
  transition: all 0.3s ease;

  &.input-focused {
    .input-wrapper {
      border-color: rgba(0, 217, 255, 0.6);
      box-shadow: 
        0 0 0 2rpx rgba(0, 217, 255, 0.2),
        0 8rpx 24rpx rgba(0, 217, 255, 0.2);
      transform: translateY(-2rpx);

      .input-icon-wrapper {
        background: linear-gradient(135deg, rgba(0, 217, 255, 0.3), rgba(255, 0, 214, 0.3));
      }
    }
  }

  .input-wrapper {
    display: flex;
    align-items: center;
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 20rpx;
    padding: 0 24rpx;
    height: 96rpx;
    transition: all 0.3s ease;

    .input-icon-wrapper {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 60rpx;
      height: 60rpx;
      background: rgba(255, 255, 255, 0.05);
      border-radius: 12rpx;
      margin-right: 16rpx;
      transition: all 0.3s ease;

      .input-icon {
        font-size: 36rpx;
      }
    }

    .input-field {
      flex: 1;
      font-size: 28rpx;
      color: #ffffff;
      
      &::placeholder {
        color: rgba(255, 255, 255, 0.4);
      }
    }

    .eye-icon {
      font-size: 36rpx;
      padding: 10rpx;
      cursor: pointer;
      transition: transform 0.2s ease;

      &:active {
        transform: scale(0.9);
      }
    }

    .input-check {
      font-size: 32rpx;
      color: #00ff88;
      animation: checkPop 0.3s ease;
    }
  }

  .password-strength {
    display: flex;
    align-items: center;
    gap: 16rpx;
    margin-top: 12rpx;
    padding: 0 24rpx;

    .strength-bar {
      flex: 1;
      height: 6rpx;
      background: rgba(255, 255, 255, 0.1);
      border-radius: 3rpx;
      overflow: hidden;
      position: relative;

      &::after {
        content: '';
        position: absolute;
        left: 0;
        top: 0;
        height: 100%;
        border-radius: 3rpx;
        transition: all 0.3s ease;
      }

      &.weak::after {
        width: 33%;
        background: linear-gradient(90deg, #ff4444, #ff6666);
      }

      &.medium::after {
        width: 66%;
        background: linear-gradient(90deg, #ffaa00, #ffcc00);
      }

      &.strong::after {
        width: 100%;
        background: linear-gradient(90deg, #00ff88, #00ffcc);
      }
    }

    .strength-text {
      font-size: 24rpx;
      min-width: 40rpx;
      
      &:has(.weak) {
        color: #ff6666;
      }
    }
  }
}

@keyframes checkPop {
  0% {
    transform: scale(0);
  }
  50% {
    transform: scale(1.2);
  }
  100% {
    transform: scale(1);
  }
}

.agreement {
  margin: 32rpx 0;
  
  .agreement-label {
    display: flex;
    align-items: flex-start;
    gap: 12rpx;
    font-size: 24rpx;

    .agreement-checkbox {
      margin-top: 4rpx;
    }

    .agreement-text {
      color: rgba(255, 255, 255, 0.7);
      flex: 1;
      line-height: 1.6;

      .link {
        color: #00D9FF;
        text-decoration: underline;
      }
    }
  }
}

.register-btn {
  width: 100%;
  height: 96rpx;
  background: linear-gradient(135deg, #FF00D6 0%, #00D9FF 100%);
  color: #ffffff;
  font-size: 32rpx;
  font-weight: 700;
  border-radius: 24rpx;
  border: none;
  margin-top: 8rpx;
  position: relative;
  overflow: hidden;
  box-shadow: 
    0 8rpx 24rpx rgba(255, 0, 214, 0.4),
    0 16rpx 48rpx rgba(0, 217, 255, 0.2);
  transition: all 0.3s ease;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
    transition: left 0.5s ease;
  }

  &:active {
    transform: scale(0.98);
    box-shadow: 
      0 4rpx 12rpx rgba(255, 0, 214, 0.3),
      0 8rpx 24rpx rgba(0, 217, 255, 0.15);
  }

  &:not(:disabled):active::before {
    left: 100%;
  }

  &:disabled {
    opacity: 0.6;
  }

  .btn-content {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12rpx;

    .btn-icon {
      font-size: 36rpx;
    }

    .btn-loading {
      font-size: 36rpx;
      animation: rotate 1s linear infinite;
    }

    .btn-text {
      font-size: 32rpx;
    }
  }
}

@keyframes rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.form-footer {
  text-align: center;
  margin-top: 32rpx;
  padding-top: 32rpx;
  border-top: 1px solid rgba(255, 255, 255, 0.1);

  .footer-text {
    font-size: 26rpx;
    color: rgba(255, 255, 255, 0.6);
    margin-right: 8rpx;
  }

  .footer-link {
    font-size: 26rpx;
    color: #00D9FF;
    font-weight: 600;
    text-decoration: underline;
    cursor: pointer;
    transition: all 0.2s ease;

    &:active {
      color: #FF00D6;
    }
  }
}
</style>

