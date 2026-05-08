<template>
  <view class="writing-create-page">
    <view class="form">
      <view class="form-item">
        <text class="label">标题</text>
        <input v-model="form.title" class="input" placeholder="给作品起个标题" maxlength="50" />
      </view>

      <view class="form-item">
        <text class="label">分类</text>
        <view class="category-list">
          <view
            v-for="cat in categories"
            :key="cat"
            class="category-tag"
            :class="{ active: form.category === cat }"
            @click="form.category = cat"
          >
            {{ cat }}
          </view>
        </view>
      </view>

      <view class="form-item">
        <text class="label">内容</text>
        <textarea
          v-model="form.content"
          class="textarea"
          placeholder="开始你的创作吧..."
          maxlength="5000"
        />
        <text class="char-count">{{ form.content.length }}/5000</text>
      </view>
    </view>

    <view class="actions">
      <button class="btn save-btn" @click="saveDraft">保存草稿</button>
      <button class="btn publish-btn" @click="publish">发布</button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { getStorage, setStorage } from '@/utils/storage'
import { generateId } from '@/utils'

const categories = ['随笔', '日记', '诗歌', '小说', '感悟', '其他']

const form = ref({
  title: '',
  category: '随笔',
  content: ''
})

const saveDraft = () => {
  if (!form.value.title || !form.value.content) {
    return uni.showToast({ title: '请填写标题和内容', icon: 'none' })
  }

  const drafts = getStorage<any[]>('writingDrafts', [])
  drafts.unshift({ ...form.value, id: generateId(), time: Date.now() })
  setStorage('writingDrafts', drafts.slice(0, 10))

  uni.showToast({ title: '草稿已保存', icon: 'success' })
}

const publish = () => {
  if (!form.value.title || !form.value.content) {
    return uni.showToast({ title: '请填写标题和内容', icon: 'none' })
  }

  const works = getStorage<any[]>('writingWorks', [])
  works.unshift({
    ...form.value,
    id: generateId(),
    author: '我',
    time: Date.now(),
    likes: 0,
    comments: []
  })
  setStorage('writingWorks', works)

  uni.showToast({ title: '发布成功', icon: 'success' })
  setTimeout(() => {
    uni.navigateBack()
  }, 1000)
}
</script>

<style lang="scss" scoped>
.writing-create-page {
  min-height: 100vh;
  background: var(--theme-background);
  padding: 30rpx;
}

.form {
  .form-item {
    margin-bottom: 30rpx;

    .label {
      display: block;
      font-size: 28rpx;
      color: var(--theme-text-secondary);
      margin-bottom: 15rpx;
    }

    .input,
    .textarea {
      width: 100%;
      padding: 20rpx;
      background: var(--theme-surface);
      border-radius: 15rpx;
      font-size: 28rpx;
      border: 1rpx solid #e0e0e0;
    }

    .textarea {
      min-height: 400rpx;
      line-height: 1.8;
    }

    .char-count {
      display: block;
      text-align: right;
      font-size: 22rpx;
      color: var(--theme-text-secondary);
      margin-top: 10rpx;
    }

    .category-list {
      display: flex;
      flex-wrap: wrap;
      gap: 15rpx;

      .category-tag {
        padding: 15rpx 30rpx;
        background: var(--theme-surface);
        border-radius: 50rpx;
        font-size: 26rpx;
        color: var(--theme-text-secondary);
        border: 2rpx solid #e0e0e0;

        &.active {
          background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
          color: #ffffff;
          border-color: var(--theme-primary);
        }
      }
    }
  }
}

.actions {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 20rpx 30rpx 40rpx;
  background: var(--theme-surface);
  box-shadow: 0 -5rpx 15rpx rgba(0, 0, 0, 0.05);
  display: flex;
  gap: 20rpx;

  .btn {
    flex: 1;
    height: 80rpx;
    border-radius: 50rpx;
    font-size: 30rpx;
    border: none;
    line-height: 80rpx;

    &::after { border: none; }
  }

  .save-btn {
    background: var(--theme-background);
    color: var(--theme-text-secondary);
  }

  .publish-btn {
    background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
    color: #ffffff;
  }
}
</style>
