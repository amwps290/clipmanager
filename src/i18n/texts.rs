//! Text constants for different languages

use super::TextKey;
use std::collections::HashMap;

/// Get English text constants
pub fn get_english_texts() -> HashMap<TextKey, &'static str> {
    let mut texts = HashMap::new();

    // Application
    texts.insert(TextKey::AppTitle, "ClipManager");
    texts.insert(TextKey::AppVersion, "ClipManager v0.1.0");

    // Main Window
    texts.insert(TextKey::SearchPlaceholder, "Search:");
    texts.insert(TextKey::TypeFilterAll, "All");
    texts.insert(TextKey::TypeFilterText, "Text");
    texts.insert(TextKey::TypeFilterImage, "Image");
    texts.insert(TextKey::ClearAll, "Clear All");
    texts.insert(TextKey::Settings, "Settings");

    // Item List
    texts.insert(TextKey::NoRecords, "No clipboard history records");
    texts.insert(TextKey::RecordsCount, "records");
    texts.insert(TextKey::CharactersCount, "characters");
    texts.insert(TextKey::UsedTimes, "times used");

    // Actions
    texts.insert(TextKey::Copy, "Copy");
    texts.insert(TextKey::Delete, "Delete");

    // Status Messages
    texts.insert(TextKey::LoadDataFailed, "Failed to load data: {}");
    texts.insert(TextKey::CopyFailed, "Failed to copy: {}");
    texts.insert(TextKey::DeleteFailed, "Failed to delete: {}");
    texts.insert(TextKey::ClearFailed, "Failed to clear: {}");
    texts.insert(
        TextKey::UpdateAccessFailed,
        "Failed to update access record: {}",
    );

    // Database
    texts.insert(TextKey::ContentExists, "Content already exists");
    texts.insert(TextKey::DatabaseError, "Database error: {}");
    texts.insert(TextKey::ClipboardError, "Clipboard operation error: {}");
    texts.insert(TextKey::IoError, "IO error: {}");
    texts.insert(TextKey::SerializationError, "Serialization error: {}");
    texts.insert(TextKey::ConfigError, "Configuration error: {}");
    texts.insert(
        TextKey::ContentTooLarge,
        "Content too large: {} bytes, maximum allowed {} bytes",
    );
    texts.insert(TextKey::UnsupportedContentType, "Unsupported content type");

    // Time
    texts.insert(TextKey::TimeFormat, "%H:%M:%S");

    // Context Menu
    texts.insert(TextKey::ContextCopy, "Copy");
    texts.insert(TextKey::ContextDelete, "Delete");
    texts.insert(TextKey::Favorite, "Favorite");
    texts.insert(TextKey::Unfavorite, "Unfavorite");

    texts
}

/// Get Chinese text constants (for future use)
pub fn get_chinese_texts() -> HashMap<TextKey, &'static str> {
    let mut texts = HashMap::new();

    // Application
    texts.insert(TextKey::AppTitle, "剪切板管理器");
    texts.insert(TextKey::AppVersion, "ClipManager v0.1.0");

    // Main Window
    texts.insert(TextKey::SearchPlaceholder, "搜索:");
    texts.insert(TextKey::TypeFilterAll, "全部");
    texts.insert(TextKey::TypeFilterText, "文本");
    texts.insert(TextKey::TypeFilterImage, "图片");
    texts.insert(TextKey::ClearAll, "清空全部");
    texts.insert(TextKey::Settings, "设置");

    // Item List
    texts.insert(TextKey::NoRecords, "暂无剪切板历史记录");
    texts.insert(TextKey::RecordsCount, "条记录");
    texts.insert(TextKey::CharactersCount, "字符");
    texts.insert(TextKey::UsedTimes, "次使用");

    // Actions
    texts.insert(TextKey::Copy, "复制");
    texts.insert(TextKey::Delete, "删除");
    texts.insert(TextKey::Favorite, "收藏");
    texts.insert(TextKey::Unfavorite, "取消收藏");

    // Status Messages
    texts.insert(TextKey::LoadDataFailed, "加载数据失败: {}");
    texts.insert(TextKey::CopyFailed, "复制失败: {}");
    texts.insert(TextKey::DeleteFailed, "删除失败: {}");
    texts.insert(TextKey::ClearFailed, "清空失败: {}");
    texts.insert(TextKey::UpdateAccessFailed, "更新访问记录失败: {}");

    // Database
    texts.insert(TextKey::ContentExists, "内容已存在");
    texts.insert(TextKey::DatabaseError, "数据库错误: {}");
    texts.insert(TextKey::ClipboardError, "剪切板操作错误: {}");
    texts.insert(TextKey::IoError, "IO 错误: {}");
    texts.insert(TextKey::SerializationError, "序列化错误: {}");
    texts.insert(TextKey::ConfigError, "配置错误: {}");
    texts.insert(
        TextKey::ContentTooLarge,
        "内容过大: {} 字节，最大允许 {} 字节",
    );
    texts.insert(TextKey::UnsupportedContentType, "不支持的内容类型");

    // Time
    texts.insert(TextKey::TimeFormat, "%H:%M:%S");

    // Context Menu
    texts.insert(TextKey::ContextCopy, "复制");
    texts.insert(TextKey::ContextDelete, "删除");

    texts
}
