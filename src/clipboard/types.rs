use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContentType {
    Text,
    Image,
    // Html, // 后续版本支持
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardContent {
    Text(String),
    Image(ImageData),
    // Html { html: String, text: String }, // 后续版本支持
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub format: ImageFormat,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    Bmp,
}

impl ClipboardItem {
    pub fn new_text(content: String) -> Self {
        let content_size = content.len();
        let content_hash = Self::calculate_hash_text(&content);
        let now = Utc::now();

        Self {
            id: None,
            content_type: ContentType::Text,
            content: ClipboardContent::Text(content),
            content_hash,
            content_size,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            is_favorite: false,
        }
    }

    pub fn new_image(image_data: ImageData) -> Self {
        let content_size = image_data.data.len();
        let content_hash = Self::calculate_hash_bytes(&image_data.data);
        let now = Utc::now();

        Self {
            id: None,
            content_type: ContentType::Image,
            content: ClipboardContent::Image(image_data),
            content_hash,
            content_size,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            is_favorite: false,
        }
    }

    // 保持向后兼容性
    pub fn new(content: String) -> Self {
        Self::new_text(content)
    }

    fn calculate_hash_text(content: &str) -> String {
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn calculate_hash_bytes(data: &[u8]) -> String {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn preview(&self, max_length: usize) -> String {
        match &self.content {
            ClipboardContent::Text(text) => {
                // 安全地截断字符串，避免在多字节字符中间截断
                if text.chars().count() <= max_length {
                    text.clone()
                } else {
                    let truncated: String = text.chars().take(max_length).collect();
                    format!("{}...", truncated)
                }
            }
            ClipboardContent::Image(image_data) => {
                format!(
                    "Image ({}x{}, {} bytes)",
                    image_data.width,
                    image_data.height,
                    image_data.data.len()
                )
            }
        }
    }

    pub fn get_text_content(&self) -> Option<&str> {
        match &self.content {
            ClipboardContent::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn get_image_data(&self) -> Option<&ImageData> {
        match &self.content {
            ClipboardContent::Image(image_data) => Some(image_data),
            _ => None,
        }
    }

    pub fn update_access(&mut self) {
        self.accessed_at = Utc::now();
        self.access_count += 1;
    }
}

impl ImageData {
    pub fn new(data: Vec<u8>, format: ImageFormat, width: u32, height: u32) -> Self {
        Self {
            data,
            format,
            width,
            height,
        }
    }

    pub fn size_mb(&self) -> f64 {
        self.data.len() as f64 / (1024.0 * 1024.0)
    }
}

impl ImageFormat {
    pub fn from_mime_type(mime_type: &str) -> Option<Self> {
        match mime_type {
            "image/png" => Some(ImageFormat::Png),
            "image/jpeg" | "image/jpg" => Some(ImageFormat::Jpeg),
            "image/gif" => Some(ImageFormat::Gif),
            "image/bmp" => Some(ImageFormat::Bmp),
            _ => None,
        }
    }

    pub fn to_mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Png => "image/png",
            ImageFormat::Jpeg => "image/jpeg",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Bmp => "image/bmp",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::Gif => "gif",
            ImageFormat::Bmp => "bmp",
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preview_with_chinese_characters() {
        // 测试包含中文字符的文本预览
        let chinese_text = "Rust 中是否有库可以获取各种系统中标准字体的路径呢？这是一个很长的中文文本用来测试字符串截断功能。";
        let item = ClipboardItem {
            id: Some(1),
            content_type: ContentType::Text,
            content: ClipboardContent::Text(chinese_text.to_string()),
            content_hash: "test_hash".to_string(),
            content_size: chinese_text.len(),
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            is_favorite: false,
        };

        // 测试不同长度的预览
        let preview_10 = item.preview(10);
        let preview_25 = item.preview(25);
        let preview_50 = item.preview(50);

        println!("Original text: {}", chinese_text);
        println!("Preview 10: {}", preview_10);
        println!("Preview 25: {}", preview_25);
        println!("Preview 50: {}", preview_50);

        // 验证预览不会panic
        assert!(preview_10.len() > 0);
        assert!(preview_25.len() > 0);
        assert!(preview_50.len() > 0);

        // 验证短预览包含省略号
        assert!(preview_10.contains("..."));
        assert!(preview_25.contains("..."));

        println!("✓ Chinese character preview test passed");
    }

    #[test]
    fn test_preview_with_mixed_characters() {
        // 测试混合字符（英文、中文、数字、符号）
        let mixed_text = "Hello 世界! 123 测试 ABC 中文 456 English 测试文本";
        let item = ClipboardItem {
            id: Some(1),
            content_type: ContentType::Text,
            content: ClipboardContent::Text(mixed_text.to_string()),
            content_hash: "test_hash".to_string(),
            content_size: mixed_text.len(),
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            is_favorite: false,
        };

        // 测试各种长度
        for length in [5, 10, 15, 20, 30] {
            let preview = item.preview(length);
            println!("Length {}: {}", length, preview);

            // 验证不会panic
            assert!(preview.len() > 0);

            // 如果原文本比预览长度长，应该包含省略号
            if mixed_text.chars().count() > length {
                assert!(preview.contains("..."));
            }
        }

        println!("✓ Mixed character preview test passed");
    }

    #[test]
    fn test_preview_edge_cases() {
        // 测试边界情况

        // 空字符串
        let empty_item = ClipboardItem {
            id: Some(1),
            content_type: ContentType::Text,
            content: ClipboardContent::Text("".to_string()),
            content_hash: "test_hash".to_string(),
            content_size: 0,
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            is_favorite: false,
        };

        let empty_preview = empty_item.preview(10);
        assert_eq!(empty_preview, "");

        // 单个中文字符
        let single_char_item = ClipboardItem {
            id: Some(1),
            content_type: ContentType::Text,
            content: ClipboardContent::Text("中".to_string()),
            content_hash: "test_hash".to_string(),
            content_size: 3, // UTF-8中一个中文字符占3字节
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            access_count: 0,
            is_favorite: false,
        };

        let single_preview = single_char_item.preview(1);
        assert_eq!(single_preview, "中");

        println!("✓ Edge cases preview test passed");
    }
}
