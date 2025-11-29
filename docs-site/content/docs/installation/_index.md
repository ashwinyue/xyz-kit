---
title: "安装指南"
weight: 1
---

# 安装指南

## 系统要求

### macOS
- macOS 10.13 或更高版本
- Apple Silicon (M1/M2/M3) 或 Intel 处理器

### Windows
- Windows 10 或更高版本
- x64 架构处理器

## 下载安装包

前往 [Releases 页面](https://github.com/ashwinyue/xyz-kit/releases/latest) 下载对应平台的安装包：

| 平台 | 架构 | 文件名 |
|------|------|--------|
| macOS | Apple Silicon | `xyz-kit_*_aarch64.dmg` |
| macOS | Intel | `xyz-kit_*_x86_64.dmg` |
| Windows | x64 | `xyz-kit_*_x64-setup.exe` |

## macOS 安装步骤

### 1. 下载并打开 DMG 文件

下载对应架构的 `.dmg` 文件后，双击打开。

### 2. 安装应用

将 `xyz-kit.app` 拖入 `Applications` 文件夹。

### 3. 首次运行

由于应用未经过 Apple 公证，首次运行时需要：

1. 在 Finder 中找到应用
2. 按住 `Control` 键点击应用图标
3. 选择"打开"
4. 在弹出的对话框中点击"打开"

### 4. 授予权限

为了使用全局快捷键功能，需要授予辅助功能权限：

1. 打开"系统设置" > "隐私与安全性" > "辅助功能"
2. 点击左下角的锁图标解锁
3. 找到 `xyz-kit` 并勾选
4. 重启应用

## Windows 安装步骤

### 1. 下载安装程序

下载 `.exe` 安装包。

### 2. 运行安装程序

双击运行安装程序，如果出现 Windows Defender SmartScreen 警告：

1. 点击"更多信息"
2. 点击"仍要运行"

### 3. 完成安装

按照安装向导的提示完成安装。

### 4. 首次运行

安装完成后，应用会自动启动并最小化到系统托盘。

## 验证安装

安装完成后，按下全局快捷键验证：

- **macOS**: `Cmd + Shift + F`
- **Windows**: `Ctrl + Shift + F`

如果窗口成功弹出，说明安装成功！

## 卸载

### macOS

1. 在 Finder 中找到 `Applications` 文件夹
2. 将 `xyz-kit.app` 拖入废纸篓
3. 清空废纸篓

### Windows

1. 打开"设置" > "应用" > "已安装的应用"
2. 找到 `xyz-kit`
3. 点击"卸载"

## 故障排除

### macOS 无法打开应用

**问题**: 提示"无法打开，因为它来自身份不明的开发者"

**解决方案**:
```bash
# 在终端中运行
xattr -cr /Applications/xyz-kit.app
```

### Windows 快捷键不生效

**问题**: 按下 `Ctrl + Shift + F` 没有反应

**解决方案**:
1. 检查是否有其他应用占用了该快捷键
2. 尝试以管理员身份运行应用
3. 重启应用

### 应用无法启动

**问题**: 双击应用图标后没有反应

**解决方案**:
1. 检查系统日志查看错误信息
2. 确保系统满足最低要求
3. 尝试重新安装应用

如果问题仍未解决，请[提交 Issue](https://github.com/ashwinyue/xyz-kit/issues)。
