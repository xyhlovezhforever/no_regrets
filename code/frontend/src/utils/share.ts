/**
 * 分享工具函数
 */

interface ShareOptions {
  title: string
  content: string
  image?: string
  link?: string
}

/**
 * 通用分享方法
 * 支持生成分享图片和分享链接
 */
export function shareContent(options: ShareOptions) {
  uni.showActionSheet({
    itemList: ['生成分享图片', '复制分享文本', '分享链接'],
    success: (res) => {
      if (res.tapIndex === 0) {
        // 生成分享图片
        generateShareImage(options)
      } else if (res.tapIndex === 1) {
        // 复制分享文本
        const shareText = `${options.title}\n\n${options.content}${options.link ? `\n\n链接：${options.link}` : ''}`
        uni.setClipboardData({
          data: shareText,
          success: () => {
            uni.showToast({ title: '已复制到剪贴板', icon: 'success' })
          }
        })
      } else if (res.tapIndex === 2) {
        // 分享链接
        if (options.link) {
          uni.setClipboardData({
            data: options.link,
            success: () => {
              uni.showToast({ title: '链接已复制', icon: 'success' })
            }
          })
        } else {
          uni.showToast({ title: '暂无链接', icon: 'none' })
        }
      }
    }
  })
}

/**
 * 生成分享图片
 * 使用 canvas 绘制分享卡片
 */
export function generateShareImage(options: ShareOptions) {
  // 显示生成中提示
  uni.showLoading({ title: '生成中...' })
  
  // 创建 canvas 上下文
  const query = uni.createSelectorQuery()
  query.select('#shareCanvas').fields({ node: true, size: true }).exec((res) => {
    if (!res || !res[0]) {
      uni.hideLoading()
      uni.showToast({ title: '生成失败', icon: 'none' })
      return
    }
    
    const canvas = res[0].node
    const ctx = canvas.getContext('2d')
    
    // 设置 canvas 尺寸
    const dpr = uni.getSystemInfoSync().pixelRatio
    canvas.width = 750 * dpr
    canvas.height = 1200 * dpr
    ctx.scale(dpr, dpr)
    
    // 绘制背景
    const gradient = ctx.createLinearGradient(0, 0, 0, 1200)
    gradient.addColorStop(0, '#667eea')
    gradient.addColorStop(1, '#764ba2')
    ctx.fillStyle = gradient
    ctx.fillRect(0, 0, 750, 1200)
    
    // 绘制标题
    ctx.fillStyle = '#ffffff'
    ctx.font = 'bold 48px sans-serif'
    ctx.textAlign = 'center'
    ctx.fillText(options.title, 375, 200)
    
    // 绘制内容
    ctx.font = '32px sans-serif'
    const contentLines = wrapText(ctx, options.content, 650)
    let yPos = 300
    contentLines.forEach((line: string) => {
      ctx.fillText(line, 375, yPos)
      yPos += 50
    })
    
    // 绘制二维码或链接（如果有）
    if (options.link) {
      ctx.font = '24px sans-serif'
      ctx.fillStyle = 'rgba(255, 255, 255, 0.8)'
      ctx.fillText('扫码或访问链接查看详情', 375, 1000)
      ctx.fillText(options.link, 375, 1050)
    }
    
    // 绘制底部信息
    ctx.font = '28px sans-serif'
    ctx.fillStyle = 'rgba(255, 255, 255, 0.9)'
    ctx.fillText('— 无悔青春 —', 375, 1150)
    
    // 导出图片
    setTimeout(() => {
      uni.canvasToTempFilePath({
        canvas,
        success: (res) => {
          uni.hideLoading()
          uni.previewImage({
            urls: [res.tempFilePath],
            success: () => {
              uni.showToast({ title: '长按图片保存', icon: 'none' })
            }
          })
        },
        fail: () => {
          uni.hideLoading()
          uni.showToast({ title: '生成失败', icon: 'none' })
        }
      })
    }, 500)
  })
}

/**
 * 文本换行处理
 */
function wrapText(ctx: any, text: string, maxWidth: number): string[] {
  const lines: string[] = []
  let currentLine = ''
  
  for (let i = 0; i < text.length; i++) {
    const testLine = currentLine + text[i]
    const metrics = ctx.measureText(testLine)
    
    if (metrics.width > maxWidth && i > 0) {
      lines.push(currentLine)
      currentLine = text[i]
    } else {
      currentLine = testLine
    }
  }
  
  if (currentLine) {
    lines.push(currentLine)
  }
  
  return lines
}

/**
 * 快速分享（简化版）
 * 直接复制文本到剪贴板
 */
export function quickShare(title: string, content: string) {
  const shareText = `${title}\n\n${content}\n\n— 来自无悔青春`
  
  uni.setClipboardData({
    data: shareText,
    success: () => {
      uni.showToast({ title: '已复制，快去分享吧', icon: 'success' })
    }
  })
}

