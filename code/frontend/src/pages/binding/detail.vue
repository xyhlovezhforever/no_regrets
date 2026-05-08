<template>
  <view class="detail-page">
    <!-- 用户信息 -->
    <view v-if="detail" class="user-section">
      <image
        :src="detail.binding.bound_user_info?.avatar || '/static/default-avatar.png'"
        class="avatar"
      />
      <view class="user-info">
        <view class="name">{{ detail.binding.bound_user_info?.nickname || '未知用户' }}</view>
        <view class="bio">{{ detail.binding.bound_user_info?.bio || '暂无简介' }}</view>
      </view>
      <button class="unbind-btn" @click="confirmUnbind">解除绑定</button>
    </view>

    <!-- 权限列表 -->
    <view v-if="detail" class="permissions-section">
      <view class="section-title">应用访问权限</view>
      <view class="permissions-list">
        <view
          v-for="perm in permissionList"
          :key="perm.app_type"
          class="permission-item"
        >
          <view class="app-info">
            <view class="app-icon">{{ perm.icon }}</view>
            <view class="app-details">
              <view class="app-name">{{ perm.name }}</view>
              <view class="app-desc">{{ perm.description }}</view>
            </view>
          </view>
          <view class="permission-control">
            <view
              v-for="level in permissionLevels"
              :key="level.value"
              :class="['level-option', { active: perm.permission_level === level.value }]"
              @click="updatePermission(perm.app_type, level.value)"
            >
              {{ level.label }}
            </view>
          </view>
        </view>
      </view>
    </view>

    <!-- 说明 -->
    <view class="tips-section">
      <view class="tips-title">权限说明</view>
      <view class="tips-item">
        <text class="tips-label">无权限：</text>
        <text class="tips-text">对方无法访问您的该应用数据</text>
      </view>
      <view class="tips-item">
        <text class="tips-label">只读：</text>
        <text class="tips-text">对方可以查看但无法修改您的数据</text>
      </view>
      <view class="tips-item">
        <text class="tips-label">读写：</text>
        <text class="tips-text">对方可以查看和修改您的数据</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { onLoad } from '@dcloudio/uni-app'
import { 
  getBoundUserDetailApi, 
  updateAppPermissionApi,
  cancelBindingApi 
} from '@/api/binding'
import type { BoundUserDetail } from '@/types'
import { showToast, showLoading, hideLoading, showModal } from '@/utils'

const detail = ref<BoundUserDetail | null>(null)
const userId = ref('')

const appConfig = [
  { app_type: 'note', name: '便签', icon: '📝', description: '查看和编辑便签内容' },
  { app_type: 'account', name: '记账', icon: '💰', description: '查看和管理账单记录' },
  { app_type: 'love_letter', name: '情书', icon: '💌', description: '查看和编写情书' },
  { app_type: 'writing', name: '创作', icon: '✍️', description: '查看和发布创作内容' },
  { app_type: 'forum', name: '论坛', icon: '💬', description: '查看和发布论坛内容' },
  { app_type: 'private_space', name: '私人空间', icon: '🏠', description: '访问私人空间设置' },
  { app_type: 'idol', name: '偶像', icon: '⭐', description: '查看和管理偶像信息' },
  { app_type: 'chat', name: 'AI聊天', icon: '🤖', description: '查看AI聊天记录' },
]

const permissionLevels = [
  { label: '无', value: 'none' },
  { label: '只读', value: 'read' },
  { label: '读写', value: 'write' },
]

const permissionList = computed(() => {
  if (!detail.value) return []
  
  return appConfig.map(app => {
    const perm = detail.value!.permissions.find(p => p.app_type === app.app_type)
    return {
      ...app,
      id: perm?.id || '',
      permission_level: perm?.permission_level || 'none',
    }
  })
})

onLoad((options) => {
  if (options?.userId) {
    userId.value = options.userId
  }
})

onMounted(() => {
  if (userId.value) {
    loadDetail()
  }
})

const loadDetail = async () => {
  try {
    showLoading('加载中...')
    const res = await getBoundUserDetailApi(userId.value)
    // 响应拦截器已经解包了数据，直接使用 res
    detail.value = res
  } catch (error: any) {
    showToast(error.message || '加载失败')
    setTimeout(() => {
      goBack()
    }, 1500)
  } finally {
    hideLoading()
  }
}

const updatePermission = async (appType: string, level: string) => {
  try {
    showLoading('更新中...')
    await updateAppPermissionApi(userId.value, appType, {
      app_type: appType,
      permission_level: level as 'none' | 'read' | 'write',
    })
    
    // 更新本地数据
    if (detail.value) {
      const perm = detail.value.permissions.find(p => p.app_type === appType)
      if (perm) {
        perm.permission_level = level
      } else {
        detail.value.permissions.push({
          id: '',
          user_id: '',
          bound_user_id: userId.value,
          app_type: appType as any,
          permission_level: level as any,
          created_at: new Date().toISOString(),
          updated_at: new Date().toISOString(),
        })
      }
    }
    
    showToast('权限已更新')
  } catch (error: any) {
    showToast(error.message || '更新失败')
  } finally {
    hideLoading()
  }
}

const confirmUnbind = () => {
  showModal({
    title: '解除绑定',
    content: '解除绑定后，对方将无法访问您的任何数据，您也无法访问对方的数据。确定要解除绑定吗？',
    success: (res) => {
      if (res.confirm) {
        unbind()
      }
    },
  })
}

const unbind = async () => {
  if (!detail.value) return

  try {
    showLoading('解除中...')
    await cancelBindingApi(detail.value.binding.id)
    showToast('已解除绑定')
    setTimeout(() => {
      goBack()
    }, 1500)
  } catch (error: any) {
    showToast(error.message || '解除失败')
  } finally {
    hideLoading()
  }
}

const goBack = () => {
  uni.navigateBack()
}
</script>

<style scoped lang="scss">
.detail-page {
  min-height: 100vh;
  background-color: #f5f5f5;
  padding-top: 20rpx;
}

.user-section {
  display: flex;
  align-items: center;
  background-color: #fff;
  padding: 40rpx 30rpx;
  margin-bottom: 20rpx;

  .avatar {
    width: 120rpx;
    height: 120rpx;
    border-radius: 50%;
    margin-right: 30rpx;
  }

  .user-info {
    flex: 1;

    .name {
      font-size: 32rpx;
      font-weight: bold;
      color: #333;
      margin-bottom: 10rpx;
    }

    .bio {
      font-size: 26rpx;
      color: #666;
    }
  }

  .unbind-btn {
    padding: 15rpx 30rpx;
    background-color: #ff3b30;
    color: #fff;
    border-radius: 8rpx;
    font-size: 26rpx;
  }
}

.permissions-section {
  background-color: #fff;
  padding: 30rpx;
  margin-bottom: 20rpx;

  .section-title {
    font-size: 32rpx;
    font-weight: bold;
    color: #333;
    margin-bottom: 30rpx;
  }

  .permissions-list {
    .permission-item {
      padding: 30rpx 0;
      border-bottom: 1px solid #f0f0f0;

      &:last-child {
        border-bottom: none;
      }

      .app-info {
        display: flex;
        align-items: center;
        margin-bottom: 20rpx;

        .app-icon {
          font-size: 48rpx;
          margin-right: 20rpx;
        }

        .app-details {
          flex: 1;

          .app-name {
            font-size: 30rpx;
            font-weight: bold;
            color: #333;
            margin-bottom: 5rpx;
          }

          .app-desc {
            font-size: 24rpx;
            color: #999;
          }
        }
      }

      .permission-control {
        display: flex;
        gap: 20rpx;

        .level-option {
          flex: 1;
          padding: 15rpx 0;
          text-align: center;
          background-color: #f5f5f5;
          border-radius: 8rpx;
          font-size: 26rpx;
          color: #666;
          transition: all 0.3s;

          &.active {
            background-color: #007aff;
            color: #fff;
          }
        }
      }
    }
  }
}

.tips-section {
  background-color: #fff;
  padding: 30rpx;

  .tips-title {
    font-size: 28rpx;
    font-weight: bold;
    color: #333;
    margin-bottom: 20rpx;
  }

  .tips-item {
    padding: 15rpx 0;
    font-size: 26rpx;

    .tips-label {
      color: #666;
      font-weight: bold;
    }

    .tips-text {
      color: #999;
    }
  }
}
</style>
