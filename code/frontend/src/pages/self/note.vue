<template>
  <view class="note-page">
    <!-- 文件夹标签栏 -->
    <scroll-view v-if="!isViewMode" class="folder-tabs" scroll-x>
      <view class="folder-tab" 
        :class="{ active: currentFolderId === null }" 
        @click="switchFolder(null)">
        <text>📁 全部</text>
      </view>
      <view v-for="folder in folders" 
        :key="folder.id" 
        class="folder-tab" 
        :class="{ active: currentFolderId === folder.id }"
        @click="switchFolder(folder.id)">
        <text>{{ folder.name }}</text>
      </view>
      <view class="folder-tab add-folder" @click="showFolderDialog = true">
        <text>+ 文件夹</text>
      </view>
    </scroll-view>

    <!-- 查看模式提示 -->
    <view v-if="isViewMode && viewedUserInfo" class="view-mode-banner">
      <text class="banner-icon">👁️</text>
      <text class="banner-text">正在查看 {{ viewedUserInfo.nickname }} 的便签（只读模式）</text>
    </view>

    <!-- 顶部工具栏 -->
    <view class="toolbar" v-if="notes.length > 0 && !isViewMode">
      <button v-if="!multiSelectMode" class="tool-btn" @click="enterMultiSelect">
        <text>批量管理</text>
      </button>
      <template v-else>
        <button class="tool-btn" @click="exitMultiSelect">
          <text>取消</text>
        </button>
        <button class="tool-btn primary" @click="batchMove" :disabled="selectedNotes.size === 0">
          <text>移动 ({{selectedNotes.size}})</text>
        </button>
        <button class="tool-btn danger" @click="batchDelete" :disabled="selectedNotes.size === 0">
          <text>删除 ({{selectedNotes.size}})</text>
        </button>
      </template>
    </view>

    <!-- 便签列表 -->
    <view class="note-list" v-if="filteredNotes.length > 0">
      <view
        v-for="note in filteredNotes"
        :key="note.id"
        class="note-item"
        :class="{ selected: selectedNotes.has(note.id) }"
        :style="{ background: note.color }"
        @click="handleNoteClick(note)"
      >
        <!-- 多选模式复选框 -->
        <view v-if="multiSelectMode" class="checkbox-wrapper">
          <view class="checkbox" :class="{ checked: selectedNotes.has(note.id) }">
            <text v-if="selectedNotes.has(note.id)">✓</text>
          </view>
        </view>

        <view class="note-content-wrapper">
          <view class="note-header">
            <text class="note-title" :style="{ color: note.fontColor || '#1a202c' }">{{ note.title }}</text>
          </view>
          <text class="note-content" :style="{ color: note.fontColor || '#2d3748' }">{{ note.content }}</text>
          <text class="note-time" :style="{ color: note.fontColor || '#4a5568' }">{{ note.updatedAt }}</text>
        </view>

        <!-- 单个便签操作按钮 -->
        <view v-if="!multiSelectMode && !isViewMode" class="note-actions">
          <button class="action-btn" @click.stop="togglePin(note)">
            <text class="action-icon">{{ note.isPinned ? '📌' : '📄' }}</text>
            <text class="action-text">{{ note.isPinned ? '已置顶' : '置顶' }}</text>
          </button>
          <button class="action-btn" @click.stop="showMoveNoteDialog(note)">
            <text class="action-icon">📁</text>
            <text class="action-text">移动</text>
          </button>
          <button class="action-btn danger" @click.stop="deleteNote(note)">
            <text class="action-icon">🗑️</text>
            <text class="action-text">删除</text>
          </button>
        </view>
      </view>
    </view>

    <view v-else class="empty-state">
      <text class="empty-icon">📝</text>
      <text class="empty-text">还没有笔记</text>
      <text class="empty-hint">点击右下角 + 号创建</text>
    </view>

    <!-- 添加按钮 -->
    <view v-if="!isViewMode" class="add-btn" @click="createNote">
      <text>+</text>
    </view>

    <!-- 文件夹创建/编辑对话框 -->
    <view v-if="showFolderDialog" class="dialog-mask" @click="closeFolderDialog">
      <view class="dialog-content" @click.stop>
        <view class="dialog-header">
          <text class="dialog-title">{{ folderForm.id ? '编辑文件夹' : '新建文件夹' }}</text>
          <view class="dialog-close" @click="closeFolderDialog">
            <text class="close-icon">×</text>
          </view>
        </view>
        <view class="dialog-body">
          <view class="form-item">
            <text class="form-label">名称</text>
            <view class="input-wrapper">
              <input 
                :value="folderForm.name" 
                class="form-input" 
                type="text"
                placeholder="请输入文件夹名称"
                placeholder-style="color: var(--theme-text-secondary)"
                confirm-type="done"
                :maxlength="20"
                cursor-spacing="100"
                :adjust-position="true"
                @input="handleNameInput"
                @blur="handleNameBlur"
              />
            </view>
          </view>
          <view class="form-item">
            <text class="form-label">颜色</text>
            <view class="color-grid">
              <view v-for="color in folderColors" 
                :key="color" 
                class="color-item"
                :class="{ active: folderForm.color === color }"
                :style="{ background: color }"
                @click="folderForm.color = color">
              </view>
            </view>
          </view>
        </view>
        <view class="dialog-footer">
          <button class="dialog-btn" @click="closeFolderDialog">取消</button>
          <button class="dialog-btn primary" @click="saveFolder">保存</button>
        </view>
      </view>
    </view>

    <!-- 移动到文件夹对话框 -->
    <view v-if="showMoveDialog" class="dialog-mask" @click="closeMoveDialog">
      <view class="dialog-content" @click.stop>
        <view class="dialog-header">
          <text class="dialog-title">移动到文件夹</text>
          <view class="dialog-close" @click="closeMoveDialog">
            <text class="close-icon">×</text>
          </view>
        </view>
        <view class="dialog-body">
          <view class="folder-list">
            <view class="folder-list-item" @click="moveToFolder(null)">
              <text>📁 未分类</text>
            </view>
            <view v-for="folder in folders" 
              :key="folder.id" 
              class="folder-list-item"
              @click="moveToFolder(folder.id)">
              <text>{{ folder.name }}</text>
            </view>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { onShow, onLoad } from '@dcloudio/uni-app'
import { getStorage, setStorage } from '@/utils/storage'
import { navigateTo, formatDate, generateId, showToast } from '@/utils'
import { useViewMode } from '@/composables/useViewMode'
import { getBoundUserNotesApi, type BoundNote } from '@/api/boundData'

interface Note {
  id: string
  title: string
  content: string
  color: string
  fontColor?: string
  folderId?: string | null
  isPinned: boolean
  createdAt: string
  updatedAt: string
}

interface Folder {
  id: string
  name: string
  color: string
  sortOrder: number
  createdAt: string
  updatedAt: string
}

// 查看模式
const { isViewMode, viewedUserId, viewedUserInfo, canEdit, canDelete, canCreate, setViewMode, clearViewMode } = useViewMode()

const notes = ref<Note[]>([])
const folders = ref<Folder[]>([])
const currentFolderId = ref<string | null>(null)
const multiSelectMode = ref(false)
const selectedNotes = ref<Set<string>>(new Set())
const showFolderDialog = ref(false)
const showMoveDialog = ref(false)
const currentMoveNote = ref<Note | null>(null)

// 页面参数
const pageParams = ref<any>({})

const folderForm = ref({
  id: '',
  name: '',
  color: '#667eea',
})

const folderColors = [
  '#667eea', '#764ba2', '#f093fb', '#4facfe',
  '#43e97b', '#fa709a', '#fee140', '#30cfd0',
]

// 过滤当前文件夹的便签
const filteredNotes = computed(() => {
  let result = notes.value
  if (currentFolderId.value !== null) {
    result = result.filter(n => n.folderId === currentFolderId.value)
  }
  
  return result.sort((a, b) => {
    if (a.isPinned && !b.isPinned) return -1
    if (!a.isPinned && b.isPinned) return 1
    return new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
  })
})

const loadData = async () => {
  // 如果是查看模式，从API加载绑定用户的数据
  if (isViewMode.value && viewedUserId.value) {
    try {
      const boundNotes = await getBoundUserNotesApi(viewedUserId.value)
      // 转换为本地Note格式
      notes.value = boundNotes.map((note: BoundNote) => ({
        id: note.id,
        title: note.title,
        content: note.content,
        color: '#FFE4B5',
        fontColor: '#333333',
        folderId: null,
        isPinned: note.is_pinned,
        createdAt: note.created_at,
        updatedAt: note.updated_at,
      }))
      folders.value = [] // 查看模式不显示文件夹
    } catch (error: any) {
      showToast(error.message || '加载失败')
      notes.value = []
    }
  } else {
    // 普通模式，从本地存储加载
    notes.value = getStorage<Note[]>('notes', [])
    folders.value = getStorage<Folder[]>('note_folders', [])
    
    // 如果没有笔记，添加示例数据
    if (notes.value.length === 0) {
      notes.value = [
        {
          id: '1',
          title: '今日待办',
          content: '1. 完成项目文档\n2. 给妈妈打电话\n3. 运动30分钟',
          color: '#FFE4B5',
          fontColor: '#333333',
          folderId: null,
          isPinned: true,
          createdAt: formatDate(new Date()),
          updatedAt: formatDate(new Date()),
        },
        {
          id: '2',
          title: '灵感记录',
          content: '今天突然想到一个好点子...',
          color: '#E0F7FA',
          fontColor: '#333333',
          folderId: null,
          isPinned: false,
          createdAt: formatDate(new Date()),
          updatedAt: formatDate(new Date()),
        },
      ]
      setStorage('notes', notes.value)
    }
  }
}

onLoad((options: any) => {
  pageParams.value = options || {}
  
  // 如果传入了查看用户的参数，设置查看模式
  if (options?.viewUserId) {
    const userInfo = options.userInfo ? JSON.parse(decodeURIComponent(options.userInfo)) : null
    setViewMode({
      isViewMode: true,
      viewedUserId: options.viewUserId,
      viewedUserInfo: userInfo,
      appType: 'note',
    })
  } else {
    clearViewMode()
  }
})

onMounted(() => {
  loadData()
})

onShow(() => {
  loadData()
})

// 切换文件夹
const switchFolder = (folderId: string | null) => {
  currentFolderId.value = folderId
  exitMultiSelect()
}

// 进入多选模式
const enterMultiSelect = () => {
  multiSelectMode.value = true
  selectedNotes.value.clear()
}

// 退出多选模式
const exitMultiSelect = () => {
  multiSelectMode.value = false
  selectedNotes.value.clear()
}

// 处理便签点击
const handleNoteClick = (note: Note) => {
  // 查看模式下，只能查看详情，不能编辑
  if (isViewMode.value) {
    showToast('查看模式下不能编辑便签')
    return
  }
  
  if (multiSelectMode.value) {
    // 多选模式：切换选中状态
    if (selectedNotes.value.has(note.id)) {
      selectedNotes.value.delete(note.id)
    } else {
      selectedNotes.value.add(note.id)
    }
    // 强制更新
    selectedNotes.value = new Set(selectedNotes.value)
  } else {
    // 普通模式：编辑便签
    editNote(note)
  }
}

// 批量删除
const batchDelete = () => {
  if (selectedNotes.value.size === 0) return
  
  uni.showModal({
    title: '确认删除',
    content: `确定要删除选中的 ${selectedNotes.value.size} 条笔记吗？`,
    success: (res) => {
      if (res.confirm) {
        const notesList = getStorage<Note[]>('notes', []) || []
        const newNotes = notesList.filter(n => !selectedNotes.value.has(n.id))
        setStorage('notes', newNotes)
        loadData()
        exitMultiSelect()
        uni.showToast({ title: '删除成功', icon: 'success' })
      }
    }
  })
}

// 显示单个便签的移动对话框
const showMoveNoteDialog = (note: Note) => {
  currentMoveNote.value = note
  selectedNotes.value = new Set([note.id])
  showMoveDialog.value = true
}

// 批量移动
const batchMove = () => {
  if (selectedNotes.value.size === 0) return
  currentMoveNote.value = null
  showMoveDialog.value = true
}

// 移动到指定文件夹
const moveToFolder = (folderId: string | null) => {
  const notesList = getStorage<Note[]>('notes', []) || []
  let movedCount = 0
  
  notesList.forEach(note => {
    if (selectedNotes.value.has(note.id)) {
      note.folderId = folderId
      note.updatedAt = formatDate(new Date())
      movedCount++
    }
  })
  
  setStorage('notes', notesList)
  loadData()
  closeMoveDialog()
  
  // 如果是单个移动，不退出多选模式
  if (currentMoveNote.value) {
    currentMoveNote.value = null
  } else {
    exitMultiSelect()
  }
  
  const folderName = folderId ? folders.value.find(f => f.id === folderId)?.name : '未分类'
  uni.showToast({ 
    title: `已移动 ${movedCount} 条到「${folderName}」`, 
    icon: 'success' 
  })
}

const createNote = () => {
  if (!canCreate.value) {
    showToast('查看模式下不能创建便签')
    return
  }
  navigateTo('/pages/self/note-edit', { folderId: currentFolderId.value || '' })
}

const editNote = (note: Note) => {
  navigateTo('/pages/self/note-edit', { id: note.id })
}

const togglePin = (note: Note) => {
  if (!canEdit.value) {
    showToast('查看模式下不能置顶便签')
    return
  }
  const notesList = getStorage<Note[]>('notes', []) || []
  const index = notesList.findIndex(n => n.id === note.id)
  if (index >= 0) {
    notesList[index].isPinned = !notesList[index].isPinned
    notesList[index].updatedAt = formatDate(new Date())
    setStorage('notes', notesList)
    loadData()
    uni.showToast({
      title: notesList[index].isPinned ? '已置顶' : '已取消置顶',
      icon: 'none'
    })
  }
}

const deleteNote = (note: Note) => {
  if (!canDelete.value) {
    showToast('查看模式下不能删除便签')
    return
  }
  uni.showModal({
    title: '确认删除',
    content: '确定要删除这条笔记吗？',
    success: (res) => {
      if (res.confirm) {
        const notesList = getStorage<Note[]>('notes', []) || []
        const newNotes = notesList.filter(n => n.id !== note.id)
        setStorage('notes', newNotes)
        loadData()
        uni.showToast({ title: '已删除', icon: 'success' })
      }
    }
  })
}

// 文件夹管理
const closeFolderDialog = () => {
  showFolderDialog.value = false
  folderForm.value = { id: '', name: '', color: '#667eea' }
}

const closeMoveDialog = () => {
  showMoveDialog.value = false
  currentMoveNote.value = null
  // 如果不是多选模式，清空选中
  if (!multiSelectMode.value) {
    selectedNotes.value.clear()
  }
}

const handleNameInput = (e: any) => {
  console.log('input event:', e.detail.value)
  folderForm.value.name = e.detail.value
}

const handleNameBlur = (e: any) => {
  console.log('blur event:', e.detail.value)
  folderForm.value.name = e.detail.value
}

const saveFolder = () => {
  if (!folderForm.value.name.trim()) {
    uni.showToast({ title: '请输入文件夹名称', icon: 'none' })
    return
  }

  const foldersList = getStorage<Folder[]>('note_folders', []) || []
  
  if (folderForm.value.id) {
    // 编辑
    const index = foldersList.findIndex(f => f.id === folderForm.value.id)
    if (index >= 0) {
      foldersList[index].name = folderForm.value.name
      foldersList[index].color = folderForm.value.color
      foldersList[index].updatedAt = formatDate(new Date())
    }
  } else {
    // 新建
    foldersList.push({
      id: generateId(),
      name: folderForm.value.name,
      color: folderForm.value.color,
      sortOrder: foldersList.length,
      createdAt: formatDate(new Date()),
      updatedAt: formatDate(new Date()),
    })
  }
  
  setStorage('note_folders', foldersList)
  loadData()
  closeFolderDialog()
  uni.showToast({ title: '保存成功', icon: 'success' })
}
</script>

<style lang="scss" scoped>
@import '@/styles/cyber-theme.scss';

.note-page {
  @include cyber-page-bg;
  min-height: 100vh;
  padding-bottom: 150rpx;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 30% 20%, rgba(255, 0, 214, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 70% 80%, rgba(0, 217, 255, 0.12) 0%, transparent 50%);
    pointer-events: none;
    animation: bgPulse 8s ease-in-out infinite;
    z-index: 0;
  }
}

// 查看模式横幅
.view-mode-banner {
  @include neon-card;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 15rpx;
  padding: 20rpx 30rpx;
  margin: 20rpx 30rpx;
  background: linear-gradient(135deg, rgba(255, 152, 0, 0.2) 0%, rgba(255, 193, 7, 0.2) 100%);
  border: 2rpx solid rgba(255, 152, 0, 0.5);
  border-radius: 20rpx;
  position: relative;
  z-index: 1;
  box-shadow: 
    0 8rpx 32rpx rgba(255, 152, 0, 0.3),
    inset 0 0 30rpx rgba(255, 152, 0, 0.1);

  .banner-icon {
    font-size: 32rpx;
  }

  .banner-text {
    font-size: 28rpx;
    color: #fff;
    font-weight: 500;
  }
}

// 文件夹标签栏
.folder-tabs {
  @include neon-card;
  display: flex;
  white-space: nowrap;
  padding: 25rpx 30rpx;
  border-bottom: 2rpx solid rgba(255, 0, 214, 0.3);
  position: relative;
  z-index: 1;
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 50rpx rgba(255, 0, 214, 0.3),
    inset 0 0 40rpx rgba(255, 0, 214, 0.1);

  .folder-tab {
    display: inline-flex;
    align-items: center;
    padding: 14rpx 28rpx;
    margin-right: 18rpx;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
    backdrop-filter: blur(10rpx);
    border-radius: 25rpx;
    font-size: 26rpx;
    @include neon-text(#b8c5d6);
    border: 2rpx solid rgba(138, 92, 246, 0.4);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    animation: slideDown 0.6s ease-out backwards;

    &.active {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border-color: rgba(255, 0, 214, 0.8);
      box-shadow: 
        0 6rpx 20rpx rgba(255, 0, 214, 0.5),
        0 0 30rpx rgba(255, 0, 214, 0.4);
      transform: scale(1.05);
    }

    &.add-folder {
      background: transparent;
      border: 2rpx dashed rgba(0, 217, 255, 0.6);
      @include neon-text(#00D9FF);
      
      &:active {
        border-style: solid;
        background: linear-gradient(135deg, rgba(0, 217, 255, 0.2) 0%, rgba(138, 92, 246, 0.2) 100%);
        box-shadow: 0 0 30rpx rgba(0, 217, 255, 0.4);
      }
    }
    
    &:active {
      transform: scale(0.95);
    }
  }
}

// 工具栏
.toolbar {
  @include neon-card;
  display: flex;
  gap: 18rpx;
  padding: 25rpx 30rpx;
  border-bottom: 2rpx solid rgba(0, 217, 255, 0.3);
  position: relative;
  z-index: 1;
  box-shadow: 
    0 10rpx 40rpx rgba(0, 0, 0, 0.5),
    0 0 50rpx rgba(0, 217, 255, 0.3),
    inset 0 0 40rpx rgba(0, 217, 255, 0.1);

  .tool-btn {
    flex: 1;
    padding: 18rpx;
    font-size: 26rpx;
    font-weight: 500;
    border-radius: 18rpx;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.6) 0%, rgba(30, 36, 66, 0.4) 100%);
    backdrop-filter: blur(10rpx);
    border: 2rpx solid rgba(138, 92, 246, 0.5);
    @include neon-text(#b8c5d6);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    
    &::after {
      border: none;
    }

    &.primary {
      background: linear-gradient(135deg, rgba(0, 217, 255, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
      @include neon-text(#ffffff);
      border-color: rgba(0, 217, 255, 0.7);
      box-shadow: 
        0 6rpx 20rpx rgba(0, 217, 255, 0.5),
        0 0 30rpx rgba(0, 217, 255, 0.4);
    }

    &.danger {
      background: linear-gradient(135deg, rgba(255, 0, 79, 0.9) 0%, rgba(255, 107, 107, 0.9) 100%);
      @include neon-text(#ffffff);
      border-color: rgba(255, 0, 79, 0.7);
      box-shadow: 
        0 6rpx 20rpx rgba(255, 0, 79, 0.5),
        0 0 30rpx rgba(255, 0, 79, 0.4);
    }

    &[disabled] {
      opacity: 0.5;
      cursor: not-allowed;
    }
    
    &:active:not([disabled]) {
      transform: scale(0.95);
    }
  }
}

.note-list {
  padding: 30rpx;
  display: flex;
  flex-direction: column;
  gap: 25rpx;
  position: relative;
  z-index: 1;
}

.note-item {
  border-radius: 25rpx;
  display: flex;
  flex-direction: column;
  box-shadow: 
    0 15rpx 50rpx rgba(0, 0, 0, 0.6),
    0 0 60rpx rgba(255, 0, 214, 0.4),
    0 0 100rpx rgba(0, 217, 255, 0.2),
    inset 0 0 60rpx rgba(255, 255, 255, 0.08);
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
  border: 3rpx solid transparent;
  background-image: 
    linear-gradient(rgba(30, 36, 66, 0.3), rgba(30, 36, 66, 0.3)),
    linear-gradient(135deg, 
      rgba(255, 0, 214, 0.6) 0%, 
      rgba(138, 92, 246, 0.6) 25%, 
      rgba(0, 217, 255, 0.6) 50%,
      rgba(138, 92, 246, 0.6) 75%,
      rgba(255, 0, 214, 0.6) 100%);
  background-origin: border-box;
  background-clip: padding-box, border-box;
  animation: itemSlideIn 0.5s ease-out backwards, cardGlow 4s ease-in-out infinite;
  
  &::before {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: conic-gradient(
      from 0deg,
      transparent 0deg,
      rgba(255, 0, 214, 0.15) 45deg,
      transparent 90deg,
      rgba(0, 217, 255, 0.15) 135deg,
      transparent 180deg,
      rgba(255, 0, 214, 0.15) 225deg,
      transparent 270deg,
      rgba(0, 217, 255, 0.15) 315deg,
      transparent 360deg
    );
    animation: bgRotate 15s linear infinite;
    z-index: 0;
    pointer-events: none;
  }
  
  &::after {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
      90deg,
      transparent 0%,
      rgba(255, 255, 255, 0.3) 50%,
      transparent 100%
    );
    animation: shimmer 3s ease-in-out infinite;
    z-index: 1;
    pointer-events: none;
  }

  &.selected {
    border: 4rpx solid rgba(255, 0, 214, 1);
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.7),
      0 0 80rpx rgba(255, 0, 214, 0.9),
      0 0 120rpx rgba(255, 0, 214, 0.6),
      inset 0 0 80rpx rgba(255, 0, 214, 0.3);
    transform: translateY(-8rpx) scale(1.02);
    animation: selectedPulse 1.5s ease-in-out infinite;
  }

  &:active {
    transform: scale(0.97) translateY(-6rpx);
    box-shadow: 
      0 18rpx 55rpx rgba(0, 0, 0, 0.7),
      0 0 70rpx rgba(255, 0, 214, 0.6),
      0 0 110rpx rgba(0, 217, 255, 0.4),
      inset 0 0 70rpx rgba(255, 255, 255, 0.12);
  }

  .checkbox-wrapper {
    position: absolute;
    top: 20rpx;
    left: 20rpx;
    z-index: 10;

    .checkbox {
      width: 48rpx;
      height: 48rpx;
      border: 2rpx solid rgba(138, 92, 246, 0.6);
      border-radius: 50%;
      display: flex;
      align-items: center;
      justify-content: center;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.8) 0%, rgba(30, 36, 66, 0.6) 100%);
      backdrop-filter: blur(10rpx);
      box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.3);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

      &.checked {
        background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
        border-color: rgba(255, 0, 214, 0.8);
        @include neon-text(#ffffff);
        box-shadow: 
          0 6rpx 20rpx rgba(255, 0, 214, 0.5),
          0 0 30rpx rgba(255, 0, 214, 0.4);
        animation: checkPulse 0.3s ease-out;
      }
    }
  }

  .note-content-wrapper {
    flex: 1;
    padding: 28rpx;
    display: flex;
    flex-direction: column;
    gap: 14rpx;
    position: relative;
    z-index: 1;
  }

  .note-header {
    .note-title {
      font-size: 34rpx;
      font-weight: bold;
      line-height: 1.5;
      word-break: break-word;
      text-shadow: 
        0 2rpx 8rpx rgba(0, 0, 0, 0.5),
        0 0 15rpx rgba(255, 255, 255, 0.3);
      filter: brightness(1.1);
    }
  }

  .note-content {
    font-size: 27rpx;
    line-height: 1.65;
    word-break: break-word;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    text-shadow: 
      0 1rpx 4rpx rgba(0, 0, 0, 0.3),
      0 0 10rpx rgba(255, 255, 255, 0.15);
  }

  .note-time {
    font-size: 23rpx;
    margin-top: 10rpx;
    opacity: 0.85;
    text-shadow: 
      0 1rpx 3rpx rgba(0, 0, 0, 0.3),
      0 0 8rpx rgba(255, 255, 255, 0.2);
    font-weight: 500;
  }

  .note-actions {
    display: flex;
    justify-content: flex-end;
    gap: 20rpx;
    padding: 20rpx 26rpx;
    background: linear-gradient(135deg, rgba(0, 0, 0, 0.25) 0%, rgba(0, 0, 0, 0.15) 100%);
    backdrop-filter: blur(15rpx);
    border-top: 2rpx solid rgba(255, 0, 214, 0.2);
    position: relative;
    z-index: 1;
    
    &::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      height: 2rpx;
      background: linear-gradient(90deg, 
        transparent 0%,
        rgba(255, 0, 214, 0.8) 20%,
        rgba(0, 217, 255, 0.8) 50%,
        rgba(255, 0, 214, 0.8) 80%,
        transparent 100%);
      animation: borderFlow 3s linear infinite;
    }

    .action-btn {
      display: flex;
      align-items: center;
      gap: 8rpx;
      padding: 12rpx 20rpx;
      background: linear-gradient(135deg, rgba(255, 255, 255, 0.2) 0%, rgba(255, 255, 255, 0.1) 100%);
      backdrop-filter: blur(8rpx);
      border: 2rpx solid rgba(255, 255, 255, 0.3);
      border-radius: 15rpx;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      box-shadow: 
        0 4rpx 12rpx rgba(0, 0, 0, 0.3),
        0 0 20rpx rgba(255, 255, 255, 0.1);
      position: relative;
      overflow: hidden;
      
      &::before {
        content: '';
        position: absolute;
        top: -50%;
        left: -50%;
        width: 200%;
        height: 200%;
        background: conic-gradient(
          from 0deg,
          transparent 0deg,
          rgba(255, 255, 255, 0.1) 90deg,
          transparent 180deg
        );
        animation: rotate 3s linear infinite;
        opacity: 0;
        transition: opacity 0.3s;
      }

      &::after {
        border: none;
      }

      &:hover::before {
        opacity: 1;
      }

      &:active {
        transform: scale(0.92);
        background: linear-gradient(135deg, rgba(255, 255, 255, 0.3) 0%, rgba(255, 255, 255, 0.2) 100%);
        box-shadow: 
          0 6rpx 16rpx rgba(0, 0, 0, 0.4),
          0 0 30rpx rgba(255, 255, 255, 0.3),
          inset 0 0 20rpx rgba(255, 255, 255, 0.2);
          
        &::before {
          opacity: 1;
        }
      }

      .action-icon {
        font-size: 30rpx;
        filter: drop-shadow(0 0 12rpx rgba(255, 255, 255, 0.8));
        position: relative;
        z-index: 1;
      }

      .action-text {
        font-size: 25rpx;
        font-weight: 600;
        text-shadow: 
          0 1rpx 3rpx rgba(0, 0, 0, 0.5),
          0 0 10rpx rgba(255, 255, 255, 0.4);
        position: relative;
        z-index: 1;
      }

      &.danger {
        border-color: rgba(255, 0, 79, 0.5);
        
        &::before {
          background: conic-gradient(
            from 0deg,
            transparent 0deg,
            rgba(255, 0, 79, 0.3) 90deg,
            transparent 180deg
          );
        }
        
        .action-icon {
          filter: drop-shadow(0 0 15rpx rgba(255, 0, 79, 1));
        }

        .action-text {
          @include neon-text(#FF004F);
          text-shadow: 
            0 0 12rpx rgba(255, 0, 79, 0.8),
            0 0 20rpx rgba(255, 0, 79, 0.5),
            0 1rpx 3rpx rgba(0, 0, 0, 0.5);
        }
        
        &:active {
          box-shadow: 
            0 6rpx 16rpx rgba(0, 0, 0, 0.4),
            0 0 40rpx rgba(255, 0, 79, 0.6),
            inset 0 0 25rpx rgba(255, 0, 79, 0.2);
        }
      }
    }
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 200rpx 30rpx;
  position: relative;
  z-index: 1;

  .empty-icon {
    font-size: 140rpx;
    margin-bottom: 35rpx;
    filter: drop-shadow(0 0 40rpx rgba(255, 0, 214, 0.8));
    animation: emptyFloat 3s ease-in-out infinite;
  }

  .empty-text {
    font-size: 36rpx;
    @include neon-title(#FF00D6);
    margin-bottom: 15rpx;
  }

  .empty-hint {
    font-size: 28rpx;
    @include neon-text(#00D9FF);
  }
}

.add-btn {
  position: fixed;
  bottom: 100rpx;
  right: 40rpx;
  width: 110rpx;
  height: 110rpx;
  background: linear-gradient(135deg, rgba(255, 0, 214, 0.9) 0%, rgba(138, 92, 246, 0.9) 100%);
  border-radius: 55rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  border: 3rpx solid rgba(255, 0, 214, 0.7);
  box-shadow: 
    0 10rpx 40rpx rgba(255, 0, 214, 0.6),
    0 0 60rpx rgba(255, 0, 214, 0.5),
    inset 0 0 40rpx rgba(255, 0, 214, 0.2);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  z-index: 100;
  animation: btnPulse 3s ease-in-out infinite;
  
  &::before {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: conic-gradient(
      from 0deg,
      transparent 0deg,
      rgba(255, 255, 255, 0.3) 90deg,
      transparent 180deg
    );
    animation: rotate 3s linear infinite;
    border-radius: 50%;
  }

  text {
    font-size: 64rpx;
    @include neon-text(#ffffff);
    font-weight: 300;
    position: relative;
    z-index: 1;
    filter: drop-shadow(0 0 20rpx rgba(255, 255, 255, 1));
  }

  &:active {
    transform: scale(0.92) rotate(90deg);
    box-shadow: 
      0 12rpx 45rpx rgba(255, 0, 214, 0.8),
      0 0 80rpx rgba(255, 0, 214, 0.7),
      inset 0 0 50rpx rgba(255, 0, 214, 0.3);
  }
}

// 对话框
.dialog-mask {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.75);
  backdrop-filter: blur(10rpx);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.3s ease-out;
}

.dialog-content {
  width: 620rpx;
  @include neon-card;
  border-radius: 30rpx;
  overflow: visible;
  max-height: 80vh;
  border: 3rpx solid rgba(255, 0, 214, 0.6);
  box-shadow: 
    0 20rpx 60rpx rgba(0, 0, 0, 0.7),
    0 0 80rpx rgba(255, 0, 214, 0.5),
    inset 0 0 60rpx rgba(255, 0, 214, 0.15);
  animation: scaleIn 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 35rpx;
  border-bottom: 2rpx solid rgba(255, 0, 214, 0.3);

  .dialog-title {
    font-size: 36rpx;
    font-weight: bold;
    @include neon-title(#FFD600);
  }

  .dialog-close {
    width: 60rpx;
    height: 60rpx;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, rgba(30, 36, 66, 0.8) 0%, rgba(30, 36, 66, 0.6) 100%);
    backdrop-filter: blur(10rpx);
    border-radius: 50%;
    border: 2rpx solid rgba(255, 0, 214, 0.5);
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.3);

    &:active {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.4) 0%, rgba(138, 92, 246, 0.4) 100%);
      transform: scale(0.9) rotate(90deg);
      box-shadow: 
        0 6rpx 20rpx rgba(0, 0, 0, 0.4),
        0 0 30rpx rgba(255, 0, 214, 0.5);
    }

    .close-icon {
      font-size: 48rpx;
      font-weight: 300;
      @include neon-text(#FF00D6);
      line-height: 1;
    }
  }
}

.dialog-body {
  padding: 35rpx;

  .form-item {
    margin-bottom: 35rpx;

    .form-label {
      display: block;
      margin-bottom: 18rpx;
      font-size: 28rpx;
      @include neon-text(#00D9FF);
      font-weight: 500;
    }

    .input-wrapper {
      position: relative;
      width: 100%;
    }

    .form-input {
      @include neon-input;
      width: 100%;
      height: 88rpx;
      padding: 0 25rpx;
      border-radius: 18rpx;
      font-size: 28rpx;
      color: #ffffff;
      box-sizing: border-box;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      line-height: 88rpx;

      &:focus {
        border-color: rgba(255, 0, 214, 0.8);
        box-shadow: 
          0 0 40rpx rgba(255, 0, 214, 0.5),
          inset 0 0 30rpx rgba(255, 0, 214, 0.1);
      }
    }

    .color-grid {
      display: grid;
      grid-template-columns: repeat(4, 1fr);
      gap: 18rpx;

      .color-item {
        aspect-ratio: 1;
        border-radius: 12rpx;
        border: 2rpx solid rgba(255, 255, 255, 0.2);
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        cursor: pointer;
        box-shadow: 0 4rpx 12rpx rgba(0, 0, 0, 0.3);

        &.active {
          border-color: rgba(255, 0, 214, 0.9);
          transform: scale(1.15);
          box-shadow: 
            0 6rpx 20rpx rgba(0, 0, 0, 0.4),
            0 0 30rpx rgba(255, 0, 214, 0.6);
          animation: checkPulse 0.3s ease-out;
        }
        
        &:active {
          transform: scale(0.95);
        }
      }
    }
  }

  .folder-list {
    .folder-list-item {
      padding: 28rpx;
      margin-bottom: 12rpx;
      background: linear-gradient(135deg, rgba(30, 36, 66, 0.4) 0%, rgba(30, 36, 66, 0.2) 100%);
      backdrop-filter: blur(10rpx);
      border-radius: 18rpx;
      border: 2rpx solid rgba(138, 92, 246, 0.3);
      font-size: 28rpx;
      @include neon-text(#ffffff);
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      cursor: pointer;

      &:active {
        transform: translateX(10rpx);
        border-color: rgba(255, 0, 214, 0.6);
        background: linear-gradient(135deg, rgba(255, 0, 214, 0.3) 0%, rgba(138, 92, 246, 0.3) 100%);
        box-shadow: 
          0 6rpx 20rpx rgba(0, 0, 0, 0.4),
          0 0 30rpx rgba(255, 0, 214, 0.4);
      }
    }
  }
}

.dialog-footer {
  display: flex;
  border-top: 2rpx solid rgba(255, 0, 214, 0.3);

  .dialog-btn {
    flex: 1;
    padding: 28rpx;
    background: transparent;
    border: none;
    border-right: 2rpx solid rgba(255, 0, 214, 0.3);
    font-size: 30rpx;
    @include neon-text(#b8c5d6);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

    &::after {
      border: none;
    }

    &:last-child {
      border-right: none;
    }

    &.primary {
      @include neon-text(#FFD600);
      font-weight: bold;
    }
    
    &:active {
      background: linear-gradient(135deg, rgba(255, 0, 214, 0.2) 0%, rgba(138, 92, 246, 0.2) 100%);
      box-shadow: inset 0 0 30rpx rgba(255, 0, 214, 0.3);
    }
  }
}

@keyframes bgPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

@keyframes bgRotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-30rpx);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes itemSlideIn {
  from {
    opacity: 0;
    transform: translateX(-30rpx);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes emptyFloat {
  0%, 100% {
    transform: translateY(0) rotate(0deg);
  }
  50% {
    transform: translateY(-25rpx) rotate(10deg);
  }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes scaleIn {
  from {
    transform: scale(0.85);
    opacity: 0;
  }
  to {
    transform: scale(1);
    opacity: 1;
  }
}

@keyframes checkPulse {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.15);
  }
}

@keyframes btnPulse {
  0%, 100% {
    box-shadow: 
      0 10rpx 40rpx rgba(255, 0, 214, 0.6),
      0 0 60rpx rgba(255, 0, 214, 0.5),
      inset 0 0 40rpx rgba(255, 0, 214, 0.2);
  }
  50% {
    box-shadow: 
      0 12rpx 50rpx rgba(255, 0, 214, 0.8),
      0 0 80rpx rgba(255, 0, 214, 0.7),
      inset 0 0 50rpx rgba(255, 0, 214, 0.3);
  }
}

@keyframes rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes cardGlow {
  0%, 100% {
    box-shadow: 
      0 15rpx 50rpx rgba(0, 0, 0, 0.6),
      0 0 60rpx rgba(255, 0, 214, 0.4),
      0 0 100rpx rgba(0, 217, 255, 0.2),
      inset 0 0 60rpx rgba(255, 255, 255, 0.08);
  }
  50% {
    box-shadow: 
      0 18rpx 55rpx rgba(0, 0, 0, 0.7),
      0 0 80rpx rgba(255, 0, 214, 0.6),
      0 0 120rpx rgba(0, 217, 255, 0.4),
      inset 0 0 70rpx rgba(255, 255, 255, 0.12);
  }
}

@keyframes shimmer {
  0% {
    left: -100%;
  }
  50%, 100% {
    left: 100%;
  }
}

@keyframes selectedPulse {
  0%, 100% {
    box-shadow: 
      0 20rpx 60rpx rgba(0, 0, 0, 0.7),
      0 0 80rpx rgba(255, 0, 214, 0.9),
      0 0 120rpx rgba(255, 0, 214, 0.6),
      inset 0 0 80rpx rgba(255, 0, 214, 0.3);
    transform: translateY(-8rpx) scale(1.02);
  }
  50% {
    box-shadow: 
      0 25rpx 70rpx rgba(0, 0, 0, 0.8),
      0 0 100rpx rgba(255, 0, 214, 1),
      0 0 150rpx rgba(255, 0, 214, 0.8),
      inset 0 0 100rpx rgba(255, 0, 214, 0.4);
    transform: translateY(-10rpx) scale(1.03);
  }
}

@keyframes borderFlow {
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 200% 50%;
  }
}
</style>
