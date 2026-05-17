<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useAnnotationStore } from '../stores/annotation'
import type { Selection, Annotation, Point, ToolType } from '../types'

const props = defineProps<{
  screenshotData: string
  selection: Selection
}>()

const emit = defineEmits<{
  (e: 'complete', imageData: string): void
  (e: 'cancel'): void
}>()

const store = useAnnotationStore()
const canvasRef = ref<HTMLCanvasElement | null>(null)
const ctx = ref<CanvasRenderingContext2D | null>(null)
const isDrawing = ref(false)
const currentPoints = ref<Point[]>([])
const currentText = ref('')
const textPosition = ref<Point | null>(null)
const isEditingText = ref(false)

const hslToRgb = (h: number, s: number, l: number): string => {
  s /= 100
  l /= 100
  const c = (1 - Math.abs(2 * l - 1)) * s
  const x = c * (1 - Math.abs((h / 60) % 2 - 1))
  const m = l - c / 2
  let r = 0, g = 0, b = 0
  if (h < 60) { r = c; g = x; b = 0 }
  else if (h < 120) { r = x; g = c; b = 0 }
  else if (h < 180) { r = 0; g = c; b = x }
  else if (h < 240) { r = 0; g = x; b = c }
  else if (h < 300) { r = x; g = 0; b = c }
  else { r = c; g = 0; b = x }
  return `rgb(${Math.round((r + m) * 255)}, ${Math.round((g + m) * 255)}, ${Math.round((b + m) * 255)})`
}

const generateId = () => Math.random().toString(36).substr(2, 9)

const drawAnnotation = (annotation: Annotation) => {
  if (!ctx.value) return
  
  const color = hslToRgb(annotation.color.h, annotation.color.s, annotation.color.l)
  ctx.value.strokeStyle = color
  ctx.value.fillStyle = color
  ctx.value.lineWidth = annotation.lineWidth
  ctx.value.lineCap = 'round'
  ctx.value.lineJoin = 'round'
  
  switch (annotation.type) {
    case 'rectangle':
      if (annotation.points.length >= 2) {
        const [start, end] = annotation.points
        ctx.value.strokeRect(start.x, start.y, end.x - start.x, end.y - start.y)
      }
      break
      
    case 'ellipse':
      if (annotation.points.length >= 2) {
        const [start, end] = annotation.points
        const centerX = (start.x + end.x) / 2
        const centerY = (start.y + end.y) / 2
        const radiusX = Math.abs(end.x - start.x) / 2
        const radiusY = Math.abs(end.y - start.y) / 2
        ctx.value.beginPath()
        ctx.value.ellipse(centerX, centerY, radiusX, radiusY, 0, 0, Math.PI * 2)
        ctx.value.stroke()
      }
      break
      
    case 'line':
      if (annotation.points.length >= 2) {
        const [start, end] = annotation.points
        ctx.value.beginPath()
        ctx.value.moveTo(start.x, start.y)
        ctx.value.lineTo(end.x, end.y)
        ctx.value.stroke()
      }
      break
      
    case 'arrow':
      if (annotation.points.length >= 2) {
        const [start, end] = annotation.points
        const angle = Math.atan2(end.y - start.y, end.x - start.x)
        const headLength = 15
        
        ctx.value.beginPath()
        ctx.value.moveTo(start.x, start.y)
        ctx.value.lineTo(end.x, end.y)
        ctx.value.stroke()
        
        ctx.value.beginPath()
        ctx.value.moveTo(end.x, end.y)
        ctx.value.lineTo(
          end.x - headLength * Math.cos(angle - Math.PI / 6),
          end.y - headLength * Math.sin(angle - Math.PI / 6)
        )
        ctx.value.moveTo(end.x, end.y)
        ctx.value.lineTo(
          end.x - headLength * Math.cos(angle + Math.PI / 6),
          end.y - headLength * Math.sin(angle + Math.PI / 6)
        )
        ctx.value.stroke()
      }
      break
      
    case 'pencil':
      if (annotation.points.length >= 2) {
        ctx.value.beginPath()
        ctx.value.moveTo(annotation.points[0].x, annotation.points[0].y)
        for (let i = 1; i < annotation.points.length; i++) {
          ctx.value.lineTo(annotation.points[i].x, annotation.points[i].y)
        }
        ctx.value.stroke()
      }
      break
      
    case 'text':
      if (annotation.text && annotation.points.length >= 1) {
        ctx.value.font = `${annotation.fontSize || 24}px sans-serif`
        ctx.value.fillText(annotation.text, annotation.points[0].x, annotation.points[0].y)
      }
      break
      
    case 'mosaic':
      if (annotation.points.length >= 2 && ctx.value) {
        const blockSize = annotation.mosaicBlockSize || store.mosaicOptions.value.blockSize
        const radius = blockSize * 2
        
        for (const point of annotation.points) {
          const x = Math.max(0, point.x - radius)
          const y = Math.max(0, point.y - radius)
          const width = Math.min(radius * 2, props.selection.width - x)
          const height = Math.min(radius * 2, props.selection.height - y)
          
          if (width <= 0 || height <= 0) continue
          
          const imageData = ctx.value.getImageData(x, y, width, height)
          const data = imageData.data
          
          for (let bx = 0; bx < width; bx += blockSize) {
            for (let by = 0; by < height; by += blockSize) {
              let r = 0, g = 0, b = 0, count = 0
              
              for (let dx = 0; dx < blockSize && bx + dx < width; dx++) {
                for (let dy = 0; dy < blockSize && by + dy < height; dy++) {
                  const idx = ((by + dy) * width + (bx + dx)) * 4
                  r += data[idx]
                  g += data[idx + 1]
                  b += data[idx + 2]
                  count++
                }
              }
              
              r = Math.round(r / count)
              g = Math.round(g / count)
              b = Math.round(b / count)
              
              ctx.value.fillStyle = `rgb(${r}, ${g}, ${b})`
              ctx.value.fillRect(x + bx, y + by, 
                Math.min(blockSize, width - bx), 
                Math.min(blockSize, height - by))
            }
          }
        }
      }
      break
  }
}

let backgroundImage: HTMLImageElement | null = null

const redraw = () => {
  if (!ctx.value || !canvasRef.value) return
  
  // 更新 canvas 大小
  canvasRef.value.width = props.selection.width
  canvasRef.value.height = props.selection.height
  
  ctx.value.clearRect(0, 0, props.selection.width, props.selection.height)
  
  if (backgroundImage) {
    ctx.value.drawImage(
      backgroundImage,
      props.selection.x, props.selection.y,
      props.selection.width, props.selection.height,
      0, 0,
      props.selection.width, props.selection.height
    )
  }
  
  store.annotations.value.forEach(drawAnnotation)
}

watch(() => store.annotations.value, redraw, { deep: true })
watch(() => props.selection, redraw, { deep: true })

const getCanvasPoint = (e: MouseEvent): Point => {
  const rect = canvasRef.value!.getBoundingClientRect()
  return {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top
  }
}

const onMouseDown = (e: MouseEvent) => {
  if (e.button !== 0) return
  
  const point = getCanvasPoint(e)
  
  if (store.currentTool.value === 'text') {
    textPosition.value = point
    isEditingText.value = true
    currentText.value = ''
    return
  }
  
  isDrawing.value = true
  currentPoints.value = [point]
}

const onMouseMove = (e: MouseEvent) => {
  if (!isDrawing.value) return
  
  const point = getCanvasPoint(e)
  currentPoints.value.push(point)
  
  if (store.currentTool.value === 'pencil' || store.currentTool.value === 'mosaic') {
    redraw()
    if (ctx.value && currentPoints.value.length >= 2) {
      if (store.currentTool.value === 'pencil') {
        const color = hslToRgb(store.currentColor.value.h, store.currentColor.value.s, store.currentColor.value.l)
        ctx.value.strokeStyle = color
        ctx.value.lineWidth = store.currentLineWidth.value
        ctx.value.lineCap = 'round'
        ctx.value.lineJoin = 'round'
        ctx.value.beginPath()
        ctx.value.moveTo(currentPoints.value[0].x, currentPoints.value[0].y)
        for (let i = 1; i < currentPoints.value.length; i++) {
          ctx.value.lineTo(currentPoints.value[i].x, currentPoints.value[i].y)
        }
        ctx.value.stroke()
      } else {
        const tempAnnotation: Annotation = {
          id: 'temp',
          type: 'mosaic',
          points: [...currentPoints.value],
          color: store.currentColor.value,
          lineWidth: store.currentLineWidth.value,
          mosaicBlockSize: store.mosaicOptions.value.blockSize
        }
        drawAnnotation(tempAnnotation)
      }
    }
  } else {
    redraw()
    const tempAnnotation: Annotation = {
      id: 'temp',
      type: store.currentTool.value,
      points: [currentPoints.value[0], point],
      color: store.currentColor.value,
      lineWidth: store.currentLineWidth.value
    }
    drawAnnotation(tempAnnotation)
  }
}

const onMouseUp = (e: MouseEvent) => {
  if (!isDrawing.value) return
  isDrawing.value = false
  
  if (currentPoints.value.length < 2) return
  
  const annotation: Annotation = {
    id: generateId(),
    type: store.currentTool.value,
    points: (store.currentTool.value === 'pencil' || store.currentTool.value === 'mosaic') ? [...currentPoints.value] : [currentPoints.value[0], currentPoints.value[currentPoints.value.length - 1]],
    color: store.currentColor.value,
    lineWidth: store.currentLineWidth.value,
    mosaicBlockSize: store.currentTool.value === 'mosaic' ? store.mosaicOptions.value.blockSize : undefined
  }
  
  store.addAnnotation(annotation)
  currentPoints.value = []
}

const onMouseLeave = () => {
  if (isDrawing.value) {
    isDrawing.value = false
    if (currentPoints.value.length >= 2) {
      const annotation: Annotation = {
        id: generateId(),
        type: store.currentTool.value,
        points: (store.currentTool.value === 'pencil' || store.currentTool.value === 'mosaic') ? [...currentPoints.value] : [currentPoints.value[0], currentPoints.value[currentPoints.value.length - 1]],
        color: store.currentColor.value,
        lineWidth: store.currentLineWidth.value,
        mosaicBlockSize: store.currentTool.value === 'mosaic' ? store.mosaicOptions.value.blockSize : undefined
      }
      store.addAnnotation(annotation)
    }
    currentPoints.value = []
  }
}

const onTextSubmit = () => {
  if (!textPosition.value || !currentText.value.trim()) {
    isEditingText.value = false
    return
  }
  
  const annotation: Annotation = {
    id: generateId(),
    type: 'text',
    points: [textPosition.value],
    color: store.currentColor.value,
    lineWidth: store.currentLineWidth.value,
    text: currentText.value,
    fontSize: store.currentFontSize.value
  }
  
  store.addAnnotation(annotation)
  isEditingText.value = false
  currentText.value = ''
  textPosition.value = null
}

const onTextKeydown = (e: KeyboardEvent) => {
  if (e.key === 'Enter') {
    onTextSubmit()
  } else if (e.key === 'Escape') {
    isEditingText.value = false
    currentText.value = ''
    textPosition.value = null
  }
}

onMounted(() => {
  if (canvasRef.value) {
    canvasRef.value.width = props.selection.width
    canvasRef.value.height = props.selection.height
    ctx.value = canvasRef.value.getContext('2d')
    
    if (props.screenshotData) {
      const img = new Image()
      img.onload = () => {
        backgroundImage = img
        if (ctx.value) {
          ctx.value.drawImage(
            img,
            props.selection.x, props.selection.y,
            props.selection.width, props.selection.height,
            0, 0,
            props.selection.width, props.selection.height
          )
        }
      }
      img.src = props.screenshotData
    }
  }
})
</script>

<template>
  <canvas
    ref="canvasRef"
    class="annotation-canvas"
    :class="{ 'select-mode': store.currentTool.value === 'select' }"
    :style="{
      left: `${selection.x}px`,
      top: `${selection.y}px`,
    }"
    @mousedown="onMouseDown"
    @mousemove="onMouseMove"
    @mouseup="onMouseUp"
    @mouseleave="onMouseLeave"
  />
  <div
    v-if="isEditingText && textPosition"
    class="text-input-container"
    :style="{
      left: `${selection.x + textPosition.x}px`,
      top: `${selection.y + textPosition.y}px`,
    }"
  >
    <input
      ref="textInput"
      v-model="currentText"
      class="text-input"
      :style="{
        fontSize: `${store.currentFontSize.value}px`,
        color: hslToRgb(store.currentColor.value.h, store.currentColor.value.s, store.currentColor.value.l),
      }"
      placeholder="输入文字..."
      @keydown="onTextKeydown"
      @blur="onTextSubmit"
      autofocus
    />
  </div>
</template>

<style scoped>
.annotation-canvas {
  position: absolute;
  cursor: crosshair;
  z-index: 20;
}

.annotation-canvas.select-mode {
  pointer-events: none;
  cursor: default;
}

.text-input-container {
  position: absolute;
  z-index: 10000;
}

.text-input {
  background: transparent;
  border: 1px dashed rgba(255, 255, 255, 0.5);
  outline: none;
  min-width: 100px;
  padding: 2px 4px;
}
</style>
