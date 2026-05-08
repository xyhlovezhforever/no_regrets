/**
 * 用户相关 API
 */

import { get, put } from '@/utils/request'
import type { UserInfo } from '@/types'

/**
 * 获取用户信息
 */
export function getUserInfoApi() {
  return get<UserInfo>('/user/info')
}

/**
 * 获取其他用户的公开信息
 */
export function getPublicUserInfoApi(userId: string, config?: any) {
  return get<UserInfo>(`/user/${userId}/public`, undefined, config)
}

/**
 * 更新用户信息
 */
export function updateUserInfoApi(data: Partial<UserInfo>) {
  return put<UserInfo>('/user/update', data)
}

/**
 * 修改密码
 */
export function changePasswordApi(data: { old_password: string; new_password: string }) {
  return put('/user/change-password', data)
}

