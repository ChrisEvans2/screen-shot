# 截图工具实现计划

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建一款轻量级跨平台桌面截图工具，支持热键触发截图、多种标注工具、复制到剪切板功能。

**Architecture:** 单窗口全屏透明方案，Rust 后端处理热键、截图、剪切板，Vue 3 前端处理框选和标注交互。

**Tech Stack:** Tauri, Rust, Vue 3, TypeScript, HTML5 Canvas, Vite

---

## Chunk 1: 项目初始化与基础架构

### Task 1: 初始化 Tauri + Vue 3 项目

**Files:**
- Create: `package.json`
- Create: `vite.config.ts`
- Create: `tsconfig.json`
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/src/main.rs`
- Create: `src/main.ts`
- Create: `src/App.vue`
- Create: `src/index.html`

- [ ] **Step 1: 创建 package.json**

```json
{
  "name": "screen-shot",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vue-tsc --noEmit && vite build",
    "preview": "vite preview",
    "tauri": "tauri"
  },
  "dependencies": {
    "vue": "^3.4.0",
    "@tauri-apps/api": "^1.5.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^1.5.0",
    "@vitejs/plugin-vue": "^5.0.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0",
    "vue-tsc": "^1.8.0"
  }
}
```

- [ ] **Step 2: 创建 vite.config.ts**

```typescript
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
})
```

- [ ] **Step 3: 创建 tsconfig.json**

```json
{
  "compilerOptions": {
    "target": "ES2021",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2021", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src/**/*.ts", "src/**/*.tsx", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

- [ ] **Step 4: 创建 src-tauri/Cargo.toml**

```toml
[package]
name = "screen-shot"
version = "0.1.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "1.5", features = [] }

[dependencies]
tauri = { version = "1.5", features = ["clipboard-write-text", "global-shortcut", "shell-open", "system-tray"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
screenshots = "0.8"
image = "0.24"
```

- [ ] **Step 5: 创建 src-tauri/tauri.conf.json**

```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "distDir": "../dist"
  },
  "package": {
    "productName": "ScreenShot",
    "version": "0.1.0"
  },
  "tauri": {
    "allowlist": {
      "clipboard": {
        "writeImage": true
      },
      "globalShortcut": {
        "all": true
      },
      "shell": {
        "all": true,
        "open": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all",
      "identifier": "com.screen-shot.app",
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    },
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "transparent": true,
        "decorations": false,
        "alwaysOnTop": true,
        "skipTaskbar": true,
        "visible": false,
        "width": 1920,
        "height": 1080
      }
    ],
    "systemTray": {
      "iconPath": "icons/icon.png",
      "iconAsTemplate": true
    }
  }
}
```

- [ ] **Step 6: 创建 src-tauri/src/main.rs**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[tauri::command]
fn capture_screen() -> Result<String, String> {
    // TODO: 实现截图功能
    Ok("".to_string())
}

#[tauri::command]
fn save_to_clipboard(image_data: String) -> Result<(), String> {
    // TODO: 保存到剪切板
    Ok(())
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let screenshot = CustomMenuItem::new("screenshot".to_string(), "截图");
    let tray_menu = SystemTrayMenu::new()
        .add_item(screenshot)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(quit);
    
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    match id.as_str() {
                        "quit" => {
                            std::process::exit(0);
                        }
                        "screenshot" => {
                            let window = app.get_window("main").unwrap();
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![capture_screen, save_to_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 7: 创建 src/main.ts**

```typescript
import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')
```

- [ ] **Step 8: 创建 src/App.vue**

```vue
<script setup lang="ts">
import { ref } from 'vue'

const isVisible = ref(false)
</script>

<template>
  <div v-if="isVisible" class="screenshot-overlay">
    <h1>截图工具</h1>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  overflow: hidden;
}

.screenshot-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.5);
  z-index: 9999;
}
</style>
```

- [ ] **Step 9: 创建 src/index.html**

```html
<!DOCTYPE html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>截图工具</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

- [ ] **Step 10: 安装依赖并验证项目结构**

Run: `npm install`
Expected: 依赖安装成功

- [ ] **Step 11: 提交**

```bash
git add .
git commit -m "feat: initialize tauri + vue3 project structure"
```

---

## Chunk 2: Rust 后端核心功能

### Task 2: 实现屏幕截图功能

**Files:**
- Modify: `src-tauri/src/main.rs`
- Create: `src-tauri/src/screenshot.rs`

- [ ] **Step 1: 创建 screenshot.rs**

```rust
use screenshots::Screen;
use image::{ImageBuffer, Rgba};

pub fn capture_full_screen() -> Result<Vec<u8>, String> {
    let screens = Screen::all().map_err(|e| e.to_string())?;
    
    if let Some(screen) = screens.first() {
        let image = screen.capture().map_err(|e| e.to_string())?;
        let png_data = image.to_png().map_err(|e| e.to_string())?;
        Ok(png_data)
    } else {
        Err("No screen found".to_string())
    }
}
```

- [ ] **Step 2: 修改 main.rs 导入截图模块**

```rust
mod screenshot;

#[tauri::command]
fn capture_screen() -> Result<Vec<u8>, String> {
    screenshot::capture_full_screen()
}
```

- [ ] **Step 3: 验证编译**

Run: `cd src-tauri && cargo build`
Expected: 编译成功

- [ ] **Step 4: 提交**

```bash
git add src-tauri/src/screenshot.rs src-tauri/src/main.rs
git commit -m "feat: implement screen capture functionality"
```

### Task 3: 实现剪切板写入功能

**Files:**
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: 修改 save_to_clipboard 命令**

```rust
#[tauri::command]
fn save_to_clipboard(image_data: Vec<u8>) -> Result<(), String> {
    use std::io::Write;
    use std::process::Command;
    
    // 保存临时文件
    let temp_path = std::env::temp_dir().join("screenshot_temp.png");
    let mut file = std::fs::File::create(&temp_path).map_err(|e| e.to_string())?;
    file.write_all(&image_data).map_err(|e| e.to_string())?;
    
    // 根据平台选择剪切板命令
    #[cfg(target_os = "windows")]
    {
        Command::new("powershell")
            .args(&[
                "-command",
                &format!(
                    "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.Clipboard]::SetImage([System.Drawing.Image]::FromFile('{}'))",
                    temp_path.display()
                ),
            ])
            .output()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("osascript")
            .args(&[
                "-e",
                &format!(
                    "set the clipboard to (read (POSIX file \"{}\") as JPEG picture)",
                    temp_path.display()
                ),
            ])
            .output()
            .map_err(|e| e.to_string())?;
    }
    
    // 清理临时文件
    let _ = std::fs::remove_file(temp_path);
    
    Ok(())
}
```

- [ ] **Step 2: 验证编译**

Run: `cd src-tauri && cargo build`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add src-tauri/src/main.rs
git commit -m "feat: implement clipboard write functionality"
```

### Task 4: 实现全局热键监听

**Files:**
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: 添加热键注册逻辑**

```rust
use tauri::GlobalShortcutManager;

fn main() {
    // ... 现有代码 ...
    
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let window_clone = window.clone();
            
            // 注册全局热键
            let mut shortcut = app.global_shortcut();
            shortcut
                .register("CommandOrControl+Shift+A", move || {
                    window_clone.show().unwrap();
                    window_clone.set_focus().unwrap();
                })
                .expect("failed to register hotkey");
            
            Ok(())
        })
        // ... 其余代码 ...
}
```

- [ ] **Step 2: 验证编译**

Run: `cd src-tauri && cargo build`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add src-tauri/src/main.rs
git commit -m "feat: implement global hotkey listener"
```

---

## Chunk 3: Vue 前端框选功能

### Task 5: 实现全屏覆盖层和框选功能

**Files:**
- Create: `src/components/ScreenshotOverlay.vue`
- Create: `src/components/SelectionArea.vue`
- Modify: `src/App.vue`

- [ ] **Step 1: 创建 ScreenshotOverlay.vue**

```vue
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import SelectionArea from './SelectionArea.vue'

const isVisible = ref(false)
const screenshotData = ref<string>('')

const show = () => {
  isVisible.value.value = true
}

const hide = () => {
  isVisible.value = false
}

defineExpose({ show, hide })
</script>

<template>
  <div v-if="isVisible" class="overlay">
    <SelectionArea />
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
  cursor: crosshair;
}
</style>
```

- [ ] **Step 2: 创建 SelectionArea.vue**

```vue
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

const getSelection = (): Selection => {
  const x = Math.min(startPoint.value.x, endPoint.value.x)
  const y = Math.min(startPoint.value.y, endPoint.value.y)
  const width = Math.abs(endPoint.value.x - startPoint.value.x)
  const height = Math.abs(endPoint.value.y - startPoint.value.y)
  return { x, y, width, height }
}

const onMouseDown = (e: MouseEvent) => {
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

const onMouseUp = () => {
  isSelecting.value = false
  if (selection.value && selection.value.width > 10 && selection.value.height > 10) {
    // 进入标注模式
    emit('selectionComplete', selection.value)
  }
}

const emit = defineEmits<{
  (e: 'selectionComplete', selection: Selection): void
}>()

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
```

- [ ] **Step 3: 修改 App.vue 集成组件**

```vue
<script setup lang="ts">
import { ref } from 'vue'
import ScreenshotOverlay from './components/ScreenshotOverlay.vue'

const overlayRef = ref<InstanceType<typeof ScreenshotOverlay> | null>(null)

// 监听 Tauri 事件
import { listen } from '@tauri-apps/api/event'

listen('show-screenshot', () => {
  overlayRef.value?.show()
})
</script>

<template>
  <ScreenshotOverlay ref="overlayRef" />
</template>
```

- [ ] **Step 4: 验证开发服务器**

Run: `npm run dev`
Expected: 浏览器中可以看到组件结构

- [ ] **Step 5: 提交**

```bash
git add src/components/ScreenshotOverlay.vue src/components/SelectionArea.vue src/App.vue
git commit -m "feat: implement screenshot overlay and selection area"
```

---

## Chunk 4: 标注工具实现

### Task 6: 实现 Canvas 标注画布

**Files:**
- Create: `src/components/AnnotationCanvas.vue`
- Create: `src/stores/annotation.ts`
- Create: `src/types/index.ts`

- [ ] **Step 1: 创建 types/index.ts**

```typescript
export interface Point {
  x: number
  y: number
}

export interface Selection {
  x: number
  y: number
  width: number
  height: number
}

export type ToolType = 
  | 'rectangle'
  | 'ellipse'
  | 'arrow'
  | 'line'
  | 'pencil'
  | 'mosaic'
  | 'image'
  | 'text'

export interface Annotation {
  id: string
  type: ToolType
  points: Point[]
  color: HSLColor
  lineWidth: number
  text?: string
  fontSize?: number
  imageData?: string
}

export interface HSLColor {
  h: number
  s: number
  l: number
}

export interface MosaicOptions {
  blockSize: number
}
```

- [ ] **Step 2: 创建 stores/annotation.ts**

```typescript
import { ref, computed } from 'vue'
import type { Annotation, ToolType, HSLColor, MosaicOptions } from '../types'

const annotations = ref<Annotation[]>([])
const currentTool = ref<ToolType>('rectangle')
const currentColor = ref<HSLColor>({ h: 0, s: 100, l: 50 })
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
```

- [ ] **Step 3: 创建 AnnotationCanvas.vue**

```vue
<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useAnnotationStore } from '../stores/annotation'
import type { Selection, Annotation, Point } from '../types'

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

const hslToRgb = (h: number, s: number, l: number): string => {
  s /= 100
  l /= 100
  const a = s * Math.min(l, 1 - l)
  const f = (n: number) => {
    const k = (n + h / 30) % 12
    return l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1)
  }
  return `rgb(${Math.round(f(0) * 255)}, ${Math.round(f(8) * 255)}, ${Math.round(f(4) * 255)})`
}

const drawAnnotation = (annotation: Annotation) => {
  if (!ctx.value) return
  
  ctx.value.strokeStyle = hslToRgb(annotation.color.h, annotation.color.s, annotation.color.l)
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
  }
}

const redraw = () => {
  if (!ctx.value) return
  
  ctx.value.clearRect(0, 0, props.selection.width, props.selection.height)
  
  store.annotations.value.forEach(drawAnnotation)
}

watch(() => store.annotations.value, redraw, { deep: true })

onMounted(() => {
  if (canvasRef.value) {
    canvasRef.value.width = props.selection.width
    canvasRef.value.height = props.selection.height
    ctx.value = canvasRef.value.getContext('2d')
  }
})
</script>

<template>
  <canvas
    ref="canvasRef"
    class="annotation-canvas"
    :style="{
      left: `${selection.x}px`,
      top: `${selection.y}px`,
    }"
  />
</template>

<style scoped>
.annotation-canvas {
  position: absolute;
  cursor: crosshair;
}
</style>
```

- [ ] **Step 4: 验证编译**

Run: `npm run build`
Expected: TypeScript 编译成功

- [ ] **Step 5: 提交**

```bash
git add src/components/AnnotationCanvas.vue src/stores/annotation.ts src/types/index.ts
git commit -m "feat: implement annotation canvas and store"
```

### Task 7: 实现工具栏组件

**Files:**
- Create: `src/components/Toolbar.vue`
- Create: `src/components/tools/ColorPicker.vue`
- Create: `src/components/tools/LineWidthSlider.vue`

- [ ] **Step 1: 创建 ColorPicker.vue (HSL 模块)**

```vue
<script setup lang="ts">
import { ref, computed } from 'vue'
import type { HSLColor } from '../../types'

const props = defineProps<{
  modelValue: HSLColor
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: HSLColor): void
}>()

const localColor = ref({ ...props.modelValue })

const updateColor = () => {
  emit('update:modelValue', { ...localColor.value })
}
</script>

<template>
  <div class="color-picker">
    <div class="slider-group">
      <label>H: {{ localColor.h }}°</label>
      <input
        type="range"
        v-model.number="localColor.h"
        min="0"
        max="360"
        @input="updateColor"
      />
    </div>
    <div class="slider-group">
      <label>S: {{ localColor.s }}%</label>
      <input
        type="range"
        v-model.number="localColor.s"
        min="0"
        max="100"
        @input="updateColor"
      />
    </div>
    <div class="slider-group">
      <label>L: {{ localColor.l }}%</label>
      <input
        type="range"
        v-model.number="localColor.l"
        min="0"
        max="100"
        @input="updateColor"
      />
    </div>
    <div
      class="color-preview"
      :style="{
        backgroundColor: `hsl(${localColor.h}, ${localColor.s}%, ${localColor.l}%)`
      }"
    />
  </div>
</template>

<style scoped>
.color-picker {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 8px;
  background: rgba(0, 0, 0, 0.8);
  border-radius: 8px;
}

.slider-group {
  display: flex;
  align-items: center;
  gap: 8px;
}

.slider-group label {
  width: 60px;
  font-size: 12px;
  color: white;
}

.slider-group input[type="range"] {
  flex: 1;
  height: 4px;
}

.color-preview {
  width: 100%;
  height: 24px;
  border-radius: 4px;
  border: 1px solid white;
}
</style>
```

- [ ] **Step 2: 创建 LineWidthSlider.vue**

```vue
<script setup lang="ts">
const props = defineProps<{
  modelValue: number
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: number): void
}>()
</script>

<template>
  <div class="line-width-slider">
    <label>线宽: {{ modelValue }}px</label>
    <input
      type="range"
      :value="modelValue"
      min="1"
      max="10"
      @input="emit('update:modelValue', Number($event.target.value))"
    />
  </div>
</template>

<style scoped>
.line-width-slider {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: rgba(0, 0, 0, 0.8);
  border-radius: 8px;
}

.line-width-slider label {
  font-size: 12px;
  color: white;
  white-space: nowrap;
}

.line-width-slider input[type="range"] {
  flex: 1;
  height: 4px;
}
</style>
```

- [ ] **Step 3: 创建 Toolbar.vue**

```vue
<script setup lang="ts">
import { computed } from 'vue'
import { useAnnotationStore } from '../stores/annotation'
import type { ToolType } from '../types'
import ColorPicker from './tools/ColorPicker.vue'
import LineWidthSlider from './tools/LineWidthSlider.vue'

const store = useAnnotationStore()

const tools: { type: ToolType; icon: string; label: string }[] = [
  { type: 'rectangle', icon: '□', label: '矩形' },
  { type: 'ellipse', icon: '○', label: '椭圆' },
  { type: 'arrow', icon: '→', label: '箭头' },
  { type: 'line', icon: '╱', label: '线条' },
  { type: 'pencil', icon: '✏', label: '涂鸦' },
  { type: 'mosaic', icon: '▦', label: '马赛克' },
  { type: 'image', icon: '🖼', label: '图片' },
  { type: 'text', icon: 'T', label: '文本' },
]

const needsColor = computed(() => 
  ['rectangle', 'ellipse', 'arrow', 'line', 'pencil', 'text'].includes(store.currentTool.value)
)

const needsLineWidth = computed(() => 
  ['rectangle', 'ellipse', 'arrow', 'line', 'pencil'].includes(store.currentTool.value)
)

const selectTool = (type: ToolType) => {
  store.currentTool.value = type
}
</script>

<template>
  <div class="toolbar">
    <div class="tool-buttons">
      <button
        v-for="tool in tools"
        :key="tool.type"
        :class="{ active: store.currentTool.value === tool.type }"
        @click="selectTool(tool.type)"
        :title="tool.label"
      >
        {{ tool.icon }}
      </button>
    </div>
    
    <div class="tool-options">
      <ColorPicker
        v-if="needsColor"
        v-model="store.currentColor.value"
      />
      <LineWidthSlider
        v-if="needsLineWidth"
        v-model="store.currentLineWidth.value"
      />
    </div>
    
    <div class="action-buttons">
      <button @click="store.undo" :disabled="!store.canUndo.value">撤销</button>
      <button @click="store.redo" :disabled="!store.canRedo.value">重做</button>
      <button class="cancel" @click="$emit('cancel')">取消</button>
      <button class="complete" @click="$emit('complete')">完成</button>
    </div>
  </div>
</template>

<style scoped>
.toolbar {
  position: fixed;
  top: 10px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  gap: 16px;
  align-items: flex-start;
  background: rgba(0, 0, 0, 0.9);
  padding: 12px;
  border-radius: 12px;
  z-index: 10000;
}

.tool-buttons {
  display: flex;
  gap: 4px;
}

.tool-buttons button {
  width: 36px;
  height: 36px;
  border: none;
  background: transparent;
  color: white;
  font-size: 18px;
  cursor: pointer;
  border-radius: 6px;
}

.tool-buttons button:hover {
  background: rgba(255, 255, 255, 0.1);
}

.tool-buttons button.active {
  background: #007bff;
}

.tool-options {
  display: flex;
  gap: 8px;
}

.action-buttons {
  display: flex;
  gap: 8px;
}

.action-buttons button {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
}

.action-buttons button.cancel {
  background: #6c757d;
  color: white;
}

.action-buttons button.complete {
  background: #28a745;
  color: white;
}

.action-buttons button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
```

- [ ] **Step 4: 验证编译**

Run: `npm run build`
Expected: TypeScript 编译成功

- [ ] **Step 5: 提交**

```bash
git add src/components/Toolbar.vue src/components/tools/ColorPicker.vue src/components/tools/LineWidthSlider.vue
git commit -m "feat: implement toolbar with color picker and line width slider"
```

---

## Chunk 5: 完整流程集成

### Task 8: 集成所有组件，实现完整流程

**Files:**
- Modify: `src/App.vue`
- Modify: `src/components/ScreenshotOverlay.vue`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: 修改 ScreenshotOverlay.vue 集成标注功能**

```vue
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import SelectionArea from './SelectionArea.vue'
import AnnotationCanvas from './AnnotationCanvas.vue'
import Toolbar from './Toolbar.vue'
import type { Selection } from '../types'

const isVisible = ref(false)
const screenshotData = ref('')
const selection = ref<Selection | null>(null)
const isAnnotating = ref(false)

const show = async () => {
  try {
    const data = await invoke<number[]>('capture_screen')
    const blob = new Blob([new Uint8Array(data)], { type: 'image/png' })
    screenshotData.value = URL.createObjectURL(blob)
    isVisible.value = true
  } catch (error) {
    console.error('Failed to capture screen:', error)
  }
}

const hide = () => {
  isVisible.value = false
  selection.value = null
  isAnnotating.value = false
}

const onSelectionComplete = (sel: Selection) => {
  selection.value = sel
  isAnnotating.value = true
}

const onComplete = async () => {
  // TODO: 合并截图和标注，保存到剪切板
  hide()
}

const onCancel = () => {
  hide()
}

// 监听 ESC 键和右键
onMounted(() => {
  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === 'Escape') {
      onCancel()
    }
  }
  
  const handleContextMenu = (e: MouseEvent) => {
    e.preventDefault()
    onCancel()
  }
  
  window.addEventListener('keydown', handleKeyDown)
  window.addEventListener('contextmenu', handleContextMenu)
  
  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeyDown)
    window.removeEventListener('contextmenu', handleContextMenu)
  })
})

defineExpose({ show, hide })
</script>

<template>
  <div v-if="isVisible" class="overlay">
    <div
      class="screenshot-background"
      :style="{ backgroundImage: `url(${screenshotData})` }"
    />
    
    <SelectionArea
      v-if="!isAnnotating"
      @selectionComplete="onSelectionComplete"
    />
    
    <AnnotationCanvas
      v-if="isAnnotating && selection"
      :screenshotData="screenshotData"
      :selection="selection"
    />
    
    <Toolbar
      v-if="isAnnotating"
      @complete="onComplete"
      @cancel="onCancel"
    />
  </div>
</template>

<style scoped>
.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 9999;
}

.screenshot-background {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-size: cover;
  filter: brightness(0.5);
}
</style>
```

- [ ] **Step 2: 修改 App.vue 监听 Tauri 事件**

```vue
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event'
import ScreenshotOverlay from './components/ScreenshotOverlay.vue'

const overlayRef = ref<InstanceType<typeof ScreenshotOverlay> | null>(null)

onMounted(async () => {
  await listen('show-screenshot', () => {
    overlayRef.value?.show()
  })
})
</script>

<template>
  <ScreenshotOverlay ref="overlayRef" />
</template>
```

- [ ] **Step 3: 修改 main.rs 发送事件**

```rust
use tauri::Manager;

// 在热键回调中
let app_handle = app.handle();
app_handle.emit_all("show-screenshot", ()).unwrap();
```

- [ ] **Step 4: 验证编译**

Run: `npm run build && cd src-tauri && cargo build`
Expected: 前后端编译成功

- [ ] **Step 5: 提交**

```bash
git add src/App.vue src/components/ScreenshotOverlay.vue src-tauri/src/main.rs
git commit -m "feat: integrate all components into complete flow"
```

### Task 9: 实现保存到剪切板功能

**Files:**
- Modify: `src/components/ScreenshotOverlay.vue`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: 修改 onComplete 函数**

```typescript
const onComplete = async () => {
  if (!selection.value) return
  
  // 创建临时 canvas 合并截图和标注
  const canvas = document.createElement('canvas')
  canvas.width = selection.value.width
  canvas.height = selection.value.height
  const ctx = canvas.getContext('2d')!
  
  // 绘制截图底图
  const img = new Image()
  img.src = screenshotData.value
  await new Promise((resolve) => { img.onload = resolve })
  
  ctx.drawImage(
    img,
    selection.value.x,
    selection.value.y,
    selection.value.width,
    selection.value.height,
    0,
    0,
    selection.value.width,
    selection.value.height
  )
  
  // 绘制标注
  const annotationCanvas = document.querySelector('.annotation-canvas') as HTMLCanvasElement
  if (annotationCanvas) {
    ctx.drawImage(annotationCanvas, 0, 0)
  }
  
  // 转换为 PNG 数据
  const blob = await new Promise<Blob>((resolve) => {
    canvas.toBlob((blob) => {
      resolve(blob!)
    }, 'image/png')
  })
  
  const arrayBuffer = await blob.arrayBuffer()
  const uint8Array = Array.from(new Uint8Array(arrayBuffer))
  
  // 保存到剪切板
  try {
    await invoke('save_to_clipboard', { imageData: uint8Array })
    console.log('Screenshot saved to clipboard')
  } catch (error) {
    console.error('Failed to save to clipboard:', error)
  }
  
  hide()
}
```

- [ ] **Step 2: 验证编译**

Run: `npm run build && cd src-tauri && cargo build`
Expected: 编译成功

- [ ] **Step 3: 提交**

```bash
git add src/components/ScreenshotOverlay.vue
git commit -m "feat: implement save to clipboard functionality"
```

---

## Chunk 6: 测试与优化

### Task 10: 完整功能测试

- [ ] **Step 1: 运行开发服务器**

Run: `npm run tauri dev`
Expected: 应用启动，托盘图标显示

- [ ] **Step 2: 测试热键**

操作: 按 `Ctrl+Shift+A`
Expected: 屏幕变暗，进入框选模式

- [ ] **Step 3: 测试框选**

操作: 鼠标拖拽选择区域
Expected: 显示选区，选区外变暗

- [ ] **Step 4: 测试标注工具**

操作: 选择各种工具进行标注
Expected: 标注正常显示

- [ ] **Step 5: 测试保存到剪切板**

操作: 点击完成，然后在其他应用粘贴
Expected: 截图正确粘贴

- [ ] **Step 6: 测试取消操作**

操作: 按 ESC 或右键
Expected: 取消截图，窗口关闭

- [ ] **Step 7: 提交最终版本**

```bash
git add .
git commit -m "feat: complete screenshot tool with annotation features"
```

---

## 附录: 常用命令

```bash
# 开发
npm run tauri dev

# 构建
npm run tauri build

# 类型检查
npm run build

# Rust 编译检查
cd src-tauri && cargo check
```
