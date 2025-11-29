# xyz-kit - 轻量级文本处理工具

一个基于 Tauri + React 构建的桌面文本处理工具，专为开发者设计，提供快速的 ID 格式转换功能。

## ✨ 特性

- 🚀 **全局快捷键唤醒** - `Cmd+Shift+F` (macOS) / `Ctrl+Shift+F` (Windows/Linux)
- 📋 **自动读取剪贴板** - 唤醒时自动加载剪贴板内容
- 🔄 **智能格式转换** - 自动识别并转换 ID 列表格式
- ⚡ **即时处理** - 处理结果自动复制到剪贴板
- 🎯 **轻量级** - 常驻系统托盘，不占用任务栏
- 🌐 **跨平台** - 支持 macOS、Windows、Linux

## 🎯 核心功能

### ID 拼接转换

智能识别并转换以下三种格式：

1. **逗号分隔** → **引号格式**
   ```
   输入: 1,2,3
   输出: "1","2","3"
   ```

2. **引号格式** → **正则表达式**
   ```
   输入: "1","2","3"
   输出: ^1$|^2$|^3$
   ```

3. **正则表达式** → **逗号分隔**
   ```
   输入: ^1$|^2$|^3$
   输出: 1,2,3
   ```

4. **多行文本** → **引号格式**
   ```
   输入:
   1
   2
   3
   
   输出: "1","2","3"
   ```

## 🚀 快速开始

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run dev
```

### 构建应用

```bash
npm run build
```

构建完成后，安装包位于 `src-tauri/target/release/bundle/` 目录。

## 📖 使用方法

1. **启动应用** - 应用会自动隐藏到系统托盘
2. **唤醒窗口** - 按 `Cmd+Shift+F` (macOS) 或 `Ctrl+Shift+F` (Windows/Linux)
3. **编辑文本** - 窗口会自动加载剪贴板内容，你可以直接编辑
4. **处理转换** - 点击右侧功能图标或按 `Cmd+Enter` / `Ctrl+Enter`
5. **自动复制** - 处理结果会自动复制到剪贴板
6. **隐藏窗口** - 按 `Escape` 或点击窗口外部自动隐藏

## ⌨️ 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Cmd+Shift+F` / `Ctrl+Shift+F` | 唤醒/显示窗口 |
| `Cmd+Enter` / `Ctrl+Enter` | 处理文本 |
| `Escape` | 隐藏窗口 |

## 🏗️ 项目结构

```
.
├── src/                    # 前端源代码
│   ├── api/               # Tauri API 封装
│   ├── assets/            # 静态资源（字体等）
│   ├── components/        # React 组件
│   │   ├── EnterButton.jsx      # 执行按钮
│   │   ├── FunctionBar.jsx      # 功能栏
│   │   └── FunctionIcon.jsx     # 功能图标
│   ├── constants/         # 常量配置
│   ├── store/             # Zustand 状态管理
│   ├── utils/             # 工具函数
│   ├── App.jsx            # 主应用组件
│   ├── main.jsx           # 应用入口
│   └── index.css          # 全局样式
├── src-tauri/             # Tauri 后端（Rust）
│   ├── src/
│   │   ├── commands.rs    # Tauri 命令
│   │   ├── processor.rs   # 文本处理逻辑
│   │   ├── shortcut.rs    # 全局快捷键
│   │   ├── store.rs       # 偏好设置存储
│   │   ├── tray.rs        # 系统托盘
│   │   ├── lib.rs         # 库入口
│   │   └── main.rs        # 应用入口
│   ├── icons/             # 应用图标
│   ├── Cargo.toml         # Rust 依赖配置
│   └── tauri.conf.json    # Tauri 配置
├── public/                # 公共静态资源
└── ...                    # 配置文件
```

## 🛠️ 技术栈

### 前端
- **React 18** - UI 框架
- **Vite** - 构建工具
- **Tailwind CSS** - 样式框架
- **Ant Design** - UI 组件库
- **Zustand** - 状态管理

### 后端
- **Tauri 2.0** - 桌面应用框架
- **Rust** - 系统级编程语言
- **arboard** - 跨平台剪贴板操作

### 核心插件
- `tauri-plugin-global-shortcut` - 全局快捷键
- `tauri-plugin-single-instance` - 单实例运行
- `tauri-plugin-opener` - 系统默认应用打开

## 🔧 配置说明

### 窗口配置

应用窗口默认配置（`src-tauri/tauri.conf.json`）：
- 尺寸: 680x400
- 无边框、透明背景
- 始终置顶
- 不显示在任务栏

### 自定义快捷键

修改 `src-tauri/src/shortcut.rs` 中的快捷键配置：

```rust
let shortcut = Shortcut::new(
    Some(Modifiers::SUPER | Modifiers::SHIFT), 
    Code::KeyF
);
```

## 📝 开发说明

### 添加新的文本处理功能

1. 在 `src-tauri/src/processor.rs` 中添加处理函数
2. 在 `get_functions()` 中注册新功能
3. 在 `process_text()` 命令中添加分支处理
4. 在 `public/icons/` 中添加对应图标

### 调试技巧

```bash
# 查看 Rust 后端日志
RUST_LOG=debug npm run dev

# 打开开发者工具
# 在应用中按 Cmd+Option+I (macOS) 或 Ctrl+Shift+I (Windows/Linux)
```

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

---

**提示**: 首次运行时，应用会请求辅助功能权限（macOS）以支持全局快捷键功能。
