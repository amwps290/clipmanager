use crate::clipboard::types::{ClipboardItem, ImageData, ImageFormat};
use crate::error::Result;
use arboard::Clipboard;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct ClipboardMonitor {
    clipboard: Clipboard,
    last_text_content: String,
    last_image_hash: Option<String>,
    sender: mpsc::Sender<ClipboardItem>,
    is_running: bool,
}

impl ClipboardMonitor {
    pub fn new() -> Result<(Self, mpsc::Receiver<ClipboardItem>)> {
        let clipboard = Clipboard::new()?;
        let (sender, receiver) = mpsc::channel();

        let monitor = Self {
            clipboard,
            last_text_content: String::new(),
            last_image_hash: None,
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
            self.last_text_content = content;
        }

        let sender = self.sender.clone();
        let mut clipboard = Clipboard::new()?;
        let mut last_text_content = self.last_text_content.clone();
        let mut last_image_hash: Option<String> = self.last_image_hash.clone();

        thread::spawn(move || {
            log::info!("Clipboard monitor thread started");
            loop {
                thread::sleep(Duration::from_millis(500)); // 每500ms检查一次

                // 检查文本内容
                if let Ok(current_text) = clipboard.get_text() {
                    if current_text != last_text_content && !current_text.trim().is_empty() {
                        log::info!(
                            "Detected new text content: {} characters",
                            current_text.len()
                        );
                        let item = ClipboardItem::new_text(current_text.clone());

                        if let Err(e) = sender.send(item) {
                            log::error!("Failed to send clipboard item: {}", e);
                            // 接收端已关闭，退出监控
                            break;
                        }

                        log::info!("Successfully sent text item to handler");
                        last_text_content = current_text;
                        // 文本内容变化时清除图片哈希
                        last_image_hash = None;
                    }
                } else {
                    // 如果获取文本失败，记录但不退出
                    // log::debug!("Failed to get clipboard text, continuing...");
                }

                // 检查图片内容
                if let Ok(image_data) = clipboard.get_image() {
                    let image_bytes = image_data.bytes.to_vec();
                    let current_image_hash = Self::calculate_image_hash(&image_bytes);

                    if last_image_hash.as_ref() != Some(&current_image_hash) {
                        // 尝试检测图片格式
                        if let Some(format) = Self::detect_image_format(&image_bytes) {
                            let clipboard_image = ImageData::new(
                                image_bytes,
                                format,
                                image_data.width as u32,
                                image_data.height as u32,
                            );

                            let item = ClipboardItem::new_image(clipboard_image);

                            if let Err(_) = sender.send(item) {
                                // 接收端已关闭，退出监控
                                break;
                            }

                            last_image_hash = Some(current_image_hash);
                            // 图片内容变化时清除文本内容
                            last_text_content.clear();
                        }
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
        self.last_text_content = content.to_string();
        Ok(())
    }

    fn calculate_image_hash(data: &[u8]) -> String {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn detect_image_format(data: &[u8]) -> Option<ImageFormat> {
        if data.len() < 8 {
            return None;
        }

        // PNG signature: 89 50 4E 47 0D 0A 1A 0A
        if data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return Some(ImageFormat::Png);
        }

        // JPEG signature: FF D8 FF
        if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Some(ImageFormat::Jpeg);
        }

        // GIF signature: GIF87a or GIF89a
        if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
            return Some(ImageFormat::Gif);
        }

        // BMP signature: BM
        if data.starts_with(b"BM") {
            return Some(ImageFormat::Bmp);
        }

        None
    }
}
