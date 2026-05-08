<template>
  <view class="chat-page">
    <!-- 消息列表 -->
    <scroll-view class="message-list" scroll-y :scroll-into-view="scrollToView" :scroll-with-animation="true">
      <!-- 加载历史记录按钮 -->
      <view v-if="hasMoreHistory && !loadingHistory" class="load-history-btn" @click="loadMoreHistory">
        <text>查看历史记录</text>
      </view>
      <view v-if="loadingHistory" class="load-history-loading">
        <text>加载中...</text>
      </view>
      
      <view
        v-for="message in messages"
        :key="message.id"
        :id="`msg-${message.id}`"
        class="message-item"
        :class="{ 'message-mine': message.isMine }"
      >
        <view v-if="message.isMine ? myAvatar : friendAvatar" class="message-avatar-wrapper">
          <image
            class="message-avatar"
            :src="message.isMine ? myAvatar : friendAvatar"
            mode="aspectFill"
          />
        </view>
        <view v-else class="message-avatar-wrapper message-avatar-emoji">
          <text>{{ message.isMine ? '👤' : '🤖' }}</text>
        </view>
        <view class="message-content">
          <text class="message-text">{{ message.content }}</text>
          <text class="message-time">{{ formatTime(message.timestamp) }}</text>
        </view>
      </view>

      <!-- 正在输入 -->
      <view v-if="isTyping" class="message-item">
        <view v-if="friendAvatar" class="message-avatar-wrapper">
          <image class="message-avatar" :src="friendAvatar" mode="aspectFill" />
        </view>
        <view v-else class="message-avatar-wrapper message-avatar-emoji">
          <text>🤖</text>
        </view>
        <view class="message-content">
          <text class="typing-indicator">对方正在输入...</text>
        </view>
      </view>
    </scroll-view>

    <!-- 输入框 -->
    <view class="input-bar">
      <text class="emoji-btn" @click="showEmojiPicker">😊</text>
      <input
        class="message-input"
        v-model="inputText"
        placeholder="说点什么..."
        confirm-type="send"
        @confirm="sendMessage"
        @focus="onInputFocus"
      />
      <view v-if="inputText.trim()" class="send-btn" @click="sendMessage">
        <text>发送</text>
      </view>
      <view v-else class="more-btn" @click="showMoreActionsMenu">
        <text>+</text>
      </view>
    </view>

    <!-- 更多操作 -->
    <view v-if="showMoreActions" class="more-actions-mask" @click="hideMoreActions">
      <view class="more-actions" @click.stop>
        <view class="action-grid">
          <view class="action-item" @click="chooseImage">
            <view class="action-icon" style="background: #07c160">🖼️</view>
            <text class="action-text">图片</text>
          </view>
          <view class="action-item" @click="takePhoto">
            <view class="action-icon" style="background: #1989fa">📷</view>
            <text class="action-text">拍摄</text>
          </view>
          <view class="action-item" @click="sendVoice">
            <view class="action-icon" style="background: #ff976a">🎤</view>
            <text class="action-text">语音</text>
          </view>
          <view class="action-item" @click="sendLocation">
            <view class="action-icon" style="background: #ed7b2f">📍</view>
            <text class="action-text">位置</text>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, nextTick, onUnmounted } from 'vue'
import { onLoad, onShow } from '@dcloudio/uni-app'
import { showToast } from '@/utils'
import { getChatHistoryApi, sendFriendMessageApi, getFriendsApi, markMessagesAsReadApi, type FriendMessage } from '@/api/friend'
import { useUserStore } from '@/store'
import { getWebSocketClient, type WsMessage } from '@/utils/websocket'

interface Message {
  id: string
  content: string
  isMine: boolean
  timestamp: number
}

interface Friend {
  id: string
  name: string
  avatar: string
}

const userStore = useUserStore()
const friendId = ref('')
const friend = ref<Friend | null>(null)
const messages = ref<Message[]>([])
const inputText = ref('')
const isTyping = ref(false)
const scrollToView = ref('')
const showMoreActions = ref(false)
const loadingHistory = ref(false)
const currentPage = ref(1)
const hasMoreHistory = ref(true)

const myAvatar = ref('')
const friendAvatar = ref('')
const wsClient = getWebSocketClient()

// WebSocket 消息处理函数
const handleWsMessage = (message: WsMessage) => {
  // console.log('[Chat] ========== 收到WebSocket消息 ==========')
  // console.log('[Chat] 消息内容:', message)
  // console.log('[Chat] 当前friendId:', friendId.value, typeof friendId.value)
  // console.log('[Chat] 当前userId:', userStore.userId, typeof userStore.userId)
  
  if (message.type === 'message') {
    // 收到新消息
    const msg = message as any
    // console.log('[Chat] 消息详情:', {
    //   from_user_id: msg.from_user_id,
    //   to_user_id: msg.to_user_id,
    //   content: msg.content
    // })
    
    // 检查是否是当前聊天对象的消息
    // 消息可能是：
    // 1. 好友发送给我的：from_user_id是好友，to_user_id是当前用户
    // 2. 我发送给好友的：from_user_id是当前用户，to_user_id是好友
    
    // 确保ID类型一致（都转为字符串比较）
    const msgFromId = String(msg.from_user_id || '')
    const msgToId = String(msg.to_user_id || '')
    const currentFriendId = String(friendId.value || '')
    const currentUserId = String(userStore.userId || '')
    
    const isFromFriend = msgFromId === currentFriendId && msgToId === currentUserId
    const isMyMessage = msgFromId === currentUserId && msgToId === currentFriendId
    
    // console.log('[Chat] 消息匹配检查:', {
    //   msgFromId,
    //   msgToId,
    //   currentFriendId,
    //   currentUserId,
    //   isFromFriend,
    //   isMyMessage
    // })
    
    if (isFromFriend || isMyMessage) {
      const newMessage: Message = {
        id: msg.id,
        content: msg.content,
        isMine: msgFromId === currentUserId, // 使用转换后的ID比较
        timestamp: new Date(msg.created_at).getTime()
      }
      
      // 检查消息是否已存在（避免重复）
      const existingIndex = messages.value.findIndex(m => m.id === newMessage.id)
      if (existingIndex === -1) {
        // 如果是自己发送的消息，检查是否有临时消息需要替换
        if (newMessage.isMine) {
          const tempIndex = messages.value.findIndex(m => m.id.startsWith('temp-'))
          if (tempIndex !== -1) {
            // 替换临时消息
            messages.value[tempIndex] = newMessage
            // console.log('[Chat] ✅ 替换临时消息:', newMessage)
          } else {
            // 没有临时消息，直接添加
            messages.value.push(newMessage)
            // console.log('[Chat] ✅ 添加新消息（自己发送）:', newMessage)
          }
        } else {
          // 对方发送的消息，直接添加
          messages.value.push(newMessage)
          // console.log('[Chat] ✅ 添加新消息（对方发送）:', newMessage)
          
          // 在聊天页面收到消息，立即标记为已读，禁用加载提示
          markMessagesAsReadApi(friendId.value, { custom: { showLoading: false } }).catch(err => {
            // console.error('[Chat] 标记消息为已读失败:', err)
          })
          
          // 显示通知（H5环境）
          // @ts-ignore
          if (typeof window !== 'undefined' && !window.navigator.userAgent.includes('miniProgram')) {
            uni.showToast({
              title: '收到新消息',
              icon: 'none',
              duration: 1500
            })
          }
        }
        
        // 按时间排序
        messages.value.sort((a, b) => a.timestamp - b.timestamp)
        
        // 滚动到底部
        nextTick(() => {
          scrollToView.value = `msg-${newMessage.id}`
        })
      } else {
        // console.log('[Chat] ⚠️ 消息已存在，跳过:', newMessage.id)
      }
    } else {
      // console.log('[Chat] ⚠️ 消息不是当前聊天对象的，跳过')
    }
  }
}

onLoad((options: any) => {
  if (options.friendId) {
    friendId.value = String(options.friendId) // 确保是字符串
    // console.log('[Chat] 页面加载，friendId:', friendId.value, 'userId:', userStore.userId)
    loadFriend()
    loadMessages()
    
    // 立即注册消息处理器（避免消息丢失）
    // console.log('[Chat] ✅ 立即注册WebSocket消息处理器')
    wsClient.onMessage(handleWsMessage)
    // console.log('[Chat] 消息处理器注册完成，当前连接状态:', wsClient.isConnected())
    
    // 连接 WebSocket
    if (!wsClient.isConnected()) {
      // console.log('[Chat] 🔌 WebSocket未连接，开始连接...')
      wsClient.connect()
      
      // 等待连接建立后再确保处理器已注册
      wsClient.onConnect(() => {
        // console.log('[Chat] ✅ WebSocket已连接并认证成功，再次确保消息处理器已注册')
        // 确保消息处理器已注册（可能已经注册过了，但再次注册确保不会丢失）
        wsClient.onMessage(handleWsMessage)
      })
      
      // 添加错误处理
      wsClient.onError((error) => {
        // console.error('[Chat] ❌ WebSocket错误:', error)
      })
    } else {
      // console.log('[Chat] ✅ WebSocket已连接，消息处理器已注册')
    }
  }
})

onShow(() => {
  // 页面显示时刷新消息列表（从其他页面返回时）
  if (friendId.value) {
    loadMessages()
    
    // 确保消息处理器已注册
    // console.log('[Chat] 页面显示，确保消息处理器已注册')
    wsClient.onMessage(handleWsMessage)
    
    // 确保 WebSocket 已连接
    if (!wsClient.isConnected()) {
      // console.log('[Chat] 页面显示，WebSocket未连接，开始连接...')
      wsClient.connect()
      
      // 等待连接建立后确保处理器已注册
      wsClient.onConnect(() => {
        // console.log('[Chat] WebSocket已连接并认证成功')
        wsClient.onMessage(handleWsMessage)
      })
    } else {
      // console.log('[Chat] WebSocket已连接，消息处理器已注册')
    }
  }
})

onUnmounted(() => {
  // 移除消息处理器
  // console.log('[Chat] 页面卸载，移除消息处理器')
  wsClient.offMessage(handleWsMessage)
})

const loadFriend = async () => {
  try {
    // 禁用加载提示
    const friends = await getFriendsApi({ custom: { showLoading: false } })
    const foundFriend = (friends as any[]).find((f: any) => (f.user_id || f.id) === friendId.value)
    
    if (foundFriend) {
      friend.value = {
        id: foundFriend.user_id || foundFriend.id,
        name: foundFriend.nickname || foundFriend.username,
        avatar: foundFriend.avatar || ''
      }
      
      uni.setNavigationBarTitle({ title: friend.value.name })
      friendAvatar.value = friend.value.avatar || ''
      myAvatar.value = userStore.avatar || ''
    }
  } catch (error: any) {
    // console.error('加载好友信息失败:', error)
  }
}

const loadMessages = async () => {
  try {
    currentPage.value = 1
    
    // 默认加载最新50条（第一页），禁用加载提示
    const data = await getChatHistoryApi(friendId.value, { page: 1, pageSize: 50 }, { custom: { showLoading: false } })
    
    if (!data || !Array.isArray(data)) {
      // console.warn('[Chat] 返回的数据格式不正确:', data)
      messages.value = []
      hasMoreHistory.value = false
      return
    }
    
    // 转换为页面需要的格式，并按时间排序（升序：最早的在前）
    const newMessages = (data as any[]).map((msg: any) => ({
      id: msg.id,
      content: msg.content,
      isMine: msg.from_user_id === userStore.userId,
      timestamp: new Date(msg.created_at).getTime()
    })).sort((a, b) => a.timestamp - b.timestamp) // 按时间升序排序
    
    // 检查是否有新消息（通过比较消息数量或最后一条消息ID）
    const hasNewMessages = messages.value.length !== newMessages.length || 
      (newMessages.length > 0 && messages.value.length > 0 && 
       newMessages[newMessages.length - 1].id !== messages.value[messages.value.length - 1].id)
    
    messages.value = newMessages
    
    // 判断是否还有更多历史记录
    // 如果返回的消息正好是50条，说明可能还有更多；如果少于50条，说明没有更多了
    hasMoreHistory.value = newMessages.length >= 50
    // console.log('[Chat] 加载消息完成，消息数:', newMessages.length, '是否有更多历史:', hasMoreHistory.value)
    
    // 如果有新消息，滚动到底部
    if (hasNewMessages && messages.value.length > 0) {
      nextTick(() => {
        const lastMsg = messages.value[messages.value.length - 1]
        scrollToView.value = `msg-${lastMsg.id}`
      })
    }
  } catch (error: any) {
    // console.error('加载聊天记录失败:', error)
    showToast(error.message || '加载聊天记录失败', 'none')
    hasMoreHistory.value = false
  }
}

// 加载更多历史记录
const loadMoreHistory = async () => {
  if (loadingHistory.value || !hasMoreHistory.value) return
  
  try {
    loadingHistory.value = true
    currentPage.value += 1
    
    // 加载更早的消息（从第二页开始），禁用加载提示
    const data = await getChatHistoryApi(friendId.value, { 
      page: currentPage.value, 
      pageSize: 20 
    }, { custom: { showLoading: false } })
    
    if (!data || (data as any[]).length === 0) {
      hasMoreHistory.value = false
      showToast('没有更多历史记录了', 'none')
      return
    }
    
    // 转换为页面需要的格式，并按时间排序（升序：最早的在前）
    const olderMessages = (data as any[]).map((msg: any) => ({
      id: msg.id,
      content: msg.content,
      isMine: msg.from_user_id === userStore.userId,
      timestamp: new Date(msg.created_at).getTime()
    })).sort((a, b) => a.timestamp - b.timestamp)
    
    // 将更早的消息插入到列表顶部
    messages.value = [...olderMessages, ...messages.value]
    
    // 如果返回的消息少于20条，说明没有更多历史记录了
    if ((data as any[]).length < 20) {
      hasMoreHistory.value = false
    }
    
    // 滚动到加载的历史记录位置（第一条新加载的消息）
    if (olderMessages.length > 0) {
      await nextTick()
      scrollToView.value = `msg-${olderMessages[0].id}`
    }
  } catch (error: any) {
    // console.error('加载历史记录失败:', error)
    showToast(error.message || '加载历史记录失败', 'none')
    currentPage.value -= 1 // 回退页码
  } finally {
    loadingHistory.value = false
  }
}

const sendMessage = async () => {
  if (!inputText.value.trim()) return

  const content = inputText.value.trim()
  inputText.value = ''

  // 优先使用 WebSocket 发送
  if (wsClient.isConnected()) {
    // console.log('[Chat] 📤 通过WebSocket发送消息:', {
    //   to_user_id: friendId.value,
    //   content,
    //   content_len: content.length
    // })
    
    const success = wsClient.send({
      type: 'send_message',
      to_user_id: friendId.value,
      content,
      message_type: 'text'
    })
    
    if (success) {
      // console.log('[Chat] ✅ WebSocket消息发送成功，等待服务器确认')
      
      // WebSocket 发送成功，等待服务器推送消息
      // 消息会通过 handleWsMessage 自动添加到列表
      // 为了更好的用户体验，先添加一个临时消息（乐观更新）
      const tempMessage: Message = {
        id: `temp-${Date.now()}`,
        content,
        isMine: true,
        timestamp: Date.now()
      }
      messages.value.push(tempMessage)
      messages.value.sort((a, b) => a.timestamp - b.timestamp)
      
      // 滚动到底部
      await nextTick()
      scrollToView.value = `msg-${tempMessage.id}`
      
      // 当收到真实消息后，临时消息会被替换（通过ID匹配）
      return
    } else {
      // console.warn('[Chat] ⚠️ WebSocket消息发送失败，将使用HTTP API')
    }
  } else {
    // console.warn('[Chat] ⚠️ WebSocket未连接，将使用HTTP API发送消息')
  }
  
  // WebSocket 不可用时，使用 HTTP API，禁用加载提示
  try {
    const result = await sendFriendMessageApi({
      to_user_id: friendId.value,
      content,
      message_type: 'text'
    }, { custom: { showLoading: false } })
    
    // HTTP API发送的消息也会通过WebSocket推送，但为了确保显示，这里也添加
    // 检查是否已存在（可能WebSocket已经推送了）
    const existingIndex = messages.value.findIndex(m => m.id === result.id)
    if (existingIndex === -1) {
      const message: Message = {
        id: result.id,
        content: result.content,
        isMine: true,
        timestamp: new Date(result.created_at).getTime()
      }
      
      messages.value.push(message)
      messages.value.sort((a, b) => a.timestamp - b.timestamp)
      
      // 滚动到底部
      await nextTick()
      scrollToView.value = `msg-${message.id}`
    }
  } catch (error: any) {
    // console.error('发送消息失败:', error)
    showToast(error.message || '发送失败', 'none')
    // 发送失败，恢复输入内容
    inputText.value = content
  }
}

const onInputFocus = () => {
  // 输入框获得焦点时滚动到底部
  if (messages.value.length > 0) {
    nextTick(() => {
      const lastMsg = messages.value[messages.value.length - 1]
      scrollToView.value = `msg-${lastMsg.id}`
    })
  }
}

const showEmojiPicker = () => {
  uni.showToast({ title: '表情功能开发中', icon: 'none' })
}

const showMoreActionsMenu = () => {
  showMoreActions.value = true
}

const hideMoreActions = () => {
  showMoreActions.value = false
}

const chooseImage = () => {
  hideMoreActions()
  uni.chooseImage({
    count: 1,
    success: () => {
      uni.showToast({ title: '发送图片成功', icon: 'success' })
    }
  })
}

const takePhoto = () => {
  hideMoreActions()
  uni.chooseImage({
    count: 1,
    sourceType: ['camera'],
    success: () => {
      uni.showToast({ title: '发送照片成功', icon: 'success' })
    }
  })
}

const sendVoice = () => {
  hideMoreActions()
  uni.showToast({ title: '语音功能开发中', icon: 'none' })
}

const sendLocation = () => {
  hideMoreActions()
  uni.showToast({ title: '位置功能开发中', icon: 'none' })
}

const formatTime = (timestamp: number): string => {
  const date = new Date(timestamp)
  const hours = String(date.getHours()).padStart(2, '0')
  const minutes = String(date.getMinutes()).padStart(2, '0')
  return `${hours}:${minutes}`
}
</script>

<style lang="scss" scoped>
.chat-page {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--theme-background);
}

.message-list {
  flex: 1;
  padding: 20rpx 30rpx;
  overflow-y: auto;
}

.load-history-btn {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20rpx;
  margin: 20rpx 0;
  background: var(--theme-surface);
  border-radius: 10rpx;
  box-shadow: 0 2rpx 8rpx rgba(0, 0, 0, 0.1);
  
  text {
    color: var(--theme-primary);
    font-size: 28rpx;
  }
  
  &:active {
    opacity: 0.7;
  }
}

.load-history-loading {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 20rpx;
  margin: 20rpx 0;
  
  text {
    color: var(--theme-text-secondary);
    font-size: 28rpx;
  }
}

.message-item {
  display: flex;
  margin-bottom: 30rpx;

  &.message-mine {
    flex-direction: row-reverse;

    .message-content {
      align-items: flex-end;
      
      .message-text {
        background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
        color: #ffffff;
      }
    }
  }

  .message-avatar-wrapper {
    width: 70rpx;
    height: 70rpx;
    flex-shrink: 0;

    .message-avatar {
      width: 100%;
      height: 100%;
      border-radius: 35rpx;
    }

    &.message-avatar-emoji {
      display: flex;
      align-items: center;
      justify-content: center;
      background: #f0f0f0;
      border-radius: 35rpx;
      font-size: 40rpx;
    }
  }

  .message-content {
    max-width: 500rpx;
    margin: 0 15rpx;
    display: flex;
    flex-direction: column;

    .message-text {
      background: var(--theme-surface);
      border-radius: 15rpx;
      padding: 20rpx;
      font-size: 28rpx;
      line-height: 1.6;
      word-break: break-word;
    }

    .message-time {
      font-size: 20rpx;
      color: #cccccc;
      margin-top: 8rpx;
      padding: 0 10rpx;
    }

    .typing-indicator {
      background: var(--theme-surface);
      border-radius: 15rpx;
      padding: 20rpx;
      font-size: 28rpx;
      color: var(--theme-text-secondary);
    }
  }
}

.input-bar {
  display: flex;
  align-items: center;
  padding: 20rpx 30rpx;
  background: var(--theme-surface);
  border-top: 1rpx solid #e5e5e5;

  .emoji-btn {
    font-size: 48rpx;
    margin-right: 15rpx;
  }

  .message-input {
    flex: 1;
    height: 70rpx;
    background: var(--theme-background);
    border-radius: 35rpx;
    padding: 0 30rpx;
    font-size: 28rpx;
  }

  .send-btn,
  .more-btn {
    width: 120rpx;
    height: 70rpx;
    background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
    color: #ffffff;
    border-radius: 35rpx;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-left: 15rpx;
    font-size: 28rpx;
    font-weight: bold;

    &:active {
      opacity: 0.8;
    }
  }

  .more-btn {
    font-size: 48rpx;
    font-weight: normal;
  }
}

.more-actions-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 999;
  animation: fadeIn 0.3s;
}

.more-actions {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  background: var(--theme-surface);
  padding: 40rpx 30rpx;
  border-radius: 20rpx 20rpx 0 0;
  animation: slideUp 0.3s;

  .action-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 30rpx;

    .action-item {
      display: flex;
      flex-direction: column;
      align-items: center;

      &:active {
        opacity: 0.7;
      }

      .action-icon {
        width: 100rpx;
        height: 100rpx;
        border-radius: 20rpx;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 50rpx;
        margin-bottom: 15rpx;
      }

      .action-text {
        font-size: 24rpx;
        color: var(--theme-text-secondary);
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

@keyframes slideUp {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}
</style>

