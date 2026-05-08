/**
 * HTTP 请求封装
 * 基于 luch-request
 */

import Request from 'luch-request'
import { BASE_API, API_CONFIG, TOKEN_KEY, STORAGE_KEYS } from '@/config'
import { useUserStore } from '@/store'

// 创建请求实例
const http = new Request({
  baseURL: BASE_API,
  timeout: API_CONFIG.timeout,
  header: {
    'Content-Type': 'application/json',
  },
})

// 请求拦截器
http.interceptors.request.use(
  config => {
    // 添加 token
    const token = uni.getStorageSync(STORAGE_KEYS.TOKEN)
    if (token) {
      config.header = {
        ...config.header,
        [TOKEN_KEY]: `Bearer ${token}`,
      }
    }

    // 显示加载提示
    if (config.custom?.showLoading !== false) {
      uni.showLoading({
        title: '加载中...',
        mask: true,
      })
    }

    return config
  },
  error => {
    return Promise.reject(error)
  }
)

// 处理401未授权，跳转到登录页
const handleUnauthorized = () => {
  // 清除store中的用户信息
  try {
    const userStore = useUserStore()
    userStore.clearUserInfo()
  } catch (error) {
    // 如果store未初始化，直接清除本地存储
    // console.warn('Failed to clear user store:', error)
    uni.removeStorageSync(STORAGE_KEYS.TOKEN)
    uni.removeStorageSync(STORAGE_KEYS.REFRESH_TOKEN)
    uni.removeStorageSync(STORAGE_KEYS.USER_INFO)
  }
  
  // 显示提示
  uni.showToast({
    title: '登录已失效，请重新登录',
    icon: 'none',
    duration: 2000,
  })
  
  // 延迟跳转，确保提示显示
  setTimeout(() => {
    uni.reLaunch({
      url: '/pages/auth/login',
    })
  }, 500)
}

// 响应拦截器
http.interceptors.response.use(
  response => {
    // 隐藏加载提示
    uni.hideLoading()

    const { data, statusCode } = response

    // HTTP 401 状态码检查（未授权）
    if (statusCode === 401) {
      handleUnauthorized()
      return Promise.reject(new Error('登录已失效，请重新登录'))
    }

    // 其他HTTP错误
    if (statusCode !== 200) {
      uni.showToast({
        title: `请求失败: ${statusCode}`,
        icon: 'none',
        duration: 2000,
      })
      return Promise.reject(new Error(`HTTP Error: ${statusCode}`))
    }

    // 业务状态码检查
    if (data.code !== 0) {
      // token 过期或无效（业务401）
      if (data.code === 401) {
        handleUnauthorized()
        return Promise.reject(new Error('登录已失效，请重新登录'))
      }

      // 其他业务错误
      uni.showToast({
        title: data.message || '请求失败',
        icon: 'none',
        duration: 2000,
      })
      return Promise.reject(new Error(data.message || '请求失败'))
    }

    return data.data
  },
  error => {
    uni.hideLoading()

    // 检查是否是401错误
    if (error.statusCode === 401 || error.status === 401) {
      handleUnauthorized()
      return Promise.reject(new Error('登录已失效，请重新登录'))
    }

    let message = '网络错误'
    if (error.errMsg) {
      if (error.errMsg.includes('timeout')) {
        message = '请求超时'
      } else if (error.errMsg.includes('network')) {
        message = '网络连接失败'
      } else if (error.errMsg.includes('401')) {
        handleUnauthorized()
        return Promise.reject(new Error('登录已失效，请重新登录'))
      }
    }

    uni.showToast({
      title: message,
      icon: 'none',
      duration: 2000,
    })

    return Promise.reject(error)
  }
)

/**
 * GET 请求
 */
export function get<T = any>(url: string, params?: any, config?: any): Promise<T> {
  return http.get(url, { params, ...config })
}

/**
 * POST 请求
 */
export function post<T = any>(url: string, data?: any, config?: any): Promise<T> {
  return http.post(url, data, config)
}

/**
 * PUT 请求
 */
export function put<T = any>(url: string, data?: any, config?: any): Promise<T> {
  return http.put(url, data, config)
}

/**
 * DELETE 请求
 */
export function del<T = any>(url: string, params?: any, config?: any): Promise<T> {
  return http.delete(url, { params, ...config })
}

/**
 * 文件上传
 */
export function upload<T = any>(url: string, filePath: string, config?: any): Promise<T> {
  return new Promise((resolve, reject) => {
    const token = uni.getStorageSync(STORAGE_KEYS.TOKEN)

    uni.uploadFile({
      url: `${BASE_API}${url}`,
      filePath,
      name: 'file',
      timeout: 60000, // 60秒超时，适合大文件上传
      header: {
        [TOKEN_KEY]: `Bearer ${token}`,
      },
      ...config,
      success: res => {
        if (res.statusCode === 200) {
          const data = JSON.parse(res.data)
          if (data.code === 0) {
            resolve(data.data)
          } else {
            reject(new Error(data.message))
          }
        } else {
          reject(new Error(`上传失败: ${res.statusCode}`))
        }
      },
      fail: error => {
        reject(error)
      },
    })
  })
}

export default http

