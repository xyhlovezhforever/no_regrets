/**
 * 应用配置
 */

// 获取环境变量
export const ENV = import.meta.env.VITE_APP_ENV || 'development'
export const BASE_API = import.meta.env.VITE_APP_BASE_API || 'http://localhost:8000/api/v1'
export const APP_TITLE = import.meta.env.VITE_APP_TITLE || '情绪价值小程序'

// API 配置
export const API_CONFIG = {
  timeout: 30000, // 请求超时时间
  retryCount: 3, // 重试次数
  retryDelay: 1000, // 重试延迟
}

// Token 配置
export const TOKEN_KEY = 'Authorization'
export const TOKEN_PREFIX = 'Bearer '

// 存储键名
export const STORAGE_KEYS = {
  TOKEN: 'access_token',
  REFRESH_TOKEN: 'refresh_token',
  USER_INFO: 'user_info',
  THEME: 'theme',
  LANGUAGE: 'language',
}

// 页面路径
export const PAGE_PATHS = {
  HOME: '/pages/index/index',
  LOGIN: '/pages/auth/login',
  REGISTER: '/pages/auth/register',
  USER_CENTER: '/pages/user/center',
  AI_CHAT: '/pages/ai/chat',
  WARMTH: '/pages/warmth/index',
  IDOL: '/pages/idol/index',
  ENCOURAGE: '/pages/encourage/index',
  PHILOSOPHY: '/pages/philosophy/index',
  RELEASE: '/pages/release/index',
  SELF: '/pages/self/index',
}

// 是否为开发环境
export const isDevelopment = ENV === 'development'

// 是否为生产环境
export const isProduction = ENV === 'production'

export default {
  ENV,
  BASE_API,
  APP_TITLE,
  API_CONFIG,
  TOKEN_KEY,
  TOKEN_PREFIX,
  STORAGE_KEYS,
  PAGE_PATHS,
  isDevelopment,
  isProduction,
}

