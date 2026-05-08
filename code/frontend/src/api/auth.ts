/**
 * 认证相关 API
 */

import { get, post } from '@/utils/request'
import type { LoginRequest, LoginResponse, RegisterRequest, UserInfo } from '@/types'

/**
 * 用户登录
 */
export function loginApi(data: LoginRequest) {
  return post<LoginResponse>('/auth/login', data)
}

/**
 * 用户注册
 */
export function registerApi(data: RegisterRequest) {
  return post<LoginResponse>('/auth/register', data)
}

/**
 * 获取用户信息
 */
export function getUserInfoApi(config?: any) {
  return get<UserInfo>('/auth/user-info', undefined, config)
}

/**
 * 刷新 token
 */
export function refreshTokenApi(refreshToken: string) {
  return post<{ token: string; refreshToken: string }>('/auth/refresh-token', { refreshToken })
}

/**
 * 退出登录
 */
export function logoutApi() {
  return post('/auth/logout')
}

/**
 * 更新用户信息
 */
export function updateUserInfoApi(data: Partial<UserInfo>) {
  return post<UserInfo>('/auth/update-profile', data)
}

/**
 * 修改密码
 */
export function changePasswordApi(data: { oldPassword: string; newPassword: string }) {
  return post('/auth/change-password', data)
}

