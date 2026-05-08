/**
 * 文件上传 API
 */

import { upload } from '@/utils/request'

export interface UploadResponse {
  url: string
  filename: string
}

/**
 * 上传图片
 */
export function uploadImageApi(filePath: string): Promise<UploadResponse> {
  return upload<UploadResponse>('/upload/image', filePath)
}

/**
 * 上传视频
 */
export function uploadVideoApi(filePath: string): Promise<UploadResponse> {
  return upload<UploadResponse>('/upload/video', filePath)
}

/**
 * 上传音频
 */
export function uploadAudioApi(filePath: string): Promise<UploadResponse> {
  return upload<UploadResponse>('/upload/audio', filePath)
}
