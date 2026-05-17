<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface Selection {
  x: number
  y: number
  width: number
  height: number
}

type HandleType = 'nw' | 'n' | 'ne' | 'w' | 'e' | 'sw' | 's' | 'se' | 'move'

const isSelecting = ref(false)
const isAdjusting = ref(false)
const startPoint = ref({ x: 0, y: 0 })
const endPoint = ref({ x: 0, y: 0 })
const selection = ref<Selection | null>(null)
const activeHandle = ref<HandleType | null>(null)
const dragStart = ref({ x: 0, y: 0 })
const originalSelection = ref<Selection | null>(null)

const emit = defineEmits<{
  (e: 'selectionComplete', selection: Selection): void
  (e: 'cancel'): void
}>()

const getSelection = (): Selection => {
  const x = Math.min(startPoint.value.x, endPoint.value.x)
  const y = Math.min(startPoint.value.y, endPoint.value.y)
  const width = Math.abs(endPoint.value.x - startPoint.value.x)
  const height = Math.abs(endPoint.value.y - startPoint.value.y)
  return { x, y, width, height }
}

const onMouseDown = (e: MouseEvent) => {
  if (e.button === 2) {
    emit('cancel')
    return
  }
  
  // 如果正在调整模式，点击外部取消
  if (isAdjusting.value) {
    const rect = selection.value
    if (rect) {
      const inX = e.clientX >= rect.x && e.clientX <= rect.x + rect.width
      const inY = e.clientY >= rect.y && e.clientY <= rect.y + rect.height
      if (!inX || !inY) {
        emit('cancel')
        return
      }
    }
    return
  }
  
  isSelecting.value = true
  startPoint.value = { x: e.clientX, y: e.clientY }
  endPoint.value = { x: e.clientX, y: e.clientY }
}

const onMouseMove = (e: MouseEvent) => {
  if (isSelecting.value) {
    endPoint.value = { x: e.clientX, y: e.clientY }
    selection.value = getSelection()
    return
  }
  
  if (isAdjusting.value && activeHandle.value && originalSelection.value) {
    const dx = e.clientX - dragStart.value.x
    const dy = e.clientY - dragStart.value.y
    const orig = originalSelection.value
    
    let newX = orig.x
    let newY = orig.y
    let newWidth = orig.width
    let newHeight = orig.height
    
    switch (activeHandle.value) {
      case 'move':
        newX = orig.x + dx
        newY = orig.y + dy
        break
      case 'nw':
        newX = orig.x + dx
        newY = orig.y + dy
        newWidth = orig.width - dx
        newHeight = orig.height - dy
        break
      case 'n':
        newY = orig.y + dy
        newHeight = orig.height - dy
        break
      case 'ne':
        newY = orig.y + dy
        newWidth = orig.width + dx
        newHeight = orig.height - dy
        break
      case 'w':
        newX = orig.x + dx
        newWidth = orig.width - dx
        break
      case 'e':
        newWidth = orig.width + dx
        break
      case 'sw':
        newX = orig.x + dx
        newWidth = orig.width - dx
        newHeight = orig.height + dy
        break
      case 's':
        newHeight = orig.height + dy
        break
      case 'se':
        newWidth = orig.width + dx
        newHeight = orig.height + dy
        break
    }
    
    // 确保最小尺寸
    if (newWidth < 20) {
      if (activeHandle.value.includes('w')) {
        newX = orig.x + orig.width - 20
      }
      newWidth = 20
    }
    if (newHeight < 20) {
      if (activeHandle.value.includes('n')) {
        newY = orig.y + orig.height - 20
      }
      newHeight = 20
    }
    
    selection.value = { x: newX, y: newY, width: newWidth, height: newHeight }
  }
}

const onMouseUp = (e: MouseEvent) => {
  if (e.button === 2) return
  
  if (isSelecting.value) {
    isSelecting.value = false
    if (selection.value && selection.value.width > 20 && selection.value.height > 20) {
      isAdjusting.value = true
    } else {
      selection.value = null
    }
    return
  }
  
  if (isAdjusting.value && activeHandle.value) {
    activeHandle.value = null
    originalSelection.value = null
  }
}

const onHandleMouseDown = (e: MouseEvent, handle: HandleType) => {
  e.stopPropagation()
  e.preventDefault()
  activeHandle.value = handle
  dragStart.value = { x: e.clientX, y: e.clientY }
  originalSelection.value = { ...selection.value! }
}

const onConfirm = () => {
  if (selection.value) {
    emit('selectionComplete', selection.value)
  }
}

const onCancel = () => {
  emit('cancel')
}

const onKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    emit('cancel')
  } else if (e.key === 'Enter' && isAdjusting.value) {
    onConfirm()
  }
}

const onContextMenu = (e: MouseEvent) => {
  e.preventDefault()
}

onMounted(() => {
  window.addEventListener('mousemove', onMouseMove)
  window.addEventListener('mouseup', onMouseUp)
  window.addEventListener('keydown', onKeyDown)
  window.addEventListener('contextmenu', onContextMenu)
})

onUnmounted(() => {
  window.removeEventListener('mousemove', onMouseMove)
  window.removeEventListener('mouseup', onMouseUp)
  window.removeEventListener('keydown', onKeyDown)
  window.removeEventListener('contextmenu', onContextMenu)
})
</script>

<template>
  <div class="selection-container" @mousedown="onMouseDown">
    <div v-if="selection" class="selection-rect" :style="{
      left: `${selection.x}px`,
      top: `${selection.y}px`,
      width: `${selection.width}px`,
      height: `${selection.height}px`
    }">
      <!-- 8 个调整手柄 -->
      <div v-if="isAdjusting" class="handles">
        <div class="handle handle-nw" @mousedown="onHandleMouseDown($event, 'nw')"></div>
        <div class="handle handle-n" @mousedown="onHandleMouseDown($event, 'n')"></div>
        <div class="handle handle-ne" @mousedown="onHandleMouseDown($event, 'ne')"></div>
        <div class="handle handle-w" @mousedown="onHandleMouseDown($event, 'w')"></div>
        <div class="handle handle-e" @mousedown="onHandleMouseDown($event, 'e')"></div>
        <div class="handle handle-sw" @mousedown="onHandleMouseDown($event, 'sw')"></div>
        <div class="handle handle-s" @mousedown="onHandleMouseDown($event, 's')"></div>
        <div class="handle handle-se" @mousedown="onHandleMouseDown($event, 'se')"></div>
        
        <!-- 移动区域 -->
        <div class="move-area" @mousedown="onHandleMouseDown($event, 'move')"></div>
      </div>
      
      <!-- 尺寸标签 -->
      <span class="size-label">{{ Math.round(selection.width) }} × {{ Math.round(selection.height) }}</span>
      
      <!-- 确认/取消按钮 -->
      <div v-if="isAdjusting" class="actions">
        <button class="btn btn-confirm" @click="onConfirm">✓</button>
        <button class="btn btn-cancel" @click="onCancel">✕</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.selection-container {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  cursor: crosshair;
}

.selection-rect {
  position: absolute;
  border: 2px solid #00ffff;
  background: transparent;
  box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.3);
}

.handles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
}

.handle {
  position: absolute;
  width: 10px;
  height: 10px;
  background: #00ffff;
  border: 1px solid #008888;
  z-index: 10;
}

.handle-nw {
  top: -5px;
  left: -5px;
  cursor: nw-resize;
}

.handle-n {
  top: -5px;
  left: 50%;
  transform: translateX(-50%);
  cursor: n-resize;
}

.handle-ne {
  top: -5px;
  right: -5px;
  cursor: ne-resize;
}

.handle-w {
  top: 50%;
  left: -5px;
  transform: translateY(-50%);
  cursor: w-resize;
}

.handle-e {
  top: 50%;
  right: -5px;
  transform: translateY(-50%);
  cursor: e-resize;
}

.handle-sw {
  bottom: -5px;
  left: -5px;
  cursor: sw-resize;
}

.handle-s {
  bottom: -5px;
  left: 50%;
  transform: translateX(-50%);
  cursor: s-resize;
}

.handle-se {
  bottom: -5px;
  right: -5px;
  cursor: se-resize;
}

.move-area {
  position: absolute;
  top: 10px;
  left: 10px;
  right: 10px;
  bottom: 10px;
  cursor: move;
}

.size-label {
  position: absolute;
  bottom: -25px;
  left: 50%;
  transform: translateX(-50%);
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  white-space: nowrap;
}

.actions {
  position: absolute;
  bottom: -35px;
  right: 0;
  display: flex;
  gap: 8px;
}

.btn {
  width: 28px;
  height: 28px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: transform 0.1s;
}

.btn:hover {
  transform: scale(1.1);
}

.btn-confirm {
  background: #00ffff;
  color: #000;
}

.btn-cancel {
  background: #ff4444;
  color: #fff;
}
</style>
