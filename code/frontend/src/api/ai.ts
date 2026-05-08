/**
 * AI 聊天相关 API
 */

import { get, post, put, del } from '@/utils/request'
import type { ChatSession, ChatMessage, PageParams, PageResponse } from '@/types'

/**
 * 获取聊天会话列表
 */
export function getChatSessionsApi(params: PageParams) {
  return get<PageResponse<ChatSession>>('/chat/sessions', params)
}

/**
 * 创建聊天会话
 */
export function createChatSessionApi(data: { title: string }) {
  return post<ChatSession>('/chat/sessions', data)
}

/**
 * 获取聊天会话详情
 */
export function getChatSessionApi(id: string) {
  return get<ChatSession>(`/chat/sessions/${id}`)
}

/**
 * 删除聊天会话
 */
export function deleteChatSessionApi(id: string) {
  return del(`/chat/sessions/${id}`)
}

/**
 * 获取聊天消息列表
 */
export function getChatMessagesApi(sessionId: string, params: PageParams) {
  return get<PageResponse<ChatMessage>>(`/chat/sessions/${sessionId}/messages`, params)
}

/**
 * 发送聊天消息
 */
export function sendChatMessageApi(sessionId: string, data: { session_id: string; content: string }) {
  return post<ChatMessage>(`/chat/sessions/${sessionId}/messages`, data)
}

/**
 * 获取专属AI列表
 */
export function getCustomAIsApi() {
  return get('/chat/custom-ais')
}

/**
 * 创建专属AI
 */
export function createCustomAIApi(data: {
  name: string
  avatar: string
  personality: string[]
  style: string
  background?: string
  nickname?: string
  catchphrase?: string
}) {
  return post('/chat/custom-ais', data)
}

/**
 * 获取专属AI详情
 */
export function getCustomAIApi(id: string) {
  return get(`/chat/custom-ais/${id}`)
}

/**
 * 更新专属AI
 */
export function updateCustomAIApi(
  id: string,
  data: {
    name: string
    avatar: string
    personality: string[]
    style: string
    background?: string
    nickname?: string
    catchphrase?: string
  }
) {
  return put(`/chat/custom-ais/${id}`, data)
}

/**
 * 删除专属AI
 */
export function deleteCustomAIApi(id: string) {
  return del(`/chat/custom-ais/${id}`)
}

