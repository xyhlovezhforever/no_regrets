<script setup lang="ts">
import { onLaunch, onShow, onHide } from '@dcloudio/uni-app'
import { useAppStore, useUserStore } from '@/store'

const appStore = useAppStore()
const userStore = useUserStore()

onLaunch(async () => {
  // console.log('App Launch')
  
  // 验证 token 是否有效
  if (userStore.token) {
    await userStore.validateToken()
  }
  
  // 检查是否首次访问，显示欢迎弹窗
  if (appStore.isFirstVisit) {
    appStore.setShowWelcomeModal(true)
  }
})

onShow(() => {
  // console.log('App Show')
})

onHide(() => {
  // console.log('App Hide')
})
</script>

<style lang="scss">
@import '@/styles/index.scss';
</style>
