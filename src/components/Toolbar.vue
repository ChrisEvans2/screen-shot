<script setup lang="ts">
import { computed } from 'vue'
import { useAnnotationStore } from '../stores/annotation'
import type { ToolType } from '../types'

const emit = defineEmits<{
  (e: 'save'): void
  (e: 'cancel'): void
}>()

const store = useAnnotationStore()

const tools: { type: ToolType; label: string; icon: string }[] = [
  { type: 'rectangle', label: '矩形', icon: '□' },
  { type: 'ellipse', label: '椭圆', icon: '○' },
  { type: 'arrow', label: '箭头', icon: '→' },
  { type: 'line', label: '线段', icon: '─' },
  { type: 'pencil', label: '铅笔', icon: '✎' },
  { type: 'mosaic', label: '马赛克', icon: '▦' },
  { type: 'text', label: '文字', icon: 'T' },
]

const fontSizes = [12, 16, 20, 24, 32, 48, 72]

const selectTool = (tool: ToolType) => {
  // 再次点击已选工具，取消选择，回到空状态
  if (store.currentTool.value === tool) {
    store.currentTool.value = 'select'
  } else {
    store.currentTool.value = tool
  }
}

const onColorChange = (channel: 'h' | 's' | 'l', value: number) => {
  store.currentColor.value = { ...store.currentColor.value, [channel]: value }
}

const hslToString = (h: number, s: number, l: number) => `hsl(${h}, ${s}%, ${l}%)`

const hueGradient = computed(() => {
  return `linear-gradient(to right, 
    hsl(0, ${store.currentColor.value.s}%, ${store.currentColor.value.l}%), 
    hsl(60, ${store.currentColor.value.s}%, ${store.currentColor.value.l}%), 
    hsl(120, ${store.currentColor.value.s}%, ${store.currentColor.value.l}%), 
    hsl(180, ${store.currentColor.value.s}%, ${store.currentColor.value.l}%), 
    hsl(240, ${store.currentColor.value.s}%, ${store.currentColor.value.l}%), 
    hsl(300, ${store.currentColor.value.s}%, ${store.currentColor.value.l}%), 
    hsl(360, ${store.currentColor.value.s}%, ${store.currentColor.value.l}%)
  )`
})

const satGradient = computed(() => {
  return `linear-gradient(to right, 
    hsl(${store.currentColor.value.h}, 0%, ${store.currentColor.value.l}%), 
    hsl(${store.currentColor.value.h}, 100%, ${store.currentColor.value.l}%)
  )`
})

const lightGradient = computed(() => {
  return `linear-gradient(to right, 
    hsl(${store.currentColor.value.h}, ${store.currentColor.value.s}%, 0%), 
    hsl(${store.currentColor.value.h}, ${store.currentColor.value.s}%, 50%), 
    hsl(${store.currentColor.value.h}, ${store.currentColor.value.s}%, 100%)
  )`
})
</script>

<template>
  <div class="toolbar">
    <div class="tool-group">
      <button
        v-for="tool in tools"
        :key="tool.type"
        :class="['tool-btn', { active: store.currentTool.value === tool.type }]"
        :title="tool.label"
        @click="selectTool(tool.type)"
      >
        <span class="tool-icon">{{ tool.icon }}</span>
      </button>
    </div>

    <div class="divider"></div>

    <div class="color-section">
      <div class="color-preview" :style="{ background: hslToString(store.currentColor.value.h, store.currentColor.value.s, store.currentColor.value.l) }"></div>
      <div class="color-sliders">
        <label>
          <span class="slider-label">H</span>
          <input type="range" min="0" max="360" :value="store.currentColor.value.h" @input="onColorChange('h', +($event.target as HTMLInputElement).value)" class="hue-slider" :style="{ background: hueGradient }" />
        </label>
        <label>
          <span class="slider-label">S</span>
          <input type="range" min="0" max="100" :value="store.currentColor.value.s" @input="onColorChange('s', +($event.target as HTMLInputElement).value)" class="sat-slider" :style="{ background: satGradient }" />
        </label>
        <label>
          <span class="slider-label">L</span>
          <input type="range" min="0" max="100" :value="store.currentColor.value.l" @input="onColorChange('l', +($event.target as HTMLInputElement).value)" class="light-slider" :style="{ background: lightGradient }" />
        </label>
      </div>
    </div>

    <div class="divider"></div>

    <div class="size-section" v-if="store.currentTool.value !== 'text' && store.currentTool.value !== 'mosaic'">
      <span class="size-label">{{ store.currentLineWidth.value }}px</span>
      <input type="range" min="1" max="10" v-model.number="store.currentLineWidth.value" />
    </div>

    <div class="size-section" v-if="store.currentTool.value === 'mosaic'">
      <span class="size-label">马赛克: {{ store.mosaicOptions.value.blockSize }}px</span>
      <input type="range" min="5" max="30" v-model.number="store.mosaicOptions.value.blockSize" />
    </div>

    <div class="size-section" v-if="store.currentTool.value === 'text'">
      <select v-model.number="store.currentFontSize.value" class="font-select">
        <option v-for="size in fontSizes" :key="size" :value="size">{{ size }}px</option>
      </select>
    </div>

    <div class="divider"></div>

    <div class="actions">
      <button class="action-btn" :disabled="!store.canUndo.value" @click="store.undo()" title="撤销">
        ↩
      </button>
      <button class="action-btn" :disabled="!store.canRedo.value" @click="store.redo()" title="重做">
        ↪
      </button>
    </div>

    <div class="divider"></div>

    <div class="primary-actions">
      <button class="cancel-btn" @click="emit('cancel')">取消</button>
      <button class="save-btn" @click="emit('save')">保存</button>
    </div>
  </div>
</template>

<style scoped>
.toolbar {
  position: fixed;
  bottom: 24px;
  left: 50%;
  transform: translateX(-50%);
  background: #1a1a1a;
  border: 1px solid #333;
  border-radius: 12px;
  padding: 8px 12px;
  display: flex;
  align-items: center;
  gap: 8px;
  z-index: 10001;
  color: #e0e0e0;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(10px);
}

.divider {
  width: 1px;
  height: 24px;
  background: #333;
  margin: 0 4px;
}

.tool-group {
  display: flex;
  gap: 2px;
}

.tool-btn {
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  color: #999;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.tool-btn:hover {
  background: #2a2a2a;
  color: #fff;
}

.tool-btn.active {
  background: #4a9eff;
  color: #fff;
  box-shadow: 0 0 0 2px #4a9eff;
}

.tool-icon {
  font-size: 16px;
}

.color-section {
  display: flex;
  align-items: center;
  gap: 8px;
}

.color-preview {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: 1px solid #444;
}

.color-sliders {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.color-sliders label {
  display: flex;
  align-items: center;
  gap: 4px;
  font-size: 10px;
}

.slider-label {
  width: 12px;
  color: #666;
  font-size: 10px;
}

.color-sliders input[type="range"] {
  width: 48px;
  height: 8px;
  -webkit-appearance: none;
  border-radius: 4px;
  outline: none;
}

.color-sliders input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 12px;
  height: 12px;
  background: #fff;
  border-radius: 50%;
  cursor: pointer;
  border: 1px solid #666;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
}

.size-section {
  display: flex;
  align-items: center;
  gap: 6px;
}

.size-label {
  font-size: 11px;
  color: #888;
  min-width: 24px;
}

.size-section input[type="range"] {
  width: 48px;
  height: 8px;
  -webkit-appearance: none;
  background: #333;
  border-radius: 4px;
  outline: none;
}

.size-section input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 10px;
  height: 10px;
  background: #fff;
  border-radius: 50%;
  cursor: pointer;
}

.font-select {
  background: #2a2a2a;
  color: #e0e0e0;
  border: 1px solid #444;
  border-radius: 6px;
  padding: 4px 8px;
  font-size: 12px;
  cursor: pointer;
}

.actions {
  display: flex;
  gap: 2px;
}

.action-btn {
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  color: #999;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.15s ease;
}

.action-btn:hover:not(:disabled) {
  background: #2a2a2a;
  color: #fff;
}

.action-btn:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

.primary-actions {
  display: flex;
  gap: 6px;
}

.cancel-btn,
.save-btn {
  padding: 6px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all 0.15s ease;
}

.cancel-btn {
  background: transparent;
  color: #999;
}

.cancel-btn:hover {
  background: #2a2a2a;
  color: #fff;
}

.save-btn {
  background: #fff;
  color: #000;
}

.save-btn:hover {
  background: #e0e0e0;
}
</style>
