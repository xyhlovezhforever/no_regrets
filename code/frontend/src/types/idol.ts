/**
 * 偶像相关类型定义
 */

export interface Idol {
  id: string
  user_id: string
  name: string
  description?: string
  avatar_url?: string
  birth_date?: string
  nationality?: string
  profession?: string
  tags?: string[]
  is_public: boolean
  created_at: string
  updated_at: string
}

export interface IdolQuote {
  id: string
  idol_id: string
  content: string
  source?: string
  quote_date?: string
  created_at: string
  updated_at: string
}

export interface IdolWork {
  id: string
  idol_id: string
  title: string
  description?: string
  work_type: 'audio' | 'video' | 'image' | 'other'
  file_url: string
  thumbnail_url?: string
  file_size?: number
  duration?: number
  release_date?: string
  tags?: string[]
  view_count: number
  like_count: number
  created_at: string
  updated_at: string
}

export interface IdolDetail extends Idol {
  quotes: IdolQuote[]
  works: IdolWork[]
}

export interface CreateIdolRequest {
  name: string
  description?: string
  avatar_url?: string
  birth_date?: string
  nationality?: string
  profession?: string
  tags?: string[]
  is_public?: boolean
}

export interface UpdateIdolRequest {
  name?: string
  description?: string
  avatar_url?: string
  birth_date?: string
  nationality?: string
  profession?: string
  tags?: string[]
  is_public?: boolean
}

export interface CreateQuoteRequest {
  content: string
  source?: string
  quote_date?: string
}

export interface UpdateQuoteRequest {
  content?: string
  source?: string
  quote_date?: string
}

export interface CreateWorkRequest {
  title: string
  description?: string
  work_type: string
  file_url: string
  thumbnail_url?: string
  file_size?: number
  duration?: number
  release_date?: string
  tags?: string[]
}

export interface UpdateWorkRequest {
  title?: string
  description?: string
  work_type?: string
  file_url?: string
  thumbnail_url?: string
  file_size?: number
  duration?: number
  release_date?: string
  tags?: string[]
}

export interface IdolQueryParams {
  page?: number
  page_size?: number
  search?: string
  tags?: string[]
  only_public?: boolean
}

export interface PaginatedIdols {
  items: Idol[]
  total: number
  page: number
  page_size: number
  total_pages: number
}
