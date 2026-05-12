<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface Selection {
  x: number
  y: number
  width: number
  height: number
}

const isSelecting = ref(false)
const startPoint = ref({ x: 0, y: 0 })
const endPoint = ref({ x: 0, y: 0 })
const selection = ref<Selection | null>(null)

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
  isSelecting.value = true
  startPoint.value = { x: e.clientX, y: e.clientY }
  endPoint.value = { x: e.clientX, y: e.clientY }
}

const onMouseMove = (e: MouseEvent) => {
  if (isSelecting.value) {
    endPoint.value = { x: e.clientX, y: e.clientY }
    selection.value = getSelection()
  }
}

const onMouseUp = (e: MouseEvent) => {
  if (e.button === 2) return
  isSelecting.value = false
  if (selection.value && selection.value.width > 10 && selection.value.height > 10) {
    emit('selectionComplete', selection.value)
  }
}

const onKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    emit('cancel')
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
      <span class="size-label">{{ selection.width }} × {{ selection.height }}</span>
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
}

.selection-rect {
  position: absolute;
  border: 2px dashed #00ff00;
  background: transparent;
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
}
</style>
