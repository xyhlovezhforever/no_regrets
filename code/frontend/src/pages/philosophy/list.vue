<template>
  <view class="list-page">
    <view v-for="item in list" :key="item.id" class="list-item">
      <text class="item-title">{{ item.title }}</text>
      <text class="item-content">{{ item.content }}</text>
      <text class="item-author" v-if="item.author">—— {{ item.author }}</text>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const list = ref([
  { id: '1', title: '人生的意义', content: '人生的意义不在于我们拥有什么，而在于我们成为了什么样的人。', author: '佚名' },
  { id: '2', title: '时间的价值', content: '时间是最公平的资源，每个人每天都拥有24小时，关键在于如何使用它。', author: '佚名' },
])
</script>

<style lang="scss" scoped>
.list-page {
  min-height: 100vh;
  background: var(--theme-background);
  padding: 20rpx 30rpx;
}

.list-item {
  background: var(--theme-surface);
  border-radius: 20rpx;
  padding: 40rpx 30rpx;
  margin-bottom: 20rpx;

  .item-title {
    display: block;
    font-size: 32rpx;
    font-weight: bold;
    color: var(--theme-text);
    margin-bottom: 15rpx;
  }

  .item-content {
    display: block;
    font-size: 28rpx;
    color: var(--theme-text-secondary);
    line-height: 1.8;
    margin-bottom: 15rpx;
  }

  .item-author {
    display: block;
    font-size: 24rpx;
    color: var(--theme-text-secondary);
    font-style: italic;
    text-align: right;
  }
}
</style>

