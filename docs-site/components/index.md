# 组件

xyz-kit 采用**模块化设计**，每个组件都有明确的职责和功能。

## 核心组件

### 全局快捷键 (Global Shortcut)

**功能**：注册和管理全局快捷键

**特性**：
- 支持跨平台快捷键配置
- macOS 使用 `Cmd + Shift + F`
- Windows 使用 `Ctrl + Shift + F`
- 自动处理快捷键冲突

**实现**：
```rust
// src-tauri/src/shortcut.rs
pub fn register_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyF);
    
    #[cfg(not(target_os = "macos"))]
    let shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyF);
    
    // 注册快捷键回调
    app.global_shortcut().on_shortcut(shortcut, |app, _shortcut, _event| {
        // 显示窗口并加载剪贴板
    })?;
    
    Ok(())
}
```

### 剪贴板管理 (Clipboard)

**功能**：读取和写入系统剪贴板

**特性**：
- 自动读取剪贴板内容
- 处理结果自动复制
- 支持文本格式

**实现**：
```rust
// src-tauri/src/commands.rs
#[tauri::command]
pub fn get_clipboard_text() -> Result<String, String> {
    let mut clipboard = Clipboard::new()
        .map_err(|e| format!("Failed to access clipboard: {}", e))?;
    
    clipboard.get_text()
        .map_err(|e| format!("Failed to get clipboard text: {}", e))
}
```

### 文本处理器 (Text Processor)

**功能**：处理各种文本格式转换

**特性**：
- 自动识别输入格式
- 支持多种转换规则
- 高性能处理

**转换规则**：

1. **逗号分隔 → 引号格式**
   - 输入: `1,2,3`
   - 输出: `"1","2","3"`

2. **引号格式 → 正则表达式**
   - 输入: `"1","2","3"`
   - 输出: `^1$|^2$|^3$`

3. **正则表达式 → 逗号分隔**
   - 输入: `^1$|^2$|^3$`
   - 输出: `1,2,3`

4. **多行文本 → 引号格式**
   - 输入: 多行文本
   - 输出: `"1","2","3"`

### 系统托盘 (System Tray)

**功能**：管理系统托盘图标和菜单

**特性**：
- 常驻系统托盘
- 右键菜单
- 显示/隐藏窗口
- 退出应用

**实现**：
```rust
// src-tauri/src/tray.rs
pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "show" => {
                    // 显示窗口
                }
                "quit" => {
                    // 退出应用
                }
                _ => {}
            }
        })
        .build(app)?;
    
    Ok(())
}
```

### 状态管理 (State Management)

**功能**：管理应用状态

**特性**：
- 使用 Zustand 管理状态
- 响应式更新
- 持久化存储

**实现**：
```typescript
// src/store/useAppStore.js
export const useAppStore = create((set, get) => ({
  text: '',
  selectedFunc: null,
  functions: [],
  
  setText: (text) => set({ text }),
  setSelectedFunc: (func) => set({ selectedFunc: func }),
  
  loadClipboard: async () => {
    const text = await invoke('get_clipboard_text');
    set({ text });
  },
  
  processText: async () => {
    const { text, selectedFunc } = get();
    const result = await invoke('process_text', { 
      text, 
      funcName: selectedFunc 
    });
    await invoke('set_clipboard_text', { text: result });
  }
}));
```

## UI 组件

### 文本编辑器 (Text Editor)

**功能**：可编辑的文本区域

**特性**：
- contentEditable 实现
- 自动聚焦
- 文本选中
- 快捷键支持

### 功能栏 (Function Bar)

**功能**：显示可用的转换功能

**特性**：
- 图标展示
- 点击切换
- 高亮选中

### 执行按钮 (Enter Button)

**功能**：触发文本处理

**特性**：
- 动画效果
- 快捷键绑定
- 视觉反馈

## 下一步

- 查看[开发文档](/guide/development)了解如何开发
- 查看[常见问题](/guide/faq)解决使用问题
