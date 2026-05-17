<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'

interface Selection {
  x: number
  y: number
  width: number
  height: number
}

type HandleType = 'nw' | 'n' | 'ne' | 'w' | 'e' | 'sw' | 's' | 'se'

const props = defineProps<{
  mode: 'create' | 'adjust'
  initialSelection?: Selection
}>()

const emit = defineEmits<{
  (e: 'selectionComplete', selection: Selection): void
  (e: 'selectionChange', selection: Selection): void
  (e: 'dragStart'): void
  (e: 'dragEnd'): void
  (e: 'cancel'): void
}>()

const isSelecting = ref(false)
const startPoint = ref({ x: 0, y: 0 })
const endPoint = ref({ x: 0, y: 0 })
const selection = ref<Selection | null>(props.initialSelection || null)
const activeHandle = ref<HandleType | null>(null)
const dragStart = ref({ x: 0, y: 0 })
const originalSelection = ref<Selection | null>(null)

watch(() => props.initialSelection, (newVal) => {
  if (newVal) {
    selection.value = { ...newVal }
  }
})

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
  
  if (props.mode === 'create') {
    isSelecting.value = true
    startPoint.value = { x: e.clientX, y: e.clientY }
    endPoint.value = { x: e.clientX, y: e.clientY }
  }
}

const onMouseMove = (e: MouseEvent) => {
  if (props.mode === 'create' && isSelecting.value) {
    endPoint.value = { x: e.clientX, y: e.clientY }
    selection.value = getSelection()
    return
  }
  
  if (props.mode === 'adjust' && activeHandle.value && originalSelection.value) {
    const dx = e.clientX - dragStart.value.x
    const dy = e.clientY - dragStart.value.y
    const orig = originalSelection.value
    
    let newX = orig.x
    let newY = orig.y
    let newWidth = orig.width
    let newHeight = orig.height
    
    switch (activeHandle.value) {
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
    emit('selectionChange', selection.value)
  }
}

const onMouseUp = (e: MouseEvent) => {
  if (e.button === 2) return
  
  if (props.mode === 'create' && isSelecting.value) {
    isSelecting.value = false
    if (selection.value && selection.value.width > 20 && selection.value.height > 20) {
      emit('selectionComplete', selection.value)
    } else {
      selection.value = null
    }
    return
  }
  
  if (props.mode === 'adjust' && activeHandle.value) {
    activeHandle.value = null
    originalSelection.value = null
    emit('dragEnd')
  }
}

const onHandleMouseDown = (e: MouseEvent, handle: HandleType) => {
  // 只响应左键
  if (e.button !== 0) return
  
  e.stopPropagation()
  e.preventDefault()
  activeHandle.value = handle
  dragStart.value = { x: e.clientX, y: e.clientY }
  originalSelection.value = { ...selection.value! }
  emit('dragStart')
}

onMounted(() => {
  window.addEventListener('mousemove', onMouseMove)
  window.addEventListener('mouseup', onMouseUp)
})

onUnmounted(() => {
  window.removeEventListener('mousemove', onMouseMove)
  window.removeEventListener('mouseup', onMouseUp)
})
</script>

<template>
  <div 
    class="selection-container" 
    :class="{ 'create-mode': mode === 'create' }"
    @mousedown="onMouseDown"
  >
    <div v-if="selection" class="selection-rect" :style="{
      left: `${selection.x}px`,
      top: `${selection.y}px`,
      width: `${selection.width}px`,
      height: `${selection.height}px`
    }">
      <!-- 调整手柄（仅调整模式） -->
      <div v-if="mode === 'adjust'" class="handles">
        <div class="handle handle-nw" @mousedown="onHandleMouseDown($event, 'nw')"></div>
        <div class="handle handle-n" @mousedown="onHandleMouseDown($event, 'n')"></div>
        <div class="handle handle-ne" @mousedown="onHandleMouseDown($event, 'ne')"></div>
        <div class="handle handle-w" @mousedown="onHandleMouseDown($event, 'w')"></div>
        <div class="handle handle-e" @mousedown="onHandleMouseDown($event, 'e')"></div>
        <div class="handle handle-sw" @mousedown="onHandleMouseDown($event, 'sw')"></div>
        <div class="handle handle-s" @mousedown="onHandleMouseDown($event, 's')"></div>
        <div class="handle handle-se" @mousedown="onHandleMouseDown($event, 'se')"></div>
      </div>
      
      <!-- 尺寸标签 -->
      <span class="size-label">{{ Math.round(selection.width) }} × {{ Math.round(selection.height) }}</span>
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
  z-index: 10;
  pointer-events: none;
}

.selection-container.create-mode {
  cursor: crosshair;
  pointer-events: auto;
}

.selection-rect {
  position: absolute;
  border: 2px solid #00ffff;
  background: transparent;
  box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.3);
  pointer-events: none;
}

.handles {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  pointer-events: none;
}

.handle {
  position: absolute;
  width: 10px;
  height: 10px;
  background: #00ffff;
  border: 1px solid #008888;
  z-index: 10;
  pointer-events: auto;
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
</style>
