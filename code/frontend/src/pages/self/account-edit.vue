<template>
  <view class="account-edit-page">
    <view class="type-switch">
      <view
        class="type-item"
        :class="{ active: formData.record_type === 'expense' }"
        @click="formData.record_type = 'expense'"
      >
        <text class="type-icon">💸</text>
        <text class="type-text">支出</text>
      </view>
      <view
        class="type-item"
        :class="{ active: formData.record_type === 'income' }"
        @click="formData.record_type = 'income'"
      >
        <text class="type-icon">💰</text>
        <text class="type-text">收入</text>
      </view>
    </view>

    <view class="amount-section">
      <text class="amount-label">金额</text>
      <view class="amount-input-wrapper">
        <text class="currency">¥</text>
        <input
          v-model="formData.amount"
          class="amount-input"
          type="digit"
          placeholder="0.00"
          :placeholder-style="'color: #cccccc; font-size: 70rpx;'"
          @input="handleAmountInput"
          @blur="handleAmountBlur"
        />
      </view>
    </view>

    <view class="category-section">
      <text class="section-label">分类</text>
      <view class="category-list">
        <view
          v-for="cat in currentCategories"
          :key="cat"
          class="category-item"
          :class="{ active: formData.category === cat }"
          @click="formData.category = cat"
        >
          <text>{{ cat }}</text>
        </view>
      </view>
    </view>

    <view class="form-section">
      <view class="form-item">
        <text class="form-label">日期</text>
        <picker mode="date" :value="formData.date" @change="onDateChange">
          <view class="picker-value">{{ formData.date || '选择日期' }}</view>
        </picker>
      </view>

      <view class="form-item">
        <text class="form-label">备注</text>
        <input
          v-model="formData.description"
          class="form-input"
          placeholder="添加备注"
          maxlength="100"
        />
      </view>
    </view>

    <view class="action-buttons">
      <button v-if="recordId" class="btn btn-danger" @click="deleteRecord">删除</button>
      <button class="btn btn-primary" @click="saveRecord">保存</button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { formatDate } from '@/utils'
import { 
  createAccountRecordApi, 
  updateAccountRecordApi, 
  deleteAccountRecordApi,
  getAccountRecordApi 
} from '@/api/account'

interface AccountRecord {
  id?: string
  record_type: 'income' | 'expense'
  category: string
  description: string
  amount: number
  date: string
}

const recordId = ref('')
// 获取今天的日期字符串
const getTodayDate = () => {
  const today = new Date()
  const year = today.getFullYear()
  const month = String(today.getMonth() + 1).padStart(2, '0')
  const day = String(today.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

const formData = ref<AccountRecord>({
  record_type: 'expense',
  category: '',
  description: '',
  amount: 0,
  date: getTodayDate()
})
const loading = ref(false)

const expenseCategories = ['餐饮', '交通', '购物', '娱乐', '医疗', '教育', '居住', '其他']
const incomeCategories = ['工资', '奖金', '兼职', '投资', '红包', '其他']

const currentCategories = computed(() => {
  return formData.value.record_type === 'expense' ? expenseCategories : incomeCategories
})

// 当切换类型时，重置分类
watch(() => formData.value.record_type, () => {
  formData.value.category = currentCategories.value[0]
})

onLoad((options: any) => {
  if (options.id) {
    recordId.value = options.id
    loadRecord()
  } else {
    // 新建记录时设置默认分类和日期为今天
    formData.value.category = currentCategories.value[0]
    formData.value.date = getTodayDate()
    // console.log('新建记录，设置日期为今天:', formData.value.date)
  }
})

onMounted(() => {
  // 确保新建记录时日期为今天
  if (!recordId.value && !formData.value.date) {
    formData.value.date = getTodayDate()
    // console.log('onMounted: 设置日期为今天:', formData.value.date)
  }
})

const loadRecord = async () => {
  try {
    const res = await getAccountRecordApi(recordId.value)
    // console.log('加载记录响应:', res)
    
    // 响应拦截器已经返回了 data.data，所以 res 就是数据本身
    if (res) {
      formData.value = {
        record_type: res.record_type as 'income' | 'expense',
        category: res.category || '',
        description: res.description || '',
        amount: (res.amount || 0) / 100, // 从分转换为元
        date: res.date || getTodayDate()
      }
      // console.log('加载后的表单数据:', formData.value)
      // console.log('金额:', formData.value.amount)
      // console.log('日期:', formData.value.date)
    } else {
      // console.warn('记录数据为空')
      uni.showToast({ title: '记录不存在', icon: 'none' })
    }
  } catch (error) {
    console.error('加载记录失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  }
}

const onDateChange = (e: any) => {
  formData.value.date = e.detail.value
}

// 处理金额输入，只允许数字和小数点
const handleAmountInput = (e: any) => {
  let value = String(e.detail?.value || '')
  const originalValue = value
  
  // 检查是否包含非数字和小数点的字符
  const hasInvalidChars = /[^\d.]/.test(value)
  
  // 移除所有非数字和小数点的字符
  value = value.replace(/[^\d.]/g, '')
  
  // 如果有无效字符，显示提示
  if (hasInvalidChars && value !== originalValue) {
    uni.showToast({
      title: '只能输入数字',
      icon: 'none',
      duration: 1500
    })
  }
  
  // 只允许一个小数点
  const parts = value.split('.')
  if (parts.length > 2) {
    value = parts[0] + '.' + parts.slice(1).join('')
  }
  
  // 限制小数点后最多2位
  if (parts.length === 2 && parts[1].length > 2) {
    value = parts[0] + '.' + parts[1].substring(0, 2)
    uni.showToast({
      title: '最多保留两位小数',
      icon: 'none',
      duration: 1500
    })
  }
  
  // 限制最大金额（999999.99）
  const numValue = parseFloat(value)
  if (value && !isNaN(numValue) && numValue > 999999.99) {
    value = '999999.99'
    uni.showToast({
      title: '最大金额为999999.99',
      icon: 'none',
      duration: 1500
    })
  }
  
  // 更新表单数据（保持为数字类型）
  if (value === '' || value === '.') {
    formData.value.amount = 0
  } else {
    const num = parseFloat(value)
    formData.value.amount = isNaN(num) ? 0 : num
  }
}

// 处理金额失焦，格式化显示
const handleAmountBlur = () => {
  if (formData.value.amount && formData.value.amount > 0) {
    // 确保是数字类型
    const numValue = parseFloat(String(formData.value.amount))
    if (!isNaN(numValue)) {
      formData.value.amount = numValue
    }
  }
}

const saveRecord = async () => {
  if (!formData.value.amount || formData.value.amount <= 0) {
    uni.showToast({ title: '请输入金额', icon: 'none' })
    return
  }

  if (!formData.value.category) {
    uni.showToast({ title: '请选择分类', icon: 'none' })
    return
  }

  try {
    loading.value = true
    
    const data = {
      record_type: formData.value.record_type,
      amount: Math.round(Number(formData.value.amount) * 100), // 转换为分（后端存储单位）
      category: formData.value.category,
      description: formData.value.description,
      date: formData.value.date
    }

    if (recordId.value) {
      // 更新
      await updateAccountRecordApi(recordId.value, data)
      uni.showToast({ title: '更新成功', icon: 'success' })
    } else {
      // 新增
      await createAccountRecordApi(data)
      uni.showToast({ title: '保存成功', icon: 'success' })
    }

    // 触发刷新事件
    uni.$emit('refreshAccountList')

    setTimeout(() => {
      uni.navigateBack()
    }, 500)
  } catch (error: any) {
    console.error('保存失败:', error)
    if (error?.message?.includes('401') || error?.message?.includes('Unauthorized')) {
      uni.showToast({ title: '请先登录', icon: 'none' })
    } else {
      uni.showToast({ title: '保存失败', icon: 'none' })
    }
  } finally {
    loading.value = false
  }
}

const deleteRecord = () => {
  uni.showModal({
    title: '确认删除',
    content: '确定要删除这条记录吗？',
    success: async (res) => {
      if (res.confirm) {
        try {
          await deleteAccountRecordApi(recordId.value)
          uni.showToast({ title: '已删除', icon: 'success' })
          
          // 触发刷新事件
          uni.$emit('refreshAccountList')
          
          setTimeout(() => {
            uni.navigateBack()
          }, 500)
        } catch (error) {
          console.error('删除失败:', error)
          uni.showToast({ title: '删除失败', icon: 'none' })
        }
      }
    }
  })
}
</script>

<style lang="scss" scoped>
.account-edit-page {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  animation: fadeIn 0.3s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.type-switch {
  display: flex;
  background: var(--theme-surface);
  padding: 30rpx;
  gap: 20rpx;
  box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.05);

  .type-item {
    flex: 1;
    padding: 35rpx 0;
    border-radius: 20rpx;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12rpx;
    background: #f5f7fa;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    border: 2rpx solid transparent;

    &:active {
      transform: scale(0.95);
    }

    &.active {
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      border-color: transparent;
      box-shadow: 0 8rpx 20rpx rgba(102, 126, 234, 0.3);
      transform: scale(1.02);

      .type-icon,
      .type-text {
        color: #ffffff;
      }
    }

    .type-icon {
      font-size: 52rpx;
      transition: transform 0.3s ease;
    }

    .type-text {
      font-size: 28rpx;
      font-weight: 500;
      color: var(--theme-text-secondary);
      transition: color 0.3s ease;
    }

    &.active .type-icon {
      transform: scale(1.1);
    }
  }
}

.amount-section {
  background: var(--theme-surface);
  padding: 45rpx 35rpx 60rpx;
  margin-top: 20rpx;
  box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.05);

  .amount-label {
    display: block;
    font-size: 28rpx;
    color: var(--theme-text-secondary);
    margin-bottom: 30rpx;
    font-weight: 500;
  }

  .amount-input-wrapper {
    display: flex;
    align-items: center;
    min-height: 120rpx;
    padding: 10rpx 0;

    .currency {
      font-size: 80rpx;
      color: var(--theme-primary);
      font-weight: bold;
      margin-right: 20rpx;
      flex-shrink: 0;
    }

    .amount-input {
      flex: 1;
      font-size: 90rpx;
      font-weight: bold;
      color: var(--theme-text);
      line-height: 1.2;
      min-width: 0;
      width: 100%;
      height: 120rpx;
      padding: 0;
      text-align: left;
    }
  }
}

.category-section {
  background: var(--theme-surface);
  padding: 35rpx;
  margin-top: 20rpx;
  box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.05);

  .section-label {
    display: block;
    font-size: 30rpx;
    color: var(--theme-text);
    font-weight: bold;
    margin-bottom: 25rpx;
  }

  .category-list {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 20rpx;

    .category-item {
      padding: 22rpx;
      border-radius: 18rpx;
      background: #f5f7fa;
      text-align: center;
      font-size: 26rpx;
      color: var(--theme-text-secondary);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      border: 2rpx solid transparent;

      &.active {
        background: var(--theme-primary);
        color: #ffffff;
        border-color: var(--theme-primary);
        box-shadow: 0 4rpx 12rpx rgba(102, 126, 234, 0.3);
        transform: scale(1.05);
      }

      &:active {
        transform: scale(0.95);
      }
    }
  }
}

.form-section {
  background: var(--theme-surface);
  padding: 35rpx;
  margin-top: 20rpx;
  box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.05);

  .form-item {
    display: flex;
    align-items: center;
    padding: 28rpx 0;
    border-bottom: 1rpx solid #f0f0f0;

    &:last-child {
      border-bottom: none;
    }

    .form-label {
      font-size: 30rpx;
      font-weight: 500;
      color: var(--theme-text);
      width: 120rpx;
    }

    .picker-value,
    .form-input {
      flex: 1;
      font-size: 28rpx;
      color: var(--theme-text-secondary);
      text-align: right;
    }

    .picker-value {
      padding: 10rpx 0;
    }
  }
}

.action-buttons {
  display: flex;
  gap: 20rpx;
  padding: 50rpx 30rpx;

  .btn {
    flex: 1;
    height: 95rpx;
    border-radius: 50rpx;
    border: none;
    font-size: 32rpx;
    font-weight: 500;
    line-height: 95rpx;
    transition: all 0.3s ease;
    
    &::after {
      border: none;
    }

    &.btn-primary {
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      color: #ffffff;
      box-shadow: 0 8rpx 20rpx rgba(102, 126, 234, 0.3);

      &:active {
        transform: scale(0.98);
        box-shadow: 0 4rpx 12rpx rgba(102, 126, 234, 0.4);
      }
    }

    &.btn-danger {
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      color: #ffffff;
      box-shadow: 0 8rpx 20rpx rgba(255, 77, 79, 0.25);

      &:active {
        transform: scale(0.98);
        box-shadow: 0 4rpx 12rpx rgba(255, 77, 79, 0.35);
      }
    }
  }
}
</style>

