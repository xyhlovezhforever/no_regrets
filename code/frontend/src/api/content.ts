/**
 * 内容相关 API (鼓励卡片、哲学命题)
 */

import { get } from '@/utils/request'
import type { EncourageCard, PhilosophyCard } from '@/types'

/**
 * 获取随机鼓励卡片
 */
export function getRandomEncourageCardApi() {
  return get<EncourageCard>('/encourage/random')
}

/**
 * 获取鼓励卡片列表
 */
export function getEncourageCardsApi() {
  return get<EncourageCard[]>('/encourage/list')
}

/**
 * 获取随机哲学命题
 */
export function getRandomPhilosophyCardApi() {
  return get<PhilosophyCard>('/philosophy/random')
}

/**
 * 获取哲学命题列表
 */
export function getPhilosophyCardsApi() {
  return get<PhilosophyCard[]>('/philosophy/list')
}

