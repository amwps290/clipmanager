# ClipManager

一个使用 Rust + eframe (egui) 开发的跨平台剪切板管理工具。

## Features

### Current Version (MVP v0.1.0)
- ✅ **Real-time Clipboard Monitoring**: Automatically detects and saves clipboard content changes
- ✅ **History Management**: Stores the latest 100 clipboard records
- ✅ **Text Search**: Supports keyword search through history records
- ✅ **One-click Copy**: Click any history record to copy to clipboard
- ✅ **Content Deduplication**: Automatically avoids storing duplicate content
- ✅ **Local Storage**: Uses SQLite database for local storage, protecting privacy
- ✅ **Internationalization**: English interface with i18n architecture for future multi-language support

### Planned Features
- 🔄 **Image Support**: Support for image clipboard content
- 🔄 **Rich Text Support**: Support for HTML format content
- 🔄 **Tag System**: Add custom tags to clipboard entries
- 🔄 **Favorites**: Mark important content as favorites
- 🔄 **System Tray**: System tray integration
- 🔄 **Global Hotkeys**: Custom hotkeys to invoke interface
- 🔄 **Theme Support**: Multiple interface themes
- 🔄 **Cloud Sync**: Optional cloud synchronization functionality

## Technical Architecture

- **Language**: Rust 2021 Edition
- **GUI Framework**: eframe (egui) 0.32.0
- **Clipboard Operations**: arboard 3.4
- **Data Storage**: SQLite (rusqlite 0.31)
- **Async Runtime**: tokio 1.0
- **Serialization**: serde + serde_json
- **Error Handling**: thiserror + anyhow
- **Logging**: log + env_logger
- **Internationalization**: Custom i18n module with English/Chinese support

## Project Structure

```
src/
├── main.rs              # Application entry point
├── app.rs               # Main application structure
├── error.rs             # Error type definitions
├── i18n/                # Internationalization module
│   ├── mod.rs           # i18n manager and global functions
│   └── texts.rs         # Text constants for different languages
├── clipboard/           # Clipboard-related modules
│   ├── mod.rs
│   ├── monitor.rs       # Clipboard monitoring
│   ├── types.rs         # Data type definitions
│   └── handler.rs       # Operation handling
├── storage/             # Data storage modules
│   ├── mod.rs
│   ├── database.rs      # Database operations
│   └── models.rs        # Data models
├── ui/                  # User interface modules
│   ├── mod.rs
│   ├── main_window.rs   # Main window
│   └── components/      # UI components
└── config/              # Configuration management
    ├── mod.rs
    └── settings.rs      # Application settings
```

## Quick Start

### Requirements

- Rust 1.70+ (latest stable version recommended)
- Supported operating systems:
  - Windows 10/11
  - macOS 10.15+
  - Linux (mainstream distributions)

### Font Configuration

The application now uses English interface by default to avoid font rendering issues. The font system is configured to:

- Use system default fonts for optimal compatibility
- Support Unicode characters for international content
- Provide fallback fonts for missing characters
- Future support for custom font loading

### Internationalization

The application includes a built-in i18n system:

- **Current**: English interface (default)
- **Available**: Chinese text constants (for future use)
- **Architecture**: Centralized text management with `TextKey` identifiers
- **Future**: Easy language switching and custom language packs

### Installation

```bash
# Clone the project
git clone <repository-url>
cd clipmanager

# Install dependencies
cargo build
```

### Development

```bash
# Run development version
cargo run

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Build Release Version

```bash
# Build optimized version
cargo build --release

# Executable located at target/release/clipmanager
```

### Cross-Platform Builds

Use the provided scripts for cross-platform builds:

```bash
# Build for all supported platforms
./scripts/build-release.sh

# Test CI pipeline locally
./scripts/test-ci.sh

# Test with cross-compilation
./scripts/test-ci.sh --cross-compile
```

## CI/CD Pipeline

This project includes comprehensive GitHub Actions workflows for continuous integration and deployment:

### Automated Testing
- **Code Quality**: Format checking with `rustfmt` and linting with `clippy`
- **Test Suite**: Automated unit and integration tests
- **Cross-Platform**: Builds and tests on Linux and Windows

### Automated Releases
- **Binary Builds**: Automatic compilation for Linux and Windows
- **GitHub Releases**: Automatic release creation when tags are pushed
- **Artifacts**: Pre-built binaries available for download

### Supported Platforms
- **Linux**: `x86_64-unknown-linux-gnu`
- **Windows**: `x86_64-pc-windows-gnu` (cross-compiled)

### Release Process
1. Update version in `Cargo.toml`
2. Commit and push changes
3. Create and push a version tag: `git tag v0.1.0 && git push origin v0.1.0`
4. GitHub Actions automatically builds and creates a release

For detailed CI/CD documentation, see [docs/ci-cd-guide.md](docs/ci-cd-guide.md).

## 使用说明

### 基本操作

1. **启动应用**: 运行 `clipmanager` 可执行文件
2. **复制内容**: 应用会自动监控剪切板变化
3. **查看历史**: 在主界面查看剪切板历史记录
4. **搜索内容**: 在搜索框输入关键词过滤记录
5. **复制到剪切板**: 点击任意历史记录项即可复制
6. **删除记录**: 右键点击记录项选择删除

### 界面说明

- **搜索栏**: 输入关键词实时搜索历史记录
- **记录列表**: 显示剪切板历史记录，按时间倒序排列
- **状态栏**: 显示记录总数和应用版本信息

## 开发指南

### 代码规范

- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 进行代码检查
- 编写充分的单元测试
- 添加适当的文档注释

### 运行检查

```bash
# 代码格式化
cargo fmt

# 代码检查
cargo clippy

# 运行所有测试
cargo test

# 检查代码覆盖率
cargo tarpaulin --out Html
```

### 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。详见 [LICENSE-MIT](LICENSE-MIT) 和 [LICENSE-APACHE](LICENSE-APACHE) 文件。

## 致谢

- [egui](https://github.com/emilk/egui) - 优秀的即时模式 GUI 框架
- [arboard](https://github.com/1Password/arboard) - 跨平台剪切板库
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite Rust 绑定
- Rust 社区的所有贡献者

## 联系方式

- 项目主页: [GitHub Repository](https://github.com/clipmanager/clipmanager)
- 问题反馈: [GitHub Issues](https://github.com/clipmanager/clipmanager/issues)
- 功能建议: [GitHub Discussions](https://github.com/clipmanager/clipmanager/discussions)

---

**注意**: 本项目目前处于早期开发阶段，功能可能不完整或存在 bug。欢迎提交问题报告和功能建议！
