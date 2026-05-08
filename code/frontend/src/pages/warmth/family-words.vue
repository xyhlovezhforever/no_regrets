<template>
  <view class="page">
    <view class="tabs">
      <view
        v-for="(tab, index) in tabs"
        :key="index"
        class="tab-item"
        :class="{ active: activeTab === index }"
        @click="activeTab = index"
      >
        {{ tab }}
      </view>
    </view>

    <view class="content">
      <textarea
        class="textarea"
        v-model="content"
        :placeholder="activeTab === 0 ? '对孩子说的话...' : '对父母说的话...'"
        :maxlength="500"
      />
      <button class="btn-generate" @click="generate">AI 优化润色</button>
      <view class="result" v-if="result">
        <textarea class="result-text" :value="result" :auto-height="true" />
        <button class="btn-send" @click="doubleTapToSend">双击确认发送</button>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const tabs = ['对孩子说', '对父母说']
const activeTab = ref(0)
const content = ref('')
const result = ref('')

const generate = () => {
  if (!content.value.trim()) {
    return uni.showToast({ title: '请输入内容', icon: 'none' })
  }
  result.value = `亲爱的，${content.value}\n\n让我们一起成长！`
}

let tapCount = 0
let tapTimer: any = null

const doubleTapToSend = () => {
  tapCount++
  if (tapCount === 1) {
    tapTimer = setTimeout(() => {
      tapCount = 0
    }, 300)
  } else if (tapCount === 2) {
    clearTimeout(tapTimer)
    tapCount = 0
    uni.showToast({ title: '发送成功', icon: 'success' })
  }
}
</script>

<style lang="scss" scoped>
.page {
  min-height: 100vh;
  background: var(--theme-background);
}

.tabs {
  display: flex;
  background: var(--theme-surface);
  padding: 20rpx;

  .tab-item {
    flex: 1;
    text-align: center;
    padding: 20rpx;
    font-size: 28rpx;
    color: var(--theme-text-secondary);

    &.active {
      color: #fa709a;
      font-weight: bold;
      border-bottom: 4rpx solid #fa709a;
    }
  }
}

.content {
  padding: 30rpx;

  .textarea {
    width: 100%;
    min-height: 300rpx;
    background: var(--theme-surface);
    border-radius: 15rpx;
    padding: 30rpx;
    font-size: 28rpx;
    line-height: 1.8;
    margin-bottom: 20rpx;
  }

  .btn-generate {
    width: 100%;
    height: 90rpx;
    background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
    color: #ffffff;
    border-radius: 50rpx;
    border: none;
    font-size: 32rpx;
    font-weight: bold;
  }

  .result {
    margin-top: 40rpx;

    .result-text {
      width: 100%;
      min-height: 400rpx;
      background: var(--theme-surface);
      border-radius: 15rpx;
      padding: 30rpx;
      font-size: 28rpx;
      line-height: 1.8;
      margin-bottom: 20rpx;
    }

    .btn-send {
      width: 100%;
      height: 80rpx;
      background: #fa709a;
      color: #ffffff;
      border-radius: 50rpx;
      border: none;
      font-size: 30rpx;
    }
  }
}
</style>

