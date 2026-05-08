<template>
  <view class="mirror-page">
    <camera
      device-position="front"
      flash="off"
      class="camera"
      @error="onCameraError"
    >
      <cover-view class="camera-overlay">
        <cover-view class="encouragement-text">
          <cover-view class="main-text">{{ currentEncouragement }}</cover-view>
          <cover-view class="sub-text" @click="changeEncouragement">点击换一句</cover-view>
        </cover-view>
      </cover-view>
    </camera>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const encouragements = [
  '你今天真好看！✨',
  '保持微笑，你最棒！😊',
  '自信的你最美丽！💖',
  '每一天都是新的开始 🌅',
  '相信自己，你能行！💪',
  '今天也要开心哦～ 🌈',
  '你的笑容很温暖 ☀️',
  '遇见更好的自己 🌟',
  '你值得所有美好 🎁',
  '保持热爱，奔赴山海 🌊',
  '愿你眼里有光，心中有爱 💫',
  '做自己的小太阳 🌞',
]

const currentEncouragement = ref('')

onMounted(() => {
  currentEncouragement.value = encouragements[Math.floor(Math.random() * encouragements.length)]
})

const onCameraError = (e: any) => {
  console.error('相机错误:', e)
  uni.showModal({
    title: '提示',
    content: '无法访问相机，请检查权限设置',
    showCancel: false
  })
}

const changeEncouragement = () => {
  const newIndex = Math.floor(Math.random() * encouragements.length)
  currentEncouragement.value = encouragements[newIndex]
}
</script>

<style lang="scss" scoped>
.mirror-page {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: #000000;
}

.camera {
  width: 100%;
  height: 100%;

  .camera-overlay {
    width: 100%;
    height: 100%;
    position: relative;
  }
}

.encouragement-text {
  position: absolute;
  top: 80rpx;
  right: 30rpx;
  max-width: 400rpx;
  text-align: right;
  padding: 20rpx 25rpx;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(20rpx);
  border-radius: 15rpx;

  .main-text {
    font-size: 26rpx;
    color: #ffffff;
    line-height: 1.4;
    text-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.5);
  }

  .sub-text {
    font-size: 22rpx;
    color: rgba(255, 255, 255, 0.6);
    margin-top: 8rpx;
    display: block;
  }
}
</style>

