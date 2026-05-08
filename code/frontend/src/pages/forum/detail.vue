<template>
  <view class="detail-page">
    <scroll-view class="content-scroll" scroll-y>
      <!-- 帖子内容 -->
      <view v-if="post" class="post-container">
        <!-- 帖子头部 -->
        <view class="post-header">
          <view class="post-type-badge" :class="`type-${post.post_type}`">
            <text>{{ getTypeLabel(post.post_type) }}</text>
          </view>
          <text class="post-time">{{ formatTime(post.created_at) }}</text>
        </view>

        <!-- 帖子标题 -->
        <text class="post-title">{{ post.title }}</text>

        <!-- 灵感卡片展示 -->
        <view v-if="post.post_type === 'inspiration'" class="inspiration-card">
          <view class="card-icon">
            {{ post.card_category === 'encourage' ? '💪' : '📚' }}
          </view>
          <text class="card-content">{{ post.content }}</text>
          <text v-if="post.author" class="card-author">— {{ post.author }}</text>
        </view>

        <!-- 话题展示 -->
        <view v-else-if="post.post_type === 'topic'" class="topic-card">
          <view class="topic-content">
            <text>{{ post.content }}</text>
          </view>
          <view v-if="post.topic_answer" class="answer-section">
            <text class="answer-label">我的想法：</text>
            <text class="answer-text">{{ post.topic_answer }}</text>
          </view>
        </view>

        <!-- 创作展示 -->
        <view v-else-if="post.post_type === 'creation'" class="creation-card">
          <view class="creation-meta">
            <text class="category-tag">{{ post.creation_category }}</text>
            <view v-if="post.creation_tags && post.creation_tags.length > 0" class="tags">
              <text
                v-for="tag in post.creation_tags"
                :key="tag"
                class="tag"
              >#{{ tag }}</text>
            </view>
          </view>
          <view class="creation-content">
            <text>{{ post.content }}</text>
          </view>
        </view>

        <!-- 帖子统计 -->
        <view class="post-stats">
          <view class="stat-item" @click="handleLike">
            <text class="stat-icon">👍</text>
            <text class="stat-count">{{ post.likes }}</text>
          </view>
          <view class="stat-item">
            <text class="stat-icon">👁️</text>
            <text class="stat-count">{{ post.views }}</text>
          </view>
          <view class="stat-item">
            <text class="stat-icon">💬</text>
            <text class="stat-count">{{ post.comments_count }}</text>
          </view>
        </view>
      </view>

      <!-- 评论列表 -->
      <view class="comments-section">
        <view class="section-title">
          <text>评论 ({{ comments.length }})</text>
        </view>

        <view v-if="comments.length === 0" class="empty-comments">
          <text class="empty-icon">💬</text>
          <text class="empty-text">还没有评论，快来抢沙发吧~</text>
        </view>

        <view v-else class="comments-list">
          <!-- 扁平化渲染所有评论（包含层级信息） -->
          <view
            v-for="comment in flattenedComments"
            :key="comment.id"
            class="comment-item"
            :style="{ paddingLeft: (comment.level * 40) + 'rpx' }"
            :class="{ 'child-item': comment.level > 0 }"
          >
            <view class="comment-header">
              <view class="comment-author-section">
                <text 
                  class="comment-author" 
                  :class="{ 'clickable': comment.user_id }"
                  @click="comment.user_id ? handleViewUser(comment.user_id) : null"
                >
                  {{ comment.nickname || comment.username || comment.guest_name || '匿名用户' }}
                </text>
                <!-- 如果是子评论，显示回复关系 -->
                <template v-if="comment.level > 0 && comment.parentComment">
                  <text class="reply-to">回复</text>
                  <text 
                    class="reply-target" 
                    :class="{ 'clickable': comment.parentComment.user_id }"
                    @click="comment.parentComment.user_id ? handleViewUser(comment.parentComment.user_id) : null"
                  >
                    @{{ comment.parentComment.nickname || comment.parentComment.username || comment.parentComment.guest_name || '匿名用户' }}
                  </text>
                </template>
              </view>
              <text class="comment-time">{{ formatTime(comment.created_at) }}</text>
            </view>
            
            <text class="comment-content">{{ comment.content }}</text>
            
            <view class="comment-footer">
              <view class="comment-actions">
                <view class="comment-like" @click="handleLikeComment(comment.id)">
                  <text class="like-icon">👍</text>
                  <text class="like-count">{{ comment.likes }}</text>
                </view>
                <view class="comment-reply" @click="handleReply(comment)">
                  <text class="reply-icon">💬</text>
                  <text class="reply-text">回复</text>
                </view>
                <!-- 展开/收起按钮 -->
                <view 
                  v-if="comment.childrenCount > 0"
                  class="expand-btn" 
                  @click="toggleExpand(comment.id)"
                >
                  <text class="expand-icon">{{ expandedCommentIds[comment.id] ? '▼' : '▶' }}</text>
                  <text class="expand-text">
                    {{ expandedCommentIds[comment.id] ? '收起' : `展开 ${comment.childrenCount} 条回复` }}
                  </text>
                </view>
              </view>
            </view>
          </view>
        </view>
      </view>
    </scroll-view>

    <!-- 评论输入栏 -->
    <view class="comment-input-bar">
      <view class="input-wrapper">
        <view v-if="replyToUser" class="reply-hint">
          <text>回复 @{{ replyToUser }}</text>
          <text class="cancel-reply" @click="cancelReply">✕</text>
        </view>
        <textarea
          v-model="commentInput"
          class="comment-input"
          :placeholder="replyToUser ? `回复 @${replyToUser}` : '写下你的评论...'"
          :disabled="!userStore.isLoggedIn && !guestName"
          :auto-height="true"
          :maxlength="500"
          @focus="handleInputFocus"
        />
      </view>
      <button
        class="send-btn"
        :disabled="!commentInput.trim()"
        @click="handleSendComment"
      >
        发送
      </button>
    </view>

    <!-- 游客名称输入弹窗 -->
    <view v-if="showGuestModal" class="guest-modal" @click="showGuestModal = false">
      <view class="modal-content" @click.stop>
        <text class="modal-title">请输入你的昵称</text>
        <input
          v-model="guestName"
          class="modal-input"
          placeholder="昵称"
          maxlength="20"
        />
        <view class="modal-actions">
          <button class="modal-btn cancel" @click="showGuestModal = false">取消</button>
          <button class="modal-btn confirm" @click="handleConfirmGuest">确定</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import {
  getPostApi,
  getCommentsApi,
  createCommentApi,
  likePostApi,
  likeCommentApi,
  type ForumPost,
  type ForumComment
} from '@/api/forum'
import { useUserStore } from '@/store'
import { getFriendsApi } from '@/api/friend'

const userStore = useUserStore()

const postId = ref('')
const post = ref<ForumPost | null>(null)
const comments = ref<ForumComment[]>([])
const commentInput = ref('')
const guestName = ref('')
const showGuestModal = ref(false)
const loading = ref(false)
const replyToComment = ref<string | null>(null)
const replyToUser = ref<string>('')
// 展开的评论ID集合
const expandedCommentIds = ref<Record<string, boolean>>({})

// 构建评论树的辅助接口
interface CommentTreeNode extends ForumComment {
  level: number
  childrenCount: number
  parentComment?: ForumComment
}

// 将树形评论结构扁平化为列表（用于渲染）
const flattenedComments = computed(() => {
  // 构建评论映射
  const commentMap = new Map<string, ForumComment>()
  const childrenMap = new Map<string, ForumComment[]>()
  
  comments.value.forEach(comment => {
    commentMap.set(comment.id, comment)
    const parentId = comment.parent_id || 'root'
    if (!childrenMap.has(parentId)) {
      childrenMap.set(parentId, [])
    }
    childrenMap.get(parentId)!.push(comment)
  })

  const result: CommentTreeNode[] = []

  // 递归函数：将树形结构展开为扁平列表
  const flatten = (commentId: string, level: number, parentComment?: ForumComment) => {
    const comment = commentMap.get(commentId)
    if (!comment) return

    const children = childrenMap.get(commentId) || []
    
    // 添加当前评论
    result.push({
      ...comment,
      level,
      childrenCount: children.length,
      parentComment
    })

    // 如果当前评论展开，则添加其子评论
    if (expandedCommentIds.value[commentId]) {
      children.forEach(child => {
        flatten(child.id, level + 1, comment)
      })
    }
  }

  // 从根评论开始
  const rootComments = childrenMap.get('root') || []
  rootComments.forEach(comment => {
    flatten(comment.id, 0)
  })

  return result
})

// 切换评论展开/收起
const toggleExpand = (commentId: string) => {
  expandedCommentIds.value[commentId] = !expandedCommentIds.value[commentId]
}

onMounted(() => {
  const pages = getCurrentPages()
  const currentPage = pages[pages.length - 1]
  postId.value = (currentPage as any).options?.id || ''
  if (postId.value) {
    loadPost()
    loadComments()
  }
})

const loadPost = async () => {
  try {
    const res = await getPostApi(postId.value)
    if (res) {
      const responseData = (res as any).data || res
      post.value = responseData
    }
  } catch (error) {
    console.error('加载帖子失败:', error)
    uni.showToast({ title: '加载失败', icon: 'none' })
  }
}

const loadComments = async () => {
  if (loading.value) return
  loading.value = true
  try {
    const res = await getCommentsApi(postId.value, { page: 1, page_size: 100 })
    if (res) {
      const responseData = (res as any).data || res
      comments.value = Array.isArray(responseData) ? responseData : responseData.list || []
      console.log('加载的评论数据:', comments.value)
      console.log('评论总数:', comments.value.length)
      console.log('顶级评论:', comments.value.filter(c => !c.parent_id).length)
      console.log('回复评论:', comments.value.filter(c => c.parent_id).length)
    }
  } catch (error) {
    console.error('加载评论失败:', error)
  } finally {
    loading.value = false
  }
}

const handleLike = async () => {
  if (!post.value) return
  try {
    await likePostApi(post.value.id)
    if (post.value) {
      post.value.likes++
    }
    uni.showToast({ title: '点赞成功', icon: 'success' })
  } catch (error) {
    console.error('点赞失败:', error)
  }
}

const handleLikeComment = async (commentId: string) => {
  try {
    await likeCommentApi(commentId)
    const comment = comments.value.find(c => c.id === commentId)
    if (comment) {
      comment.likes++
    }
  } catch (error) {
    console.error('点赞评论失败:', error)
  }
}

const handleInputFocus = () => {
  if (!userStore.isLoggedIn && !guestName.value) {
    showGuestModal.value = true
  }
}

const handleConfirmGuest = () => {
  if (!guestName.value.trim()) {
    uni.showToast({ title: '请输入昵称', icon: 'none' })
    return
  }
  showGuestModal.value = false
}

const handleSendComment = async () => {
  if (!commentInput.value.trim()) return

  if (!userStore.isLoggedIn && !guestName.value.trim()) {
    showGuestModal.value = true
    return
  }

  try {
    const data: any = {
      post_id: postId.value,
      content: commentInput.value.trim()
    }

    if (!userStore.isLoggedIn) {
      data.guest_name = guestName.value.trim()
    }

    // 如果是回复评论，添加parent_id
    if (replyToComment.value) {
      data.parent_id = replyToComment.value
    }

    await createCommentApi(postId.value, data)
    commentInput.value = ''
    replyToComment.value = null
    replyToUser.value = ''
    uni.showToast({ title: '评论成功', icon: 'success' })
    loadComments()
    if (post.value) {
      post.value.comments_count++
    }
  } catch (error: any) {
    console.error('评论失败:', error)
    uni.showToast({
      title: error?.message || '评论失败',
      icon: 'none'
    })
  }
}

const handleReply = (comment: ForumComment) => {
  // 树形结构：parent_id 就是被回复评论的 id
  replyToComment.value = comment.id
  replyToUser.value = comment.nickname || comment.username || comment.guest_name || '该用户'
  // 聚焦到输入框
  uni.showToast({ title: `回复 @${replyToUser.value}`, icon: 'none' })
}

const cancelReply = () => {
  replyToComment.value = null
  replyToUser.value = ''
}

const handleViewUser = async (userId: string) => {
  // 检查是否是当前用户
  if (userStore.isLoggedIn && userStore.userId === userId) {
    // 如果是自己，跳转到自己的个人信息页
    uni.navigateTo({
      url: '/pages/user/profile'
    })
    return
  }

  // 如果不是当前用户，检查是否是好友
  if (userStore.isLoggedIn) {
    try {
      // 获取好友列表
      const friends = await getFriendsApi()
      if (friends && Array.isArray(friends)) {
        // 检查是否是好友（好友列表中的 user_id 字段）
        const isFriend = friends.some((friend: any) => friend.user_id === userId || friend.id === userId)
        
        if (isFriend) {
          // 如果是好友，跳转到他的个人信息页
          uni.navigateTo({
            url: `/pages/user/profile?userId=${userId}`
          })
        } else {
          // 如果不是好友，跳转到他的信息页并显示加好友按钮
          uni.navigateTo({
            url: `/pages/user/profile?userId=${userId}&showAddFriend=true`
          })
        }
      } else {
        // 如果获取好友列表失败，默认跳转到用户信息页并显示加好友按钮
        uni.navigateTo({
          url: `/pages/user/profile?userId=${userId}&showAddFriend=true`
        })
      }
    } catch (error) {
      console.error('获取好友列表失败:', error)
      // 如果出错，默认跳转到用户信息页并显示加好友按钮
      uni.navigateTo({
        url: `/pages/user/profile?userId=${userId}&showAddFriend=true`
      })
    }
  } else {
    // 如果未登录，跳转到用户信息页并显示加好友按钮（游客也可以查看用户信息）
    uni.navigateTo({
      url: `/pages/user/profile?userId=${userId}&showAddFriend=true`
    })
  }
}

const getTypeLabel = (type: string) => {
  const map: Record<string, string> = {
    inspiration: '灵感',
    topic: '话题',
    creation: '创作'
  }
  return map[type] || type
}

const formatTime = (time: string) => {
  const date = new Date(time)
  const now = new Date()
  const diff = now.getTime() - date.getTime()
  const minutes = Math.floor(diff / 60000)
  const hours = Math.floor(diff / 3600000)
  const days = Math.floor(diff / 86400000)

  if (minutes < 1) return '刚刚'
  if (minutes < 60) return `${minutes}分钟前`
  if (hours < 24) return `${hours}小时前`
  if (days < 7) return `${days}天前`
  
  return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.detail-page {
  @include cyber-page-bg;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 25% 35%, rgba(138, 92, 246, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 75% 65%, rgba(0, 217, 255, 0.15) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

.content-scroll {
  flex: 1;
  padding-bottom: 120rpx;
  position: relative;
  z-index: 1;
}

.post-container {
  @include neon-card;
  background: linear-gradient(135deg, rgba(20, 26, 56, 0.95) 0%, rgba(15, 20, 45, 0.98) 100%);
  backdrop-filter: blur(30rpx);
  padding: 35rpx;
  margin: 20rpx;
  border-radius: 30rpx;
  border: 3rpx solid rgba(138, 92, 246, 0.5);
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(138, 92, 246, 0.4),
    inset 0 0 50rpx rgba(138, 92, 246, 0.08);
}

.post-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 25rpx;

  .post-type-badge {
    @include neon-tag(#FFD600);
    padding: 10rpx 24rpx;
    border-radius: 25rpx;
    font-size: 24rpx;
    font-weight: bold;

    &.type-inspiration {
      @include neon-tag(#FFD600);
    }

    &.type-topic {
      @include neon-tag(#00D9FF);
    }

    &.type-creation {
      @include neon-tag(#FF00D6);
    }
  }

  .post-time {
    font-size: 24rpx;
    @include neon-text(#6b7b93);
  }
}

.post-title {
  display: block;
  font-size: 40rpx;
  font-weight: bold;
  @include neon-title(#ffffff);
  margin-bottom: 35rpx;
  line-height: 1.6;
  filter: brightness(1.1);
}

.inspiration-card {
  background: linear-gradient(135deg, rgba(255, 214, 0, 0.2) 0%, rgba(138, 92, 246, 0.2) 100%);
  backdrop-filter: blur(20rpx);
  border-radius: 30rpx;
  border: 3rpx solid rgba(255, 214, 0, 0.5);
  padding: 45rpx;
  margin-bottom: 35rpx;
  text-align: center;
  box-shadow: 
    0 12rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 50rpx rgba(255, 214, 0, 0.4),
    inset 0 0 40rpx rgba(255, 214, 0, 0.1);
  animation: cardFloat 4s ease-in-out infinite;

  .card-icon {
    font-size: 100rpx;
    margin-bottom: 25rpx;
    filter: drop-shadow(0 0 20rpx rgba(255, 214, 0, 0.8));
    animation: iconPulse 2s ease-in-out infinite;
  }

  .card-content {
    display: block;
    font-size: 34rpx;
    line-height: 1.9;
    margin-bottom: 25rpx;
    @include neon-text(#ffffff);
    text-shadow: 
      0 2rpx 8rpx rgba(0, 0, 0, 0.3),
      0 0 15rpx rgba(255, 255, 255, 0.3);
  }

  .card-author {
    display: block;
    font-size: 28rpx;
    @include neon-text(#FFD600);
    font-weight: 600;
  }
}

.topic-card {
  .topic-content {
    padding: 35rpx;
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.15) 0%, rgba(138, 92, 246, 0.15) 100%);
    backdrop-filter: blur(15rpx);
    border-radius: 25rpx;
    border: 2rpx solid rgba(0, 217, 255, 0.4);
    margin-bottom: 25rpx;
    font-size: 32rpx;
    @include neon-text(#ffffff);
    line-height: 1.9;
    white-space: pre-wrap;
    box-shadow: 
      0 8rpx 24rpx rgba(0, 0, 0, 0.4),
      0 0 40rpx rgba(0, 217, 255, 0.2);
    text-shadow: 
      0 1rpx 3rpx rgba(0, 0, 0, 0.3),
      0 0 10rpx rgba(255, 255, 255, 0.15);
  }

  .answer-section {
    padding: 30rpx;
    background: linear-gradient(135deg, rgba(255, 214, 0, 0.15) 0%, rgba(255, 214, 0, 0.08) 100%);
    backdrop-filter: blur(15rpx);
    border-radius: 25rpx;
    border-left: 4rpx solid rgba(255, 214, 0, 0.8);
    box-shadow: 
      0 6rpx 20rpx rgba(0, 0, 0, 0.3),
      0 0 30rpx rgba(255, 214, 0, 0.2);

    .answer-label {
      display: block;
      font-size: 26rpx;
      @include neon-text(#FFD600);
      margin-bottom: 15rpx;
      font-weight: 600;
    }

    .answer-text {
      font-size: 30rpx;
      @include neon-text(#ffffff);
      line-height: 1.8;
      text-shadow: 
        0 1rpx 3rpx rgba(0, 0, 0, 0.3),
        0 0 10rpx rgba(255, 255, 255, 0.15);
    }
  }
}

.creation-card {
  .creation-meta {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 18rpx;
    margin-bottom: 25rpx;

    .category-tag {
      @include neon-tag(#FF00D6);
      padding: 10rpx 24rpx;
      border-radius: 25rpx;
      font-size: 26rpx;
      font-weight: bold;
    }

    .tags {
      display: flex;
      flex-wrap: wrap;
      gap: 12rpx;

      .tag {
        font-size: 24rpx;
        @include neon-tag(#00D9FF);
        padding: 8rpx 18rpx;
        border-radius: 20rpx;
      }
    }
  }

  .creation-content {
    font-size: 32rpx;
    @include neon-text(#ffffff);
    line-height: 1.9;
    white-space: pre-wrap;
    text-shadow: 
      0 1rpx 3rpx rgba(0, 0, 0, 0.3),
      0 0 10rpx rgba(255, 255, 255, 0.15);
  }
}

.post-stats {
  display: flex;
  gap: 45rpx;
  padding-top: 35rpx;
  border-top: 2rpx solid rgba(138, 92, 246, 0.3);

  .stat-item {
    display: flex;
    align-items: center;
    gap: 12rpx;
    cursor: pointer;
    transition: all 0.3s;
    padding: 8rpx 15rpx;
    border-radius: 20rpx;

    &:active {
      background: linear-gradient(135deg, rgba(138, 92, 246, 0.2) 0%, rgba(0, 217, 255, 0.2) 100%);
      transform: scale(1.1);
    }

    .stat-icon {
      font-size: 36rpx;
      filter: drop-shadow(0 0 10rpx rgba(255, 255, 255, 0.4));
    }

    .stat-count {
      font-size: 30rpx;
      @include neon-text(#b8c5d6);
      font-weight: 600;
    }
  }
}

.comments-section {
  @include neon-card;
  background: linear-gradient(135deg, rgba(20, 26, 56, 0.95) 0%, rgba(15, 20, 45, 0.98) 100%);
  backdrop-filter: blur(30rpx);
  padding: 35rpx;
  margin: 20rpx;
  border-radius: 30rpx;
  border: 3rpx solid rgba(0, 217, 255, 0.5);
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(0, 217, 255, 0.4),
    inset 0 0 50rpx rgba(0, 217, 255, 0.08);

  .section-title {
    font-size: 36rpx;
    font-weight: bold;
    @include neon-title(#00D9FF);
    margin-bottom: 35rpx;
  }

  .empty-comments {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 100rpx 0;
    @include neon-text(#6b7b93);

    .empty-icon {
      font-size: 100rpx;
      margin-bottom: 25rpx;
      filter: drop-shadow(0 0 20rpx rgba(0, 217, 255, 0.6));
      animation: emptyFloat 3s ease-in-out infinite;
    }
    
    .empty-text {
      font-size: 28rpx;
    }
  }

  .comments-list {
    .comment-item {
      padding: 25rpx 20rpx 25rpx 0;
      border-bottom: 2rpx solid rgba(0, 217, 255, 0.2);
      position: relative;
      transition: all 0.3s;
      animation: commentFadeIn 0.5s ease-out backwards;

      &:last-child {
        border-bottom: none;
      }

      // 子评论样式
      &.child-item {
        background: linear-gradient(135deg, rgba(0, 217, 255, 0.08) 0%, rgba(138, 92, 246, 0.05) 100%);
        backdrop-filter: blur(10rpx);
        border-left: 4rpx solid rgba(0, 217, 255, 0.6);
        border-radius: 0 20rpx 20rpx 0;
        padding-left: 20rpx !important;
        margin: 10rpx 0;
        box-shadow: 
          0 4rpx 12rpx rgba(0, 0, 0, 0.3),
          0 0 20rpx rgba(0, 217, 255, 0.2);
        
        &:before {
          content: '';
          position: absolute;
          left: 0;
          top: 0;
          width: 4rpx;
          height: 100%;
          background: linear-gradient(to bottom, rgba(0, 217, 255, 0.8) 0%, transparent 100%);
        }
      }

      .comment-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 15rpx;

        .comment-author-section {
          display: flex;
          align-items: center;
          gap: 10rpx;
          flex-wrap: wrap;
          flex: 1;
        }

        .comment-author {
          font-size: 30rpx;
          @include neon-text(#00D9FF);
          font-weight: 600;

          &.clickable {
            cursor: pointer;
            text-decoration: underline;
            
            &:active {
              text-shadow: 
                0 0 20rpx rgba(0, 217, 255, 1),
                0 0 40rpx rgba(0, 217, 255, 0.8);
            }
          }
        }

        .reply-to {
          font-size: 24rpx;
          @include neon-text(#6b7b93);
        }

        .reply-target {
          font-size: 26rpx;
          @include neon-text(#8B5CF6);
          font-weight: 600;

          &.clickable {
            cursor: pointer;
            text-decoration: underline;
            
            &:active {
              text-shadow: 
                0 0 20rpx rgba(138, 92, 246, 1),
                0 0 40rpx rgba(138, 92, 246, 0.8);
            }
          }
        }

        .comment-time {
          font-size: 24rpx;
          @include neon-text(#6b7b93);
          flex-shrink: 0;
        }
      }

      .comment-content {
        display: block;
        font-size: 30rpx;
        @include neon-text(#ffffff);
        line-height: 1.8;
        margin-bottom: 18rpx;
        word-wrap: break-word;
        text-shadow: 
          0 1rpx 3rpx rgba(0, 0, 0, 0.3),
          0 0 10rpx rgba(255, 255, 255, 0.15);
      }

      .comment-footer {
        display: flex;
        justify-content: flex-end;

        .comment-actions {
          display: flex;
          gap: 18rpx;
          align-items: center;

          .comment-like,
          .comment-reply,
          .expand-btn {
            display: flex;
            align-items: center;
            gap: 10rpx;
            padding: 10rpx 18rpx;
            background: linear-gradient(135deg, rgba(30, 36, 66, 0.5) 0%, rgba(30, 36, 66, 0.3) 100%);
            backdrop-filter: blur(10rpx);
            border: 2rpx solid rgba(138, 92, 246, 0.4);
            border-radius: 25rpx;
            cursor: pointer;
            transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.3);

            &:active {
              transform: scale(0.92);
              border-color: rgba(138, 92, 246, 0.7);
              box-shadow: 
                0 6rpx 18rpx rgba(0, 0, 0, 0.4),
                0 0 30rpx rgba(138, 92, 246, 0.5);
            }

            .like-icon,
            .reply-icon,
            .expand-icon {
              font-size: 28rpx;
              filter: drop-shadow(0 0 8rpx rgba(255, 255, 255, 0.3));
            }

            .like-count,
            .reply-text,
            .expand-text {
              font-size: 24rpx;
              @include neon-text(#b8c5d6);
              font-weight: 600;
            }
          }

          .expand-btn {
            background: linear-gradient(135deg, rgba(0, 217, 255, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
            border-color: rgba(0, 217, 255, 0.6);
            box-shadow: 
              0 4rpx 12rpx rgba(0, 0, 0, 0.3),
              0 0 20rpx rgba(0, 217, 255, 0.3);

            .expand-icon {
              font-size: 22rpx;
              @include neon-text(#00D9FF);
              transition: transform 0.3s ease;
            }

            .expand-text {
              @include neon-text(#00D9FF);
              font-weight: 600;
            }

            &:active {
              background: linear-gradient(135deg, rgba(0, 217, 255, 0.5) 0%, rgba(138, 92, 246, 0.5) 100%);
              box-shadow: 
                0 6rpx 18rpx rgba(0, 0, 0, 0.4),
                0 0 40rpx rgba(0, 217, 255, 0.5);
            }
          }
        }
      }
    }
  }
}

.comment-input-bar {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 25rpx 30rpx;
  padding-bottom: calc(25rpx + env(safe-area-inset-bottom));
  @include neon-card;
  background: linear-gradient(180deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 1) 100%);
  backdrop-filter: blur(30rpx);
  box-shadow: 
    0 -15rpx 50rpx rgba(0, 0, 0, 0.7),
    0 0 80rpx rgba(255, 0, 214, 0.4),
    inset 0 2rpx 0 rgba(255, 0, 214, 0.3);
  border-top: 3rpx solid rgba(255, 0, 214, 0.6);
  display: flex;
  align-items: flex-end;
  gap: 25rpx;
  z-index: 100;

  .input-wrapper {
    flex: 1;
    display: flex;
    flex-direction: column;

    .reply-hint {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 12rpx 24rpx;
      background: linear-gradient(135deg, rgba(0, 217, 255, 0.2) 0%, rgba(138, 92, 246, 0.2) 100%);
      backdrop-filter: blur(10rpx);
      border-radius: 20rpx 20rpx 0 0;
      font-size: 26rpx;
      @include neon-text(#00D9FF);
      border: 2rpx solid rgba(0, 217, 255, 0.4);
      border-bottom: none;

      .cancel-reply {
        font-size: 32rpx;
        font-weight: bold;
        @include neon-text(#FF00D6);
        cursor: pointer;
        padding: 0 12rpx;
        transition: all 0.3s;

        &:active {
          transform: rotate(90deg);
          text-shadow: 
            0 0 30rpx rgba(255, 0, 214, 1),
            0 0 50rpx rgba(255, 0, 214, 0.8);
        }
      }
    }

    .comment-input {
      width: 100%;
      min-height: 80rpx;
      max-height: 300rpx;
      padding: 28rpx 32rpx;
      @include neon-input;
      border-radius: 25rpx;
      font-size: 30rpx;
      @include neon-text(#ffffff);
      line-height: 1.6;
      box-sizing: border-box;
      transition: all 0.4s;

      &:focus {
        border-color: rgba(255, 0, 214, 0.8);
        box-shadow: 
          0 0 50rpx rgba(255, 0, 214, 0.5),
          inset 0 0 30rpx rgba(255, 0, 214, 0.1);
      }
    }
  }

  .send-btn {
    padding: 28rpx 48rpx;
    background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
    @include neon-text(#ffffff);
    border: 2rpx solid rgba(255, 0, 214, 0.7);
    border-radius: 25rpx;
    font-size: 32rpx;
    font-weight: bold;
    min-width: 140rpx;
    box-shadow: 
      0 8rpx 24rpx rgba(255, 0, 214, 0.5),
      0 0 40rpx rgba(255, 0, 214, 0.4);
    text-shadow: 
      0 0 12rpx rgba(255, 255, 255, 0.8),
      0 2rpx 5rpx rgba(0, 0, 0, 0.3);
    transition: all 0.3s;

    &[disabled] {
      background: linear-gradient(135deg, rgba(100, 100, 100, 0.4) 0%, rgba(80, 80, 80, 0.4) 100%);
      @include neon-text(#6b7b93);
      border-color: rgba(100, 100, 100, 0.4);
      box-shadow: none;
      text-shadow: none;
    }

    &:active:not([disabled]) {
      transform: scale(0.95);
      box-shadow: 
        0 10rpx 30rpx rgba(255, 0, 214, 0.7),
        0 0 60rpx rgba(255, 0, 214, 0.6);
    }
  }
}

.guest-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.75);
  backdrop-filter: blur(10rpx);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.3s ease-out;

  .modal-content {
    width: 600rpx;
    @include neon-card;
    background: linear-gradient(135deg, rgba(20, 26, 56, 0.98) 0%, rgba(15, 20, 45, 1) 100%);
    backdrop-filter: blur(30rpx);
    border-radius: 30rpx;
    border: 3rpx solid rgba(255, 0, 214, 0.6);
    padding: 45rpx;
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.7),
      0 0 80rpx rgba(255, 0, 214, 0.5),
      inset 0 0 60rpx rgba(255, 0, 214, 0.15);
    animation: scaleIn 0.4s cubic-bezier(0.4, 0, 0.2, 1);

    .modal-title {
      display: block;
      font-size: 36rpx;
      font-weight: bold;
      @include neon-title(#FFD600);
      margin-bottom: 35rpx;
      text-align: center;
    }

    .modal-input {
      @include neon-input;
      width: 100%;
      padding: 25rpx;
      border-radius: 20rpx;
      font-size: 30rpx;
      @include neon-text(#ffffff);
      margin-bottom: 35rpx;
      box-sizing: border-box;

      &:focus {
        border-color: rgba(255, 0, 214, 0.8);
        box-shadow: 
          0 0 50rpx rgba(255, 0, 214, 0.5),
          inset 0 0 30rpx rgba(255, 0, 214, 0.1);
      }
    }

    .modal-actions {
      display: flex;
      gap: 25rpx;

      .modal-btn {
        flex: 1;
        padding: 25rpx;
        border-radius: 25rpx;
        font-size: 30rpx;
        font-weight: bold;
        border: none;
        transition: all 0.3s;

        &.cancel {
          background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
          backdrop-filter: blur(10rpx);
          @include neon-text(#b8c5d6);
          border: 2rpx solid rgba(138, 92, 246, 0.5);
          box-shadow: 0 6rpx 18rpx rgba(0, 0, 0, 0.4);

          &:active {
            transform: scale(0.95);
          }
        }

        &.confirm {
          background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
          @include neon-text(#ffffff);
          border: 2rpx solid rgba(255, 0, 214, 0.7);
          box-shadow: 
            0 8rpx 24rpx rgba(255, 0, 214, 0.5),
            0 0 40rpx rgba(255, 0, 214, 0.4);
          text-shadow: 
            0 0 12rpx rgba(255, 255, 255, 0.8),
            0 2rpx 5rpx rgba(0, 0, 0, 0.3);

          &:active {
            transform: scale(0.95);
            box-shadow: 
              0 10rpx 30rpx rgba(255, 0, 214, 0.7),
              0 0 60rpx rgba(255, 0, 214, 0.6);
          }
        }
      }
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes cardFloat {
  0%, 100% {
    box-shadow: 
      0 12rpx 40rpx rgba(0, 0, 0, 0.5),
      0 0 50rpx rgba(255, 214, 0, 0.4),
      inset 0 0 40rpx rgba(255, 214, 0, 0.1);
  }
  50% {
    box-shadow: 
      0 15rpx 45rpx rgba(0, 0, 0, 0.6),
      0 0 70rpx rgba(255, 214, 0, 0.6),
      inset 0 0 50rpx rgba(255, 214, 0, 0.15);
  }
}

@keyframes iconPulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

@keyframes emptyFloat {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-20rpx); }
}

@keyframes commentFadeIn {
  from {
    opacity: 0;
    transform: translateX(-20rpx);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes scaleIn {
  from {
    transform: scale(0.85);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}
</style>

