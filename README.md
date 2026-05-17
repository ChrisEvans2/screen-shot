# ScreenShot - 轻量级截图标注工具

一款类似微信截图的轻量级桌面截图标注工具，基于 Tauri + Vue 3 开发。

## 功能特性

- **全屏截图**：一键截取全屏画面
- **区域选择**：自由框选截图区域，支持拖拽调整大小和位置
- **多种标注工具**：
  - □ 矩形
  - ○ 椭圆
  - → 箭头
  - ─ 线段
  - ✎ 画笔
  - ▦ 马赛克
  - T 文字
- **颜色选择**：HSL 颜色滑块，自由调整颜色
- **线条粗细**：1-10px 可调
- **撤销/重做**：支持操作历史回退
- **保存到剪贴板**：截图自动复制到系统剪贴板

## 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl + Shift + A` | 启动截图 |
| `ESC` | 取消截图 |
| `Enter` | 确认选区 |

## 操作流程

1. 按 `Ctrl + Shift + A` 或右键托盘图标选择"截图"
2. 框选要截取的区域
3. 拖拽边框手柄调整选区大小（可选）
4. 使用工具栏进行标注（可选）
5. 点击"保存"复制到剪贴板

## 技术栈

- **前端**：Vue 3 + TypeScript + Vite
- **后端**：Rust + Tauri v1
- **截图库**：screenshots (Rust)
- **构建工具**：tauri-cli

## 开发环境

### 前置要求

- Node.js >= 16
- Rust >= 1.70
- Windows: Microsoft Visual C++ Build Tools

### 安装依赖

```bash
npm install
```

### 开发运行

```bash
npm run tauri dev
```

### 构建 Release

```bash
npm run tauri build
```

构建产物位置：
- MSI 安装包：`src-tauri/target/release/bundle/msi/`
- EXE 安装程序：`src-tauri/target/release/bundle/nsis/`

## 下载安装

前往 [Releases](https://github.com/ChrisEvans2/screen-shot/releases) 页面下载最新版本：

- `ScreenShot_0.1.0_x64-setup.exe` - NSIS 安装程序（推荐）
- `ScreenShot_0.1.0_x64_en-US.msi` - MSI 安装包

## 项目结构

```
screen_shot/
├── src/                        # 前端源码
│   ├── components/             # Vue 组件
│   │   ├── AnnotationCanvas.vue   # 标注画布
│   │   ├── SelectionArea.vue      # 选区组件
│   │   ├── ScreenshotOverlay.vue  # 截图覆盖层
│   │   └── Toolbar.vue           # 工具栏
│   ├── stores/                 # 状态管理
│   │   └── annotation.ts         # 标注状态
│   ├── types/                  # 类型定义
│   │   └── index.ts
│   ├── App.vue                 # 根组件
│   └── main.ts                 # 入口文件
├── src-tauri/                  # Tauri 后端
│   ├── src/
│   │   ├── main.rs             # Rust 入口
│   │   └── screenshot.rs       # 截图模块
│   ├── Cargo.toml              # Rust 依赖
│   └── tauri.conf.json         # Tauri 配置
├── docs/                       # 文档
│   ├── superpowers/specs/      # 设计规格
│   └── superpowers/plans/      # 实现计划
├── package.json                # Node 依赖
└── README.md                   # 项目说明
```

## 许可证

MIT License

## 作者

ChrisEvans2 - 193560006@qq.com
