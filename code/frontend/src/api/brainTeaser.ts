/**
 * 脑筋急转弯 API
 */

import { get, post, del } from '@/utils/request'

export interface BrainTeaser {
  id: string
  user_id: string
  question: string
  answer: string
  is_public: boolean
  created_at: string
  updated_at: string
}

export interface CreateBrainTeaserRequest {
  question: string
  answer: string
  is_public?: boolean
}

/**
 * 创建脑筋急转弯
 */
export function createBrainTeaserApi(data: CreateBrainTeaserRequest) {
  return post<BrainTeaser>('/brain-teasers', data)
}

/**
 * 获取用户的脑筋急转弯列表
 */
export function getUserBrainTeasersApi() {
  return get<BrainTeaser[]>('/brain-teasers')
}

/**
 * 获取公开的脑筋急转弯列表
 */
export function getPublicBrainTeasersApi() {
  return get<BrainTeaser[]>('/brain-teasers/public')
}

/**
 * 删除脑筋急转弯
 */
export function deleteBrainTeaserApi(id: string) {
  return del(`/brain-teasers/${id}`)
}
