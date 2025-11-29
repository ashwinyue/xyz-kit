---
title: "开发环境搭建"
weight: 4
---

# 开发环境搭建

{{< callout type="info" >}}
  本指南将帮助你搭建 xyz-kit 的开发环境
{{< /callout >}}

本指南将帮助你搭建 xyz-kit 的开发环境。

## 系统要求

### 通用要求
- Node.js 18+ 
- npm 或 yarn
- Git

### macOS
- Xcode Command Line Tools
- Rust (通过 rustup 安装)

### Windows
- Visual Studio Build Tools
- Rust (通过 rustup 安装)

## 安装依赖

### 1. 安装 Node.js

从 [Node.js 官网](https://nodejs.org/) 下载并安装 LTS 版本。

验证安装：
```bash
node --version
npm --version
```

### 2. 安装 Rust

访问 [rustup.rs](https://rustup.rs/) 按照说明安装 Rust。

**macOS/Linux**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows**:
下载并运行 `rustup-init.exe`

验证安装：
```bash
rustc --version
cargo --version
```

### 3. 安装平台特定依赖

**macOS**:
```bash
xcode-select --install
```

**Windows**:
安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)，确保选中 "C++ 生成工具"。

## 克隆项目

```bash
git clone https://github.com/ashwinyue/xyz-kit.git
cd xyz-kit
```

## 安装项目依赖

```bash
npm install
```

这会安装所有前端和 Tauri 相关的依赖。

## 开发模式

启动开发服务器：

```bash
npm run tauri dev
```

这会：
1. 启动 Vite 开发服务器（前端）
2. 编译并运行 Tauri 应用（后端）
3. 启用热重载功能

## 项目结构

```
xyz-kit/
├── src/                    # 前端源代码
│   ├── api/               # Tauri API 封装
│   ├── assets/            # 静态资源
│   ├── components/        # React 组件
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
├── .github/               # GitHub Actions 工作流
├── docs/                  # 文档
└── package.json           # Node.js 依赖配置
```

## 开发工具

### 推荐的 IDE

- **VS Code** + 扩展:
  - rust-analyzer
  - Tauri
  - ESLint
  - Prettier

### 调试

**前端调试**:
在应用中按 `Cmd/Ctrl + Option/Alt + I` 打开开发者工具。

**后端调试**:
```bash
RUST_LOG=debug npm run tauri dev
```

### 代码格式化

**前端**:
```bash
npm run format
```

**后端**:
```bash
cd src-tauri
cargo fmt
```

### 代码检查

**前端**:
```bash
npm run lint
```

**后端**:
```bash
cd src-tauri
cargo clippy
```

## 常见开发任务

### 添加新的 Tauri 命令

1. 在 `src-tauri/src/commands.rs` 中定义命令函数
2. 在 `src-tauri/src/lib.rs` 的 `invoke_handler` 中注册
3. 在前端通过 `invoke()` 调用

### 添加新的文本处理功能

1. 在 `src-tauri/src/processor.rs` 中添加处理函数
2. 在 `get_functions()` 中注册新功能
3. 在 `public/icons/` 中添加对应图标

### 修改窗口配置

编辑 `src-tauri/tauri.conf.json` 中的 `windows` 配置。

### 修改全局快捷键

编辑 `src-tauri/src/shortcut.rs` 中的快捷键定义。

## 构建应用

### 开发构建

```bash
npm run tauri build
```

### 生产构建

```bash
npm run tauri build --release
```

构建产物位于 `src-tauri/target/release/bundle/`。

## 测试

### 运行测试

```bash
# 前端测试
npm test

# 后端测试
cd src-tauri
cargo test
```

## 故障排除

### 依赖安装失败

```bash
# 清除缓存
npm cache clean --force
rm -rf node_modules package-lock.json

# 重新安装
npm install
```

### Rust 编译错误

```bash
# 更新 Rust
rustup update

# 清除构建缓存
cd src-tauri
cargo clean
```

### 热重载不工作

1. 检查 Vite 开发服务器是否正常运行
2. 检查 `tauri.conf.json` 中的 `devUrl` 配置
3. 重启开发服务器

## 下一步

- 查看[项目架构](Architecture)了解代码组织
- 查看[贡献指南](Contributing)了解如何提交代码
- 查看[构建与发布](Build-and-Release)了解发布流程
