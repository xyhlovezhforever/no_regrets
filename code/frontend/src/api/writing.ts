/**
 * 写作交流相关 API
 */

import { get, post, put, del } from '@/utils/request'
import type { WritingWork, Comment, PageParams, PageResponse } from '@/types'

/**
 * 获取写作作品列表
 */
export function getWorksApi(params: PageParams) {
  return get<PageResponse<WritingWork>>('/writing/works', params)
}

/**
 * 创建写作作品
 */
export function createWorkApi(data: {
  title: string
  content: string
  category: string
  tags: string[]
}) {
  return post<WritingWork>('/writing/works', data)
}

/**
 * 获取写作作品详情
 */
export function getWorkApi(id: string) {
  return get<WritingWork>(`/writing/works/${id}`)
}

/**
 * 更新写作作品
 */
export function updateWorkApi(
  id: string,
  data: {
    title: string
    content: string
    category: string
    tags: string[]
  }
) {
  return put<WritingWork>(`/writing/works/${id}`, data)
}

/**
 * 删除写作作品
 */
export function deleteWorkApi(id: string) {
  return del(`/writing/works/${id}`)
}

/**
 * 获取作品评论列表
 */
export function getCommentsApi(workId: string, params: PageParams) {
  return get<PageResponse<Comment>>(`/writing/works/${workId}/comments`, params)
}

/**
 * 创建作品评论
 */
export function createCommentApi(workId: string, data: {
  work_id: string
  parent_id?: string
  content: string
}) {
  return post<Comment>(`/writing/works/${workId}/comments`, data)
}

/**
 * 点赞作品
 */
export function likeWorkApi(workId: string) {
  return post(`/writing/works/${workId}/like`)
}

