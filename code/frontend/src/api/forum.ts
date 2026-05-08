/**
 * 论坛相关 API
 * 统一管理灵感、话题、创作等内容
 */

import { get, post, del } from '@/utils/request'
import type { PageParams, PageResponse } from '@/types'

export interface ForumPost {
  id: string
  user_id: string
  post_type: 'inspiration' | 'topic' | 'creation'
  title: string
  content: string
  card_category?: string // encourage, philosophy
  author?: string
  topic_question?: string
  topic_options?: string[]
  topic_answer?: string
  creation_category?: string
  creation_tags?: string[]
  image_url?: string
  likes: number
  views: number
  comments_count: number
  is_pinned: boolean
  created_at: string
  updated_at: string
}

export interface ForumComment {
  id: string
  post_id: string
  user_id?: string
  parent_id?: string
  content: string
  guest_name?: string
  username?: string
  nickname?: string
  avatar?: string | null
  likes: number
  created_at: string
  replies_count?: number
}

/**
 * 获取帖子列表
 */
export function getPostsApi(params: PageParams & { post_type?: string }) {
  return get<PageResponse<ForumPost>>('/forum/posts', params)
}

/**
 * 创建帖子
 */
export function createPostApi(data: {
  post_type: 'inspiration' | 'topic' | 'creation'
  title: string
  content: string
  card_category?: string
  author?: string
  topic_question?: string
  topic_options?: string[]
  topic_answer?: string
  creation_category?: string
  creation_tags?: string[]
  image_url?: string
}) {
  return post<ForumPost>('/forum/posts', data)
}

/**
 * 获取帖子详情
 */
export function getPostApi(id: string) {
  return get<ForumPost>(`/forum/posts/${id}`)
}

/**
 * 删除帖子
 */
export function deletePostApi(id: string) {
  return del(`/forum/posts/${id}`)
}

/**
 * 点赞帖子
 */
export function likePostApi(id: string) {
  return post(`/forum/posts/${id}/like`)
}

/**
 * 获取评论列表
 */
export function getCommentsApi(postId: string, params: PageParams) {
  return get<PageResponse<ForumComment>>(`/forum/posts/${postId}/comments`, params)
}

/**
 * 创建评论（支持游客）
 */
export function createCommentApi(postId: string, data: {
  post_id: string
  parent_id?: string
  content: string
  guest_name?: string
}) {
  return post<ForumComment>(`/forum/posts/${postId}/comments`, data)
}

/**
 * 点赞评论
 */
export function likeCommentApi(id: string) {
  return post(`/forum/comments/${id}/like`)
}

