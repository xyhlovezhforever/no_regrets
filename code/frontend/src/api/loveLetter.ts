/**
 * 情书相关 API
 */

import { get, post, put, del } from '@/utils/request'
import type { LoveLetter, Friend } from '@/types'

/**
 * 获取好友列表
 */
export function getFriendsApi() {
  return get<Friend[]>('/friends')
}

/**
 * 创建情书
 */
export function createLoveLetterApi(data: {
  title: string
  content: string
  to_user_id: string
}) {
  return post<LoveLetter>('/love-letters', data)
}

/**
 * 获取收到的情书列表
 */
export function getReceivedLettersApi() {
  return get<LoveLetter[]>('/love-letters/received')
}

/**
 * 获取发送的情书列表
 */
export function getSentLettersApi() {
  return get<LoveLetter[]>('/love-letters/sent')
}

/**
 * 获取情书详情
 */
export function getLetterDetailApi(id: string) {
  return get<LoveLetter>(`/love-letters/${id}`)
}

/**
 * 标记情书为已读
 */
export function markLetterAsReadApi(id: string) {
  return put(`/love-letters/${id}/read`)
}

/**
 * 删除情书
 */
export function deleteLetterApi(id: string) {
  return del(`/love-letters/${id}`)
}
