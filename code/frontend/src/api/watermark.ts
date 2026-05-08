import { BASE_API } from '@/config'

export interface WatermarkResponse {
  file_url: string
  filename: string
}

/**
 * 上传并去除水印
 */
export function removeWatermarkApi(
  file: File | string,
  options: {
    type: string
    intensity: number
  }
): Promise<WatermarkResponse> {
  // 小程序端使用 uni.uploadFile
  // #ifndef H5
  return new Promise((resolve, reject) => {
    uni.uploadFile({
      url: `${BASE_API}/watermark/upload`,
      filePath: file as string,
      name: 'file',
      formData: {
        type: options.type,
        intensity: options.intensity.toString()
      },
      success: (res) => {
        if (res.statusCode === 200) {
          const data = JSON.parse(res.data)
          if (data.code === 0) {
            resolve(data.data)
          } else {
            reject(new Error(data.message || '处理失败'))
          }
        } else {
          reject(new Error(`上传失败: ${res.statusCode}`))
        }
      },
      fail: (error) => {
        reject(error)
      }
    })
  })
  // #endif
  
  // H5端使用 XMLHttpRequest
  // #ifdef H5
  return new Promise((resolve, reject) => {
    const formData = new FormData()
    formData.append('file', file as File)
    formData.append('type', options.type)
    formData.append('intensity', options.intensity.toString())
    
    const xhr = new XMLHttpRequest()
    const url = `${BASE_API}/watermark/upload`
    
    console.log('上传URL:', url)
    console.log('上传参数:', { type: options.type, intensity: options.intensity })
    
    xhr.open('POST', url, true)
    
    xhr.onload = function() {
      console.log('响应状态:', xhr.status)
      console.log('响应内容:', xhr.responseText)
      
      if (xhr.status === 200) {
        try {
          const data = JSON.parse(xhr.responseText)
          if (data.code === 0) {
            resolve(data.data)
          } else {
            reject(new Error(data.message || '处理失败'))
          }
        } catch (e) {
          reject(new Error(`JSON解析失败: ${xhr.responseText}`))
        }
      } else {
        reject(new Error(`HTTP错误 ${xhr.status}: ${xhr.responseText}`))
      }
    }
    
    xhr.onerror = function() {
      console.error('网络错误')
      reject(new Error('网络错误：无法连接到服务器'))
    }
    
    xhr.send(formData)
  })
  // #endif
}
