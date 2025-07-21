# ClipManager - 跨平台剪切板管理工具技术设计文档

## 1. 项目概述

ClipManager 是一个使用 Rust + eframe (egui) 开发的跨平台剪切板管理工具，旨在提供高效、安全、易用的剪切板历史记录管理功能。

### 1.1 目标平台
- Windows 10/11
- macOS 10.15+
- Linux (主流发行版)
- Web (通过 WebAssembly)

### 1.2 核心价值
- **跨平台一致性**: 在所有支持的平台上提供统一的用户体验
- **高性能**: 基于 Rust 的内存安全和性能优势
- **轻量级**: 最小化系统资源占用
- **隐私保护**: 本地数据存储，用户数据不上传云端

## 2. 功能需求分析

### 2.1 核心功能 (MVP)

#### 2.1.1 剪切板监控
- **实时监控**: 自动检测剪切板内容变化
- **内容类型支持**: 
  - 纯文本 (UTF-8)
  - 富文本 (HTML)
  - 图片 (PNG, JPEG, GIF)
- **去重机制**: 避免重复内容存储
- **大小限制**: 单个条目最大 10MB

#### 2.1.2 历史记录管理
- **存储容量**: 默认保存最近 1000 条记录
- **自动清理**: 超过限制时自动删除最旧记录
- **手动删除**: 支持单条或批量删除
- **清空历史**: 一键清空所有历史记录

#### 2.1.3 搜索与过滤
- **文本搜索**: 支持关键词搜索文本内容
- **类型过滤**: 按内容类型筛选 (文本/图片)
- **时间过滤**: 按时间范围筛选
- **实时搜索**: 输入时即时显示结果

#### 2.1.4 快速操作
- **一键复制**: 点击历史记录项即可复制到剪切板
- **预览功能**: 鼠标悬停显示内容预览
- **快捷键**: 支持全局快捷键唤起界面

### 2.2 高级功能 (后续版本)

#### 2.2.1 分类管理
- **标签系统**: 为剪切板条目添加自定义标签
- **收藏功能**: 标记重要内容为收藏
- **分组管理**: 按项目或用途分组管理

#### 2.2.2 数据同步
- **本地备份**: 导出/导入历史记录
- **云同步**: 可选的云端同步功能
- **多设备**: 跨设备数据同步

#### 2.2.3 高级搜索
- **正则表达式**: 支持正则表达式搜索
- **模糊搜索**: 容错搜索功能
- **搜索历史**: 保存常用搜索条件

## 3. 用户界面设计

### 3.1 主界面布局

```
┌─────────────────────────────────────────────────────────┐
│ ClipManager                                    [_][□][×]│
├─────────────────────────────────────────────────────────┤
│ [搜索框                                    ] [设置] [?] │
├─────────────────────────────────────────────────────────┤
│ 过滤器: [全部▼] [文本] [图片] [今天▼]                    │
├─────────────────────────────────────────────────────────┤
│ ┌─────────────────────────────────────────────────────┐ │
│ │ 📄 Hello World                          2分钟前    │ │
│ │    这是一段示例文本内容...                          │ │
│ ├─────────────────────────────────────────────────────┤ │
│ │ 🖼️ screenshot.png                       5分钟前    │ │
│ │    [图片预览缩略图]                                 │ │
│ ├─────────────────────────────────────────────────────┤ │
│ │ 📄 https://example.com                  10分钟前   │ │
│ │    网页链接示例                                     │ │
│ └─────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ 状态栏: 共 156 条记录 | 已用空间: 2.3MB                  │
└─────────────────────────────────────────────────────────┘
```

### 3.2 交互设计

#### 3.2.1 主要交互
- **单击**: 复制到剪切板
- **右键**: 显示上下文菜单 (删除、收藏、编辑标签)
- **双击**: 打开详细预览窗口
- **拖拽**: 支持拖拽到其他应用程序

#### 3.2.2 快捷键
- `Ctrl/Cmd + Shift + V`: 唤起主界面
- `Ctrl/Cmd + F`: 聚焦搜索框
- `Escape`: 关闭界面
- `Enter`: 复制选中项
- `Delete`: 删除选中项

### 3.3 响应式设计
- **最小窗口**: 400x300 像素
- **默认窗口**: 600x500 像素
- **自适应布局**: 根据窗口大小调整列表项显示
- **高DPI支持**: 支持高分辨率显示器

## 4. 技术架构设计

### 4.1 整体架构

```
┌─────────────────────────────────────────────────────────┐
│                    用户界面层 (egui)                     │
├─────────────────────────────────────────────────────────┤
│                    应用逻辑层                           │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐ │
│  │  UI状态管理  │ │  事件处理   │ │     业务逻辑        │ │
│  └─────────────┘ └─────────────┘ └─────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│                    服务层                               │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐ │
│  │ 剪切板服务   │ │  存储服务   │ │    配置服务         │ │
│  └─────────────┘ └─────────────┘ └─────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│                    数据层                               │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐ │
│  │   SQLite    │ │  文件系统   │ │    系统剪切板       │ │
│  └─────────────┘ └─────────────┘ └─────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

### 4.2 核心模块设计

#### 4.2.1 应用程序入口 (`main.rs`)
- 初始化 eframe 应用
- 设置窗口属性和图标
- 配置日志系统

#### 4.2.2 应用状态管理 (`app.rs`)
- 实现 `eframe::App` trait
- 管理全局应用状态
- 处理 UI 更新循环

#### 4.2.3 剪切板服务 (`clipboard/`)
- `monitor.rs`: 剪切板监控
- `types.rs`: 剪切板数据类型定义
- `handler.rs`: 剪切板操作处理

#### 4.2.4 数据存储 (`storage/`)
- `database.rs`: SQLite 数据库操作
- `models.rs`: 数据模型定义
- `migrations.rs`: 数据库迁移

#### 4.2.5 用户界面 (`ui/`)
- `main_window.rs`: 主窗口界面
- `components/`: UI 组件
- `styles.rs`: 样式定义

#### 4.2.6 配置管理 (`config/`)
- `settings.rs`: 应用设置
- `preferences.rs`: 用户偏好

### 4.3 依赖库选择

#### 4.3.1 核心依赖
```toml
[dependencies]
# GUI 框架
eframe = "0.28"
egui = "0.28"

# 剪切板操作
arboard = "3.4"

# 数据持久化
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 异步运行时
tokio = { version = "1.0", features = ["full"] }

# 日志
log = "0.4"
env_logger = "0.11"

# 错误处理
anyhow = "1.0"
thiserror = "1.0"

# 时间处理
chrono = { version = "0.4", features = ["serde"] }

# 图片处理
image = "0.25"

# 配置文件
toml = "0.8"

# 系统集成
directories = "5.0"
```

#### 4.3.2 开发依赖
```toml
[dev-dependencies]
# 测试
criterion = "0.5"
tempfile = "3.8"

# 基准测试
pprof = { version = "0.13", features = ["criterion", "flamegraph"] }
```

## 5. 数据模型设计

### 5.1 数据库表结构

#### 5.1.1 剪切板条目表 (`clipboard_items`)
```sql
CREATE TABLE clipboard_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content_type TEXT NOT NULL,           -- 'text', 'image', 'html'
    content_text TEXT,                    -- 文本内容
    content_data BLOB,                    -- 二进制数据 (图片等)
    content_hash TEXT NOT NULL UNIQUE,    -- 内容哈希 (用于去重)
    content_size INTEGER NOT NULL,        -- 内容大小 (字节)
    created_at DATETIME NOT NULL,         -- 创建时间
    accessed_at DATETIME NOT NULL,        -- 最后访问时间
    access_count INTEGER DEFAULT 0,       -- 访问次数
    is_favorite BOOLEAN DEFAULT FALSE,    -- 是否收藏
    tags TEXT,                           -- 标签 (JSON 数组)
    metadata TEXT                        -- 元数据 (JSON)
);
```

#### 5.1.2 配置表 (`settings`)
```sql
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at DATETIME NOT NULL
);
```

#### 5.1.3 索引设计
```sql
-- 提高查询性能的索引
CREATE INDEX idx_clipboard_items_created_at ON clipboard_items(created_at DESC);
CREATE INDEX idx_clipboard_items_content_type ON clipboard_items(content_type);
CREATE INDEX idx_clipboard_items_is_favorite ON clipboard_items(is_favorite);
CREATE INDEX idx_clipboard_items_content_hash ON clipboard_items(content_hash);
```

### 5.2 Rust 数据结构

#### 5.2.1 剪切板条目
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: Option<i64>,
    pub content_type: ContentType,
    pub content: ClipboardContent,
    pub content_hash: String,
    pub content_size: usize,
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub access_count: u32,
    pub is_favorite: bool,
    pub tags: Vec<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Text,
    Image,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardContent {
    Text(String),
    Image(Vec<u8>),
    Html { html: String, text: String },
}
```

#### 5.2.2 应用配置
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub max_items: usize,
    pub max_item_size: usize,
    pub auto_start: bool,
    pub show_notifications: bool,
    pub hotkey: String,
    pub theme: Theme,
    pub window: WindowConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub always_on_top: bool,
    pub start_minimized: bool,
}
```

## 6. 错误处理和日志策略

### 6.1 错误类型定义
```rust
#[derive(Debug, thiserror::Error)]
pub enum ClipManagerError {
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),
    
    #[error("剪切板操作错误: {0}")]
    Clipboard(#[from] arboard::Error),
    
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("配置错误: {message}")]
    Config { message: String },
    
    #[error("内容过大: {size} 字节，最大允许 {max_size} 字节")]
    ContentTooLarge { size: usize, max_size: usize },
}
```

### 6.2 日志配置
- **级别**: DEBUG (开发), INFO (生产)
- **输出**: 控制台 + 文件
- **格式**: 时间戳 + 级别 + 模块 + 消息
- **轮转**: 按大小轮转日志文件 (10MB)

## 7. 性能优化策略

### 7.1 内存管理
- **延迟加载**: 大型内容 (图片) 按需加载
- **缓存策略**: LRU 缓存最近访问的内容
- **内存限制**: 设置最大内存使用量

### 7.2 数据库优化
- **连接池**: 使用连接池管理数据库连接
- **批量操作**: 批量插入和删除操作
- **定期维护**: 定期执行 VACUUM 和 ANALYZE

### 7.3 UI 性能
- **虚拟滚动**: 大列表使用虚拟滚动
- **异步加载**: 图片预览异步加载
- **防抖搜索**: 搜索输入防抖处理

## 8. 安全性考虑

### 8.1 数据保护
- **本地存储**: 所有数据仅存储在本地
- **加密选项**: 可选的数据库加密
- **敏感内容**: 检测并标记敏感内容 (密码、信用卡等)

### 8.2 权限管理
- **最小权限**: 仅请求必要的系统权限
- **用户控制**: 用户可控制监控开关
- **隐私模式**: 临时禁用监控功能

## 9. 测试策略

### 9.1 单元测试
- **覆盖率**: 目标 80% 以上代码覆盖率
- **模块测试**: 每个核心模块独立测试
- **边界测试**: 测试边界条件和错误情况

### 9.2 集成测试
- **数据库测试**: 测试数据库操作的正确性
- **剪切板测试**: 测试剪切板监控和操作
- **UI 测试**: 测试用户界面交互

### 9.3 性能测试
- **基准测试**: 使用 criterion 进行性能基准测试
- **内存测试**: 监控内存使用情况
- **压力测试**: 大量数据下的性能表现

## 10. 部署和分发

### 10.1 构建配置
- **Release 优化**: 启用 LTO 和其他优化选项
- **静态链接**: 减少运行时依赖
- **资源嵌入**: 将图标和样式文件嵌入可执行文件

### 10.2 平台特定
- **Windows**: 生成 MSI 安装包
- **macOS**: 创建 .app 包和 DMG 镜像
- **Linux**: 提供 AppImage 和 .deb/.rpm 包
- **Web**: 编译为 WebAssembly

### 10.3 自动更新
- **版本检查**: 定期检查新版本
- **增量更新**: 支持增量更新机制
- **回滚功能**: 更新失败时自动回滚

## 11. 开发阶段和里程碑

### 11.1 第一阶段：基础架构 (2-3周)

#### 里程碑 1.1：项目初始化
- [ ] 设置 Cargo.toml 依赖
- [ ] 创建基本项目结构
- [ ] 配置开发环境和工具链
- [ ] 设置 CI/CD 流水线

#### 里程碑 1.2：核心服务实现
- [ ] 实现剪切板监控服务
- [ ] 实现数据存储服务
- [ ] 实现基本的数据模型
- [ ] 编写单元测试

#### 里程碑 1.3：基础 UI 框架
- [ ] 创建主窗口结构
- [ ] 实现基本的列表显示
- [ ] 添加搜索功能
- [ ] 实现基本的交互逻辑

### 11.2 第二阶段：核心功能 (3-4周)

#### 里程碑 2.1：完整的剪切板管理
- [ ] 支持多种内容类型 (文本、图片、HTML)
- [ ] 实现内容去重机制
- [ ] 添加内容预览功能
- [ ] 实现复制到剪切板功能

#### 里程碑 2.2：高级搜索和过滤
- [ ] 实现关键词搜索
- [ ] 添加类型过滤
- [ ] 实现时间范围过滤
- [ ] 优化搜索性能

#### 里程碑 2.3：用户体验优化
- [ ] 添加快捷键支持
- [ ] 实现系统托盘集成
- [ ] 添加通知功能
- [ ] 优化界面响应性

### 11.3 第三阶段：高级功能 (2-3周)

#### 里程碑 3.1：数据管理
- [ ] 实现标签系统
- [ ] 添加收藏功能
- [ ] 实现数据导出/导入
- [ ] 添加数据清理工具

#### 里程碑 3.2：配置和个性化
- [ ] 实现设置界面
- [ ] 添加主题支持
- [ ] 实现自定义快捷键
- [ ] 添加启动选项

#### 里程碑 3.3：跨平台优化
- [ ] 优化各平台的用户体验
- [ ] 实现平台特定功能
- [ ] 添加自动更新机制
- [ ] 完善安装包制作

### 11.4 第四阶段：测试和发布 (1-2周)

#### 里程碑 4.1：全面测试
- [ ] 完善单元测试覆盖率
- [ ] 进行集成测试
- [ ] 执行性能测试
- [ ] 进行用户验收测试

#### 里程碑 4.2：文档和发布
- [ ] 完善用户文档
- [ ] 准备发布材料
- [ ] 制作安装包
- [ ] 发布第一个稳定版本

## 12. MVP 功能范围

### 12.1 最小可行产品功能清单

#### 核心功能
1. **剪切板监控**
   - 实时监控文本内容变化
   - 基本的内容去重
   - 最多保存 100 条记录

2. **历史记录显示**
   - 简单的列表界面
   - 显示内容预览 (前50字符)
   - 显示创建时间

3. **基本操作**
   - 点击复制到剪切板
   - 删除单条记录
   - 清空所有记录

4. **简单搜索**
   - 文本内容关键词搜索
   - 实时搜索结果更新

#### 技术要求
- 支持 Windows、macOS、Linux
- 本地 SQLite 数据库存储
- 基本的错误处理
- 简洁的用户界面

#### 不包含的功能 (后续版本)
- 图片和富文本支持
- 标签和分类
- 云同步
- 高级搜索 (正则表达式)
- 系统托盘集成
- 自定义快捷键

### 12.2 MVP 开发时间估算

| 任务 | 预估时间 | 优先级 |
|------|----------|--------|
| 项目初始化和依赖配置 | 1天 | 高 |
| 基本数据模型和数据库 | 2天 | 高 |
| 剪切板监控服务 | 3天 | 高 |
| 基础 UI 框架 | 3天 | 高 |
| 列表显示和搜索 | 2天 | 高 |
| 复制和删除操作 | 1天 | 高 |
| 基本测试 | 2天 | 中 |
| 跨平台测试和优化 | 2天 | 中 |
| 文档和打包 | 1天 | 低 |

**总计：约 17 个工作日 (3-4周)**

## 13. 详细实现步骤

### 13.1 步骤 1：项目初始化

#### 1.1 更新 Cargo.toml
```toml
[package]
name = "clipmanager"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A cross-platform clipboard manager built with Rust and egui"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/clipmanager"

[dependencies]
# GUI 框架
eframe = { version = "0.28", default-features = false, features = [
    "default_fonts",
    "glow",
    "persistence",
] }
egui = "0.28"

# 剪切板操作
arboard = "3.4"

# 数据持久化
rusqlite = { version = "0.31", features = ["bundled", "chrono"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 异步运行时
tokio = { version = "1.0", features = ["rt-multi-thread", "macros", "time"] }

# 日志
log = "0.4"
env_logger = "0.11"

# 错误处理
anyhow = "1.0"
thiserror = "1.0"

# 时间处理
chrono = { version = "0.4", features = ["serde"] }

# 配置文件
directories = "5.0"

[dev-dependencies]
tempfile = "3.8"
criterion = "0.5"

[[bench]]
name = "clipboard_benchmark"
harness = false

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

#### 1.2 创建项目结构
```
src/
├── main.rs              # 应用程序入口
├── app.rs               # 主应用结构
├── error.rs             # 错误类型定义
├── clipboard/           # 剪切板相关模块
│   ├── mod.rs
│   ├── monitor.rs       # 剪切板监控
│   ├── types.rs         # 数据类型定义
│   └── handler.rs       # 操作处理
├── storage/             # 数据存储模块
│   ├── mod.rs
│   ├── database.rs      # 数据库操作
│   ├── models.rs        # 数据模型
│   └── migrations.rs    # 数据库迁移
├── ui/                  # 用户界面模块
│   ├── mod.rs
│   ├── main_window.rs   # 主窗口
│   ├── components/      # UI 组件
│   │   ├── mod.rs
│   │   ├── item_list.rs # 条目列表
│   │   └── search_bar.rs# 搜索栏
│   └── styles.rs        # 样式定义
└── config/              # 配置管理
    ├── mod.rs
    └── settings.rs      # 应用设置
```

### 13.2 步骤 2：核心数据结构实现

#### 2.1 错误类型定义 (`src/error.rs`)
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipManagerError {
    #[error("数据库错误: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("剪切板操作错误: {0}")]
    Clipboard(#[from] arboard::Error),

    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("配置错误: {message}")]
    Config { message: String },

    #[error("内容过大: {size} 字节，最大允许 {max_size} 字节")]
    ContentTooLarge { size: usize, max_size: usize },

    #[error("不支持的内容类型")]
    UnsupportedContentType,
}

pub type Result<T> = std::result::Result<T, ClipManagerError>;
```

#### 2.2 剪切板数据类型 (`src/clipboard/types.rs`)
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: Option<i64>,
    pub content_type: ContentType,
    pub content: String,
    pub content_hash: String,
    pub content_size: usize,
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub access_count: u32,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Text,
    // 后续版本支持
    // Image,
    // Html,
}

impl ClipboardItem {
    pub fn new(content: String) -> Self {
        let content_size = content.len();
        let content_hash = Self::calculate_hash(&content);
        let now = Utc::now();

        Self {
            id: None,
            content_type: ContentType::Text,
            content,
            content_hash,
            content_size,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            is_favorite: false,
        }
    }

    fn calculate_hash(content: &str) -> String {
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn preview(&self, max_length: usize) -> String {
        if self.content.len() <= max_length {
            self.content.clone()
        } else {
            format!("{}...", &self.content[..max_length])
        }
    }

    pub fn update_access(&mut self) {
        self.accessed_at = Utc::now();
        self.access_count += 1;
    }
}

#[derive(Debug, Clone)]
pub struct SearchFilter {
    pub query: String,
    pub content_type: Option<ContentType>,
    pub favorites_only: bool,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

impl Default for SearchFilter {
    fn default() -> Self {
        Self {
            query: String::new(),
            content_type: None,
            favorites_only: false,
            date_range: None,
        }
    }
}
```

### 13.3 步骤 3：数据库层实现

#### 3.1 数据库模型 (`src/storage/models.rs`)
```rust
use crate::clipboard::types::{ClipboardItem, ContentType};
use crate::error::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};

impl ClipboardItem {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: Some(row.get("id")?),
            content_type: match row.get::<_, String>("content_type")?.as_str() {
                "text" => ContentType::Text,
                _ => ContentType::Text, // 默认为文本
            },
            content: row.get("content")?,
            content_hash: row.get("content_hash")?,
            content_size: row.get::<_, i64>("content_size")? as usize,
            created_at: row.get("created_at")?,
            accessed_at: row.get("accessed_at")?,
            access_count: row.get::<_, i64>("access_count")? as u32,
            is_favorite: row.get("is_favorite")?,
        })
    }

    pub fn insert_params(&self) -> Vec<&dyn rusqlite::ToSql> {
        vec![
            &"text", // content_type
            &self.content,
            &self.content_hash,
            &(self.content_size as i64),
            &self.created_at,
            &self.accessed_at,
            &(self.access_count as i64),
            &self.is_favorite,
        ]
    }
}
```

#### 3.2 数据库操作 (`src/storage/database.rs`)
```rust
use crate::clipboard::types::{ClipboardItem, SearchFilter};
use crate::error::{ClipManagerError, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self { conn };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS clipboard_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_type TEXT NOT NULL,
                content TEXT NOT NULL,
                content_hash TEXT NOT NULL UNIQUE,
                content_size INTEGER NOT NULL,
                created_at DATETIME NOT NULL,
                accessed_at DATETIME NOT NULL,
                access_count INTEGER DEFAULT 0,
                is_favorite BOOLEAN DEFAULT FALSE
            )",
            [],
        )?;

        // 创建索引
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_created_at ON clipboard_items(created_at DESC)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_content_hash ON clipboard_items(content_hash)",
            [],
        )?;

        Ok(())
    }

    pub fn insert_item(&self, item: &ClipboardItem) -> Result<i64> {
        // 检查是否已存在相同内容
        if self.item_exists(&item.content_hash)? {
            return Err(ClipManagerError::Config {
                message: "内容已存在".to_string(),
            });
        }

        let mut stmt = self.conn.prepare(
            "INSERT INTO clipboard_items
             (content_type, content, content_hash, content_size, created_at, accessed_at, access_count, is_favorite)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)"
        )?;

        let id = stmt.insert(item.insert_params())?;

        // 保持最大条目数限制
        self.cleanup_old_items(100)?; // MVP 版本限制 100 条

        Ok(id)
    }

    pub fn get_items(&self, filter: &SearchFilter, limit: usize, offset: usize) -> Result<Vec<ClipboardItem>> {
        let mut query = "SELECT * FROM clipboard_items WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if !filter.query.is_empty() {
            query.push_str(" AND content LIKE ?");
            params.push(Box::new(format!("%{}%", filter.query)));
        }

        if filter.favorites_only {
            query.push_str(" AND is_favorite = 1");
        }

        query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
        params.push(Box::new(limit as i64));
        params.push(Box::new(offset as i64));

        let mut stmt = self.conn.prepare(&query)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let rows = stmt.query_map(&param_refs[..], ClipboardItem::from_row)?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }

        Ok(items)
    }

    pub fn delete_item(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM clipboard_items WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<()> {
        self.conn.execute("DELETE FROM clipboard_items", [])?;
        Ok(())
    }

    pub fn update_access(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE clipboard_items SET accessed_at = ?, access_count = access_count + 1 WHERE id = ?",
            params![Utc::now(), id],
        )?;
        Ok(())
    }

    fn item_exists(&self, content_hash: &str) -> Result<bool> {
        let exists: Option<i64> = self.conn
            .query_row(
                "SELECT 1 FROM clipboard_items WHERE content_hash = ?",
                params![content_hash],
                |row| row.get(0),
            )
            .optional()?;
        Ok(exists.is_some())
    }

    fn cleanup_old_items(&self, max_items: usize) -> Result<()> {
        self.conn.execute(
            "DELETE FROM clipboard_items WHERE id NOT IN (
                SELECT id FROM clipboard_items ORDER BY created_at DESC LIMIT ?
            )",
            params![max_items as i64],
        )?;
        Ok(())
    }

    pub fn get_item_count(&self) -> Result<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM clipboard_items",
            [],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }
}
```

### 13.4 步骤 4：剪切板监控实现

#### 4.1 剪切板监控服务 (`src/clipboard/monitor.rs`)
```rust
use crate::clipboard::types::ClipboardItem;
use crate::error::Result;
use arboard::Clipboard;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio::sync::watch;

pub struct ClipboardMonitor {
    clipboard: Clipboard,
    last_content: String,
    sender: mpsc::Sender<ClipboardItem>,
    is_running: bool,
}

impl ClipboardMonitor {
    pub fn new() -> Result<(Self, mpsc::Receiver<ClipboardItem>)> {
        let clipboard = Clipboard::new()?;
        let (sender, receiver) = mpsc::channel();

        let monitor = Self {
            clipboard,
            last_content: String::new(),
            sender,
            is_running: false,
        };

        Ok((monitor, receiver))
    }

    pub fn start(&mut self) -> Result<()> {
        if self.is_running {
            return Ok(());
        }

        self.is_running = true;

        // 获取初始剪切板内容
        if let Ok(content) = self.clipboard.get_text() {
            self.last_content = content;
        }

        let sender = self.sender.clone();
        let mut clipboard = Clipboard::new()?;
        let mut last_content = self.last_content.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(500)); // 每500ms检查一次

                if let Ok(current_content) = clipboard.get_text() {
                    if current_content != last_content && !current_content.trim().is_empty() {
                        let item = ClipboardItem::new(current_content.clone());

                        if let Err(_) = sender.send(item) {
                            // 接收端已关闭，退出监控
                            break;
                        }

                        last_content = current_content;
                    }
                }
            }
        });

        Ok(())
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn set_clipboard_content(&mut self, content: &str) -> Result<()> {
        self.clipboard.set_text(content)?;
        self.last_content = content.to_string();
        Ok(())
    }
}
```

#### 4.2 剪切板处理器 (`src/clipboard/handler.rs`)
```rust
use crate::clipboard::monitor::ClipboardMonitor;
use crate::clipboard::types::{ClipboardItem, SearchFilter};
use crate::error::Result;
use crate::storage::database::Database;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ClipboardHandler {
    database: Arc<Database>,
    monitor: Arc<Mutex<ClipboardMonitor>>,
    receiver: mpsc::Receiver<ClipboardItem>,
}

impl ClipboardHandler {
    pub fn new(database: Database) -> Result<Self> {
        let (monitor, receiver) = ClipboardMonitor::new()?;

        Ok(Self {
            database: Arc::new(database),
            monitor: Arc::new(Mutex::new(monitor)),
            receiver,
        })
    }

    pub fn start_monitoring(&mut self) -> Result<()> {
        // 启动剪切板监控
        {
            let mut monitor = self.monitor.lock().unwrap();
            monitor.start()?;
        }

        // 启动处理线程
        let database = Arc::clone(&self.database);
        let receiver = std::mem::replace(&mut self.receiver, {
            let (_, rx) = mpsc::channel();
            rx
        });

        thread::spawn(move || {
            while let Ok(item) = receiver.recv() {
                if let Err(e) = database.insert_item(&item) {
                    log::warn!("插入剪切板条目失败: {}", e);
                }
            }
        });

        Ok(())
    }

    pub fn copy_to_clipboard(&self, content: &str) -> Result<()> {
        let mut monitor = self.monitor.lock().unwrap();
        monitor.set_clipboard_content(content)
    }

    pub fn search_items(&self, filter: &SearchFilter, limit: usize, offset: usize) -> Result<Vec<ClipboardItem>> {
        self.database.get_items(filter, limit, offset)
    }

    pub fn delete_item(&self, id: i64) -> Result<()> {
        self.database.delete_item(id)
    }

    pub fn clear_all_items(&self) -> Result<()> {
        self.database.clear_all()
    }

    pub fn update_item_access(&self, id: i64) -> Result<()> {
        self.database.update_access(id)
    }

    pub fn get_item_count(&self) -> Result<usize> {
        self.database.get_item_count()
    }
}
```

### 13.5 步骤 5：用户界面实现

#### 5.1 主应用结构 (`src/app.rs`)
```rust
use crate::clipboard::handler::ClipboardHandler;
use crate::clipboard::types::{ClipboardItem, SearchFilter};
use crate::error::Result;
use crate::storage::database::Database;
use crate::ui::main_window::MainWindow;
use directories::ProjectDirs;
use eframe::egui;
use std::path::PathBuf;

pub struct ClipManagerApp {
    clipboard_handler: ClipboardHandler,
    main_window: MainWindow,
    items: Vec<ClipboardItem>,
    search_filter: SearchFilter,
    error_message: Option<String>,
}

impl ClipManagerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Result<Self> {
        // 设置数据库路径
        let db_path = Self::get_database_path()?;
        let database = Database::new(db_path)?;

        // 创建剪切板处理器
        let mut clipboard_handler = ClipboardHandler::new(database)?;
        clipboard_handler.start_monitoring()?;

        // 创建主窗口
        let main_window = MainWindow::new();

        let mut app = Self {
            clipboard_handler,
            main_window,
            items: Vec::new(),
            search_filter: SearchFilter::default(),
            error_message: None,
        };

        // 加载初始数据
        app.refresh_items();

        Ok(app)
    }

    fn get_database_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "clipmanager", "ClipManager")
            .ok_or_else(|| crate::error::ClipManagerError::Config {
                message: "无法确定数据目录".to_string(),
            })?;

        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;

        Ok(data_dir.join("clipmanager.db"))
    }

    fn refresh_items(&mut self) {
        match self.clipboard_handler.search_items(&self.search_filter, 100, 0) {
            Ok(items) => {
                self.items = items;
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(format!("加载数据失败: {}", e));
            }
        }
    }

    fn handle_item_action(&mut self, action: ItemAction) {
        match action {
            ItemAction::Copy(content) => {
                if let Err(e) = self.clipboard_handler.copy_to_clipboard(&content) {
                    self.error_message = Some(format!("复制失败: {}", e));
                }
            }
            ItemAction::Delete(id) => {
                if let Err(e) = self.clipboard_handler.delete_item(id) {
                    self.error_message = Some(format!("删除失败: {}", e));
                } else {
                    self.refresh_items();
                }
            }
            ItemAction::ClearAll => {
                if let Err(e) = self.clipboard_handler.clear_all_items() {
                    self.error_message = Some(format!("清空失败: {}", e));
                } else {
                    self.refresh_items();
                }
            }
            ItemAction::UpdateAccess(id) => {
                if let Err(e) = self.clipboard_handler.update_item_access(id) {
                    log::warn!("更新访问记录失败: {}", e);
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ItemAction {
    Copy(String),
    Delete(i64),
    ClearAll,
    UpdateAccess(i64),
}

impl eframe::App for ClipManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 处理搜索过滤器变化
        let old_filter = self.search_filter.clone();

        // 渲染主窗口
        if let Some(action) = self.main_window.show(
            ctx,
            &mut self.items,
            &mut self.search_filter,
            &self.error_message,
        ) {
            self.handle_item_action(action);
        }

        // 如果搜索条件改变，刷新数据
        if self.search_filter.query != old_filter.query {
            self.refresh_items();
        }

        // 定期刷新数据 (每5秒)
        ctx.request_repaint_after(std::time::Duration::from_secs(5));
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // 保存窗口状态等配置
        eframe::set_value(storage, "search_query", &self.search_filter.query);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // 清理资源
        log::info!("应用程序退出");
    }
}
```

#### 5.2 主窗口实现 (`src/ui/main_window.rs`)
```rust
use crate::app::ItemAction;
use crate::clipboard::types::{ClipboardItem, SearchFilter};
use eframe::egui;

pub struct MainWindow {
    selected_item: Option<usize>,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            selected_item: None,
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        items: &mut [ClipboardItem],
        search_filter: &mut SearchFilter,
        error_message: &Option<String>,
    ) -> Option<ItemAction> {
        let mut action = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            // 标题栏
            ui.horizontal(|ui| {
                ui.heading("ClipManager");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("清空全部").clicked() {
                        action = Some(ItemAction::ClearAll);
                    }
                    if ui.button("设置").clicked() {
                        // TODO: 打开设置窗口
                    }
                });
            });

            ui.separator();

            // 搜索栏
            ui.horizontal(|ui| {
                ui.label("搜索:");
                ui.text_edit_singleline(&mut search_filter.query);

                ui.separator();

                ui.label("类型:");
                egui::ComboBox::from_label("")
                    .selected_text("全部")
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut search_filter.content_type, None, "全部");
                        // TODO: 添加其他类型选项
                    });
            });

            ui.separator();

            // 错误消息显示
            if let Some(error) = error_message {
                ui.colored_label(egui::Color32::RED, error);
                ui.separator();
            }

            // 条目列表
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if items.is_empty() {
                        ui.centered_and_justified(|ui| {
                            ui.label("暂无剪切板历史记录");
                        });
                    } else {
                        for (index, item) in items.iter().enumerate() {
                            if let Some(item_action) = self.show_item(ui, item, index) {
                                action = Some(item_action);
                            }
                        }
                    }
                });

            // 状态栏
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!("共 {} 条记录", items.len()));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("ClipManager v0.1.0");
                });
            });
        });

        action
    }

    fn show_item(&mut self, ui: &mut egui::Ui, item: &ClipboardItem, index: usize) -> Option<ItemAction> {
        let mut action = None;

        let response = ui.allocate_response(
            egui::Vec2::new(ui.available_width(), 60.0),
            egui::Sense::click(),
        );

        // 绘制条目背景
        let rect = response.rect;
        let is_selected = self.selected_item == Some(index);
        let is_hovered = response.hovered();

        let bg_color = if is_selected {
            ui.style().visuals.selection.bg_fill
        } else if is_hovered {
            ui.style().visuals.widgets.hovered.bg_fill
        } else {
            ui.style().visuals.window_fill
        };

        ui.painter().rect_filled(rect, 4.0, bg_color);

        // 绘制条目内容
        ui.allocate_ui_at_rect(rect.shrink(8.0), |ui| {
            ui.horizontal(|ui| {
                // 内容类型图标
                ui.label("📄");

                ui.vertical(|ui| {
                    // 内容预览
                    let preview = item.preview(50);
                    ui.label(egui::RichText::new(preview).strong());

                    // 时间和大小信息
                    ui.horizontal(|ui| {
                        ui.label(format!("{}", item.created_at.format("%H:%M:%S")));
                        ui.separator();
                        ui.label(format!("{} 字符", item.content_size));
                        if item.access_count > 0 {
                            ui.separator();
                            ui.label(format!("使用 {} 次", item.access_count));
                        }
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("删除").clicked() {
                        if let Some(id) = item.id {
                            action = Some(ItemAction::Delete(id));
                        }
                    }
                });
            });
        });

        // 处理点击事件
        if response.clicked() {
            self.selected_item = Some(index);
            action = Some(ItemAction::Copy(item.content.clone()));

            if let Some(id) = item.id {
                // 同时更新访问记录
                if action.is_none() {
                    action = Some(ItemAction::UpdateAccess(id));
                }
            }
        }

        // 右键菜单
        response.context_menu(|ui| {
            if ui.button("复制").clicked() {
                action = Some(ItemAction::Copy(item.content.clone()));
                ui.close_menu();
            }
            if ui.button("删除").clicked() {
                if let Some(id) = item.id {
                    action = Some(ItemAction::Delete(id));
                }
                ui.close_menu();
            }
        });

        action
    }
}
```

### 13.6 步骤 6：主程序入口和模块定义

#### 6.1 主程序入口 (`src/main.rs`)
```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // 在 Windows 上隐藏控制台

mod app;
mod clipboard;
mod error;
mod storage;
mod ui;
mod config;

use app::ClipManagerApp;
use eframe::egui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!("启动 ClipManager");

    // 设置应用程序选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([400.0, 300.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    // 运行应用程序
    eframe::run_native(
        "ClipManager",
        options,
        Box::new(|cc| {
            // 设置样式
            setup_custom_style(&cc.egui_ctx);

            match ClipManagerApp::new(cc) {
                Ok(app) => Ok(Box::new(app)),
                Err(e) => {
                    log::error!("应用程序初始化失败: {}", e);
                    Err(Box::new(e))
                }
            }
        }),
    )?;

    Ok(())
}

fn load_icon() -> egui::IconData {
    // 这里应该加载应用程序图标
    // 暂时返回一个空的图标数据
    egui::IconData {
        rgba: vec![0; 32 * 32 * 4],
        width: 32,
        height: 32,
    }
}

fn setup_custom_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // 自定义字体大小
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(14.0, egui::FontFamily::Proportional),
    );

    // 自定义间距
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    style.spacing.button_padding = egui::vec2(12.0, 6.0);

    ctx.set_style(style);
}
```

#### 6.2 模块定义文件

##### `src/clipboard/mod.rs`
```rust
pub mod monitor;
pub mod types;
pub mod handler;

pub use handler::ClipboardHandler;
pub use monitor::ClipboardMonitor;
pub use types::{ClipboardItem, ContentType, SearchFilter};
```

##### `src/storage/mod.rs`
```rust
pub mod database;
pub mod models;

pub use database::Database;
```

##### `src/ui/mod.rs`
```rust
pub mod main_window;
pub mod components;

pub use main_window::MainWindow;
```

##### `src/ui/components/mod.rs`
```rust
// 为后续版本预留的组件模块
// pub mod item_list;
// pub mod search_bar;
// pub mod settings_dialog;
```

##### `src/config/mod.rs`
```rust
pub mod settings;

// 为后续版本预留
// pub use settings::AppConfig;
```

### 13.7 步骤 7：构建和测试配置

#### 7.1 基本测试 (`src/storage/database.rs` 测试部分)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_db() -> Database {
        let temp_file = NamedTempFile::new().unwrap();
        Database::new(temp_file.path()).unwrap()
    }

    #[test]
    fn test_insert_and_get_items() {
        let db = create_test_db();
        let item = ClipboardItem::new("测试内容".to_string());

        // 插入条目
        let id = db.insert_item(&item).unwrap();
        assert!(id > 0);

        // 获取条目
        let filter = SearchFilter::default();
        let items = db.get_items(&filter, 10, 0).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].content, "测试内容");
    }

    #[test]
    fn test_duplicate_content() {
        let db = create_test_db();
        let item1 = ClipboardItem::new("重复内容".to_string());
        let item2 = ClipboardItem::new("重复内容".to_string());

        // 第一次插入应该成功
        assert!(db.insert_item(&item1).is_ok());

        // 第二次插入相同内容应该失败
        assert!(db.insert_item(&item2).is_err());
    }

    #[test]
    fn test_search_filter() {
        let db = create_test_db();

        // 插入测试数据
        let items = vec![
            ClipboardItem::new("Hello World".to_string()),
            ClipboardItem::new("Rust Programming".to_string()),
            ClipboardItem::new("egui Tutorial".to_string()),
        ];

        for item in items {
            db.insert_item(&item).unwrap();
        }

        // 测试搜索
        let mut filter = SearchFilter::default();
        filter.query = "Rust".to_string();

        let results = db.get_items(&filter, 10, 0).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].content.contains("Rust"));
    }

    #[test]
    fn test_cleanup_old_items() {
        let db = create_test_db();

        // 插入超过限制的条目
        for i in 0..105 {
            let item = ClipboardItem::new(format!("内容 {}", i));
            db.insert_item(&item).unwrap();
        }

        // 验证只保留了最新的 100 条
        let count = db.get_item_count().unwrap();
        assert_eq!(count, 100);
    }
}
```

#### 7.2 基准测试 (`benches/clipboard_benchmark.rs`)
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use clipmanager::clipboard::types::ClipboardItem;
use clipmanager::storage::database::Database;
use tempfile::NamedTempFile;

fn benchmark_insert_items(c: &mut Criterion) {
    let temp_file = NamedTempFile::new().unwrap();
    let db = Database::new(temp_file.path()).unwrap();

    c.bench_function("insert_100_items", |b| {
        b.iter(|| {
            for i in 0..100 {
                let item = ClipboardItem::new(format!("测试内容 {}", black_box(i)));
                let _ = db.insert_item(&item);
            }
        })
    });
}

fn benchmark_search_items(c: &mut Criterion) {
    let temp_file = NamedTempFile::new().unwrap();
    let db = Database::new(temp_file.path()).unwrap();

    // 预先插入数据
    for i in 0..1000 {
        let item = ClipboardItem::new(format!("搜索测试内容 {}", i));
        let _ = db.insert_item(&item);
    }

    c.bench_function("search_in_1000_items", |b| {
        b.iter(|| {
            let mut filter = clipmanager::clipboard::types::SearchFilter::default();
            filter.query = black_box("测试".to_string());
            let _ = db.get_items(&filter, 50, 0);
        })
    });
}

criterion_group!(benches, benchmark_insert_items, benchmark_search_items);
criterion_main!(benches);
```

## 14. 快速开始指南

### 14.1 环境准备

1. **安装 Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **克隆项目**
   ```bash
   git clone <repository-url>
   cd clipmanager
   ```

3. **安装依赖**
   ```bash
   cargo build
   ```

### 14.2 开发运行

1. **运行开发版本**
   ```bash
   cargo run
   ```

2. **运行测试**
   ```bash
   cargo test
   ```

3. **运行基准测试**
   ```bash
   cargo bench
   ```

### 14.3 构建发布版本

1. **构建优化版本**
   ```bash
   cargo build --release
   ```

2. **生成安装包** (需要额外工具)
   ```bash
   # Windows (需要 WiX Toolset)
   cargo install cargo-wix
   cargo wix

   # macOS (需要 create-dmg)
   cargo install cargo-bundle
   cargo bundle --release

   # Linux (需要 AppImage 工具)
   cargo install cargo-appimage
   cargo appimage
   ```

## 15. 后续开发计划

### 15.1 短期目标 (1-2个月)
- [ ] 完成 MVP 功能开发
- [ ] 添加图片支持
- [ ] 实现系统托盘集成
- [ ] 添加全局快捷键
- [ ] 完善错误处理和用户反馈

### 15.2 中期目标 (3-6个月)
- [ ] 实现标签和分类系统
- [ ] 添加数据导出/导入功能
- [ ] 支持富文本 (HTML) 内容
- [ ] 实现插件系统
- [ ] 添加主题和个性化选项

### 15.3 长期目标 (6个月以上)
- [ ] 云同步功能
- [ ] 移动端应用 (Android/iOS)
- [ ] 团队协作功能
- [ ] AI 辅助内容分析
- [ ] 企业级安全功能

## 16. 贡献指南

### 16.1 代码规范
- 使用 `rustfmt` 格式化代码
- 使用 `clippy` 进行代码检查
- 编写充分的单元测试
- 添加适当的文档注释

### 16.2 提交规范
- 使用清晰的提交消息
- 每个提交只包含一个逻辑变更
- 在提交前运行所有测试
- 更新相关文档

### 16.3 问题报告
- 使用 GitHub Issues 报告 bug
- 提供详细的重现步骤
- 包含系统环境信息
- 附上相关的日志信息

---

## 总结

本技术设计文档为 ClipManager 项目提供了全面的技术方案，包括：

1. **完整的功能需求分析**：从 MVP 到高级功能的详细规划
2. **清晰的技术架构**：模块化设计，易于维护和扩展
3. **详细的实现方案**：包含完整的代码示例和最佳实践
4. **实用的开发指南**：从环境搭建到发布部署的完整流程

该文档将作为项目开发的重要参考，随着开发进度持续更新和完善。通过遵循本文档的设计方案，可以构建出一个高质量、跨平台的剪切板管理工具。

*本文档版本：v1.0*
*最后更新：2025-01-21*
