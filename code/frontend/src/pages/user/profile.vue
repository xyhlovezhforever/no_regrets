<template>
  <view class="profile-page">
    <!-- 加载中 -->
    <view v-if="loading" class="loading">
      <text>加载中...</text>
    </view>

    <!-- 用户信息 -->
    <view v-else class="profile-content">
      <!-- 头部信息卡片 -->
      <view class="profile-header">
        <view class="avatar-section">
          <view v-if="displayUser.avatar" class="avatar-wrapper">
            <image class="avatar" :src="getFullUrl(displayUser.avatar)" mode="aspectFill" />
          </view>
          <view v-else class="avatar-wrapper avatar-emoji">
            <text>👤</text>
          </view>
          <!-- 只有查看自己时才显示编辑图标 -->
          <view v-if="isOwnProfile" class="edit-avatar-btn" @click="chooseAvatar">
            <text>📷</text>
          </view>
        </view>
        <view class="user-info">
          <view class="nickname-row">
            <text class="nickname">{{ displayUser.nickname || displayUser.username || '未知用户' }}</text>
          </view>
          <text class="username">@{{ displayUser.username }}</text>
          <text v-if="displayUser.bio" class="bio">{{ displayUser.bio }}</text>
          <text v-else class="bio empty">{{ isOwnProfile ? '还没有个性签名' : 'Ta还没有个性签名' }}</text>
        </view>
      </view>

      <!-- 加好友按钮（查看他人且不是好友时显示） -->
      <view v-if="!isOwnProfile && showAddFriend" class="action-section">
        <button class="add-friend-btn" @click="handleAddFriend">
          <text class="btn-icon">➕</text>
          <text>加好友</text>
        </button>
      </view>

      <!-- 好友标识（查看他人且是好友时显示） -->
      <view v-if="!isOwnProfile && !showAddFriend" class="friend-badge">
        <text class="badge-icon">✓</text>
        <text>已是好友</text>
      </view>

      <!-- 个人信息编辑区（只有查看自己时显示） -->
      <view v-if="isOwnProfile" class="edit-section">
        <view class="section-title">
          <text>个人信息</text>
        </view>
        
        <view class="section">
          <view class="item" @click="editNickname">
            <text class="label">昵称</text>
            <view class="value">
              <text class="value-text">{{ displayUser.nickname || '未设置' }}</text>
              <text class="arrow">›</text>
            </view>
          </view>
          
          <view class="item">
            <text class="label">账号</text>
            <view class="value">
              <text class="value-text readonly">{{ displayUser.username }}</text>
            </view>
          </view>

          <view class="item" @click="editBio">
            <text class="label">个性签名</text>
            <view class="value">
              <text class="value-text">{{ displayUser.bio || '未设置' }}</text>
              <text class="arrow">›</text>
            </view>
          </view>
        </view>
      </view>
    </view>

    <!-- 编辑昵称弹窗 -->
    <view v-if="showNicknameEdit" class="modal-mask" @click="showNicknameEdit = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">修改昵称</text>
        </view>
        <view class="modal-body">
          <input
            class="modal-input"
            v-model="tempNickname"
            placeholder="请输入昵称"
            :maxlength="20"
          />
        </view>
        <view class="modal-footer">
          <button class="modal-btn" @click="showNicknameEdit = false">取消</button>
          <button class="modal-btn primary" @click="saveNickname">确定</button>
        </view>
      </view>
    </view>

    <!-- 编辑个性签名弹窗 -->
    <view v-if="showBioEdit" class="modal-mask" @click="showBioEdit = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">修改个性签名</text>
        </view>
        <view class="modal-body">
          <textarea
            class="modal-textarea"
            v-model="tempBio"
            placeholder="介绍一下自己吧"
            :maxlength="100"
          />
        </view>
        <view class="modal-footer">
          <button class="modal-btn" @click="showBioEdit = false">取消</button>
          <button class="modal-btn primary" @click="saveBio">确定</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useUserStore } from '@/store'
import { showToast } from '@/utils'
import { BASE_API } from '@/config'
import { addFriendApi, getFriendsApi } from '@/api/friend'
import { getPublicUserInfoApi, updateUserInfoApi } from '@/api/user'
import { uploadImageApi } from '@/api/upload'
import type { UserInfo } from '@/types'

const userStore = useUserStore()

const loading = ref(false)
const viewUserId = ref('')
const viewUserInfo = ref<any>(null)
const showAddFriendParam = ref(false)
const isFriend = ref(false)

// 将相对路径转换为完整URL
const getFullUrl = (url: string | null | undefined) => {
  if (!url) return ''
  if (url.startsWith('http')) return url
  return `${BASE_API.replace('/api/v1', '')}${url}`
}

// 是否是查看自己的信息
const isOwnProfile = computed(() => {
  return !viewUserId.value || viewUserId.value === userStore.userId
})

// 是否显示加好友按钮
const showAddFriend = computed(() => {
  return showAddFriendParam.value || !isFriend.value
})

// 显示的用户信息
const displayUser = computed(() => {
  if (isOwnProfile.value) {
    return {
      avatar: userStore.avatar,
      nickname: userStore.nickname,
      username: userStore.username,
      bio: userStore.userInfo?.bio,
    }
  } else {
    return viewUserInfo.value || {}
  }
})

const showNicknameEdit = ref(false)
const showBioEdit = ref(false)
const tempNickname = ref('')
const tempBio = ref('')

onMounted(async () => {
  const pages = getCurrentPages()
  const currentPage = pages[pages.length - 1]
  const options = (currentPage as any).options || {}
  
  viewUserId.value = options.userId || ''
  showAddFriendParam.value = options.showAddFriend === 'true'

  // 如果是查看他人信息，加载用户信息
  if (!isOwnProfile.value) {
    await loadUserInfo()
    await checkFriendStatus()
  }
})

// 加载用户信息
const loadUserInfo = async () => {
  if (!viewUserId.value) return

  loading.value = true
  try {
    const user = await getPublicUserInfoApi(viewUserId.value, { custom: { showLoading: false } })
    if (user) {
      viewUserInfo.value = {
        id: user.id,
        username: user.username,
        nickname: user.nickname || user.username,
        avatar: user.avatar,
        bio: user.bio,
      }
    }
  } catch (error: any) {
    console.error('加载用户信息失败:', error)
    showToast(error?.message || '加载用户信息失败', 'none')
    setTimeout(() => {
      uni.navigateBack()
    }, 1000)
  } finally {
    loading.value = false
  }
}

// 检查好友状态
const checkFriendStatus = async () => {
  if (!userStore.isLoggedIn) {
    isFriend.value = false
    return
  }

  try {
    const friends = await getFriendsApi({ custom: { showLoading: false } })
    if (friends && Array.isArray(friends)) {
      isFriend.value = friends.some((friend: any) => 
        friend.user_id === viewUserId.value || friend.id === viewUserId.value
      )
    }
  } catch (error) {
    console.error('检查好友状态失败:', error)
    isFriend.value = false
  }
}

// 添加好友
const handleAddFriend = async () => {
  if (!userStore.isLoggedIn) {
    showToast('请先登录', 'none')
    setTimeout(() => {
      uni.navigateTo({ url: '/pages/auth/login' })
    }, 1000)
    return
  }

  uni.showModal({
    title: '添加好友',
    content: `确定要添加 ${displayUser.value.nickname || displayUser.value.username} 为好友吗？`,
    success: async (res) => {
      if (res.confirm) {
        try {
          await addFriendApi({
            friend_id: viewUserId.value,
            message: `我是 ${userStore.nickname || userStore.username}`
          })
          showToast('好友申请已发送', 'success')
          showAddFriendParam.value = false
          isFriend.value = true
        } catch (error: any) {
          showToast(error.message || '发送好友申请失败', 'none')
        }
      }
    }
  })
}

const chooseAvatar = () => {
  uni.chooseImage({
    count: 1,
    sizeType: ['original', 'compressed'],
    sourceType: ['album', 'camera'],
    success: async (res) => {
      try {
        const tempFilePath = res.tempFilePaths[0]
        console.log('选择头像:', tempFilePath)
        
        // 显示上传中提示
        uni.showLoading({ title: '上传中...' })
        
        // 先上传图片到服务器
        const uploadResult = await uploadImageApi(tempFilePath)
        console.log('头像上传成功:', uploadResult.url)
        
        // 调用后端 API 保存到数据库
        await updateUserInfoApi({ avatar: uploadResult.url })
        
        // 更新本地缓存
        userStore.updateUserInfo({ avatar: uploadResult.url })
        
        uni.hideLoading()
        showToast('头像已更新', 'success')
      } catch (error: any) {
        uni.hideLoading()
        console.error('更新头像失败:', error)
        showToast(error?.message || '更新失败', 'none')
      }
    },
    fail: (error) => {
      console.error('选择头像失败:', error)
      showToast('选择图片失败', 'none')
    }
  })
}

const editNickname = () => {
  tempNickname.value = displayUser.value.nickname || ''
  showNicknameEdit.value = true
}

const saveNickname = async () => {
  if (!tempNickname.value.trim()) {
    return showToast('昵称不能为空', 'none')
  }
  try {
    // 调用后端 API 保存到数据库
    await updateUserInfoApi({ nickname: tempNickname.value.trim() })
    // 更新本地缓存
    userStore.updateUserInfo({ nickname: tempNickname.value.trim() })
    showNicknameEdit.value = false
    showToast('昵称已更新', 'success')
  } catch (error: any) {
    console.error('更新昵称失败:', error)
    showToast(error?.message || '更新失败', 'none')
  }
}

const editBio = () => {
  tempBio.value = displayUser.value.bio || ''
  showBioEdit.value = true
}

const saveBio = async () => {
  try {
    // 调用后端 API 保存到数据库
    await updateUserInfoApi({ bio: tempBio.value.trim() })
    // 更新本地缓存
    userStore.updateUserInfo({ bio: tempBio.value.trim() })
    showBioEdit.value = false
    showToast('个性签名已更新', 'success')
  } catch (error: any) {
    console.error('更新个性签名失败:', error)
    showToast(error?.message || '更新失败', 'none')
  }
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.profile-page {
  min-height: 100vh;
  @include cyber-page-bg;
  position: relative;
}

.loading {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  @include neon-text(#00D9FF);
  font-size: 28rpx;
  
  text::before {
    content: '⏳ ';
    animation: rotate 2s linear infinite;
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.profile-content {
  padding: 40rpx 30rpx;
  position: relative;
  z-index: 1;
}

.profile-header {
  @include neon-card;
  @include rainbow-border;
  border-radius: 20rpx;
  padding: 40rpx 30rpx;
  margin-bottom: 30rpx;
  animation: fadeInUp 0.6s ease-out;

  .avatar-section {
    position: relative;
    width: 160rpx;
    height: 160rpx;
    margin: 0 auto 30rpx;

    .avatar-wrapper {
      width: 100%;
      height: 100%;

      .avatar {
        width: 100%;
        height: 100%;
        border-radius: 80rpx;
        border: 6rpx solid rgba(0, 217, 255, 0.6);
        box-shadow: 
          0 0 30rpx rgba(0, 217, 255, 0.6),
          inset 0 0 30rpx rgba(0, 217, 255, 0.1);
        animation: avatarGlow 2s ease-in-out infinite;
      }

      &.avatar-emoji {
        display: flex;
        align-items: center;
        justify-content: center;
        background: linear-gradient(135deg, rgba(0, 217, 255, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
        border-radius: 80rpx;
        font-size: 80rpx;
        border: 6rpx solid rgba(255, 214, 0, 0.6);
        box-shadow: 0 0 30rpx rgba(255, 214, 0, 0.5);
        animation: avatarGlow 2s ease-in-out infinite;
      }
    }

    .edit-avatar-btn {
      position: absolute;
      bottom: 0;
      right: 0;
      width: 50rpx;
      height: 50rpx;
      @include glow-button(#FF00D6);
      border-radius: 25rpx;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 28rpx;
      cursor: pointer;
      animation: btnPulse 3s ease-in-out infinite;

      &:active {
        transform: scale(0.9);
        animation: none;
      }
    }
  }

  .user-info {
    text-align: center;

    .nickname-row {
      display: flex;
      align-items: center;
      justify-content: center;
      margin-bottom: 10rpx;

      .nickname {
        font-size: 36rpx;
        font-weight: bold;
        @include neon-title(#ffffff);
        animation: titleGlow 3s ease-in-out infinite;
      }
    }

    .username {
      font-size: 24rpx;
      @include neon-text(#00D9FF);
      margin-bottom: 20rpx;
    }

    .bio {
      font-size: 26rpx;
      color: #b8c5d6;
      line-height: 1.6;
      padding: 20rpx;
      background: rgba(30, 36, 66, 0.4);
      border: 1rpx solid rgba(0, 217, 255, 0.2);
      border-radius: 10rpx;
      text-shadow: 0 0 8rpx rgba(184, 197, 214, 0.3);

      &.empty {
        color: #6b7b93;
        font-style: italic;
      }
    }
  }
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes titleGlow {
  0%, 100% {
    text-shadow: 
      0 0 15rpx rgba(255, 255, 255, 0.8),
      0 0 30rpx rgba(0, 217, 255, 0.5),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
  50% {
    text-shadow: 
      0 0 20rpx rgba(255, 255, 255, 1),
      0 0 40rpx rgba(0, 217, 255, 0.8),
      0 4rpx 8rpx rgba(0, 0, 0, 0.5);
  }
}

.action-section {
  margin-bottom: 30rpx;
  animation: fadeInUp 0.8s ease-out;

  .add-friend-btn {
    width: 100%;
    height: 90rpx;
    @include glow-button(#00D9FF);
    background: linear-gradient(135deg, rgba(0, 217, 255, 0.8) 0%, rgba(138, 92, 246, 0.8) 100%);
    border-radius: 45rpx;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 15rpx;
    font-size: 30rpx;
    font-weight: bold;
    border: none;
    animation: btnPulse2 3s ease-in-out infinite;

    &:active {
      transform: scale(0.95);
      animation: none;
    }

    &::after {
      border: none;
    }

    .btn-icon {
      font-size: 32rpx;
    }
  }
}

@keyframes btnPulse2 {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(0, 217, 255, 0.5),
      0 8rpx 24rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(0, 217, 255, 0.2);
  }
  50% {
    box-shadow: 
      0 0 30rpx rgba(0, 217, 255, 0.8),
      0 12rpx 32rpx rgba(0, 0, 0, 0.4),
      inset 0 0 30rpx rgba(0, 217, 255, 0.3);
  }
}

.friend-badge {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10rpx;
  padding: 20rpx;
  background: rgba(30, 36, 66, 0.6);
  backdrop-filter: blur(10rpx);
  border-radius: 45rpx;
  margin-bottom: 30rpx;
  @include neon-text(#00FF88);
  font-size: 28rpx;
  font-weight: bold;
  border: 2rpx solid rgba(0, 255, 136, 0.6);
  box-shadow: 0 0 20rpx rgba(0, 255, 136, 0.4);
  animation: fadeInUp 0.8s ease-out;

  .badge-icon {
    font-size: 32rpx;
  }
}

.edit-section {
  animation: fadeInUp 1s ease-out;
  
  .section-title {
    padding: 0 0 20rpx 0;

    text {
      font-size: 28rpx;
      @include neon-text(#FFD600);
      font-weight: bold;
      letter-spacing: 2rpx;
    }
  }

  .section {
    @include neon-card;
    border-radius: 20rpx;
    overflow: hidden;

    .item {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 25rpx 30rpx;
      border-bottom: 1rpx solid rgba(0, 217, 255, 0.1);
      transition: all 0.3s ease;

      &:last-child {
        border-bottom: none;
      }

      &:active {
        background: rgba(0, 217, 255, 0.05);
        transform: translateX(5rpx);
        
        .arrow {
          transform: translateX(5rpx);
        }
      }

      .label {
        font-size: 28rpx;
        @include neon-text(#ffffff);
      }

      .value {
        display: flex;
        align-items: center;
        flex: 1;
        justify-content: flex-end;

        .value-text {
          font-size: 28rpx;
          color: #b8c5d6;
          margin-right: 10rpx;
          max-width: 400rpx;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
          text-shadow: 0 0 5rpx rgba(184, 197, 214, 0.3);

          &.readonly {
            color: #6b7b93;
          }
        }

        .arrow {
          font-size: 40rpx;
          color: rgba(0, 217, 255, 0.6);
          text-shadow: 0 0 10rpx rgba(0, 217, 255, 0.5);
          transition: all 0.3s ease;
        }
      }
    }
  }
}

.modal-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(10rpx);
  z-index: 999;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: maskFadeIn 0.3s ease-out;
}

@keyframes maskFadeIn {
  from {
    opacity: 0;
    backdrop-filter: blur(0);
  }
  to {
    opacity: 1;
    backdrop-filter: blur(10rpx);
  }
}

.modal-content {
  width: 640rpx;
  @include neon-card;
  border-radius: 30rpx;
  overflow: hidden;
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(138, 92, 246, 0.5),
    inset 0 0 80rpx rgba(138, 92, 246, 0.1);
  border: 2rpx solid rgba(138, 92, 246, 0.6);
  animation: modalSlideUp 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

@keyframes modalSlideUp {
  from {
    opacity: 0;
    transform: translateY(100rpx) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.modal-header {
  padding: 50rpx 40rpx 30rpx;
  text-align: center;
  background: linear-gradient(135deg, rgba(138, 92, 246, 0.9) 0%, rgba(102, 126, 234, 0.9) 100%);
  border-bottom: 2rpx solid rgba(138, 92, 246, 0.6);
  box-shadow: 0 0 30rpx rgba(138, 92, 246, 0.3);
  position: relative;
  
  &::after {
    content: '';
    position: absolute;
    bottom: -2rpx;
    left: 50%;
    transform: translateX(-50%);
    width: 80%;
    height: 2rpx;
    background: linear-gradient(90deg, transparent, rgba(138, 92, 246, 0.8), transparent);
    box-shadow: 0 0 20rpx rgba(138, 92, 246, 0.6);
  }

  .modal-title {
    font-size: 36rpx;
    font-weight: bold;
    @include neon-text(#ffffff);
    animation: titlePulse 3s ease-in-out infinite;
  }
}

@keyframes titlePulse {
  0%, 100% {
    text-shadow: 
      0 0 10rpx rgba(255, 255, 255, 0.8),
      0 0 20rpx rgba(138, 92, 246, 0.5);
  }
  50% {
    text-shadow: 
      0 0 15rpx rgba(255, 255, 255, 1),
      0 0 30rpx rgba(138, 92, 246, 0.8);
  }
}

.modal-body {
  padding: 40rpx;

  .modal-input,
  .modal-textarea {
    @include neon-input;
    width: 100%;
    font-size: 30rpx;
    color: #ffffff;
    line-height: 1.6;
    box-sizing: border-box;

    &:focus {
      background: rgba(30, 36, 66, 0.8);
      border-color: rgba(138, 92, 246, 0.8);
      box-shadow: 
        0 0 30rpx rgba(138, 92, 246, 0.5),
        inset 0 0 30rpx rgba(138, 92, 246, 0.1);
    }

    &::placeholder {
      color: #6b7b93;
    }
  }

  .modal-input {
    height: 90rpx;
  }

  .modal-textarea {
    min-height: 220rpx;
    line-height: 1.8;
  }
}

.modal-footer {
  display: flex;
  gap: 20rpx;
  padding: 0 40rpx 40rpx;

  .modal-btn {
    flex: 1;
    height: 90rpx;
    line-height: 90rpx;
    text-align: center;
    font-size: 30rpx;
    font-weight: 500;
    border-radius: 45rpx;
    border: none;
    transition: all 0.3s ease;

    &:first-child {
      background: rgba(30, 36, 66, 0.6);
      backdrop-filter: blur(10rpx);
      @include neon-text(#ffffff);
      border: 2rpx solid rgba(0, 217, 255, 0.4);
      box-shadow: 0 0 20rpx rgba(0, 217, 255, 0.3);

      &:active {
        transform: scale(0.95);
        border-color: rgba(0, 217, 255, 0.8);
        box-shadow: 0 0 30rpx rgba(0, 217, 255, 0.6);
      }
    }

    &.primary {
      @include glow-button(#8B5CF6);
      background: linear-gradient(135deg, rgba(138, 92, 246, 0.8) 0%, rgba(102, 126, 234, 0.8) 100%);
      animation: btnPulse3 3s ease-in-out infinite;

      &:active {
        transform: scale(0.95);
        animation: none;
      }
    }

    &::after {
      border: none;
    }
  }
}

@keyframes btnPulse3 {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(138, 92, 246, 0.5),
      0 8rpx 24rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(138, 92, 246, 0.2);
  }
  50% {
    box-shadow: 
      0 0 30rpx rgba(138, 92, 246, 0.8),
      0 12rpx 32rpx rgba(0, 0, 0, 0.4),
      inset 0 0 30rpx rgba(138, 92, 246, 0.3);
  }
}

@keyframes avatarGlow {
  0%, 100% {
    box-shadow: 
      0 0 30rpx currentColor,
      inset 0 0 30rpx currentColor;
  }
  50% {
    box-shadow: 
      0 0 45rpx currentColor,
      inset 0 0 45rpx currentColor;
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 0 20rpx rgba(255, 0, 214, 0.5),
      0 8rpx 24rpx rgba(0, 0, 0, 0.3),
      inset 0 0 20rpx rgba(255, 0, 214, 0.2);
  }
  50% {
    box-shadow: 
      0 0 30rpx rgba(255, 0, 214, 0.8),
      0 12rpx 32rpx rgba(0, 0, 0, 0.4),
      inset 0 0 30rpx rgba(255, 0, 214, 0.3);
  }
}
</style>
