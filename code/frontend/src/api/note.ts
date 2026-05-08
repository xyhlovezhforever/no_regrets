/**
 * 笔记相关 API
 */

import { get, post, put, del } from '@/utils/request'
import type { Note, PageParams, PageResponse } from '@/types'

/**
 * 获取笔记列表
 */
export function getNotesApi(params: PageParams) {
  return get<PageResponse<Note>>('/notes', params)
}

/**
 * 创建笔记
 */
export function createNoteApi(data: {
  title: string
  content: string
  color: string
  is_pinned: boolean
}) {
  return post<Note>('/notes', data)
}

/**
 * 获取笔记详情
 */
export function getNoteApi(id: string) {
  return get<Note>(`/notes/${id}`)
}

/**
 * 更新笔记
 */
export function updateNoteApi(
  id: string,
  data: {
    title: string
    content: string
    color: string
    is_pinned: boolean
  }
) {
  return put<Note>(`/notes/${id}`, data)
}

/**
 * 删除笔记
 */
export function deleteNoteApi(id: string) {
  return del(`/notes/${id}`)
}

