/**
 * Mock 数据
 * 
 * ⚠️ 注意：此文件中的数据仅供参考，不再实际使用
 * 前端已全面对接后端真实 API，所有接口均调用后端服务
 * 
 * 保留此文件的目的：
 * 1. 作为数据结构的参考示例
 * 2. 便于理解各个类型的数据格式
 * 3. 在后端服务不可用时，可以临时启用作为降级方案
 * 
 * 如需使用真实数据，请确保：
 * 1. 后端服务已启动（默认 http://localhost:8000）
 * 2. 在 .env.development 中配置了正确的 VITE_APP_BASE_API
 * 3. 已通过登录接口获取有效的 token
 */

import type {
  UserInfo,
  LoginResponse,
  ChatMessage,
  EncourageCard,
  PhilosophyCard,
  IdolInfo,
  WritingWork,
} from '@/types'

/**
 * Mock 用户信息
 */
export const mockUserInfo: UserInfo = {
  id: '1',
  username: 'test_user',
  nickname: '测试用户',
  avatar: 'https://via.placeholder.com/150',
  email: 'test@example.com',
  phone: '13800138000',
  gender: 1,
  bio: '这是一个测试用户',
  createdAt: '2024-01-01T00:00:00Z',
  updatedAt: '2024-01-01T00:00:00Z',
}

/**
 * Mock 登录响应
 */
export const mockLoginResponse: LoginResponse = {
  token: 'mock_access_token_12345',
  refreshToken: 'mock_refresh_token_67890',
  userInfo: mockUserInfo,
}

/**
 * Mock 聊天消息
 */
export const mockChatMessages: ChatMessage[] = [
  {
    id: '1',
    role: 'user',
    content: '你好，我今天心情不太好',
    timestamp: Date.now() - 10000,
    avatar: 'https://via.placeholder.com/150',
  },
  {
    id: '2',
    role: 'assistant',
    content: '很高兴认识你！我理解你现在的心情，想聊聊发生了什么吗？',
    timestamp: Date.now() - 8000,
  },
  {
    id: '3',
    role: 'user',
    content: '工作上遇到了一些困难',
    timestamp: Date.now() - 5000,
    avatar: 'https://via.placeholder.com/150',
  },
]

/**
 * Mock 鼓励卡片
 */
export const mockEncourageCards: EncourageCard[] = [
  {
    id: '1',
    title: '你是最棒的',
    content: '每一次努力都不会白费，相信自己，你一定可以的！',
    category: 'motivation',
    imageUrl: 'https://via.placeholder.com/300x200',
    createdAt: '2024-01-01T00:00:00Z',
  },
  {
    id: '2',
    title: '坚持就是胜利',
    content: '成功的道路充满挑战，但只要坚持，终会迎来曙光。',
    category: 'perseverance',
    imageUrl: 'https://via.placeholder.com/300x200',
    createdAt: '2024-01-02T00:00:00Z',
  },
  {
    id: '3',
    title: '拥抱每一天',
    content: '生活总会有起伏，但每一天都值得我们用心去感受。',
    category: 'positivity',
    imageUrl: 'https://via.placeholder.com/300x200',
    createdAt: '2024-01-03T00:00:00Z',
  },
]

/**
 * Mock 哲理命题
 */
export const mockPhilosophyCards: PhilosophyCard[] = [
  {
    id: '1',
    title: '人生的意义',
    content: '人生的意义不在于我们拥有什么，而在于我们成为了什么样的人。',
    author: '佚名',
    imageUrl: 'https://via.placeholder.com/300x200',
    createdAt: '2024-01-01T00:00:00Z',
  },
  {
    id: '2',
    title: '时间的价值',
    content: '时间是最公平的资源，每个人每天都拥有24小时，关键在于如何使用它。',
    author: '佚名',
    imageUrl: 'https://via.placeholder.com/300x200',
    createdAt: '2024-01-02T00:00:00Z',
  },
]

/**
 * Mock 偶像信息
 */
export const mockIdols: IdolInfo[] = [
  {
    id: '1',
    name: '张国荣',
    avatar: 'https://via.placeholder.com/150',
    description: '香港著名歌手、演员，华语乐坛和影坛的标杆人物。',
    category: 'singer',
    quotes: [
      '我就是我，是颜色不一样的烟火。',
      '风继续吹，不忍远离。',
      '追，寻觅一些自己的东西。',
    ],
    images: [
      'https://via.placeholder.com/300x400',
      'https://via.placeholder.com/300x400',
    ],
  },
]

/**
 * Mock 写作作品
 */
export const mockWritingWorks: WritingWork[] = [
  {
    id: '1',
    userId: '1',
    userName: '文学青年',
    userAvatar: 'https://via.placeholder.com/150',
    title: '致青春的一封信',
    content: '亲爱的青春，感谢你赋予我勇气和梦想...',
    category: 'prose',
    tags: ['青春', '回忆', '成长'],
    likes: 128,
    comments: 45,
    views: 567,
    isLiked: false,
    createdAt: '2024-01-01T00:00:00Z',
  },
  {
    id: '2',
    userId: '2',
    userName: '诗意生活',
    userAvatar: 'https://via.placeholder.com/150',
    title: '春天的诗',
    content: '春风拂面，万物复苏，这是一个充满希望的季节...',
    category: 'poetry',
    tags: ['诗歌', '春天', '希望'],
    likes: 89,
    comments: 23,
    views: 345,
    isLiked: true,
    createdAt: '2024-01-02T00:00:00Z',
  },
]

