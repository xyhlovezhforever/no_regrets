<template>
  <view class="binding-page">
    <!-- 动态背景 -->
    <view class="bg-gradient">
      <view class="bg-orb orb-1"></view>
      <view class="bg-orb orb-2"></view>
      <view class="bg-orb orb-3"></view>
    </view>

    <!-- 添加按钮（右上角） -->
    <view class="add-floating-btn" @click="showAddDialog">
      <text class="add-icon">+</text>
    </view>

    <!-- Tab 切换 -->
    <view class="tabs">
      <view class="tab-bg-indicator" :style="{ transform: `translateX(${activeTab === 'bindings' ? '0' : '100%'})` }"></view>
      <view
        v-for="tab in tabs"
        :key="tab.value"
        :class="['tab-item', { active: activeTab === tab.value }]"
        @click="switchTab(tab.value)"
      >
        <text class="tab-text">{{ tab.label }}</text>
        <view v-if="tab.value === 'requests' && pendingCount > 0" class="badge">
          {{ pendingCount }}
        </view>
      </view>
    </view>

    <!-- 绑定列表 -->
    <view v-if="activeTab === 'bindings'" class="content">
      <view v-if="bindings.length === 0" class="empty">
        <view class="empty-icon">🔗</view>
        <text class="empty-text">暂无绑定关系</text>
        <text class="empty-desc">与好友互相绑定，共享数据权限</text>
        <button class="add-btn" @click="showAddDialog">
          <text class="btn-icon">✨</text>
          <text>立即绑定</text>
        </button>
      </view>
      <view v-else class="binding-list">
        <view
          v-for="(binding, index) in bindings"
          :key="binding.id"
          class="binding-item"
          :style="{ animationDelay: `${index * 0.1}s` }"
          @click="goToDetail(binding)"
        >
          <view class="item-glow"></view>
          <view class="avatar-wrapper">
            <image
              :src="binding.bound_user_info?.avatar || '/static/default-avatar.png'"
              class="avatar"
            />
            <view class="avatar-ring"></view>
          </view>
          <view class="info">
            <view class="name">{{ binding.bound_user_info?.nickname || '未知用户' }}</view>
            <view class="time">
              <text class="time-icon">⏰</text>
              <text>{{ formatTime(binding.created_at) }}</text>
            </view>
          </view>
          <view class="action">
            <view class="action-btns">
              <view class="manage-btn" @click.stop="goToDetail(binding)">
                <text class="btn-icon">⚙️</text>
                <text class="btn-text">权限</text>
              </view>
              <view class="data-btn" @click.stop="goToData(binding)">
                <text class="btn-icon">📊</text>
                <text class="btn-text">数据</text>
              </view>
            </view>
          </view>
        </view>
      </view>
    </view>

    <!-- 待处理请求 -->
    <view v-if="activeTab === 'requests'" class="content">
      <view v-if="pendingRequests.length === 0" class="empty">
        <view class="empty-icon">📬</view>
        <text class="empty-text">暂无待处理请求</text>
        <text class="empty-desc">等待好友的绑定邀请</text>
      </view>
      <view v-else class="request-list">
        <view 
          v-for="(request, index) in pendingRequests" 
          :key="request.id" 
          class="request-item"
          :style="{ animationDelay: `${index * 0.1}s` }"
        >
          <view class="request-glow"></view>
          <view class="avatar-wrapper">
            <image
              :src="request.user_info?.avatar || '/static/default-avatar.png'"
              class="avatar"
            />
            <view class="avatar-ring"></view>
            <view class="request-pulse"></view>
          </view>
          <view class="info">
            <view class="name">{{ request.user_info?.nickname || '未知用户' }}</view>
            <view v-if="request.message" class="message">
              <text class="message-icon">💌</text>
              <text>{{ request.message }}</text>
            </view>
            <view class="time">
              <text class="time-icon">⏰</text>
              <text>{{ formatTime(request.created_at) }}</text>
            </view>
          </view>
          <view class="actions">
            <button class="accept-btn" @click.stop="handleRequest(request.id, 'accept')">
              <text class="btn-icon">✓</text>
              <text>接受</text>
            </button>
            <button class="reject-btn" @click.stop="handleRequest(request.id, 'reject')">
              <text class="btn-icon">✕</text>
              <text>拒绝</text>
            </button>
          </view>
        </view>
      </view>
    </view>

    <!-- 添加绑定对话框 -->
    <view v-if="showAdd" class="dialog-mask" @click="closeAddDialog">
      <view class="dialog-bg-blur"></view>
      <view class="dialog" @click.stop>
        <view class="dialog-glow"></view>
        <view class="dialog-header">
          <text class="dialog-title-icon">✨</text>
          <text class="dialog-title">添加绑定</text>
          <view class="close-btn" @click="closeAddDialog">
            <text>✕</text>
          </view>
        </view>
        <view class="dialog-content">
          <!-- 选择好友 -->
          <view class="select-box">
            <view class="label">
              <text class="label-icon">👥</text>
              <text>选择好友</text>
            </view>
            
            <!-- 好友列表 -->
            <view v-if="friendList.length > 0" class="friend-list">
              <view
                v-for="(friend, index) in friendList"
                :key="friend.id || friend.user_id"
                :class="['friend-item', { selected: selectedFriendIndex === index }]"
                @click="selectFriend(index)"
              >
                <view class="friend-avatar-wrapper">
                  <image 
                    :src="friend.avatar || '/static/default-avatar.png'" 
                    class="friend-avatar" 
                  />
                  <view v-if="selectedFriendIndex === index" class="check-mark">✓</view>
                </view>
                <view class="friend-info">
                  <view class="friend-name">{{ friend.nickname }}</view>
                  <view class="friend-username">@{{ friend.username }}</view>
                </view>
              </view>
            </view>
            
            <!-- 无好友提示 -->
            <view v-else class="no-friends">
              <text class="no-friends-icon">😢</text>
              <text class="no-friends-text">暂无好友</text>
              <text class="no-friends-hint">请先添加好友</text>
            </view>
          </view>

          <!-- 留言输入框 -->
          <view class="message-box">
            <view class="label">
              <text class="label-icon">💬</text>
              <text>留言（可选）</text>
            </view>
            <textarea
              v-model="bindingMessage"
              class="message-input"
              placeholder="添加留言，让对方知道你的想法..."
              maxlength="200"
            />
            <view class="char-count">{{ bindingMessage.length }}/200</view>
          </view>
        </view>
        <view class="dialog-actions">
          <view class="action-btn cancel-btn" @click="closeAddDialog">
            <text class="btn-icon">✕</text>
            <text class="btn-text">取消</text>
          </view>
          <view 
            :class="['action-btn', 'confirm-btn', { disabled: !selectedUser }]" 
            @click="sendBindingRequest"
          >
            <text class="btn-icon">✨</text>
            <text class="btn-text">发送请求</text>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { onShow } from '@dcloudio/uni-app'
import { 
  getMyBindingsApi, 
  getPendingBindingRequestsApi, 
  sendBindingRequestApi,
  handleBindingRequestApi 
} from '@/api/binding'
import { getFriendsApi } from '@/api/friend'
import type { AccountBinding } from '@/types'
import { showToast, showLoading, hideLoading } from '@/utils'
import { useUserStore } from '@/store/user'

const activeTab = ref('bindings')
const tabs = [
  { label: '我的绑定', value: 'bindings' },
  { label: '待处理', value: 'requests' },
]

const bindings = ref<AccountBinding[]>([])
const pendingRequests = ref<AccountBinding[]>([])
const pendingCount = ref(0)

const showAdd = ref(false)
const friendList = ref<any[]>([])
const selectedFriendIndex = ref(-1)
const selectedUser = ref<any>(null)
const bindingMessage = ref('')

onMounted(() => {
  loadBindings()
  loadPendingRequests()
})

const switchTab = (tab: string) => {
  activeTab.value = tab
  
  // 切换到待处理标签时，重新加载数据
  if (tab === 'requests') {
    console.log('切换到待处理标签，重新加载数据')
    loadPendingRequests()
  } else if (tab === 'bindings') {
    console.log('切换到我的绑定标签，重新加载数据')
    loadBindings()
  }
}

const loadBindings = async () => {
  try {
    console.log('=== 开始加载我的绑定列表 ===')
    showLoading('加载中...')
    
    // 响应拦截器已经解包了数据，直接使用返回值
    const bindings_data = await getMyBindingsApi()
    console.log('API 返回的绑定数据:', bindings_data)
    console.log('数据类型:', typeof bindings_data)
    console.log('是否为数组:', Array.isArray(bindings_data))
    
    // 确保是数组
    if (Array.isArray(bindings_data)) {
      bindings.value = bindings_data
    } else {
      console.error('绑定数据格式错误:', bindings_data)
      bindings.value = []
    }
    
    console.log('最终的绑定列表:', bindings.value)
    console.log('绑定数量:', bindings.value.length)
  } catch (error: any) {
    console.error('加载绑定列表失败:', error)
    showToast(error.message || '加载失败')
    bindings.value = []
  } finally {
    hideLoading()
  }
}

const loadPendingRequests = async () => {
  try {
    console.log('=== 开始加载待处理的绑定请求 ===')
    
    // 响应拦截器已经解包了数据，直接使用返回值
    const requests_data = await getPendingBindingRequestsApi()
    console.log('API 返回的待处理请求数据:', requests_data)
    console.log('数据类型:', typeof requests_data)
    console.log('是否为数组:', Array.isArray(requests_data))
    
    // 确保是数组
    if (Array.isArray(requests_data)) {
      pendingRequests.value = requests_data
    } else {
      console.error('待处理请求数据格式错误:', requests_data)
      pendingRequests.value = []
    }
    
    pendingCount.value = pendingRequests.value.length
    
    console.log('最终的待处理请求列表:', pendingRequests.value)
    console.log(`找到 ${pendingCount.value} 个待处理请求`)
    
    if (pendingCount.value > 0) {
      console.log('第一个待处理请求:', pendingRequests.value[0])
      console.log('请求字段:', Object.keys(pendingRequests.value[0]))
    } else {
      console.log('✅ 没有待处理的请求')
    }
  } catch (error: any) {
    console.error('加载待处理请求失败:', error)
    console.error('错误详情:', error.response || error)
    showToast('加载待处理请求失败')
    pendingRequests.value = []
    pendingCount.value = 0
  }
}

const handleRequest = async (id: string, action: 'accept' | 'reject') => {
  try {
    showLoading(action === 'accept' ? '接受中...' : '拒绝中...')
    await handleBindingRequestApi(id, { action })
    showToast(action === 'accept' ? '已接受' : '已拒绝')
    loadPendingRequests()
    if (action === 'accept') {
      loadBindings()
    }
  } catch (error: any) {
    showToast(error.message || '操作失败')
  } finally {
    hideLoading()
  }
}

const showAddDialog = async () => {
  showAdd.value = true
  selectedUser.value = null
  selectedFriendIndex.value = -1
  bindingMessage.value = ''
  
  // 加载好友列表
  await loadFriends()
}

const closeAddDialog = () => {
  showAdd.value = false
}

const loadFriends = async () => {
  try {
    console.log('=== 开始加载好友列表 ===')
    showLoading('加载好友列表...')
    
    // getFriendsApi 返回的数据已经被响应拦截器解包了
    // 所以 res 直接就是数据数组，不需要 res.data
    const friends = await getFriendsApi({ showLoading: false })
    console.log('API 返回的好友数据:', friends)
    console.log('数据类型:', typeof friends)
    console.log('是否为数组:', Array.isArray(friends))
    
    // 确保是数组
    if (Array.isArray(friends)) {
      friendList.value = friends
    } else if (friends && typeof friends === 'object') {
      // 如果不是数组，尝试从对象中提取
      if (Array.isArray(friends.list)) {
        friendList.value = friends.list
      } else if (Array.isArray(friends.data)) {
        friendList.value = friends.data
      } else {
        console.error('无法从响应中提取好友列表:', friends)
        friendList.value = []
      }
    } else {
      friendList.value = []
    }
    
    console.log('最终的好友列表:', friendList.value)
    console.log('好友数量:', friendList.value.length)
    
    if (friendList.value.length > 0) {
      console.log('第一个好友数据:', friendList.value[0])
      console.log('好友字段:', Object.keys(friendList.value[0]))
      showToast(`成功加载 ${friendList.value.length} 个好友`)
    } else {
      console.warn('好友列表为空')
      showToast('您还没有好友，请先添加好友')
    }
  } catch (error: any) {
    console.error('加载好友列表失败:', error)
    console.error('错误详情:', error.response || error)
    friendList.value = []
    showToast('加载好友列表失败: ' + (error.message || '未知错误'))
  } finally {
    hideLoading()
  }
}

const selectFriend = (index: number) => {
  selectedFriendIndex.value = index
  selectedUser.value = friendList.value[index]
  console.log('选择的好友:', selectedUser.value)
}

const sendBindingRequest = async () => {
  console.log('点击发送请求按钮')
  console.log('selectedUser:', selectedUser.value)
  
  if (!selectedUser.value) {
    showToast('请先选择好友')
    return
  }

  // 获取用户ID，兼容不同的字段名
  const userId = selectedUser.value.user_id || selectedUser.value.id
  
  if (!userId) {
    console.error('无法获取用户ID，selectedUser:', selectedUser.value)
    showToast('用户ID无效')
    return
  }

  try {
    showLoading('发送中...')
    
    const requestData = {
      bound_user_id: userId,
      message: bindingMessage.value.trim() || undefined,
    }
    
    console.log('发送绑定请求:', requestData)
    
    const response = await sendBindingRequestApi(requestData)
    
    console.log('绑定请求响应:', response)
    
    showToast('绑定请求已发送成功')
    closeAddDialog()
    
    // 可选：刷新列表
    setTimeout(() => {
      loadBindings()
    }, 500)
  } catch (error: any) {
    console.error('发送绑定请求失败:', error)
    console.error('错误详情:', error.response || error)
    showToast(error.message || '发送失败，请重试')
  } finally {
    hideLoading()
  }
}

const goToDetail = (binding: AccountBinding) => {
  // 获取对方用户的ID（不是自己的ID）
  const userStore = useUserStore()
  const currentUserId = userStore.userId
  
  // 判断对方是谁
  const otherUserId = binding.user_id === currentUserId 
    ? binding.bound_user_id 
    : binding.user_id
  
  uni.navigateTo({
    url: `/pages/binding/detail?userId=${otherUserId}`,
  })
}

const goToData = (binding: AccountBinding) => {
  // 获取对方用户的ID（不是自己的ID）
  const userStore = useUserStore()
  const currentUserId = userStore.userId
  
  // 判断对方是谁
  const otherUserId = binding.user_id === currentUserId 
    ? binding.bound_user_id 
    : binding.user_id
  
  const userInfo = encodeURIComponent(JSON.stringify(binding.bound_user_info))
  uni.navigateTo({
    url: `/pages/binding/data?userId=${otherUserId}&userInfo=${userInfo}`,
  })
}

const formatTime = (time: string) => {
  const date = new Date(time)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const day = 24 * 60 * 60 * 1000

  if (diff < day) {
    return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
  } else if (diff < 2 * day) {
    return '昨天'
  } else {
    return date.toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit' })
  }
}

onShow(() => {
  // TabBar组件会自动检测当前页面路由
})
</script>

<style scoped lang="scss">
.binding-page {
  min-height: 100vh;
  position: relative;
  overflow: hidden;
  background: linear-gradient(135deg, #0a0e27 0%, #1a1f3a 50%, #0f1429 100%);
}

// 动态背景
.bg-gradient {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 0;
  pointer-events: none;
  overflow: hidden;
}

.bg-orb {
  position: absolute;
  border-radius: 50%;
  filter: blur(80rpx);
  opacity: 0.4;
  animation: float 20s ease-in-out infinite;
  
  &.orb-1 {
    width: 600rpx;
    height: 600rpx;
    top: -200rpx;
    left: -100rpx;
    background: radial-gradient(circle, rgba(16, 185, 129, 0.6) 0%, transparent 70%);
    animation-delay: 0s;
  }
  
  &.orb-2 {
    width: 800rpx;
    height: 800rpx;
    top: 40%;
    right: -200rpx;
    background: radial-gradient(circle, rgba(139, 92, 246, 0.5) 0%, transparent 70%);
    animation-delay: 5s;
  }
  
  &.orb-3 {
    width: 500rpx;
    height: 500rpx;
    bottom: -100rpx;
    left: 30%;
    background: radial-gradient(circle, rgba(0, 217, 255, 0.4) 0%, transparent 70%);
    animation-delay: 10s;
  }
}

@keyframes float {
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

// 添加按钮（右上角固定）
.add-floating-btn {
  position: fixed;
  top: 20rpx;
  right: 20rpx;
  width: 80rpx;
  height: 80rpx;
  border-radius: 50%;
  background: linear-gradient(135deg, #10B981 0%, #059669 100%);
  display: flex;
    align-items: center;
    justify-content: center;
    box-shadow: 
      0 8rpx 30rpx rgba(16, 185, 129, 0.6),
      0 0 40rpx rgba(16, 185, 129, 0.4),
      inset 0 2rpx 10rpx rgba(255, 255, 255, 0.3);
    transition: all 0.3s ease;
    
    .add-icon {
  font-size: 48rpx;
  color: #fff;
  font-weight: bold;
  text-shadow: 0 2rpx 10rpx rgba(0, 0, 0, 0.3);
  }
  
  &:active {
    transform: scale(0.9);
    box-shadow: 
      0 4rpx 20rpx rgba(16, 185, 129, 0.8),
      0 0 30rpx rgba(16, 185, 129, 0.6);
  }
  
  z-index: 1000;
}

// Tab 切换
.tabs {
  position: relative;
  z-index: 10;
  display: flex;
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(139, 92, 246, 0.08) 100%);
  backdrop-filter: blur(20rpx);
  padding: 10rpx;
  margin: 20rpx 20rpx 0;
  border-radius: 20rpx;
  box-shadow: 
    0 8rpx 32rpx rgba(0, 0, 0, 0.3),
    inset 0 2rpx 10rpx rgba(16, 185, 129, 0.1);

  .tab-bg-indicator {
    position: absolute;
    top: 10rpx;
    left: 10rpx;
    width: calc(50% - 10rpx);
    height: calc(100% - 20rpx);
    background: linear-gradient(135deg, rgba(16, 185, 129, 0.3) 0%, rgba(16, 185, 129, 0.2) 100%);
    border-radius: 15rpx;
    transition: transform 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    box-shadow: 
      0 4rpx 20rpx rgba(16, 185, 129, 0.4),
      0 0 30rpx rgba(16, 185, 129, 0.2);
    z-index: 0;
  }

  .tab-item {
    flex: 1;
    padding: 25rpx 0;
    text-align: center;
    position: relative;
    z-index: 1;
    transition: all 0.3s ease;

    .tab-text {
      font-size: 28rpx;
      color: rgba(255, 255, 255, 0.6);
      font-weight: 600;
      transition: all 0.3s ease;
    }

    &.active {
      .tab-text {
        color: #fff;
        font-weight: bold;
        text-shadow: 0 0 20rpx rgba(16, 185, 129, 0.8);
        transform: scale(1.05);
      }
    }

    .badge {
      position: absolute;
      top: 15rpx;
      right: 25%;
      background: linear-gradient(135deg, #FF00D6 0%, #FF3B30 100%);
      color: #fff;
      font-size: 20rpx;
      padding: 4rpx 10rpx;
      border-radius: 12rpx;
      min-width: 36rpx;
      text-align: center;
      font-weight: bold;
      box-shadow: 
        0 4rpx 15rpx rgba(255, 0, 214, 0.6),
        0 0 20rpx rgba(255, 0, 214, 0.4);
      animation: badgePulse 2s ease-in-out infinite;
    }
  }
}

@keyframes badgePulse {
  0%, 100% {
    transform: scale(1);
    box-shadow: 
      0 4rpx 15rpx rgba(255, 0, 214, 0.6),
      0 0 20rpx rgba(255, 0, 214, 0.4);
  }
  50% {
    transform: scale(1.1);
    box-shadow: 
      0 6rpx 25rpx rgba(255, 0, 214, 0.8),
      0 0 30rpx rgba(255, 0, 214, 0.6);
  }
}

// 内容区域
.content {
  position: relative;
  z-index: 10;
  padding: 30rpx 20rpx 150rpx;
}

// 空状态
.empty {
  text-align: center;
  padding: 180rpx 40rpx;
  
  .empty-icon {
    font-size: 120rpx;
    margin-bottom: 30rpx;
    filter: drop-shadow(0 0 30rpx rgba(16, 185, 129, 0.5));
    animation: emptyFloat 3s ease-in-out infinite;
  }

  .empty-text {
    font-size: 32rpx;
    color: rgba(255, 255, 255, 0.9);
    font-weight: bold;
    display: block;
    margin-bottom: 15rpx;
    text-shadow: 0 0 20rpx rgba(255, 255, 255, 0.3);
  }
  
  .empty-desc {
    font-size: 26rpx;
    color: rgba(255, 255, 255, 0.6);
    display: block;
    margin-bottom: 50rpx;
  }

  .add-btn {
    display: inline-flex;
    align-items: center;
    gap: 10rpx;
    background: linear-gradient(135deg, #10B981 0%, #059669 100%);
    color: #fff;
    padding: 25rpx 60rpx;
    border-radius: 50rpx;
    font-size: 30rpx;
    font-weight: bold;
    box-shadow: 
      0 10rpx 40rpx rgba(16, 185, 129, 0.6),
      0 0 50rpx rgba(16, 185, 129, 0.4),
      inset 0 2rpx 10rpx rgba(255, 255, 255, 0.3);
    transition: all 0.3s ease;
    
    .btn-icon {
      font-size: 32rpx;
    }
    
    &:active {
      transform: scale(0.95);
      box-shadow: 
        0 6rpx 30rpx rgba(16, 185, 129, 0.8),
        0 0 40rpx rgba(16, 185, 129, 0.6);
    }
  }
}

@keyframes emptyFloat {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-20rpx);
  }
}

// 绑定列表 & 请求列表
.binding-list,
.request-list {
  .binding-item,
  .request-item {
    position: relative;
    display: flex;
    align-items: center;
    background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
    backdrop-filter: blur(20rpx);
    padding: 30rpx;
    margin-bottom: 20rpx;
    border-radius: 24rpx;
    border: 2rpx solid rgba(16, 185, 129, 0.2);
    overflow: hidden;
    box-shadow: 
      0 10rpx 40rpx rgba(0, 0, 0, 0.3),
      inset 0 2rpx 10rpx rgba(255, 255, 255, 0.1);
    transition: all 0.3s ease;
    animation: slideIn 0.5s ease-out forwards;
    opacity: 0;
    transform: translateY(30rpx);
    
    &:active {
      transform: translateY(0) scale(0.98);
      box-shadow: 
        0 6rpx 30rpx rgba(0, 0, 0, 0.4),
        inset 0 2rpx 10rpx rgba(255, 255, 255, 0.15);
    }

    .item-glow,
    .request-glow {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      background: linear-gradient(135deg, rgba(16, 185, 129, 0.1) 0%, rgba(139, 92, 246, 0.1) 100%);
      filter: blur(20rpx);
      opacity: 0.5;
      pointer-events: none;
      z-index: 0;
    }

    .avatar-wrapper {
      position: relative;
      margin-right: 25rpx;
      z-index: 1;
      
      .avatar {
        width: 110rpx;
        height: 110rpx;
        border-radius: 50%;
        border: 4rpx solid rgba(16, 185, 129, 0.3);
        box-shadow: 
          0 8rpx 30rpx rgba(0, 0, 0, 0.3),
          0 0 40rpx rgba(16, 185, 129, 0.3);
      }
      
      .avatar-ring {
        position: absolute;
        top: -4rpx;
        left: -4rpx;
        right: -4rpx;
        bottom: -4rpx;
        border-radius: 50%;
        border: 3rpx solid transparent;
        background: linear-gradient(135deg, rgba(16, 185, 129, 0.6), rgba(139, 92, 246, 0.6)) border-box;
        -webkit-mask: 
          linear-gradient(#fff 0 0) padding-box, 
          linear-gradient(#fff 0 0);
        -webkit-mask-composite: xor;
        mask-composite: exclude;
        animation: ringRotate 3s linear infinite;
      }
      
      .request-pulse {
        position: absolute;
        top: -10rpx;
        right: -10rpx;
        width: 30rpx;
        height: 30rpx;
        background: linear-gradient(135deg, #FF00D6 0%, #FF3B30 100%);
        border-radius: 50%;
        box-shadow: 0 0 20rpx rgba(255, 0, 214, 0.8);
        animation: requestPulseAnim 2s ease-in-out infinite;
      }
    }

    .info {
      flex: 1;
      z-index: 1;

      .name {
        font-size: 32rpx;
        font-weight: bold;
        color: #fff;
        margin-bottom: 10rpx;
        text-shadow: 0 2rpx 10rpx rgba(0, 0, 0, 0.3);
      }

      .message {
        display: flex;
        align-items: center;
        gap: 8rpx;
        font-size: 26rpx;
        color: rgba(255, 255, 255, 0.8);
        margin-bottom: 10rpx;
        
        .message-icon {
          font-size: 24rpx;
        }
      }

      .time {
        display: flex;
        align-items: center;
        gap: 8rpx;
        font-size: 24rpx;
        color: rgba(255, 255, 255, 0.6);
        
        .time-icon {
          font-size: 22rpx;
        }
      }
    }

    .action {
      z-index: 1;
      
      .action-btns {
        display: flex;
        gap: 12rpx;
      }
      
      .manage-btn,
      .data-btn {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 6rpx;
        padding: 15rpx 20rpx;
        background: linear-gradient(135deg, rgba(16, 185, 129, 0.2) 0%, rgba(16, 185, 129, 0.1) 100%);
        border-radius: 16rpx;
        border: 2rpx solid rgba(16, 185, 129, 0.4);
        transition: all 0.3s ease;
        min-width: 100rpx;
        
        .btn-icon {
          font-size: 32rpx;
          filter: drop-shadow(0 0 10rpx rgba(16, 185, 129, 0.6));
        }
        
        .btn-text {
          font-size: 22rpx;
          color: #10B981;
          font-weight: bold;
        }
        
        &:active {
          transform: scale(0.95);
          background: linear-gradient(135deg, rgba(16, 185, 129, 0.3) 0%, rgba(16, 185, 129, 0.2) 100%);
        }
      }
      
      .data-btn {
        background: linear-gradient(135deg, rgba(139, 92, 246, 0.2) 0%, rgba(139, 92, 246, 0.1) 100%);
        border-color: rgba(139, 92, 246, 0.4);
        
        .btn-icon {
          filter: drop-shadow(0 0 10rpx rgba(139, 92, 246, 0.6));
        }
        
        .btn-text {
          color: #8B5CF6;
        }
        
        &:active {
          background: linear-gradient(135deg, rgba(139, 92, 246, 0.3) 0%, rgba(139, 92, 246, 0.2) 100%);
        }
      }
    }

    .actions {
      display: flex;
      flex-direction: column;
      gap: 15rpx;
      z-index: 1;

      button {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 8rpx;
        padding: 18rpx 35rpx;
        border-radius: 50rpx;
        font-size: 26rpx;
        font-weight: bold;
        transition: all 0.3s ease;
        border: none;
        
        .btn-icon {
          font-size: 24rpx;
        }
      }

      .accept-btn {
        background: linear-gradient(135deg, #10B981 0%, #059669 100%);
        color: #fff;
        box-shadow: 
          0 6rpx 20rpx rgba(16, 185, 129, 0.5),
          0 0 30rpx rgba(16, 185, 129, 0.3);
        
        &:active {
          transform: scale(0.95);
          box-shadow: 
            0 4rpx 15rpx rgba(16, 185, 129, 0.7),
            0 0 25rpx rgba(16, 185, 129, 0.5);
        }
      }

      .reject-btn {
        background: linear-gradient(135deg, rgba(255, 255, 255, 0.15) 0%, rgba(255, 255, 255, 0.1) 100%);
        color: rgba(255, 255, 255, 0.8);
        border: 2rpx solid rgba(255, 255, 255, 0.2);
        
        &:active {
          transform: scale(0.95);
          background: linear-gradient(135deg, rgba(255, 255, 255, 0.2) 0%, rgba(255, 255, 255, 0.15) 100%);
        }
      }
    }
  }
}

@keyframes slideIn {
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes ringRotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

@keyframes requestPulseAnim {
  0%, 100% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.2);
    opacity: 0.8;
  }
}

// 对话框
.dialog-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.7);
  backdrop-filter: blur(10rpx);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.3s ease;

  .dialog-bg-blur {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: radial-gradient(circle at center, rgba(16, 185, 129, 0.2) 0%, transparent 70%);
    animation: bgPulse 3s ease-in-out infinite;
  }

  .dialog {
    position: relative;
    width: 90%;
    max-width: 700rpx;
    max-height: 85vh;
    background: linear-gradient(135deg, rgba(26, 31, 58, 0.95) 0%, rgba(15, 20, 45, 0.98) 100%);
    backdrop-filter: blur(30rpx);
    border-radius: 32rpx;
    overflow: hidden;
    border: 3rpx solid rgba(16, 185, 129, 0.3);
    box-shadow: 
      0 20rpx 80rpx rgba(0, 0, 0, 0.6),
      0 0 60rpx rgba(16, 185, 129, 0.3),
      inset 0 2rpx 20rpx rgba(16, 185, 129, 0.1);
    animation: dialogSlideIn 0.4s cubic-bezier(0.68, -0.55, 0.265, 1.55);
    
    .dialog-glow {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      height: 200rpx;
      background: radial-gradient(ellipse at top, rgba(16, 185, 129, 0.3) 0%, transparent 70%);
      filter: blur(40rpx);
      pointer-events: none;
    }

    .dialog-header {
      position: relative;
      display: flex;
      align-items: center;
      justify-content: center;
      padding: 40rpx 30rpx 30rpx;
      border-bottom: 2rpx solid rgba(16, 185, 129, 0.2);
      
      .dialog-title-icon {
        font-size: 48rpx;
        margin-right: 15rpx;
        filter: drop-shadow(0 0 20rpx rgba(16, 185, 129, 0.8));
        animation: iconPulse 2s ease-in-out infinite;
      }
      
      .dialog-title {
        font-size: 36rpx;
        font-weight: bold;
        background: linear-gradient(135deg, #10B981 0%, #8B5CF6 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
      }
      
      .close-btn {
        position: absolute;
        right: 30rpx;
        top: 50%;
        transform: translateY(-50%);
        width: 60rpx;
        height: 60rpx;
        border-radius: 50%;
        background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
        border: 2rpx solid rgba(255, 255, 255, 0.2);
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 40rpx;
        color: rgba(255, 255, 255, 0.8);
        transition: all 0.3s ease;
        
        &:active {
          transform: translateY(-50%) scale(0.9);
          background: linear-gradient(135deg, rgba(255, 255, 255, 0.15) 0%, rgba(255, 255, 255, 0.1) 100%);
        }
      }
    }

    .dialog-content {
      position: relative;
      padding: 35rpx 40rpx;
      max-height: 70vh;
      overflow-y: auto;

      .select-box {
        margin-bottom: 30rpx;

        .label {
          display: flex;
          align-items: center;
          gap: 10rpx;
          font-size: 30rpx;
          color: rgba(255, 255, 255, 0.9);
          margin-bottom: 20rpx;
          font-weight: bold;
          
          .label-icon {
            font-size: 32rpx;
          }
        }

        .friend-list {
          max-height: 400rpx;
          overflow-y: auto;
          background: linear-gradient(135deg, rgba(255, 255, 255, 0.05) 0%, rgba(255, 255, 255, 0.02) 100%);
          border: 2rpx solid rgba(16, 185, 129, 0.2);
          border-radius: 20rpx;
          padding: 10rpx;

          .friend-item {
            display: flex;
            align-items: center;
            padding: 20rpx;
            margin-bottom: 10rpx;
            border-radius: 16rpx;
            background: linear-gradient(135deg, rgba(255, 255, 255, 0.05) 0%, rgba(255, 255, 255, 0.02) 100%);
            border: 2rpx solid transparent;
            transition: all 0.3s ease;
            cursor: pointer;

            &:last-child {
              margin-bottom: 0;
            }

            &.selected {
              background: linear-gradient(135deg, rgba(16, 185, 129, 0.2) 0%, rgba(16, 185, 129, 0.15) 100%);
              border-color: rgba(16, 185, 129, 0.5);
              box-shadow: 
                0 4rpx 20rpx rgba(16, 185, 129, 0.3),
                0 0 30rpx rgba(16, 185, 129, 0.2);
              transform: scale(1.02);
            }

            &:active {
              transform: scale(0.98);
            }

            .friend-avatar-wrapper {
              position: relative;
              margin-right: 20rpx;

              .friend-avatar {
                width: 80rpx;
                height: 80rpx;
                border-radius: 50%;
                border: 3rpx solid rgba(16, 185, 129, 0.3);
                box-shadow: 0 4rpx 15rpx rgba(0, 0, 0, 0.2);
              }

              .check-mark {
                position: absolute;
                bottom: -5rpx;
                right: -5rpx;
                width: 36rpx;
                height: 36rpx;
                background: linear-gradient(135deg, #10B981 0%, #059669 100%);
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 20rpx;
                color: #fff;
                font-weight: bold;
                box-shadow: 0 4rpx 15rpx rgba(16, 185, 129, 0.6);
                animation: checkPop 0.3s cubic-bezier(0.68, -0.55, 0.265, 1.55);
              }
            }

            .friend-info {
              flex: 1;

              .friend-name {
                font-size: 32rpx;
                color: #fff;
                font-weight: bold;
                margin-bottom: 8rpx;
              }

              .friend-username {
                font-size: 26rpx;
                color: rgba(255, 255, 255, 0.6);
              }
            }
          }
        }

        .no-friends {
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          padding: 80rpx 40rpx;
          background: linear-gradient(135deg, rgba(255, 255, 255, 0.05) 0%, rgba(255, 255, 255, 0.02) 100%);
          border: 2rpx solid rgba(16, 185, 129, 0.2);
          border-radius: 20rpx;

          .no-friends-icon {
            font-size: 100rpx;
            margin-bottom: 20rpx;
            opacity: 0.6;
          }

          .no-friends-text {
            font-size: 32rpx;
            color: rgba(255, 255, 255, 0.8);
            font-weight: bold;
            margin-bottom: 10rpx;
          }

          .no-friends-hint {
            font-size: 26rpx;
            color: rgba(255, 255, 255, 0.5);
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

      .message-box {
        .label {
          display: flex;
          align-items: center;
          gap: 10rpx;
          font-size: 30rpx;
          color: rgba(255, 255, 255, 0.9);
          margin-bottom: 20rpx;
          font-weight: bold;
          
          .label-icon {
            font-size: 32rpx;
          }
        }

        .char-count {
          text-align: right;
          font-size: 24rpx;
          color: rgba(255, 255, 255, 0.5);
          margin-top: 15rpx;
        }
      }

      .message-input {
        width: 100%;
        min-height: 260rpx;
        padding: 35rpx;
        background: linear-gradient(135deg, rgba(255, 255, 255, 0.1) 0%, rgba(255, 255, 255, 0.05) 100%);
        border: 2rpx solid rgba(16, 185, 129, 0.3);
        border-radius: 20rpx;
        font-size: 36rpx;
        color: #fff;
        line-height: 2;
        resize: none;
        transition: all 0.3s ease;
        
        &::placeholder {
          color: rgba(255, 255, 255, 0.5);
        }
        
        &:focus {
          border-color: rgba(16, 185, 129, 0.6);
          box-shadow: 0 0 30rpx rgba(16, 185, 129, 0.3);
        }
      }
    }

    .dialog-actions {
      display: flex;
      padding: 35rpx 40rpx;
      gap: 25rpx;
      border-top: 2rpx solid rgba(16, 185, 129, 0.2);

      .action-btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 12rpx;
        padding: 35rpx 40rpx;
        border-radius: 50rpx;
        font-size: 36rpx;
        font-weight: bold;
        min-height: 110rpx;
        transition: all 0.3s cubic-bezier(0.68, -0.55, 0.265, 1.55);
        cursor: pointer;
        position: relative;
        overflow: hidden;

        .btn-icon {
          font-size: 38rpx;
          transition: transform 0.3s ease;
        }

        .btn-text {
          font-size: 36rpx;
        }

        &::before {
          content: '';
          position: absolute;
          top: 50%;
          left: 50%;
          width: 0;
          height: 0;
          border-radius: 50%;
          background: rgba(255, 255, 255, 0.3);
          transform: translate(-50%, -50%);
          transition: width 0.6s, height 0.6s;
        }

        &:active::before {
          width: 300%;
          height: 300%;
        }

        &.cancel-btn {
          background: linear-gradient(135deg, rgba(255, 255, 255, 0.15) 0%, rgba(255, 255, 255, 0.08) 100%);
          color: rgba(255, 255, 255, 0.9);
          border: 3rpx solid rgba(255, 255, 255, 0.3);
          box-shadow: 
            0 6rpx 25rpx rgba(0, 0, 0, 0.3),
            inset 0 2rpx 10rpx rgba(255, 255, 255, 0.1);

          &:active {
            transform: scale(0.95);
            box-shadow: 
              0 4rpx 15rpx rgba(0, 0, 0, 0.4),
              inset 0 2rpx 10rpx rgba(255, 255, 255, 0.15);
          }
        }

        &.confirm-btn {
          background: linear-gradient(135deg, #10B981 0%, #059669 100%);
          color: #fff;
          border: 3rpx solid rgba(16, 185, 129, 0.5);
          box-shadow: 
            0 10rpx 40rpx rgba(16, 185, 129, 0.6),
            0 0 50rpx rgba(16, 185, 129, 0.4),
            inset 0 2rpx 15rpx rgba(255, 255, 255, 0.3);
          animation: btnGlow 2s ease-in-out infinite;

          .btn-icon {
            animation: iconSpin 3s linear infinite;
          }

          &:active {
            transform: scale(0.95);
            box-shadow: 
              0 6rpx 30rpx rgba(16, 185, 129, 0.8),
              0 0 40rpx rgba(16, 185, 129, 0.6);

            .btn-icon {
              transform: rotate(360deg) scale(1.2);
            }
          }

          &.disabled {
            opacity: 0.5;
            background: linear-gradient(135deg, rgba(16, 185, 129, 0.3) 0%, rgba(5, 150, 105, 0.3) 100%);
            box-shadow: none;
            cursor: not-allowed;
            animation: none;

            .btn-icon {
              animation: none;
            }

            &:active {
              transform: none;
            }
          }
        }
      }
    }

    @keyframes btnGlow {
      0%, 100% {
        box-shadow: 
          0 10rpx 40rpx rgba(16, 185, 129, 0.6),
          0 0 50rpx rgba(16, 185, 129, 0.4),
          inset 0 2rpx 15rpx rgba(255, 255, 255, 0.3);
      }
      50% {
        box-shadow: 
          0 12rpx 50rpx rgba(16, 185, 129, 0.8),
          0 0 70rpx rgba(16, 185, 129, 0.6),
          inset 0 2rpx 20rpx rgba(255, 255, 255, 0.4);
      }
    }

    @keyframes iconSpin {
      0%, 90% {
        transform: rotate(0deg);
      }
      95% {
        transform: rotate(360deg);
      }
      100% {
        transform: rotate(360deg);
      }
    }
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

@keyframes dialogSlideIn {
  from {
    opacity: 0;
    transform: scale(0.8) translateY(50rpx);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

@keyframes bgPulse {
  0%, 100% {
    opacity: 0.5;
    transform: scale(1);
  }
  50% {
    opacity: 0.8;
    transform: scale(1.1);
  }
}
</style>
