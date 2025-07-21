use crate::clipboard::types::{ClipboardItem, SearchFilter};
use crate::error::{ClipManagerError, Result};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self {
            conn: Arc::new(Mutex::new(conn)),
        };
        db.initialize()?;
        Ok(db)
    }

    fn initialize(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // 创建用户版本表来跟踪数据库版本
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY
            )",
            [],
        )?;

        // 获取当前数据库版本
        let current_version: i64 = conn
            .query_row(
                "SELECT COALESCE(MAX(version), 0) FROM schema_version",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        // 执行迁移
        self.migrate(&conn, current_version)?;

        Ok(())
    }

    fn migrate(&self, conn: &Connection, current_version: i64) -> Result<()> {
        // 检查是否存在旧表结构
        let table_exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='clipboard_items'",
                [],
                |row| {
                    let count: i64 = row.get(0)?;
                    Ok(count > 0)
                },
            )
            .unwrap_or(false);

        if current_version == 0 && table_exists {
            // 情况1: 存在旧表，需要迁移
            log::info!("Migrating existing database from old schema");

            // 检查是否有旧的 content 列
            let has_old_content: bool = conn
                .prepare("SELECT content FROM clipboard_items LIMIT 1")
                .is_ok();

            if has_old_content {
                log::info!("Found old content column, performing migration");

                // 添加新列
                conn.execute(
                    "ALTER TABLE clipboard_items ADD COLUMN content_text TEXT",
                    [],
                )
                .ok();
                conn.execute(
                    "ALTER TABLE clipboard_items ADD COLUMN content_data BLOB",
                    [],
                )
                .ok();
                conn.execute(
                    "ALTER TABLE clipboard_items ADD COLUMN image_width INTEGER",
                    [],
                )
                .ok();
                conn.execute(
                    "ALTER TABLE clipboard_items ADD COLUMN image_height INTEGER",
                    [],
                )
                .ok();
                conn.execute(
                    "ALTER TABLE clipboard_items ADD COLUMN image_format TEXT",
                    [],
                )
                .ok();

                // 迁移数据：将旧的 content 复制到 content_text
                conn.execute(
                    "UPDATE clipboard_items SET content_text = content WHERE content_text IS NULL",
                    [],
                )?;

                log::info!("Data migration completed");
            }

            // 创建缺失的索引
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_created_at ON clipboard_items(created_at DESC)",
                [],
            )?;
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_content_hash ON clipboard_items(content_hash)",
                [],
            )?;
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_content_type ON clipboard_items(content_type)",
                [],
            )?;

            // 更新版本
            conn.execute(
                "INSERT OR REPLACE INTO schema_version (version) VALUES (1)",
                [],
            )?;
        } else if current_version == 0 && !table_exists {
            // 情况2: 全新安装，创建新表
            log::info!("Creating new database with current schema");

            conn.execute(
                "CREATE TABLE clipboard_items (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    content_type TEXT NOT NULL,
                    content_text TEXT,
                    content_data BLOB,
                    content_hash TEXT NOT NULL UNIQUE,
                    content_size INTEGER NOT NULL,
                    created_at DATETIME NOT NULL,
                    accessed_at DATETIME NOT NULL,
                    access_count INTEGER DEFAULT 0,
                    is_favorite BOOLEAN DEFAULT FALSE,
                    image_width INTEGER,
                    image_height INTEGER,
                    image_format TEXT
                )",
                [],
            )?;

            // 创建索引
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_created_at ON clipboard_items(created_at DESC)",
                [],
            )?;
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_content_hash ON clipboard_items(content_hash)",
                [],
            )?;
            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_content_type ON clipboard_items(content_type)",
                [],
            )?;

            // 更新版本
            conn.execute(
                "INSERT OR REPLACE INTO schema_version (version) VALUES (1)",
                [],
            )?;
        }

        // 未来版本的迁移可以在这里添加
        // if current_version < 2 { ... }

        Ok(())
    }

    pub fn insert_item(&self, item: &ClipboardItem) -> Result<i64> {
        use crate::clipboard::types::ClipboardContent;

        log::debug!("Attempting to insert item with hash: {}", item.content_hash);

        // 检查是否已存在相同内容
        if self.item_exists(&item.content_hash)? {
            log::info!(
                "Item with hash {} already exists, skipping",
                item.content_hash
            );
            return Err(ClipManagerError::Config {
                message: "Content already exists".to_string(),
            });
        }

        let conn = self.conn.lock().unwrap();

        let (content_type_str, content_text, content_data, image_width, image_height, image_format) =
            match &item.content {
                ClipboardContent::Text(text) => {
                    ("text", Some(text.as_str()), None, None, None, None)
                }
                ClipboardContent::Image(image_data) => {
                    let format_str = match image_data.format {
                        crate::clipboard::types::ImageFormat::Png => "png",
                        crate::clipboard::types::ImageFormat::Jpeg => "jpeg",
                        crate::clipboard::types::ImageFormat::Gif => "gif",
                        crate::clipboard::types::ImageFormat::Bmp => "bmp",
                    };
                    (
                        "image",
                        None,
                        Some(image_data.data.as_slice()),
                        Some(image_data.width as i64),
                        Some(image_data.height as i64),
                        Some(format_str),
                    )
                }
            };

        let mut stmt = conn.prepare(
            "INSERT INTO clipboard_items
             (content_type, content_text, content_data, content_hash, content_size,
              created_at, accessed_at, access_count, is_favorite,
              image_width, image_height, image_format)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        )?;

        let id = stmt.insert(params![
            content_type_str,
            content_text,
            content_data,
            &item.content_hash,
            item.content_size as i64,
            &item.created_at,
            &item.accessed_at,
            item.access_count as i64,
            &item.is_favorite,
            image_width,
            image_height,
            image_format,
        ])?;

        drop(stmt);
        drop(conn);

        log::info!(
            "Successfully inserted item with ID: {} (type: {}, size: {} bytes)",
            id,
            content_type_str,
            item.content_size
        );

        // 保持最大条目数限制
        self.cleanup_old_items(1000)?; // 默认限制 1000 条

        Ok(id)
    }

    pub fn get_items(
        &self,
        filter: &SearchFilter,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();

        let mut query = "SELECT * FROM clipboard_items WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        // 添加搜索条件
        if !filter.query.is_empty() {
            query.push_str(" AND content_text LIKE ?");
            params.push(Box::new(format!("%{}%", filter.query)));
        }

        // 添加内容类型过滤
        if let Some(content_type) = &filter.content_type {
            query.push_str(" AND content_type = ?");
            let type_str = match content_type {
                crate::clipboard::types::ContentType::Text => "text",
                crate::clipboard::types::ContentType::Image => "image",
            };
            params.push(Box::new(type_str.to_string()));
        }

        // 添加收藏过滤
        if filter.favorites_only {
            query.push_str(" AND is_favorite = 1");
        }

        // 添加排序和分页
        query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");
        params.push(Box::new(limit as i64));
        params.push(Box::new(offset as i64));

        let mut stmt = conn.prepare(&query)?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let rows = stmt.query_map(&param_refs[..], ClipboardItem::from_row)?;

        let mut items = Vec::new();
        for row in rows {
            items.push(row?);
        }
        Ok(items)
    }

    pub fn delete_item(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM clipboard_items WHERE id = ?", params![id])?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM clipboard_items", [])?;
        Ok(())
    }

    pub fn update_access(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE clipboard_items SET accessed_at = ?, access_count = access_count + 1 WHERE id = ?",
            params![Utc::now(), id],
        )?;
        Ok(())
    }

    pub fn update_favorite(&self, id: i64, is_favorite: bool) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE clipboard_items SET is_favorite = ? WHERE id = ?",
            params![is_favorite, id],
        )?;
        Ok(())
    }

    pub fn cleanup_with_limit(&self, max_items: usize) -> Result<()> {
        self.cleanup_old_items(max_items)
    }

    fn item_exists(&self, content_hash: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let exists: Option<i64> = conn
            .query_row(
                "SELECT 1 FROM clipboard_items WHERE content_hash = ?",
                params![content_hash],
                |row| row.get(0),
            )
            .optional()?;
        Ok(exists.is_some())
    }

    fn cleanup_old_items(&self, max_items: usize) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM clipboard_items WHERE id NOT IN (
                SELECT id FROM clipboard_items ORDER BY created_at DESC LIMIT ?
            )",
            params![max_items as i64],
        )?;
        Ok(())
    }

    pub fn get_item_count(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 =
            conn.query_row("SELECT COUNT(*) FROM clipboard_items", [], |row| row.get(0))?;
        Ok(count as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_db() -> Database {
        // 使用内存数据库进行测试
        Database::new(":memory:").unwrap()
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
        if let Some(text) = items[0].get_text_content() {
            assert_eq!(text, "测试内容");
        } else {
            panic!("Expected text content");
        }
    }

    #[test]
    fn test_new_schema_insert() {
        let db = create_test_db();

        // Test text item
        let text_item = ClipboardItem::new_text("New schema test".to_string());
        let id = db.insert_item(&text_item).unwrap();
        assert!(id > 0);
        println!("Inserted text item with ID: {}", id);

        // Test retrieval
        let filter = SearchFilter::default();
        let items = db.get_items(&filter, 10, 0).unwrap();
        assert_eq!(items.len(), 1);

        let retrieved_item = &items[0];
        assert_eq!(retrieved_item.id, Some(id));
        if let Some(text) = retrieved_item.get_text_content() {
            assert_eq!(text, "New schema test");
        } else {
            panic!("Expected text content");
        }

        println!("✓ New schema test passed");
    }

    #[test]
    fn test_favorite_functionality() {
        let db = create_test_db();

        // Insert test item
        let item = ClipboardItem::new_text("Favorite test".to_string());
        let id = db.insert_item(&item).unwrap();

        // Update as favorite
        db.update_favorite(id, true).unwrap();

        // Search for favorites
        let mut filter = SearchFilter::default();
        filter.favorites_only = true;
        let favorites = db.get_items(&filter, 10, 0).unwrap();

        assert_eq!(favorites.len(), 1);
        assert!(favorites[0].is_favorite);

        println!("✓ Favorite functionality test passed");
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
        if let Some(text) = results[0].get_text_content() {
            assert!(text.contains("Rust"));
        } else {
            panic!("Expected text content");
        }
    }

    #[test]
    fn test_cleanup_old_items() {
        let db = create_test_db();

        // 插入超过限制的条目
        for i in 0..105 {
            let item = ClipboardItem::new(format!("内容 {}", i));
            db.insert_item(&item).unwrap();
        }

        // 手动执行清理，限制为100条
        db.cleanup_with_limit(100).unwrap();

        // 验证只保留了最新的 100 条
        let count = db.get_item_count().unwrap();
        assert_eq!(count, 100);
    }
}
