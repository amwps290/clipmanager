use crate::clipboard::types::{
    ClipboardContent, ClipboardItem, ContentType, ImageData, ImageFormat,
};
use rusqlite::Row;

impl ClipboardItem {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        let content_type_str: String = row.get("content_type")?;
        let content_type = match content_type_str.as_str() {
            "text" => ContentType::Text,
            "image" => ContentType::Image,
            _ => ContentType::Text, // 默认为文本
        };

        let content = match content_type {
            ContentType::Text => {
                // 尝试从新字段获取，如果不存在则从旧字段获取
                let text = if let Ok(text) = row.get::<_, String>("content_text") {
                    text
                } else {
                    // 向后兼容：从旧的 content 字段获取
                    row.get::<_, String>("content").unwrap_or_default()
                };
                ClipboardContent::Text(text)
            }
            ContentType::Image => {
                let data: Vec<u8> = row.get("content_data")?;
                let width: i64 = row.get("image_width")?;
                let height: i64 = row.get("image_height")?;
                let format_str: String = row.get("image_format")?;

                let format = match format_str.as_str() {
                    "png" => ImageFormat::Png,
                    "jpeg" => ImageFormat::Jpeg,
                    "gif" => ImageFormat::Gif,
                    "bmp" => ImageFormat::Bmp,
                    _ => ImageFormat::Png, // 默认为 PNG
                };

                let image_data = ImageData::new(data, format, width as u32, height as u32);
                ClipboardContent::Image(image_data)
            }
        };

        Ok(Self {
            id: Some(row.get("id")?),
            content_type,
            content,
            content_hash: row.get("content_hash")?,
            content_size: row.get::<_, i64>("content_size")? as usize,
            created_at: row.get("created_at")?,
            accessed_at: row.get("accessed_at")?,
            access_count: row.get::<_, i64>("access_count")? as u32,
            is_favorite: row.get("is_favorite")?,
        })
    }
}
