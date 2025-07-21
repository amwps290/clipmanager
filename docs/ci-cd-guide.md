# CI/CD 工作流指南

本文档描述了 ClipManager 项目的 GitHub Actions CI/CD 工作流配置。

## 工作流概述

项目包含两个主要的工作流：

### 1. CI 工作流 (`.github/workflows/ci.yml`)

**触发条件：**
- 推送到 `main` 分支
- 创建 Pull Request 到 `main` 分支
- 手动触发

**功能：**
- 代码格式检查 (`cargo fmt`)
- 代码质量检查 (`cargo clippy`)
- 运行测试套件 (`cargo test`)
- 运行基准测试 (dry run)
- 为 Linux 和 Windows 构建二进制文件
- 上传构建产物作为 artifacts

### 2. 发布工作流 (`.github/workflows/release.yml`)

**触发条件：**
- 推送标签 (格式: `v*`)
- 手动触发

**功能：**
- 创建 GitHub Release
- 构建优化的二进制文件
- 自动上传二进制文件到 Release

## 支持的平台

- **Linux**: `x86_64-unknown-linux-gnu`
- **Windows**: `x86_64-pc-windows-gnu`

## 系统依赖

### Linux 构建依赖
```bash
libxcb-render0-dev
libxcb-shape0-dev
libxcb-xfixes0-dev
libxkbcommon-dev
libssl-dev
libgtk-3-dev
libxcb1-dev
libxrandr-dev
libxss-dev
libglib2.0-dev
libgdk-pixbuf2.0-dev
libasound2-dev
```

### Windows 交叉编译依赖
```bash
gcc-mingw-w64-x86-64
```

## 如何发布新版本

1. **更新版本号**
   ```bash
   # 在 Cargo.toml 中更新版本号
   version = "0.2.0"
   ```

2. **提交更改**
   ```bash
   git add Cargo.toml
   git commit -m "Bump version to 0.2.0"
   git push origin main
   ```

3. **创建标签**
   ```bash
   git tag v0.2.0
   git push origin v0.2.0
   ```

4. **自动发布**
   - GitHub Actions 会自动检测到新标签
   - 构建二进制文件
   - 创建 GitHub Release
   - 上传二进制文件

## 手动触发构建

### CI 工作流
1. 访问 GitHub Actions 页面
2. 选择 "CI/CD Pipeline" 工作流
3. 点击 "Run workflow"
4. 选择分支并运行

### 发布工作流
1. 访问 GitHub Actions 页面
2. 选择 "Release" 工作流
3. 点击 "Run workflow"
4. 输入标签名称 (如 `v0.1.0`)
5. 运行工作流

## 构建产物

### CI 构建
- 产物保存为 GitHub Actions artifacts
- 保留期：30 天
- 文件名：
  - `clipmanager-linux`
  - `clipmanager-windows.exe`

### 发布构建
- 产物上传到 GitHub Release
- 永久保存
- 包含版本信息和变更日志

## 缓存策略

工作流使用 GitHub Actions 缓存来加速构建：

- **缓存内容**：
  - Cargo registry (`~/.cargo/registry`)
  - Cargo git dependencies (`~/.cargo/git`)
  - 构建目标目录 (`target/`)

- **缓存键**：基于 `Cargo.lock` 文件的哈希值

## 故障排除

### 常见问题

1. **依赖安装失败**
   - 检查系统依赖列表是否完整
   - 确认包管理器命令正确

2. **交叉编译失败**
   - 验证目标平台工具链安装
   - 检查 `.cargo/config.toml` 配置

3. **测试失败**
   - 确保所有测试在本地通过
   - 检查测试是否依赖特定环境

4. **发布失败**
   - 确认 `GITHUB_TOKEN` 权限
   - 检查标签格式是否正确

### 调试技巧

1. **查看详细日志**
   ```yaml
   env:
     RUST_LOG: debug
     RUST_BACKTRACE: full
   ```

2. **本地测试交叉编译**
   ```bash
   # 安装目标平台
   rustup target add x86_64-pc-windows-gnu
   
   # 测试构建
   cargo build --target x86_64-pc-windows-gnu
   ```

3. **验证二进制文件**
   ```bash
   # 检查文件类型
   file target/x86_64-pc-windows-gnu/release/clipmanager.exe
   
   # 检查依赖
   ldd target/x86_64-unknown-linux-gnu/release/clipmanager
   ```

## 配置文件说明

### `.cargo/config.toml`
- 交叉编译链接器配置
- 优化设置
- 网络和注册表配置

### `Cargo.toml`
- 项目元数据
- 依赖管理
- 构建配置
- 发布优化设置

## 安全考虑

1. **Token 权限**
   - `GITHUB_TOKEN` 自动提供
   - 具有仓库读写权限
   - 用于创建 Release 和上传文件

2. **依赖安全**
   - 定期更新依赖
   - 使用 `cargo audit` 检查漏洞

3. **二进制文件验证**
   - 考虑添加校验和
   - 可选择代码签名

## 性能优化

1. **构建时间**
   - 使用缓存减少重复构建
   - 并行构建不同平台

2. **二进制大小**
   - 启用 LTO (Link Time Optimization)
   - 使用 `strip` 移除调试信息
   - 设置 `panic = "abort"`

3. **网络优化**
   - 缓存 Cargo registry
   - 使用 git 克隆优化
