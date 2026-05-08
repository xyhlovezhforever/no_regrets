/**
 * 好友管理相关 API
 */

import { get, post, put } from '@/utils/request'
import type { PageParams, PageResponse } from '@/types'

export interface Friend {
  id: string
  user_id: string
  username: string
  nickname: string
  avatar?: string | null
  online?: boolean
  last_message?: string
  last_time?: number
  unread_count?: number
  created_at: string
}

export interface FriendRequest {
  id: string
  from_user_id: string
  to_user_id: string
  from_username?: string
  from_nickname?: string
  from_avatar?: string | null
  message?: string
  status: string
  created_at: string
}

export interface FriendMessage {
  id: string
  from_user_id: string
  to_user_id: string
  content: string
  message_type: string
  is_read: boolean
  created_at: string
}

/**
 * 获取好友列表
 */
export function getFriendsApi(config?: any) {
  return get<Friend[]>('/friends', undefined, config)
}

/**
 * 获取好友申请列表
 */
export function getFriendRequestsApi() {
  return get<FriendRequest[]>('/friends/requests')
}

/**
 * 添加好友
 */
export function addFriendApi(data: { friend_id: string; message?: string }) {
  return post('/friends/requests', data)
}

/**
 * 处理好友申请
 */
export function handleFriendRequestApi(id: string, data: { friendship_id: string; action: 'accept' | 'reject' }) {
  return put(`/friends/requests/${id}`, data)
}

/**
 * 获取好友消息列表
 */
export function getFriendMessagesApi(params: PageParams) {
  return get<PageResponse<FriendMessage>>('/friends/messages', params)
}

/**
 * 发送好友消息
 */
export function sendFriendMessageApi(data: { to_user_id: string; content: string; message_type?: string }, config?: any) {
  return post<FriendMessage>('/friends/messages', data, config)
}

/**
 * 获取与指定好友的聊天记录
 */
export function getChatHistoryApi(friendId: string, params?: PageParams, config?: any) {
  return get<FriendMessage[]>(`/friends/messages/${friendId}`, params || { page: 1, pageSize: 50 }, config)
}

/**
 * 标记与指定好友的所有未读消息为已读
 */
export function markMessagesAsReadApi(friendId: string, config?: any) {
  return put(`/friends/messages/${friendId}/read`, undefined, config)
}

/**
 * 搜索用户
 */
export function searchUserApi(keyword: string) {
  return get<any[]>(`/user/search`, { keyword })
}

