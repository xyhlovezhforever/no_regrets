<template>
  <view class="topic-page">
    <!-- 头部 -->
    <view class="header">
      <text class="header-title">🍽️ 今日话题</text>
      <text class="header-date">{{ formatDate(new Date()) }}</text>
    </view>

    <!-- 话题卡片 -->
    <view class="content">
      <view class="topic-card">
        <view class="topic-icon">{{ currentTopic.icon }}</view>
        <text class="topic-title">{{ currentTopic.title }}</text>
        <text class="topic-question">{{ currentTopic.question }}</text>
        
        <!-- 选项列表 -->
        <view v-if="currentTopic.options" class="options-list">
          <view
            v-for="(option, index) in currentTopic.options"
            :key="index"
            class="option-item"
            :class="{ selected: selectedOption === index }"
            @click="selectOption(index)"
          >
            <text class="option-text">{{ option }}</text>
            <text v-if="selectedOption === index" class="check-icon">✓</text>
          </view>
        </view>

        <!-- 用户输入区 -->
        <view v-else class="input-area">
          <textarea
            v-model="userAnswer"
            class="answer-input"
            placeholder="输入你的想法..."
            maxlength="200"
          />
          <text class="char-count">{{ userAnswer.length }}/200</text>
        </view>
      </view>
    </view>

    <!-- 操作按钮 -->
    <view class="actions">
      <button class="action-btn refresh-btn" @click="refreshTopic">
        <text class="btn-icon">🎲</text>
        <text class="btn-text">换话题</text>
      </button>
      <button class="action-btn community-btn" @click="viewCommunity">
        <text class="btn-icon">👥</text>
        <text class="btn-text">看回复</text>
      </button>
      <button class="action-btn share-btn" @click="shareTopic">
        <text class="btn-icon">📤</text>
        <text class="btn-text">分享</text>
      </button>
    </view>

    <!-- 他人回复弹窗 -->
    <view v-if="showCommunityModal" class="community-modal-mask" @click="showCommunityModal = false">
      <view class="community-modal" @click.stop>
        <view class="modal-header">
          <text class="modal-title">大家的回答</text>
          <text class="modal-close" @click="showCommunityModal = false">✕</text>
        </view>
        <scroll-view class="replies-list" scroll-y>
          <view
            v-for="(reply, index) in communityReplies"
            :key="index"
            class="reply-item"
          >
            <view class="reply-header">
              <view class="user-avatar">{{ reply.avatar }}</view>
              <view class="user-info">
                <text class="user-name">{{ reply.userName }}</text>
                <text class="reply-time">{{ formatReplyTime(reply.time) }}</text>
              </view>
              <view class="like-btn" :class="{ liked: reply.liked }" @click="toggleLike(index)">
                <text>{{ reply.liked ? '❤️' : '🤍' }}</text>
                <text class="like-count">{{ reply.likes }}</text>
              </view>
            </view>
            <text class="reply-content">{{ reply.content }}</text>
          </view>
          <view v-if="communityReplies.length === 0" class="empty-replies">
            <text class="empty-icon">💬</text>
            <text class="empty-text">还没有人回复</text>
            <text class="empty-hint">快来抢沙发吧~</text>
          </view>
        </scroll-view>
        <view class="reply-input-box">
          <input
            v-model="myReply"
            class="reply-input"
            placeholder="说说你的想法..."
            maxlength="200"
          />
          <button class="send-btn" @click="submitReply">发送</button>
        </view>
      </view>
    </view>

    <!-- 历史话题 -->
    <view class="history">
      <view class="history-header">
        <text class="history-title">历史话题</text>
      </view>
      <view class="history-list">
        <view
          v-for="(item, index) in history"
          :key="index"
          class="history-item"
          @click="viewHistoryTopic(item)"
        >
          <text class="history-icon">{{ item.icon }}</text>
          <view class="history-info">
            <text class="history-question">{{ item.title }}</text>
            <text class="history-date">{{ item.date }}</text>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getStorage, setStorage } from '@/utils/storage'
import { shareContent } from '@/utils/share'

interface Topic {
  icon: string
  title: string
  question: string
  options?: string[]
  category: string
}

interface HistoryItem extends Topic {
  date: string
  answer?: string | number
}

interface CommunityReply {
  userName: string
  avatar: string
  content: string
  time: number
  likes: number
  liked: boolean
}

const currentTopic = ref<Topic>({
  icon: '🍽️',
  title: '今天吃什么？',
  question: '今天中午想吃什么呢？',
  options: ['中餐', '西餐', '日料', '火锅', '烧烤', '快餐'],
  category: 'food'
})

const selectedOption = ref<number | null>(null)
const userAnswer = ref('')
const history = ref<HistoryItem[]>([])
const showCommunityModal = ref(false)
const communityReplies = ref<CommunityReply[]>([])
const myReply = ref('')

const topics: Topic[] = [
  {
    icon: '🍽️',
    title: '今天吃什么？',
    question: '今天中午想吃什么呢？',
    options: ['中餐', '西餐', '日料', '火锅', '烧烤', '快餐'],
    category: 'food'
  },
  {
    icon: '🎬',
    title: '今天看什么电影？',
    question: '今晚想看什么类型的电影？',
    options: ['动作片', '爱情片', '喜剧片', '科幻片', '恐怖片', '纪录片'],
    category: 'entertainment'
  },
  {
    icon: '🎵',
    title: '今天听什么歌？',
    question: '现在心情适合听什么类型的音乐？',
    options: ['流行', '摇滚', '古典', '民谣', '电子', '轻音乐'],
    category: 'music'
  },
  {
    icon: '📚',
    title: '今天读什么书？',
    question: '想读哪种类型的书籍？',
    options: ['小说', '传记', '历史', '科技', '哲学', '漫画'],
    category: 'reading'
  },
  {
    icon: '🏃',
    title: '今天做什么运动？',
    question: '今天想做什么运动呢？',
    options: ['跑步', '游泳', '瑜伽', '健身', '打球', '散步'],
    category: 'exercise'
  },
  {
    icon: '🎨',
    title: '今天做点什么？',
    question: '闲暇时间想做什么呢？',
    options: ['画画', '写作', '摄影', '手工', '烹饪', '学习'],
    category: 'hobby'
  },
  {
    icon: '🌍',
    title: '今天想去哪玩？',
    question: '周末想去哪里玩呢？',
    options: ['公园', '博物馆', '电影院', '商场', '咖啡厅', '图书馆'],
    category: 'outing'
  },
  {
    icon: '💭',
    title: '今天的心情',
    question: '今天的心情怎么样？',
    category: 'mood'
  }
]

onMounted(() => {
  loadTodayTopic()
  loadHistory()
})

const loadTodayTopic = () => {
  const today = new Date().toDateString()
  const cached = getStorage<{ date: string; topic: Topic }>('dailyTopic', null)
  
  if (cached && cached.date === today) {
    currentTopic.value = cached.topic
  } else {
    const randomTopic = topics[Math.floor(Math.random() * topics.length)]
    currentTopic.value = randomTopic
    setStorage('dailyTopic', { date: today, topic: randomTopic })
  }
}

const loadHistory = () => {
  history.value = getStorage<HistoryItem[]>('topicHistory', [])
}

const saveHistory = () => {
  const historyItem: HistoryItem = {
    ...currentTopic.value,
    date: formatDate(new Date()),
    answer: selectedOption.value !== null ? selectedOption.value : userAnswer.value
  }
  
  history.value.unshift(historyItem)
  if (history.value.length > 10) {
    history.value = history.value.slice(0, 10)
  }
  
  setStorage('topicHistory', history.value)
}

const selectOption = (index: number) => {
  selectedOption.value = index
  saveHistory()
  
  uni.showToast({
    title: `已选择：${currentTopic.value.options![index]}`,
    icon: 'success'
  })
}

const refreshTopic = () => {
  const randomTopic = topics[Math.floor(Math.random() * topics.length)]
  currentTopic.value = randomTopic
  selectedOption.value = null
  userAnswer.value = ''
}

const viewCommunity = () => {
  loadCommunityReplies()
  showCommunityModal.value = true
}

const loadCommunityReplies = () => {
  // 从本地加载回复
  const topicKey = currentTopic.value.title
  const replies = getStorage<CommunityReply[]>(`topic_replies_${topicKey}`, [])
  
  // 模拟一些示例回复（首次加载）
  if (replies.length === 0) {
    const mockReplies: CommunityReply[] = [
      {
        userName: '美食家小王',
        avatar: '👨',
        content: '我选火锅！天气冷的时候最适合吃火锅了，热乎乎的特别舒服~',
        time: Date.now() - 3600000,
        likes: 12,
        liked: false
      },
      {
        userName: '健康生活',
        avatar: '👩',
        content: '中餐吧，营养均衡，而且有家的味道',
        time: Date.now() - 7200000,
        likes: 8,
        liked: false
      },
      {
        userName: '吃货小李',
        avatar: '🧑',
        content: '日料！最近爱上了寿司和刺身，新鲜又美味',
        time: Date.now() - 10800000,
        likes: 15,
        liked: false
      }
    ]
    communityReplies.value = mockReplies
  } else {
    communityReplies.value = replies
  }
}

const toggleLike = (index: number) => {
  const reply = communityReplies.value[index]
  if (reply.liked) {
    reply.likes--
    reply.liked = false
  } else {
    reply.likes++
    reply.liked = true
  }
  
  // 保存到本地
  saveCommunityReplies()
}

const submitReply = () => {
  if (!myReply.value.trim()) {
    return uni.showToast({ title: '请输入回复内容', icon: 'none' })
  }
  
  const newReply: CommunityReply = {
    userName: '我',
    avatar: '😊',
    content: myReply.value,
    time: Date.now(),
    likes: 0,
    liked: false
  }
  
  communityReplies.value.unshift(newReply)
  saveCommunityReplies()
  
  myReply.value = ''
  uni.showToast({ title: '回复成功', icon: 'success' })
}

const saveCommunityReplies = () => {
  const topicKey = currentTopic.value.title
  setStorage(`topic_replies_${topicKey}`, communityReplies.value)
}

const formatReplyTime = (timestamp: number): string => {
  const now = Date.now()
  const diff = now - timestamp
  
  const minute = 60 * 1000
  const hour = 60 * minute
  const day = 24 * hour
  
  if (diff < minute) return '刚刚'
  if (diff < hour) return `${Math.floor(diff / minute)}分钟前`
  if (diff < day) return `${Math.floor(diff / hour)}小时前`
  if (diff < 7 * day) return `${Math.floor(diff / day)}天前`
  
  const date = new Date(timestamp)
  return `${date.getMonth() + 1}月${date.getDate()}日`
}

const shareTopic = () => {
  const answer = selectedOption.value !== null
    ? currentTopic.value.options![selectedOption.value]
    : userAnswer.value || '（未回答）'
  
  shareContent({
    title: currentTopic.value.title,
    content: `${currentTopic.value.question}\n\n我的选择：${answer}`,
    link: 'https://your-app-link.com'
  })
}

const viewHistoryTopic = (item: HistoryItem) => {
  uni.showModal({
    title: item.title,
    content: `${item.question}\n\n回答时间：${item.date}`,
    showCancel: false
  })
}

const formatDate = (date: Date): string => {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}.${month}.${day}`
}
</script>

<style lang="scss" scoped>
.topic-page {
  min-height: 100vh;
  background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
}

.header {
  padding: 60rpx 30rpx 30rpx;
  text-align: center;

  .header-title {
    display: block;
    font-size: 44rpx;
    font-weight: bold;
    color: var(--theme-text);
    margin-bottom: 15rpx;
  }

  .header-date {
    font-size: 24rpx;
    color: var(--theme-text-secondary);
  }
}

.content {
  padding: 0 30rpx 30rpx;
}

.topic-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 30rpx;
  padding: 50rpx 40rpx;
  box-shadow: 0 20rpx 60rpx rgba(0, 0, 0, 0.1);

  .topic-icon {
    font-size: 100rpx;
    text-align: center;
    margin-bottom: 30rpx;
  }

  .topic-title {
    display: block;
    font-size: 40rpx;
    font-weight: bold;
    color: var(--theme-text);
    text-align: center;
    margin-bottom: 20rpx;
  }

  .topic-question {
    display: block;
    font-size: 30rpx;
    color: var(--theme-text-secondary);
    text-align: center;
    margin-bottom: 40rpx;
  }

  .options-list {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 20rpx;

    .option-item {
      position: relative;
      padding: 25rpx 20rpx;
      background: var(--theme-background);
      border-radius: 15rpx;
      border: 3rpx solid transparent;
      text-align: center;
      transition: all 0.3s;

      &.selected {
        background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
        border-color: var(--theme-primary);

        .option-text {
          color: #ffffff;
        }
      }

      &:active {
        transform: scale(0.95);
      }

      .option-text {
        font-size: 28rpx;
        color: var(--theme-text);
      }

      .check-icon {
        position: absolute;
        top: 5rpx;
        right: 10rpx;
        font-size: 24rpx;
        color: #ffffff;
      }
    }
  }

  .input-area {
    .answer-input {
      width: 100%;
      min-height: 200rpx;
      padding: 20rpx;
      background: var(--theme-background);
      border-radius: 15rpx;
      font-size: 28rpx;
      line-height: 1.6;
    }

    .char-count {
      display: block;
      text-align: right;
      font-size: 24rpx;
      color: var(--theme-text-secondary);
      margin-top: 10rpx;
    }
  }
}

.actions {
  padding: 30rpx;
  display: flex;
  gap: 20rpx;

  .action-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 30rpx 20rpx;
    border-radius: 20rpx;
    border: none;
    box-shadow: 0 8rpx 20rpx rgba(0, 0, 0, 0.1);
    transition: all 0.3s;

    &::after {
      border: none;
    }

    &:active {
      transform: scale(0.95);
    }

    .btn-icon {
      font-size: 48rpx;
      margin-bottom: 10rpx;
    }

    .btn-text {
      font-size: 26rpx;
      color: #ffffff;
    }
  }

  .refresh-btn {
    background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  }

  .community-btn {
    background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  }

  .share-btn {
    background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
  }
}

.community-modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.6);
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
}

.community-modal {
  width: 90%;
  max-width: 700rpx;
  height: 80vh;
  background: var(--theme-surface);
  border-radius: 30rpx;
  display: flex;
  flex-direction: column;
  overflow: hidden;

  .modal-header {
    padding: 40rpx 30rpx 20rpx;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1rpx solid #f0f0f0;

    .modal-title {
      font-size: 36rpx;
      font-weight: bold;
      color: var(--theme-text);
    }

    .modal-close {
      font-size: 40rpx;
      color: var(--theme-text-secondary);
      padding: 10rpx;
    }
  }

  .replies-list {
    flex: 1;
    padding: 20rpx;

    .reply-item {
      background: var(--theme-background);
      border-radius: 20rpx;
      padding: 25rpx;
      margin-bottom: 20rpx;

      .reply-header {
        display: flex;
        align-items: center;
        margin-bottom: 15rpx;

        .user-avatar {
          width: 60rpx;
          height: 60rpx;
          border-radius: 50%;
          background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
          display: flex;
          align-items: center;
          justify-content: center;
          font-size: 30rpx;
          margin-right: 15rpx;
        }

        .user-info {
          flex: 1;

          .user-name {
            display: block;
            font-size: 28rpx;
            color: var(--theme-text);
            font-weight: bold;
            margin-bottom: 5rpx;
          }

          .reply-time {
            font-size: 22rpx;
            color: var(--theme-text-secondary);
          }
        }

        .like-btn {
          display: flex;
          align-items: center;
          gap: 8rpx;
          padding: 10rpx 20rpx;
          border-radius: 50rpx;
          background: rgba(255, 255, 255, 0.8);
          font-size: 24rpx;
          transition: all 0.3s;

          &.liked {
            background: rgba(255, 105, 180, 0.1);
          }

          &:active {
            transform: scale(0.95);
          }

          .like-count {
            color: var(--theme-text-secondary);
          }
        }
      }

      .reply-content {
        font-size: 28rpx;
        color: var(--theme-text-secondary);
        line-height: 1.6;
      }
    }

    .empty-replies {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      padding: 100rpx 0;

      .empty-icon {
        font-size: 100rpx;
        margin-bottom: 20rpx;
      }

      .empty-text {
        font-size: 30rpx;
        color: var(--theme-text-secondary);
        margin-bottom: 10rpx;
      }

      .empty-hint {
        font-size: 24rpx;
        color: var(--theme-text-secondary);
      }
    }
  }

  .reply-input-box {
    padding: 20rpx;
    border-top: 1rpx solid #f0f0f0;
    display: flex;
    gap: 15rpx;

    .reply-input {
      flex: 1;
      height: 70rpx;
      padding: 0 20rpx;
      background: var(--theme-background);
      border-radius: 35rpx;
      font-size: 28rpx;
    }

    .send-btn {
      width: 120rpx;
      height: 70rpx;
      background: linear-gradient(135deg, var(--theme-primary) 0%, var(--theme-primary-light) 100%);
      color: #ffffff;
      border-radius: 35rpx;
      border: none;
      font-size: 28rpx;
      line-height: 70rpx;

      &::after {
        border: none;
      }
    }
  }
}

.history {
  padding: 0 30rpx 30rpx;

  .history-header {
    padding: 20rpx 0;

    .history-title {
      font-size: 32rpx;
      font-weight: bold;
      color: var(--theme-text);
    }
  }

  .history-list {
    .history-item {
      display: flex;
      align-items: center;
      padding: 25rpx;
      background: rgba(255, 255, 255, 0.9);
      border-radius: 20rpx;
      margin-bottom: 15rpx;

      &:active {
        background: rgba(255, 255, 255, 1);
      }

      .history-icon {
        font-size: 48rpx;
        margin-right: 20rpx;
      }

      .history-info {
        flex: 1;

        .history-question {
          display: block;
          font-size: 28rpx;
          color: var(--theme-text);
          margin-bottom: 8rpx;
        }

        .history-date {
          font-size: 22rpx;
          color: var(--theme-text-secondary);
        }
      }
    }
  }
}
</style>

