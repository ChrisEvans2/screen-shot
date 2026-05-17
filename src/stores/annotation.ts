import { ref, computed } from 'vue'
import type { Annotation, ToolType, HSLColor, MosaicOptions } from '../types'

const annotations = ref<Annotation[]>([])
const currentTool = ref<ToolType>('select')
const currentColor = ref<HSLColor>({ h: 210, s: 80, l: 60 })
const currentLineWidth = ref(2)
const currentFontSize = ref(24)
const mosaicOptions = ref<MosaicOptions>({ blockSize: 10 })

const undoStack = ref<Annotation[]>([])
const redoStack = ref<Annotation[]>([])

export function useAnnotationStore() {
  const addAnnotation = (annotation: Annotation) => {
    annotations.value.push(annotation)
    undoStack.value.push(annotation)
    redoStack.value = []
  }

  const undo = () => {
    if (undoStack.value.length > 0) {
      const annotation = undoStack.value.pop()!
      redoStack.value.push(annotation)
      annotations.value = annotations.value.filter(a => a.id !== annotation.id)
    }
  }

  const redo = () => {
    if (redoStack.value.length > 0) {
      const annotation = redoStack.value.pop()!
      undoStack.value.push(annotation)
      annotations.value.push(annotation)
    }
  }

  const clearAnnotations = () => {
    annotations.value = []
    undoStack.value = []
    redoStack.value = []
  }

  return {
    annotations,
    currentTool,
    currentColor,
    currentLineWidth,
    currentFontSize,
    mosaicOptions,
    addAnnotation,
    undo,
    redo,
    clearAnnotations,
    canUndo: computed(() => undoStack.value.length > 0),
    canRedo: computed(() => redoStack.value.length > 0),
  }
}
