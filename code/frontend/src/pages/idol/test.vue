<template>
  <view style="padding: 20px; background: var(--theme-background); min-height: 100vh;">
    <view style="background: white; padding: 20px; margin-bottom: 20px; border-radius: 10px;">
      <text style="display: block; font-size: 18px; font-weight: bold; margin-bottom: 10px;">
        超简化测试页面
      </text>
      <text style="display: block; margin-bottom: 10px;">
        偶像数量: {{ idols.length }}
      </text>
      <button @click="addData" style="background: #1890ff; color: white; border: none; padding: 10px 20px; border-radius: 5px;">
        添加数据
      </button>
    </view>

    <view style="background: white; padding: 20px; border-radius: 10px;">
      <text style="display: block; font-weight: bold; margin-bottom: 15px;">偶像列表：</text>
      
      <view v-for="(idol, index) in idols" :key="index" 
            style="padding: 15px; margin-bottom: 10px; background: #f9f9f9; border-radius: 8px; border: 1px solid #e0e0e0;">
        <text style="display: block; font-size: 16px; font-weight: bold; margin-bottom: 5px;">
          {{ idol.name }}
        </text>
        <text style="display: block; font-size: 14px; color: var(--theme-text-secondary);">
          {{ idol.description }}
        </text>
      </view>

      <view v-if="idols.length === 0" style="padding: 30px; text-align: center; color: var(--theme-text-secondary);">
        <text>暂无数据，点击上方按钮添加</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const idols = ref<any[]>([])

const addData = () => {
  idols.value = [
    { name: '测试偶像1', description: '这是第一个测试偶像' },
    { name: '测试偶像2', description: '这是第二个测试偶像' },
    { name: '测试偶像3', description: '这是第三个测试偶像' }
  ]
  console.log('数据已添加:', idols.value)
}
</script>
