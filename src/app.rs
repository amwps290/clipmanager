use crate::clipboard::handler::ClipboardHandler;
use crate::clipboard::types::{ClipboardItem, SearchFilter};
use crate::config::settings::{AppConfig, ThemeMode};
use crate::error::Result;
use crate::storage::database::Database;
use crate::ui::components::SettingsWindow;
use crate::ui::main_window::MainWindow;
use crate::ui::theme::ThemeManager;
use directories::ProjectDirs;
use eframe::egui;
use std::path::PathBuf;

pub struct ClipManagerApp {
    clipboard_handler: ClipboardHandler,
    main_window: MainWindow,
    settings_window: SettingsWindow,
    theme_manager: ThemeManager,
    items: Vec<ClipboardItem>,
    search_filter: SearchFilter,
    error_message: Option<String>,
    config: AppConfig,
    last_refresh: std::time::Instant,
    last_item_count: usize,
    copy_feedback_message: Option<String>,
    copy_feedback_timer: std::time::Instant,
}

impl ClipManagerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self> {
        // Load configuration
        let config = AppConfig::load().map_err(|e| crate::error::ClipManagerError::Config {
            message: format!("Failed to load config: {}", e),
        })?;

        // Set up database path
        let db_path = Self::get_database_path()?;
        let database = Database::new(db_path)?;

        // Create clipboard handler
        let mut clipboard_handler = ClipboardHandler::new(database)?;
        clipboard_handler.start_monitoring()?;

        // Create UI components
        let main_window = MainWindow::new();
        let settings_window = SettingsWindow::new(config.clone());
        let theme_manager = ThemeManager::new(&config.theme);

        let mut app = Self {
            clipboard_handler,
            main_window,
            settings_window,
            theme_manager,
            items: Vec::new(),
            search_filter: SearchFilter::default(),
            error_message: None,
            config,
            last_refresh: std::time::Instant::now(),
            last_item_count: 0,
            copy_feedback_message: None,
            copy_feedback_timer: std::time::Instant::now(),
        };

        // Load initial data
        app.refresh_items();

        Ok(app)
    }

    fn get_database_path() -> Result<PathBuf> {
        let proj_dirs =
            ProjectDirs::from("com", "clipmanager", "ClipManager").ok_or_else(|| {
                crate::error::ClipManagerError::Config {
                    message: "Unable to determine data directory".to_string(),
                }
            })?;

        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;

        Ok(data_dir.join("clipmanager.db"))
    }

    fn refresh_items(&mut self) {
        log::debug!(
            "Refreshing items with filter: query='{}', favorites_only={}",
            self.search_filter.query,
            self.search_filter.favorites_only
        );

        match self
            .clipboard_handler
            .search_items(&self.search_filter, self.config.max_items, 0)
        {
            Ok(items) => {
                log::info!("Loaded {} items from database", items.len());
                self.items = items;
                self.error_message = None;

                // Update item count
                if let Ok(count) = self.clipboard_handler.get_item_count() {
                    self.last_item_count = count;
                }
            }
            Err(e) => {
                log::error!("Failed to load items: {}", e);
                self.error_message = Some(format!("Failed to load data: {}", e));
            }
        }
    }

    fn apply_config_changes(&mut self) {
        // Apply max items limit
        if let Err(e) = self
            .clipboard_handler
            .cleanup_with_config(self.config.max_items)
        {
            log::warn!("Failed to apply max items config: {}", e);
        }

        // Refresh items to reflect new limits
        self.refresh_items();
    }

    fn handle_item_action(&mut self, action: ItemAction, ctx: &egui::Context) {
        match action {
            ItemAction::Copy(content) => {
                if let Err(e) = self.clipboard_handler.copy_to_clipboard(&content) {
                    self.error_message = Some(format!("Failed to copy: {}", e));
                }
            }
            ItemAction::Delete(id) => {
                if let Err(e) = self.clipboard_handler.delete_item(id) {
                    self.error_message = Some(format!("Failed to delete: {}", e));
                } else {
                    self.refresh_items();
                }
            }
            ItemAction::ClearAll => {
                if let Err(e) = self.clipboard_handler.clear_all_items() {
                    self.error_message = Some(format!("Failed to clear: {}", e));
                } else {
                    self.refresh_items();
                }
            }
            ItemAction::UpdateAccess(id) => {
                if let Err(e) = self.clipboard_handler.update_item_access(id) {
                    log::warn!("Failed to update access record: {}", e);
                }
            }
            ItemAction::ToggleFavorite(id) => {
                // 找到对应的条目并切换收藏状态
                if let Some(item) = self.items.iter().find(|item| item.id == Some(id)) {
                    let new_favorite_state = !item.is_favorite;
                    if let Err(e) = self
                        .clipboard_handler
                        .update_favorite(id, new_favorite_state)
                    {
                        self.error_message = Some(format!("Failed to update favorite: {}", e));
                    } else {
                        self.refresh_items();
                    }
                }
            }
            ItemAction::OpenSettings => {
                self.settings_window.open();
            }
            ItemAction::ToggleTheme => {
                let new_theme = self.theme_manager.toggle_theme(ctx);
                log::info!("Theme toggled to: {:?}", new_theme);

                // 更新配置
                self.config.theme.mode = new_theme;
                if let Err(e) = self.config.save() {
                    log::error!("Failed to save theme config: {}", e);
                    self.error_message = Some(format!("Failed to save theme: {}", e));
                }
            }
            ItemAction::DoubleClickCopy(content) => {
                if let Err(e) = self.clipboard_handler.copy_to_clipboard(&content) {
                    log::error!("Failed to copy content on double-click: {}", e);
                    self.error_message = Some(format!("Failed to copy: {}", e));
                } else {
                    log::info!(
                        "Content copied via double-click: {} characters",
                        content.len()
                    );
                    self.copy_feedback_message = Some("Copied!".to_string());
                    self.copy_feedback_timer = std::time::Instant::now();
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
    ToggleFavorite(i64),
    OpenSettings,
    ToggleTheme,
    DoubleClickCopy(String),
}

impl eframe::App for ClipManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        self.theme_manager.apply_theme(ctx);

        // Handle search filter changes
        let old_filter = self.search_filter.clone();

        // Render main window
        if let Some(action) = self.main_window.show(
            ctx,
            &mut self.items,
            &mut self.search_filter,
            &self.error_message,
            &self.copy_feedback_message,
        ) {
            self.handle_item_action(action, ctx);
        }

        // Render settings window
        if let Some(new_config) = self.settings_window.show(ctx) {
            self.config = new_config;
            if let Err(e) = self.config.save() {
                self.error_message = Some(format!("Failed to save config: {}", e));
            } else {
                // Apply configuration changes
                self.apply_config_changes();
            }
        }

        // Refresh data if search conditions changed
        if self.search_filter.query != old_filter.query
            || self.search_filter.content_type != old_filter.content_type
            || self.search_filter.favorites_only != old_filter.favorites_only
        {
            self.refresh_items();
        }

        // Auto-refresh mechanism: check for new items every 2 seconds
        let now = std::time::Instant::now();
        if now.duration_since(self.last_refresh) >= std::time::Duration::from_secs(2) {
            log::debug!("Performing periodic refresh check");

            // Check if item count has changed
            match self.clipboard_handler.get_item_count() {
                Ok(current_count) => {
                    if current_count != self.last_item_count {
                        log::info!(
                            "Item count changed from {} to {}, refreshing UI",
                            self.last_item_count,
                            current_count
                        );
                        self.refresh_items();
                        self.last_item_count = current_count;
                    }
                }
                Err(e) => {
                    log::warn!("Failed to get item count: {}", e);
                }
            }

            self.last_refresh = now;
        }

        // Clear copy feedback after 2 seconds
        if self.copy_feedback_message.is_some() {
            if self.copy_feedback_timer.elapsed() >= std::time::Duration::from_secs(2) {
                self.copy_feedback_message = None;
                log::debug!("Copy feedback message cleared");
            }
        }

        // Request repaint more frequently for better responsiveness
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Save window state and configuration
        storage.set_string("search_query", self.search_filter.query.clone());
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Clean up resources
        log::info!("Application exiting");
    }
}
