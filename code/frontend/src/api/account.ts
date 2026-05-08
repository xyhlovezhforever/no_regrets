/**
 * 记账相关 API
 */

import { get, post, put } from '@/utils/request'
import http from '@/utils/request'

export interface AccountRecord {
  id: string
  record_type: 'income' | 'expense'
  amount: number
  category: string
  description?: string
  date: string
  created_at: string
}

export interface AccountSummary {
  total_income: number
  total_expense: number
  balance: number
}

export interface MonthlyBillData {
  year: number
  month: number
  total_income: number
  total_expense: number
  balance: number
  daily: Array<{
    date: string
    income: number
    expense: number
  }>
  by_category: Array<{
    record_type: string
    category: string
    total: number
  }>
}

export interface YearlyBillData {
  year: number
  total_income: number
  total_expense: number
  balance: number
  monthly: Array<{
    month: string
    income: number
    expense: number
  }>
  by_category: Array<{
    record_type: string
    category: string
    total: number
  }>
}

/**
 * 获取记账记录列表
 */
export function getAccountRecordsApi(params?: any) {
  return get<AccountRecord[]>('/accounts', params)
}

/**
 * 获取单个记账记录
 */
export function getAccountRecordApi(id: string) {
  return get<AccountRecord>(`/accounts/${id}`)
}

/**
 * 创建记账记录
 */
export function createAccountRecordApi(data: {
  record_type: 'income' | 'expense'
  amount: number
  category: string
  description?: string
  date: string
}) {
  return post<AccountRecord>('/accounts', data)
}

/**
 * 更新记账记录
 */
export function updateAccountRecordApi(id: string, data: {
  record_type: 'income' | 'expense'
  amount: number
  category: string
  description?: string
  date: string
}) {
  return put<AccountRecord>(`/accounts/${id}`, data)
}

/**
 * 删除记账记录
 */
export function deleteAccountRecordApi(id: string) {
  return http.delete(`/accounts/${id}`)
}

/**
 * 获取记账汇总
 */
export function getAccountSummaryApi() {
  return get<AccountSummary>('/accounts/summary')
}

/**
 * 获取月账单
 */
export function getMonthlyBillApi(year: number, month: number) {
  return get<MonthlyBillData>('/accounts/monthly-bill', { year, month })
}

/**
 * 获取年账单
 */
export function getYearlyBillApi(year: number) {
  return get<YearlyBillData>('/accounts/yearly-bill', { year })
}
