/**
 * 全局类型定义
 */

/**
 * API 响应基础结构
 */
export interface ApiResponse<T = any> {
  code: number
  message: string
  data: T
}

/**
 * 分页参数
 */
export interface PageParams {
  page: number
  pageSize: number
}

/**
 * 分页响应
 */
export interface PageResponse<T = any> {
  list: T[]
  total: number
  page: number
  pageSize: number
  totalPages: number
}

/**
 * 用户信息
 * 注意：字段名与后端返回的 snake_case 格式保持一致
 */
export interface UserInfo {
  id: string
  username: string
  nickname: string
  avatar?: string | null
  email?: string | null
  phone?: string | null
  gender?: number | null
  birthday?: string | null
  bio?: string | null
  created_at: string
  updated_at: string
}

/**
 * 登录请求
 */
export interface LoginRequest {
  username: string
  password: string
}

/**
 * 登录响应
 * 注意：字段名与后端返回的 snake_case 格式保持一致
 */
export interface LoginResponse {
  token: string
  refresh_token: string
  user_info: UserInfo
}

/**
 * 注册请求
 */
export interface RegisterRequest {
  username: string
  password: string
  email?: string
  phone?: string
}

/**
 * AI 聊天消息
 */
export interface ChatMessage {
  id: string
  role: 'user' | 'assistant' | 'system'
  content: string
  timestamp: number
  avatar?: string
}

/**
 * AI 聊天会话
 */
export interface ChatSession {
  id: string
  title: string
  messages: ChatMessage[]
  createdAt: string
  updatedAt: string
}

/**
 * 情书数据
 */
export interface LoveLetter {
  id: string
  title: string
  content: string
  to_user_id: string  // 收信人ID
  to_name: string     // 收信人名字
  from_user_id: string // 写信人ID
  from_name: string    // 写信人名字
  is_read: boolean     // 是否已读
  created_at: string
}

/**
 * 好友信息
 */
export interface Friend {
  id: string
  user_id: string
  friend_id: string
  friend_name: string
  friend_avatar?: string
  created_at: string
}

/**
 * 鼓励卡片
 */
export interface EncourageCard {
  id: string
  title: string
  content: string
  category: string
  imageUrl?: string
  createdAt: string
}

/**
 * 哲理命题
 */
export interface PhilosophyCard {
  id: string
  title: string
  content: string
  author?: string
  imageUrl?: string
  createdAt: string
}

/**
 * 偶像信息（保留旧版本兼容）
 */
export interface IdolInfo {
  id: string
  name: string
  avatar: string
  description: string
  category: string
  motto?: string
  created_at?: string
  updated_at?: string
}

/**
 * 偶像语录
 */
export interface IdolQuote {
  id: string
  idol_id: string
  content: string
  created_at: string
}

/**
 * 偶像作品
 */
export interface IdolWork {
  id: string
  idol_id: string
  title: string
  description?: string
  work_type: 'text' | 'image' | 'audio' | 'video' // 作品类型
  file_url?: string // 文件URL（音视频图片）
  cover_url?: string // 封面图（视频用）
  duration?: number // 时长（音视频，单位秒）
  created_at: string
}

// 导出新的偶像类型
export * from './idol'

/**
 * 写作作品
 */
export interface WritingWork {
  id: string
  userId: string
  userName: string
  userAvatar: string
  title: string
  content: string
  category: string
  tags: string[]
  likes: number
  comments: number
  views: number
  isLiked: boolean
  createdAt: string
}

/**
 * 评论
 */
export interface Comment {
  id: string
  userId: string
  userName: string
  userAvatar: string
  content: string
  likes: number
  replies?: Comment[]
  createdAt: string
}

/**
 * 记账记录
 */
export interface AccountRecord {
  id: string
  type: 'income' | 'expense'
  amount: number
  category: string
  description: string
  date: string
  createdAt: string
}

/**
 * 便签
 */
export interface Note {
  id: string
  title: string
  content: string
  color: string
  isPinned: boolean
  createdAt: string
  updatedAt: string
}

/**
 * 账号绑定
 */
export interface AccountBinding {
  id: string
  user_id: string
  bound_user_id: string
  status: 'pending' | 'accepted' | 'rejected' | 'cancelled'
  initiator_id: string
  message?: string
  created_at: string
  updated_at: string
  user_info?: UserInfo
  bound_user_info?: UserInfo
}

/**
 * 应用权限
 */
export interface AppPermission {
  id: string
  user_id: string
  bound_user_id: string
  app_type: 'note' | 'account' | 'love_letter' | 'writing' | 'forum' | 'private_space' | 'idol' | 'chat'
  permission_level: 'none' | 'read' | 'write'
  created_at: string
  updated_at: string
}

/**
 * 绑定用户详情
 */
export interface BoundUserDetail {
  binding: AccountBinding
  permissions: AppPermission[]
}

