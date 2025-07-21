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
            log::info!("Clipboard handler thread started");
            while let Ok(item) = receiver.recv() {
                log::info!(
                    "Received clipboard item for insertion: {} bytes",
                    item.content_size
                );
                match database.insert_item(&item) {
                    Ok(id) => {
                        log::info!("Successfully inserted clipboard item with ID: {}", id);
                    }
                    Err(e) => {
                        log::warn!("Failed to insert clipboard item: {}", e);
                    }
                }
            }
            log::info!("Clipboard handler thread exiting");
        });

        Ok(())
    }

    pub fn copy_to_clipboard(&self, content: &str) -> Result<()> {
        let mut monitor = self.monitor.lock().unwrap();
        monitor.set_clipboard_content(content)
    }

    pub fn search_items(
        &self,
        filter: &SearchFilter,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<ClipboardItem>> {
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

    pub fn update_favorite(&self, id: i64, is_favorite: bool) -> Result<()> {
        self.database.update_favorite(id, is_favorite)
    }

    pub fn cleanup_with_config(&self, max_items: usize) -> Result<()> {
        self.database.cleanup_with_limit(max_items)
    }
}
