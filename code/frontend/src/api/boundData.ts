/**
 * 绑定用户数据访问 API
 */

import { get } from '@/utils/request'

// 便签数据
export interface BoundNote {
  id: string
  user_id: string
  title: string
  content: string
  category?: string
  tags: string[]
  is_pinned: boolean
  created_at: string
  updated_at: string
}

// 记账数据
export interface BoundAccount {
  id: string
  user_id: string
  amount: number
  category: string
  description?: string
  date: string
  type_: string // 'income' | 'expense'
  created_at: string
}

// 记账统计
export interface AccountStatistics {
  total_income: number
  total_expense: number
  balance: number
  record_count: number
}

/**
 * 获取绑定用户的便签数据
 */
export function getBoundUserNotesApi(boundUserId: string) {
  return get<BoundNote[]>(`/bound-data/${boundUserId}/notes`)
}

/**
 * 获取绑定用户的记账数据
 */
export function getBoundUserAccountsApi(
  boundUserId: string,
  params?: {
    start_date?: string
    end_date?: string
  }
) {
  return get<BoundAccount[]>(`/bound-data/${boundUserId}/accounts`, params)
}

/**
 * 获取绑定用户的记账统计
 */
export function getBoundUserAccountStatisticsApi(
  boundUserId: string,
  params?: {
    start_date?: string
    end_date?: string
  }
) {
  return get<AccountStatistics>(`/bound-data/${boundUserId}/accounts/statistics`, params)
}
