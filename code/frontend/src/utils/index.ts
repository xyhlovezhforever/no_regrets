/**
 * 工具函数入口
 */

export * from './request'
export * from './storage'

/**
 * 防抖函数
 */
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: number | null = null

  return function (...args: Parameters<T>) {
    if (timeout !== null) {
      clearTimeout(timeout)
    }
    timeout = setTimeout(() => {
      func(...args)
    }, wait) as unknown as number
  }
}

/**
 * 节流函数
 */
export function throttle<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: number | null = null
  let previous = 0

  return function (...args: Parameters<T>) {
    const now = Date.now()
    const remaining = wait - (now - previous)

    if (remaining <= 0 || remaining > wait) {
      if (timeout !== null) {
        clearTimeout(timeout)
        timeout = null
      }
      previous = now
      func(...args)
    } else if (!timeout) {
      timeout = setTimeout(() => {
        previous = Date.now()
        timeout = null
        func(...args)
      }, remaining) as unknown as number
    }
  }
}

/**
 * 格式化日期
 */
export function formatDate(date: Date | string | number, format = 'YYYY-MM-DD HH:mm:ss'): string {
  const d = new Date(date)
  const year = d.getFullYear()
  const month = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  const hours = String(d.getHours()).padStart(2, '0')
  const minutes = String(d.getMinutes()).padStart(2, '0')
  const seconds = String(d.getSeconds()).padStart(2, '0')

  return format
    .replace('YYYY', String(year))
    .replace('MM', month)
    .replace('DD', day)
    .replace('HH', hours)
    .replace('mm', minutes)
    .replace('ss', seconds)
}

/**
 * 深拷贝
 */
export function deepClone<T>(obj: T): T {
  if (obj === null || typeof obj !== 'object') {
    return obj
  }

  if (obj instanceof Date) {
    return new Date(obj.getTime()) as any
  }

  if (obj instanceof Array) {
    const clonedArr = [] as any[]
    obj.forEach((item, index) => {
      clonedArr[index] = deepClone(item)
    })
    return clonedArr as any
  }

  if (obj instanceof Object) {
    const clonedObj = {} as any
    Object.keys(obj).forEach(key => {
      clonedObj[key] = deepClone((obj as any)[key])
    })
    return clonedObj
  }

  return obj
}

/**
 * 生成唯一 ID
 */
export function generateId(): string {
  return `${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
}

/**
 * 分享内容给好友
 */
export function shareToFriend(content: string, title: string = '分享'): void {
  uni.showModal({
    title: title,
    content: content,
    confirmText: '复制',
    cancelText: '取消',
    success: (res) => {
      if (res.confirm) {
        uni.setClipboardData({
          data: content,
          success: () => {
            uni.showToast({ title: '已复制到剪贴板', icon: 'success' })
          }
        })
      }
    }
  })
}

/**
 * 页面跳转（自动判断 TabBar 页面）
 */
export function navigateTo(url: string, params?: Record<string, any>): void {
  let fullUrl = url
  if (params) {
    const queryString = Object.entries(params)
      .map(([key, value]) => `${key}=${encodeURIComponent(String(value))}`)
      .join('&')
    fullUrl = `${url}?${queryString}`
  }
  
  // TabBar 页面列表
  const tabBarPages = [
    '/pages/emotion/index',
    '/pages/leisure/index',
    '/pages/daily/index',
    '/pages/binding/index',
    '/pages/user/center',
  ]
  
  // 判断是否为 TabBar 页面
  const isTabBarPage = tabBarPages.some(page => fullUrl.includes(page))
  
  if (isTabBarPage) {
    // TabBar 页面使用 switchTab（不支持参数）
    uni.switchTab({ url: fullUrl.split('?')[0] })
  } else {
    // 普通页面使用 navigateTo
    uni.navigateTo({ url: fullUrl })
  }
}

/**
 * 页面重定向
 */
export function redirectTo(url: string, params?: Record<string, any>): void {
  let fullUrl = url
  if (params) {
    const queryString = Object.entries(params)
      .map(([key, value]) => `${key}=${encodeURIComponent(String(value))}`)
      .join('&')
    fullUrl = `${url}?${queryString}`
  }
  uni.redirectTo({ url: fullUrl })
}

/**
 * 返回上一页
 */
export function navigateBack(delta: number = 1): void {
  uni.navigateBack({ delta })
}

/**
 * 显示提示信息
 */
export function showToast(title: string, icon: 'success' | 'error' | 'none' = 'none'): void {
  uni.showToast({
    title,
    icon: icon === 'error' ? 'none' : icon,
    duration: 2000,
  })
}

/**
 * 显示加载中
 */
export function showLoading(title: string = '加载中...'): void {
  uni.showLoading({
    title,
    mask: true,
  })
}

/**
 * 隐藏加载中
 */
export function hideLoading(): void {
  uni.hideLoading()
}

/**
 * 显示模态对话框
 */
export function showModal(options: {
  title: string
  content: string
  showCancel?: boolean
  success?: (res: UniApp.ShowModalSuccess) => void
}): void
export function showModal(
  title: string,
  content: string
): Promise<UniApp.ShowModalSuccess>
export function showModal(
  titleOrOptions: string | any,
  content?: string
): Promise<UniApp.ShowModalSuccess> | void {
  if (typeof titleOrOptions === 'object') {
    // 新签名：传入对象
    uni.showModal(titleOrOptions)
  } else {
    // 旧签名：传入 title 和 content
    return new Promise(resolve => {
      uni.showModal({
        title: titleOrOptions,
        content: content!,
        success: res => {
          resolve(res)
        },
      })
    })
  }
}

