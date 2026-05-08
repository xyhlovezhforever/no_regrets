/**
 * 应用状态管理
 */

import { defineStore } from 'pinia'
import { ref } from 'vue'
import { STORAGE_KEYS } from '@/config'
import { setStorage, getStorage } from '@/utils/storage'

export const useAppStore = defineStore('app', () => {
  // 状态
  const theme = ref<'light' | 'dark'>(getStorage(STORAGE_KEYS.THEME) || 'light')
  const language = ref<string>(getStorage(STORAGE_KEYS.LANGUAGE) || 'zh-CN')
  const isFirstVisit = ref<boolean>(getStorage('isFirstVisit', true))
  const showWelcomeModal = ref<boolean>(true)

  /**
   * 设置主题
   */
  function setTheme(newTheme: 'light' | 'dark') {
    theme.value = newTheme
    setStorage(STORAGE_KEYS.THEME, newTheme)
  }

  /**
   * 切换主题
   */
  function toggleTheme() {
    const newTheme = theme.value === 'light' ? 'dark' : 'light'
    setTheme(newTheme)
  }

  /**
   * 设置语言
   */
  function setLanguage(newLanguage: string) {
    language.value = newLanguage
    setStorage(STORAGE_KEYS.LANGUAGE, newLanguage)
  }

  /**
   * 设置首次访问标记
   */
  function setFirstVisit(value: boolean) {
    isFirstVisit.value = value
    setStorage('isFirstVisit', value)
  }

  /**
   * 设置欢迎弹窗显示状态
   */
  function setShowWelcomeModal(value: boolean) {
    showWelcomeModal.value = value
  }

  return {
    // 状态
    theme,
    language,
    isFirstVisit,
    showWelcomeModal,
    // 方法
    setTheme,
    toggleTheme,
    setLanguage,
    setFirstVisit,
    setShowWelcomeModal,
  }
})

