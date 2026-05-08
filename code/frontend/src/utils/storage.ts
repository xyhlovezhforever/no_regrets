/**
 * 本地存储工具
 * 统一管理 uni.storage API
 */

/**
 * 设置存储
 */
export function setStorage(key: string, value: any): void {
  try {
    const data = typeof value === 'string' ? value : JSON.stringify(value)
    uni.setStorageSync(key, data)
  } catch (error) {
    // console.error('setStorage error:', error)
  }
}

/**
 * 获取存储
 */
export function getStorage<T = any>(key: string, defaultValue?: T): T | null {
  try {
    const value = uni.getStorageSync(key)
    if (!value) return defaultValue || null

    try {
      return JSON.parse(value)
    } catch {
      return value as T
    }
  } catch (error) {
    // console.error('getStorage error:', error)
    return defaultValue || null
  }
}

/**
 * 移除存储
 */
export function removeStorage(key: string): void {
  try {
    uni.removeStorageSync(key)
  } catch (error) {
    // console.error('removeStorage error:', error)
  }
}

/**
 * 清空存储
 */
export function clearStorage(): void {
  try {
    uni.clearStorageSync()
  } catch (error) {
    // console.error('clearStorage error:', error)
  }
}

/**
 * 获取存储信息
 */
export function getStorageInfo(): UniApp.GetStorageInfoSuccess {
  try {
    return uni.getStorageInfoSync()
  } catch (error) {
    console.error('getStorageInfo error:', error)
    return {
      keys: [],
      currentSize: 0,
      limitSize: 0,
    }
  }
}

export default {
  setStorage,
  getStorage,
  removeStorage,
  clearStorage,
  getStorageInfo,
}

