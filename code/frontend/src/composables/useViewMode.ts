/**
 * 查看模式 Composable
 * 用于判断当前是否处于查看他人数据的模式
 */

import { ref, computed } from 'vue'
import { useUserStore } from '@/store/user'

export interface ViewModeContext {
  // 是否处于查看模式（查看他人数据）
  isViewMode: boolean
  // 被查看用户的ID
  viewedUserId?: string
  // 被查看用户的信息
  viewedUserInfo?: any
  // 应用类型
  appType?: string
}

const viewModeContext = ref<ViewModeContext>({
  isViewMode: false,
})

export function useViewMode() {
  const userStore = useUserStore()

  /**
   * 设置查看模式
   */
  function setViewMode(context: ViewModeContext) {
    viewModeContext.value = context
  }

  /**
   * 清除查看模式
   */
  function clearViewMode() {
    viewModeContext.value = {
      isViewMode: false,
    }
  }

  /**
   * 是否处于查看模式
   */
  const isViewMode = computed(() => viewModeContext.value.isViewMode)

  /**
   * 被查看的用户ID
   */
  const viewedUserId = computed(() => viewModeContext.value.viewedUserId)

  /**
   * 被查看的用户信息
   */
  const viewedUserInfo = computed(() => viewModeContext.value.viewedUserInfo)

  /**
   * 应用类型
   */
  const appType = computed(() => viewModeContext.value.appType)

  /**
   * 是否是自己的数据
   */
  const isOwnData = computed(() => {
    if (!viewModeContext.value.isViewMode) {
      return true
    }
    return viewModeContext.value.viewedUserId === userStore.userId
  })

  /**
   * 是否可以编辑（只有查看自己的数据时才能编辑）
   */
  const canEdit = computed(() => isOwnData.value)

  /**
   * 是否可以删除（只有查看自己的数据时才能删除）
   */
  const canDelete = computed(() => isOwnData.value)

  /**
   * 是否可以创建（只有查看自己的数据时才能创建）
   */
  const canCreate = computed(() => isOwnData.value)

  return {
    // 状态
    viewModeContext,
    isViewMode,
    viewedUserId,
    viewedUserInfo,
    appType,
    isOwnData,
    canEdit,
    canDelete,
    canCreate,
    // 方法
    setViewMode,
    clearViewMode,
  }
}
