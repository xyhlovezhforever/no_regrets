/**
 * WebSocket 客户端
 * 生产级别的 WebSocket 实现，包含自动重连、心跳、错误处理等
 */

import { BASE_API } from '@/config'
import { STORAGE_KEYS } from '@/config'
import { useUserStore } from '@/store'

export type WsMessageType = 
  | 'ping' 
  | 'pong' 
  | 'auth' 
  | 'auth_success' 
  | 'auth_error' 
  | 'send_message' 
  | 'message' 
  | 'error' 
  | 'system'

export interface WsMessage {
  type: WsMessageType
  [key: string]: any
}

type MessageHandler = (message: WsMessage) => void
type ErrorHandler = (error: Error) => void
type ConnectHandler = () => void
type DisconnectHandler = () => void

export class WebSocketClient {
  private ws: WebSocket | null = null
  private url: string
  private token: string
  private reconnectAttempts = 0
  private maxReconnectAttempts = 10
  private reconnectDelay = 1000 // 初始重连延迟 1秒
  private maxReconnectDelay = 30000 // 最大重连延迟 30秒
  private heartbeatInterval: number | null = null
  private heartbeatDelay = 30000 // 心跳间隔 30秒
  private isManualClose = false
  private isAuthenticated = false
  
  private messageHandlers: MessageHandler[] = []
  private errorHandlers: ErrorHandler[] = []
  private connectHandlers: ConnectHandler[] = []
  private disconnectHandlers: DisconnectHandler[] = []
  
  constructor() {
    // 检测是否为微信小程序环境
    // 优先检查是否在浏览器环境（H5），如果是浏览器环境，强制使用浏览器WebSocket
    // @ts-ignore
    const isBrowser = typeof window !== 'undefined' && typeof WebSocket !== 'undefined'
    
    if (isBrowser) {
      // 浏览器环境，使用原生WebSocket
      this.isWeChat = false
      // console.log('[WebSocket] 检测到浏览器环境，使用原生WebSocket')
    } else {
      // 非浏览器环境，检查是否是微信小程序
      // @ts-ignore
      const hasWx = typeof wx !== 'undefined'
      // @ts-ignore
      const hasConnectSocket = hasWx && typeof wx.connectSocket === 'function'
      this.isWeChat = hasConnectSocket
      // console.log('[WebSocket] 非浏览器环境，检测微信小程序:', {
      //   hasWx,
      //   hasConnectSocket,
      //   isWeChat: this.isWeChat
      // })
    }
    
    // 构建 WebSocket URL
    // 处理不同的URL格式
    let baseUrl = BASE_API.replace('/api/v1', '')
    if (baseUrl.startsWith('http://')) {
      baseUrl = baseUrl.replace('http://', 'ws://')
    } else if (baseUrl.startsWith('https://')) {
      baseUrl = baseUrl.replace('https://', 'wss://')
    } else if (!baseUrl.startsWith('ws://') && !baseUrl.startsWith('wss://')) {
      // 如果没有协议，默认使用 ws://
      baseUrl = `ws://${baseUrl}`
    }
    this.url = `${baseUrl}/api/v1/ws`
    
    // 获取 token
    const userStore = useUserStore()
    this.token = userStore.token || uni.getStorageSync(STORAGE_KEYS.TOKEN) || ''
  }
  
  /**
   * 更新 token（登录后调用）
   */
  updateToken(token: string): void {
    this.token = token
    // 如果已连接，重新认证
    if (this.isConnected()) {
      this.authenticate()
    }
  }
  
  /**
   * 连接 WebSocket
   */
  connect(): void {
    if (this.isWeChat) {
      this.connectWeChat()
    } else {
      this.connectBrowser()
    }
  }
  
  /**
   * 浏览器环境连接
   */
  private connectBrowser(): void {
    if (this.ws && (this.ws as WebSocket).readyState === WebSocket.OPEN) {
      // console.log('[WebSocket] 已经连接')
      return
    }
    
    this.isManualClose = false
    
    try {
      // console.log('[WebSocket] 正在连接:', this.url)
      this.ws = new WebSocket(this.url)
      
      ;(this.ws as WebSocket).onopen = () => {
        // console.log('[WebSocket] 连接已建立（H5）')
        this.reconnectAttempts = 0
        this.reconnectDelay = 1000
        
        // 先触发连接事件（让页面可以注册消息处理器）
        this.connectHandlers.forEach(handler => handler())
        
        // 发送认证消息
        this.authenticate()
        
        // 启动心跳
        this.startHeartbeat()
      }
      
      ;(this.ws as WebSocket).onmessage = (event) => {
        try {
          // console.log('[WebSocket] H5收到原始消息:', event.data)
          const message: WsMessage = JSON.parse(event.data)
          // console.log('[WebSocket] H5解析后的消息:', message)
          // console.log('[WebSocket] 当前注册的消息处理器数量:', this.messageHandlers.length)
          this.handleMessage(message)
        } catch (error) {
          // console.error('[WebSocket] 解析消息失败:', error, event.data)
        }
      }
      
      ;(this.ws as WebSocket).onerror = (error) => {
        // console.error('[WebSocket] 连接错误:', error)
        this.errorHandlers.forEach(handler => handler(new Error('WebSocket连接错误')))
      }
      
      ;(this.ws as WebSocket).onclose = (event) => {
        // console.log('[WebSocket] 连接已关闭:', event.code, event.reason)
        this.isAuthenticated = false
        this.stopHeartbeat()
        
        // 触发断开连接事件
        this.disconnectHandlers.forEach(handler => handler())
        
        // 如果不是手动关闭，尝试重连
        if (!this.isManualClose) {
          this.reconnect()
        }
      }
    } catch (error) {
      // console.error('[WebSocket] 连接失败:', error)
      this.errorHandlers.forEach(handler => handler(error as Error))
      this.reconnect()
    }
  }
  
  /**
   * 微信小程序环境连接
   */
  private connectWeChat(): void {
    // @ts-ignore
    if (this.ws && typeof this.ws === 'object' && 'readyState' in this.ws && (this.ws as any).readyState === 1) {
      // console.log('[WebSocket] 已经连接（微信小程序）')
      return
    }
    
    this.isManualClose = false
    
    try {
      // console.log('[WebSocket] 正在连接（微信小程序）:', this.url)
      // @ts-ignore
      if (typeof wx === 'undefined' || typeof wx.connectSocket !== 'function') {
        // console.error('[WebSocket] 微信小程序API不可用，回退到浏览器模式')
        this.isWeChat = false
        this.connectBrowser()
        return
      }
      
      // @ts-ignore
      const socketTask = wx.connectSocket({
        url: this.url,
        protocols: []
      })
      
      this.ws = socketTask as any
      
      // 检查是否有 onOpen 方法
      if (typeof socketTask.onOpen !== 'function') {
        // console.error('[WebSocket] SocketTask.onOpen 不是函数，回退到浏览器模式')
        this.isWeChat = false
        this.connectBrowser()
        return
      }
      
      socketTask.onOpen(() => {
        // console.log('[WebSocket] 连接已建立（微信小程序）')
        this.reconnectAttempts = 0
        this.reconnectDelay = 1000
        
        // 先触发连接事件（让页面可以注册消息处理器）
        this.connectHandlers.forEach(handler => handler())
        
        // 发送认证消息
        this.authenticate()
        
        // 启动心跳
        this.startHeartbeat()
      })
      
      socketTask.onMessage((res: any) => {
        try {
          // console.log('[WebSocket] 微信小程序收到原始消息:', res.data)
          const message: WsMessage = JSON.parse(res.data)
          // console.log('[WebSocket] 微信小程序解析后的消息:', message)
          // console.log('[WebSocket] 当前注册的消息处理器数量:', this.messageHandlers.length)
          this.handleMessage(message)
        } catch (error) {
          // console.error('[WebSocket] 解析消息失败:', error, res.data)
        }
      })
      
      socketTask.onError((error: any) => {
        // console.error('[WebSocket] 连接错误（微信小程序）:', error)
        this.errorHandlers.forEach(handler => handler(new Error('WebSocket连接错误')))
      })
      
      socketTask.onClose((res: any) => {
        // console.log('[WebSocket] 连接已关闭（微信小程序）:', res.code, res.reason)
        this.isAuthenticated = false
        this.stopHeartbeat()
        
        // 触发断开连接事件
        this.disconnectHandlers.forEach(handler => handler())
        
        // 如果不是手动关闭，尝试重连
        if (!this.isManualClose) {
          this.reconnect()
        }
      })
    } catch (error) {
      // console.error('[WebSocket] 连接失败（微信小程序）:', error)
      // 如果微信小程序API失败，回退到浏览器模式
      if (error instanceof Error && error.message.includes('onOpen')) {
        // console.log('[WebSocket] 回退到浏览器模式')
        this.isWeChat = false
        this.connectBrowser()
      } else {
        this.errorHandlers.forEach(handler => handler(error as Error))
        this.reconnect()
      }
    }
  }
  
  /**
   * 断开连接
   */
  disconnect(): void {
    this.isManualClose = true
    this.stopHeartbeat()
    
    if (this.ws) {
      try {
        if (this.isWeChat) {
          // @ts-ignore
          const socketTask = this.ws as any
          if (typeof socketTask.close === 'function') {
            socketTask.close()
          }
        } else {
          ;(this.ws as WebSocket).close()
        }
      } catch (error) {
        // console.error('[WebSocket] 断开连接时出错:', error)
      } finally {
        this.ws = null
      }
    }
  }
  
  /**
   * 发送消息
   */
  send(message: WsMessage): boolean {
    if (!this.ws) {
      // console.warn('[WebSocket] 连接未建立，无法发送消息')
      return false
    }
    
    // 检查连接状态
    let isOpen = false
    if (this.isWeChat) {
      // @ts-ignore
      const socketTask = this.ws as any
      isOpen = socketTask && typeof socketTask.readyState !== 'undefined' && socketTask.readyState === 1
    } else {
      isOpen = this.ws instanceof WebSocket && (this.ws as WebSocket).readyState === WebSocket.OPEN
    }
    
    if (!isOpen) {
      // console.warn('[WebSocket] 连接未建立，无法发送消息')
      return false
    }
    
    if (!this.isAuthenticated && message.type !== 'auth') {
      // console.warn('[WebSocket] 未认证，无法发送消息')
      return false
    }
    
    try {
      const messageStr = JSON.stringify(message)
      if (this.isWeChat) {
        // @ts-ignore
        const socketTask = this.ws as any
        if (typeof socketTask.send === 'function') {
          socketTask.send({
            data: messageStr
          })
        } else {
          // console.error('[WebSocket] SocketTask.send 不是函数')
          return false
        }
      } else {
        ;(this.ws as WebSocket).send(messageStr)
      }
      return true
    } catch (error) {
      // console.error('[WebSocket] 发送消息失败:', error)
      this.errorHandlers.forEach(handler => handler(error as Error))
      return false
    }
  }
  
  /**
   * 发送文本消息
   */
  sendText(text: string): boolean {
    if (!this.ws) {
      return false
    }
    
    // 检查连接状态
    let isOpen = false
    if (this.isWeChat) {
      // @ts-ignore
      const socketTask = this.ws as any
      isOpen = socketTask && typeof socketTask.readyState !== 'undefined' && socketTask.readyState === 1
    } else {
      isOpen = this.ws instanceof WebSocket && (this.ws as WebSocket).readyState === WebSocket.OPEN
    }
    
    if (!isOpen) {
      return false
    }
    
    try {
      if (this.isWeChat) {
        // @ts-ignore
        const socketTask = this.ws as any
        if (typeof socketTask.send === 'function') {
          socketTask.send({
            data: text
          })
        } else {
          return false
        }
      } else {
        ;(this.ws as WebSocket).send(text)
      }
      return true
    } catch (error) {
      // console.error('[WebSocket] 发送文本失败:', error)
      return false
    }
  }
  
  /**
   * 认证
   */
  private authenticate(): void {
    if (!this.token) {
      // console.error('[WebSocket] ❌ Token 不存在，无法认证')
      return
    }
    
    // console.log('[WebSocket] 🔐 开始发送认证消息，token长度:', this.token.length)
    const authMessage = {
      type: 'auth',
      token: `Bearer ${this.token}`,
    }
    // console.log('[WebSocket] 认证消息:', { ...authMessage, token: 'Bearer ***' })
    
    const success = this.send(authMessage)
    if (success) {
      // console.log('[WebSocket] ✅ 认证消息发送成功')
    } else {
      // console.error('[WebSocket] ❌ 认证消息发送失败')
    }
  }
  
  /**
   * 处理接收到的消息
   */
  private handleMessage(message: WsMessage): void {
    // console.log('[WebSocket] handleMessage 被调用，消息类型:', message.type, '处理器数量:', this.messageHandlers.length)
    
    switch (message.type) {
      case 'pong':
        // 心跳响应，无需处理
        // console.log('[WebSocket] 收到心跳响应')
        break
      
      case 'auth_success':
        // console.log('[WebSocket] ✅ 认证成功')
        this.isAuthenticated = true
        // 认证成功后，触发连接事件（确保所有处理器都已注册）
        // console.log('[WebSocket] 触发连接事件，处理器数量:', this.connectHandlers.length)
        this.connectHandlers.forEach(handler => handler())
        break
      
      case 'auth_error':
        // console.error('[WebSocket] ❌ 认证失败:', message.message)
        this.errorHandlers.forEach(handler => handler(new Error(message.message || '认证失败')))
        // 认证失败，断开连接
        this.disconnect()
        break
      
      case 'message':
        // 收到新消息
        // console.log('[WebSocket] 📨 收到消息类型消息:', message)
        // 确保消息处理器被调用
        if (this.messageHandlers.length === 0) {
          // console.warn('[WebSocket] ⚠️ 收到消息但没有注册的消息处理器！')
        } else {
          // console.log('[WebSocket] ✅ 准备调用', this.messageHandlers.length, '个消息处理器')
        }
        break
      
      case 'error':
        // console.error('[WebSocket] ❌ 服务器错误:', message.message)
        this.errorHandlers.forEach(handler => handler(new Error(message.message || '服务器错误')))
        break
      
      default:
        // console.log('[WebSocket] ℹ️ 未知消息类型:', message.type)
        break
    }
    
    // 触发消息处理器（无论什么类型的消息都触发，让页面自己判断）
    // console.log('[WebSocket] 开始调用消息处理器...')
    this.messageHandlers.forEach((handler, index) => {
      // console.log(`[WebSocket] 调用处理器 ${index + 1}/${this.messageHandlers.length}`)
      try {
        handler(message)
      } catch (error) {
        // console.error(`[WebSocket] 处理器 ${index + 1} 执行出错:`, error)
      }
    })
    // console.log('[WebSocket] 所有消息处理器调用完成')
  }
  
  /**
   * 重连
   */
  private reconnect(): void {
    if (this.isManualClose) {
      return
    }
    
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      // console.error('[WebSocket] 达到最大重连次数，停止重连')
      return
    }
    
    this.reconnectAttempts++
    const delay = Math.min(this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1), this.maxReconnectDelay)
    
    // console.log(`[WebSocket] ${delay}ms 后尝试重连 (${this.reconnectAttempts}/${this.maxReconnectAttempts})`)
    
    setTimeout(() => {
      this.connect()
    }, delay)
  }
  
  /**
   * 启动心跳
   */
  private startHeartbeat(): void {
    this.stopHeartbeat()
    
    this.heartbeatInterval = setInterval(() => {
      if (!this.ws) return
      
      let isOpen = false
      if (this.isWeChat) {
        // @ts-ignore
        isOpen = (this.ws as WechatMiniprogram.SocketTask).readyState === 1
      } else {
        isOpen = (this.ws as WebSocket).readyState === WebSocket.OPEN
      }
      
      if (isOpen) {
        this.send({ type: 'ping' })
      }
    }, this.heartbeatDelay) as unknown as number
  }
  
  /**
   * 停止心跳
   */
  private stopHeartbeat(): void {
    if (this.heartbeatInterval !== null) {
      clearInterval(this.heartbeatInterval)
      this.heartbeatInterval = null
    }
  }
  
  /**
   * 添加消息处理器
   */
  onMessage(handler: MessageHandler): void {
    // 检查是否已存在相同的处理器（避免重复注册）
    const exists = this.messageHandlers.includes(handler)
    if (!exists) {
      this.messageHandlers.push(handler)
      // console.log('[WebSocket] ✅ 消息处理器已注册，当前总数:', this.messageHandlers.length)
    } else {
      // console.log('[WebSocket] ⚠️ 消息处理器已存在，跳过重复注册')
    }
  }
  
  /**
   * 移除消息处理器
   */
  offMessage(handler: MessageHandler): void {
    const index = this.messageHandlers.indexOf(handler)
    if (index > -1) {
      this.messageHandlers.splice(index, 1)
    }
  }
  
  /**
   * 添加错误处理器
   */
  onError(handler: ErrorHandler): void {
    this.errorHandlers.push(handler)
  }
  
  /**
   * 移除错误处理器
   */
  offError(handler: ErrorHandler): void {
    const index = this.errorHandlers.indexOf(handler)
    if (index > -1) {
      this.errorHandlers.splice(index, 1)
    }
  }
  
  /**
   * 添加连接处理器
   */
  onConnect(handler: ConnectHandler): void {
    this.connectHandlers.push(handler)
  }
  
  /**
   * 移除连接处理器
   */
  offConnect(handler: ConnectHandler): void {
    const index = this.connectHandlers.indexOf(handler)
    if (index > -1) {
      this.connectHandlers.splice(index, 1)
    }
  }
  
  /**
   * 添加断开连接处理器
   */
  onDisconnect(handler: DisconnectHandler): void {
    this.disconnectHandlers.push(handler)
  }
  
  /**
   * 移除断开连接处理器
   */
  offDisconnect(handler: DisconnectHandler): void {
    const index = this.disconnectHandlers.indexOf(handler)
    if (index > -1) {
      this.disconnectHandlers.splice(index, 1)
    }
  }
  
  /**
   * 获取连接状态
   */
  getState(): number {
    if (!this.ws) {
      return this.isWeChat ? 3 : WebSocket.CLOSED
    }
    
    if (this.isWeChat) {
      // @ts-ignore
      const socketTask = this.ws as any
      return socketTask && typeof socketTask.readyState !== 'undefined' ? socketTask.readyState : 3
    } else {
      return (this.ws as WebSocket).readyState
    }
  }
  
  /**
   * 是否已连接
   */
  isConnected(): boolean {
    if (!this.ws || !this.isAuthenticated) {
      return false
    }
    
    if (this.isWeChat) {
      // @ts-ignore
      const socketTask = this.ws as any
      return socketTask && typeof socketTask.readyState !== 'undefined' && socketTask.readyState === 1
    } else {
      return this.ws instanceof WebSocket && (this.ws as WebSocket).readyState === WebSocket.OPEN
    }
  }
}

// 单例实例
let wsClientInstance: WebSocketClient | null = null

/**
 * 获取 WebSocket 客户端单例
 */
export function getWebSocketClient(): WebSocketClient {
  if (!wsClientInstance) {
    wsClientInstance = new WebSocketClient()
  }
  return wsClientInstance
}

