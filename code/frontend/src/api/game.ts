/**
 * 游戏 API
 */

import { get, post } from '@/utils/request'

export interface GameScore {
  id: string
  user_id: string
  game_type: string
  level: number
  score: number
  time_spent?: number
  completed: boolean
  created_at: string
}

export interface SubmitScoreRequest {
  game_type: string
  level: number
  score: number
  time_spent?: number
}

export interface RankingItem {
  rank: number
  user_id: string
  username: string
  nickname?: string
  avatar?: string
  score: number
  time_spent?: number
  is_friend: boolean
  is_self: boolean
}

/**
 * 提交游戏分数
 */
export function submitScoreApi(data: SubmitScoreRequest) {
  return post<GameScore>('/games/scores', data)
}

/**
 * 获取我的游戏记录
 */
export function getMyScoresApi(gameType: string, level: number) {
  return get<GameScore[]>('/games/scores/my', { game_type: gameType, level })
}

/**
 * 获取好友排行榜
 */
export function getFriendsRankingApi(gameType: string, level: number) {
  return get<RankingItem[]>('/games/ranking/friends', { game_type: gameType, level })
}

/**
 * 获取全服排行榜
 */
export function getRankingApi(gameType: string, level: number) {
  return get<RankingItem[]>('/games/ranking', { game_type: gameType, level })
}
