import { createSSRApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import App from './App.vue'
import 'uno.css'

export function createApp() {
  const app = createSSRApp(App)

  // 创建 Pinia 实例
  const pinia = createPinia()
  pinia.use(piniaPluginPersistedstate)
  app.use(pinia)

  return {
    app,
  }
}
