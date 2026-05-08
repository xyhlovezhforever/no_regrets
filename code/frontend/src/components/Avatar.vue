<template>
  <view v-if="src" class="avatar-wrapper">
    <image class="avatar-image" :src="src" mode="aspectFill" @error="onImageError" />
  </view>
  <view v-else class="avatar-wrapper avatar-emoji">
    <text class="avatar-text">{{ emoji }}</text>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  src?: string
  emoji?: string
  size?: number
}

const props = withDefaults(defineProps<Props>(), {
  emoji: '👤',
  size: 120
})

const imageSrc = ref(props.src)
const hasError = ref(false)

const onImageError = () => {
  hasError.value = true
  imageSrc.value = ''
}
</script>

<style lang="scss" scoped>
.avatar-wrapper {
  width: 120rpx;
  height: 120rpx;
  flex-shrink: 0;
  border-radius: 50%;
  overflow: hidden;

  .avatar-image {
    width: 100%;
    height: 100%;
  }

  &.avatar-emoji {
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, #f0f0f0 0%, #e0e0e0 100%);
    border: 2rpx solid #e0e0e0;

    .avatar-text {
      font-size: 60rpx;
    }
  }
}
</style>

