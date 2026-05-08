/**
 * 账号绑定相关 API
 */

import { get, post, put, del } from '@/utils/request'

export interface AccountBinding {
  id: string
  user_id: string
  bound_user_id: string
  status: 'pending' | 'accepted' | 'rejected' | 'cancelled'
  initiator_id: string
  message?: string
  created_at: string
  updated_at: string
  user_info?: any
  bound_user_info?: any
}

export interface AppPermission {
  id: string
  user_id: string
  bound_user_id: string
  app_type: 'note' | 'account' | 'love_letter' | 'writing' | 'forum' | 'private_space' | 'idol' | 'chat'
  permission_level: 'none' | 'read' | 'write'
  created_at: string
  updated_at: string
}

export interface BoundUserDetail {
  binding: AccountBinding
  permissions: AppPermission[]
}

/**
 * 发送账号绑定请求
 */
export function sendBindingRequestApi(data: { bound_user_id: string; message?: string }) {
  return post('/bindings/requests', data)
}

/**
 * 处理账号绑定请求
 */
export function handleBindingRequestApi(id: string, data: { action: 'accept' | 'reject' }) {
  return put(`/bindings/requests/${id}`, data)
}

/**
 * 获取我的绑定列表
 */
export function getMyBindingsApi() {
  return get<AccountBinding[]>('/bindings')
}

/**
 * 获取待处理的绑定请求
 */
export function getPendingBindingRequestsApi() {
  return get<AccountBinding[]>('/bindings/requests/pending')
}

/**
 * 取消绑定
 */
export function cancelBindingApi(id: string) {
  return del(`/bindings/${id}`)
}

/**
 * 更新单个应用权限
 */
export function updateAppPermissionApi(
  boundUserId: string,
  appType: string,
  data: { app_type: string; permission_level: 'none' | 'read' | 'write' }
) {
  return put<AppPermission>(`/bindings/${boundUserId}/permissions/${appType}`, data)
}

/**
 * 批量更新应用权限
 */
export function batchUpdatePermissionsApi(
  boundUserId: string,
  permissions: Array<{ app_type: string; permission_level: string }>
) {
  return put<AppPermission[]>(`/bindings/${boundUserId}/permissions/batch`, {
    bound_user_id: boundUserId,
    permissions
  })
}

/**
 * 获取授予某个绑定用户的权限列表
 */
export function getPermissionsForBoundUserApi(boundUserId: string) {
  return get<AppPermission[]>(`/bindings/${boundUserId}/permissions`)
}

/**
 * 获取绑定用户详情（包含权限信息）
 */
export function getBoundUserDetailApi(boundUserId: string) {
  return get<BoundUserDetail>(`/bindings/${boundUserId}/detail`)
}
