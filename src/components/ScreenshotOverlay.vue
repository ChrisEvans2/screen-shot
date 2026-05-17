<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import SelectionArea from './SelectionArea.vue'
import AnnotationCanvas from './AnnotationCanvas.vue'
import Toolbar from './Toolbar.vue'
import { useAnnotationStore } from '../stores/annotation'
import type { Selection } from '../types'

const isVisible = ref(false)
const screenshotData = ref<string>('')
const selection = ref<Selection | null>(null)
const isDragging = ref(false)
const store = useAnnotationStore()

const show = async (base64Data?: string) => {
  if (base64Data) {
    const binaryString = atob(base64Data)
    const bytes = new Uint8Array(binaryString.length)
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i)
    }
    const blob = new Blob([bytes], { type: 'image/png' })
    screenshotData.value = URL.createObjectURL(blob)
  }
  
  isVisible.value = true
  selection.value = null
  isDragging.value = false
  store.clearAnnotations()
}

const hide = async () => {
  isVisible.value = false
  selection.value = null
  isDragging.value = false
  await invoke('hide_window')
}

// 框选完成，进入编辑模式
const onSelectionComplete = (sel: Selection) => {
  selection.value = sel
}

// 选区变化（拖动调整时）
const onSelectionChange = (sel: Selection) => {
  selection.value = sel
}

const onDragStart = () => {
  isDragging.value = true
}

const onDragEnd = () => {
  isDragging.value = false
}

const handleSave = async () => {
  if (!selection.value) {
    console.error('No selection')
    return
  }
  if (!screenshotData.value) {
    console.error('No screenshot data')
    return
  }
  
  const canvas = document.querySelector('.annotation-canvas') as HTMLCanvasElement
  if (!canvas) {
    console.error('No annotation canvas')
    return
  }
  
  const mergeCanvas = document.createElement('canvas')
  mergeCanvas.width = selection.value.width
  mergeCanvas.height = selection.value.height
  const mergeCtx = mergeCanvas.getContext('2d')
  
  if (!mergeCtx) {
    console.error('Failed to get 2d context')
    return
  }
  
  const img = new Image()
  
  await new Promise<void>((resolve, reject) => {
    img.onload = () => {
      mergeCtx.drawImage(
        img,
        selection.value!.x, selection.value!.y,
        selection.value!.width, selection.value!.height,
        0, 0,
        selection.value!.width, selection.value!.height
      )
      resolve()
    }
    img.onerror = (e) => {
      console.error('Image load error:', e)
      reject(e)
    }
    img.src = screenshotData.value
  })
  
  mergeCtx.drawImage(canvas, 0, 0)
  
  const imageData = mergeCanvas.toDataURL('image/png')
  console.log('Image data length:', imageData.length)
  
  try {
    await invoke('save_to_clipboard', { imageData })
    console.log('Clipboard save success')
    await hide()
  } catch (err) {
    console.error('Failed to save to clipboard:', err)
  }
}

const handleCancel = async () => {
  await hide()
}

const onKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Escape') {
    handleCancel()
  }
}

onMounted(() => {
  window.addEventListener('keydown', onKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', onKeyDown)
})

defineExpose({ show, hide })
</script>

<template>
  <div v-if="isVisible" class="overlay">
    <!-- 阶段1：框选阶段（selection 还没有） -->
    <SelectionArea
      v-if="!selection"
      mode="create"
      @selectionComplete="onSelectionComplete"
      @cancel="handleCancel"
    />
    
    <!-- 阶段2：编辑阶段（selection 已创建） -->
    <template v-else>
      <!-- 选区调整层 -->
      <SelectionArea
        mode="adjust"
        :initialSelection="selection"
        @selectionChange="onSelectionChange"
        @dragStart="onDragStart"
        @dragEnd="onDragEnd"
      />
      
      <!-- 标注画布 -->
      <AnnotationCanvas
        :screenshotData="screenshotData"
        :selection="selection"
      />
      
      <!-- 工具栏（拖动时隐藏） -->
      <Toolbar
        v-if="!isDragging"
        @save="handleSave"
        @cancel="handleCancel"
      />
    </template>
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  z-index: 9999;
}
</style>
