/**
 * 私人空间相关 API
 */

import { get, put, post, del } from '@/utils/request'

export interface PrivateSpaceSettings {
  light_color: string
  light_intensity: number
}

export interface PrivateSpaceMusic {
  id: string
  name: string
  artist?: string
  url?: string
  createdAt: string
}

/**
 * 获取私人空间设置
 */
export function getPrivateSpaceSettingsApi() {
  return get<PrivateSpaceSettings>('/private/settings')
}

/**
 * 更新私人空间设置
 */
export function updatePrivateSpaceSettingsApi(data: {
  light_color?: string
  light_intensity?: number
}) {
  return put('/private/settings', data)
}

/**
 * 获取私人空间音乐列表
 */
export function getPrivateSpaceMusicListApi() {
  return get<PrivateSpaceMusic[]>('/private/music')
}

/**
 * 添加私人空间音乐
 */
export function addPrivateSpaceMusicApi(data: {
  name: string
  artist?: string
  url?: string
}) {
  return post('/private/music', data)
}

/**
 * 删除私人空间音乐
 */
export function deletePrivateSpaceMusicApi(id: string) {
  return del(`/private/music/${id}`)
}

