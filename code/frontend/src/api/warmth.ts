/**
 * 温暖板块相关 API
 */

import { post } from '@/utils/request'
import type { LoveLetter } from '@/types'

/**
 * 生成情书
 */
export function generateLoveLetterApi(data: {
  title: string
  content: string
  to_name: string
  from_name: string
}) {
  return post<{ content: string }>('/warmth/love-letter', data)
}

/**
 * 记录拥抱
 */
export function recordHugApi() {
  return post<{ total_hugs: number }>('/warmth/hug')
}

