/**
 * 用户状态管理
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { UserInfo } from '@/types'
import { STORAGE_KEYS } from '@/config'
import { setStorage, getStorage, removeStorage } from '@/utils/storage'
import { loginApi, registerApi, logoutApi, getUserInfoApi } from '@/api/auth'
import { getWebSocketClient } from '@/utils/websocket'

export const useUserStore = defineStore('user', () => {
  // 状态
  const token = ref<string>(getStorage(STORAGE_KEYS.TOKEN) || '')
  const userInfo = ref<UserInfo | null>(getStorage(STORAGE_KEYS.USER_INFO))

  // 计算属性
  const isLoggedIn = computed(() => !!token.value)
  const userId = computed(() => userInfo.value?.id || '')
  const username = computed(() => userInfo.value?.username || '')
  const nickname = computed(() => userInfo.value?.nickname || '')
  const avatar = computed(() => userInfo.value?.avatar || '')

  /**
   * 设置 token
   */
  function setToken(newToken: string) {
    token.value = newToken
    setStorage(STORAGE_KEYS.TOKEN, newToken)
  }

  /**
   * 设置用户信息
   */
  function setUserInfo(info: UserInfo) {
    userInfo.value = info
    setStorage(STORAGE_KEYS.USER_INFO, info)
  }

  /**
   * 清除用户信息
   */
  function clearUserInfo() {
    token.value = ''
    userInfo.value = null
    removeStorage(STORAGE_KEYS.TOKEN)
    removeStorage(STORAGE_KEYS.REFRESH_TOKEN)
    removeStorage(STORAGE_KEYS.USER_INFO)
    
    // 断开 WebSocket 连接
    const wsClient = getWebSocketClient()
    wsClient.disconnect()
  }

  /**
   * 验证 token 是否有效
   */
  async function validateToken() {
    if (!token.value) {
      return false
    }

    try {
      // 静默调用获取用户信息接口验证 token（不显示加载提示）
      await getUserInfoApi({ custom: { showLoading: false } })
      return true
    } catch (error) {
      // token 无效，清除用户信息（静默处理，不显示提示）
      token.value = ''
      userInfo.value = null
      removeStorage(STORAGE_KEYS.TOKEN)
      removeStorage(STORAGE_KEYS.REFRESH_TOKEN)
      removeStorage(STORAGE_KEYS.USER_INFO)
      return false
    }
  }

  /**
   * 登录
   */
  async function login(username: string, password: string) {
    try {
      const response = await loginApi({ username, password })
      setToken(response.token)
      setStorage(STORAGE_KEYS.REFRESH_TOKEN, response.refresh_token)
      setUserInfo(response.user_info)
      
      // 更新 WebSocket token 并连接
      const wsClient = getWebSocketClient()
      wsClient.updateToken(response.token)
      if (!wsClient.isConnected()) {
        wsClient.connect()
      }
      
      return true
    } catch (error) {
      // console.error('login error:', error)
      return false
    }
  }

  /**
   * 注册
   * 注册成功后不自动登录，需要用户手动登录
   */
  async function register(username: string, password: string, email?: string) {
    try {
      await registerApi({ username, password, email })
      // 注册成功后不设置 token，让用户手动登录
      return true
    } catch (error) {
      // console.error('register error:', error)
      throw error
    }
  }

  /**
   * 退出登录
   */
  async function logout() {
    try {
      await logoutApi()
    } catch (error) {
      // console.error('logout error:', error)
    } finally {
      clearUserInfo()
      uni.reLaunch({
        url: '/pages/auth/login',
      })
    }
  }

  /**
   * 获取用户信息
   */
  async function fetchUserInfo() {
    try {
      const info = await getUserInfoApi()
      setUserInfo(info)
      return info
    } catch (error) {
      // console.error('fetchUserInfo error:', error)
      return null
    }
  }

  /**
   * 更新用户信息
   */
  function updateUserInfo(info: Partial<UserInfo>) {
    if (userInfo.value) {
      userInfo.value = { ...userInfo.value, ...info }
      setStorage(STORAGE_KEYS.USER_INFO, userInfo.value)
    }
  }

  return {
    // 状态
    token,
    userInfo,
    // 计算属性
    isLoggedIn,
    // 方法
    validateToken,
    userId,
    username,
    nickname,
    avatar,
    // 方法
    setToken,
    setUserInfo,
    clearUserInfo,
    login,
    register,
    logout,
    fetchUserInfo,
    updateUserInfo,
  }
})

