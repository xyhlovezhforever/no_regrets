<template>
  <view class="list-page">
    <view
      v-for="card in cards"
      :key="card.id"
      class="card-item"
      @click="selectCard(card)"
    >
      <text class="card-icon">💪</text>
      <view class="card-info">
        <text class="card-title">{{ card.title }}</text>
        <text class="card-content">{{ card.content }}</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const cards = ref([
  { id: '1', title: '你是最棒的', content: '每一次努力都不会白费，相信自己，你一定可以的！' },
  { id: '2', title: '坚持就是胜利', content: '成功的道路充满挑战，但只要坚持，终会迎来曙光。' },
  { id: '3', title: '拥抱每一天', content: '生活总会有起伏，但每一天都值得我们用心去感受。' },
  { id: '4', title: '勇敢前行', content: '不要害怕失败，每一次尝试都是成长的机会。' },
  { id: '5', title: '珍惜当下', content: '过去已成为历史，未来还未到来，只有当下才是最真实的。' },
])

const selectCard = (card: any) => {
  uni.navigateBack()
}
</script>

<style lang="scss" scoped>
.list-page {
  min-height: 100vh;
  background: var(--theme-background);
  padding: 20rpx 30rpx;
}

.card-item {
  background: var(--theme-surface);
  border-radius: 20rpx;
  padding: 30rpx;
  margin-bottom: 20rpx;
  display: flex;
  align-items: flex-start;

  &:active {
    background: var(--theme-background);
  }

  .card-icon {
    font-size: 48rpx;
    margin-right: 20rpx;
  }

  .card-info {
    flex: 1;

    .card-title {
      display: block;
      font-size: 32rpx;
      font-weight: bold;
      color: var(--theme-text);
      margin-bottom: 10rpx;
    }

    .card-content {
      display: block;
      font-size: 26rpx;
      color: var(--theme-text-secondary);
      line-height: 1.6;
    }
  }
}
</style>

