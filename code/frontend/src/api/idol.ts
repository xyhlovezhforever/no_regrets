/**
 * 偶像管理相关 API
 */

import { get, post, put, del } from '@/utils/request'
import type {
  Idol,
  IdolDetail,
  IdolQuote,
  IdolWork,
  CreateIdolRequest,
  UpdateIdolRequest,
  CreateQuoteRequest,
  UpdateQuoteRequest,
  CreateWorkRequest,
  UpdateWorkRequest,
  IdolQueryParams,
  PaginatedIdols
} from '@/types/idol'

/**
 * 获取偶像列表
 */
export function getIdolsApi(params?: IdolQueryParams) {
  return get<PaginatedIdols>('/idols', params)
}

/**
 * 获取我的偶像列表
 */
export function getMyIdolsApi(params?: IdolQueryParams) {
  return get<PaginatedIdols>('/idols/my', params)
}

/**
 * 创建偶像
 */
export function createIdolApi(data: CreateIdolRequest) {
  return post<Idol>('/idols', data)
}

/**
 * 获取偶像详情
 */
export function getIdolApi(id: string) {
  // UUID 不需要编码，直接使用
  return get<IdolDetail>(`/idols/${String(id).trim()}`)
}

/**
 * 更新偶像
 */
export function updateIdolApi(id: string, data: UpdateIdolRequest) {
  return put<Idol>(`/idols/${String(id).trim()}`, data)
}

/**
 * 删除偶像
 */
export function deleteIdolApi(id: string) {
  return del(`/idols/${String(id).trim()}`)
}

/**
 * 创建偶像语录
 */
export function createIdolQuoteApi(idolId: string, data: CreateQuoteRequest) {
  return post<IdolQuote>(`/idols/${String(idolId).trim()}/quotes`, data)
}

/**
 * 更新偶像语录
 */
export function updateIdolQuoteApi(quoteId: string, data: UpdateQuoteRequest) {
  return put<IdolQuote>(`/idols/quotes/${String(quoteId).trim()}`, data)
}

/**
 * 删除偶像语录
 */
export function deleteIdolQuoteApi(quoteId: string) {
  return del(`/idols/quotes/${String(quoteId).trim()}`)
}

/**
 * 创建偶像作品
 */
export function createIdolWorkApi(idolId: string, data: CreateWorkRequest) {
  return post<IdolWork>(`/idols/${String(idolId).trim()}/works`, data)
}

/**
 * 更新偶像作品
 */
export function updateIdolWorkApi(workId: string, data: UpdateWorkRequest) {
  return put<IdolWork>(`/idols/works/${String(workId).trim()}`, data)
}

/**
 * 删除偶像作品
 */
export function deleteIdolWorkApi(workId: string) {
  return del(`/idols/works/${String(workId).trim()}`)
}

/**
 * 浏览作品（增加浏览量）
 */
export function viewWorkApi(workId: string) {
  return post<IdolWork>(`/idols/works/${String(workId).trim()}/view`)
}

/**
 * 点赞作品
 */
export function likeWorkApi(workId: string) {
  return post<{ liked: boolean }>(`/idols/works/${String(workId).trim()}/like`)
}

/**
 * 取消点赞作品
 */
export function unlikeWorkApi(workId: string) {
  return del(`/idols/works/${String(workId).trim()}/like`)
}
