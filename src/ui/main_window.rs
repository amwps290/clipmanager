use crate::app::ItemAction;
use crate::clipboard::types::{ClipboardContent, ClipboardItem, SearchFilter};
use crate::i18n::{self, TextKey};
use eframe::egui;

pub struct MainWindow {
    selected_item: Option<usize>,
    image_cache: std::collections::HashMap<String, egui::TextureHandle>,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            selected_item: None,
            image_cache: std::collections::HashMap::new(),
        }
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        items: &mut [ClipboardItem],
        search_filter: &mut SearchFilter,
        error_message: &Option<String>,
        copy_feedback: &Option<String>,
    ) -> Option<ItemAction> {
        let mut action = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            // 设置最小窗口尺寸
            let min_size = egui::vec2(400.0, 300.0);
            if ui.available_size().x < min_size.x || ui.available_size().y < min_size.y {
                log::debug!(
                    "Window size too small: {:?}, minimum: {:?}",
                    ui.available_size(),
                    min_size
                );
            }

            // Title bar with responsive layout
            ui.horizontal(|ui| {
                ui.heading(i18n::t(TextKey::AppTitle));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 主题切换按钮
                    if ui.button("🎨").on_hover_text("Toggle Theme").clicked() {
                        log::info!("Theme toggle button clicked");
                        action = Some(ItemAction::ToggleTheme);
                    }

                    if ui.button(i18n::t(TextKey::ClearAll)).clicked() {
                        action = Some(ItemAction::ClearAll);
                    }
                    if ui.button(i18n::t(TextKey::Settings)).clicked() {
                        action = Some(ItemAction::OpenSettings);
                    }
                });
            });

            ui.separator();

            // Search bar
            ui.horizontal(|ui| {
                ui.label(i18n::t(TextKey::SearchPlaceholder));
                ui.text_edit_singleline(&mut search_filter.query);

                ui.separator();

                ui.label("Type:");
                let selected_text = match search_filter.content_type {
                    None => i18n::t(TextKey::TypeFilterAll),
                    Some(crate::clipboard::types::ContentType::Text) => {
                        i18n::t(TextKey::TypeFilterText)
                    }
                    Some(crate::clipboard::types::ContentType::Image) => {
                        i18n::t(TextKey::TypeFilterImage)
                    }
                };

                egui::ComboBox::from_label("")
                    .selected_text(selected_text)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut search_filter.content_type,
                            None,
                            i18n::t(TextKey::TypeFilterAll),
                        );
                        ui.selectable_value(
                            &mut search_filter.content_type,
                            Some(crate::clipboard::types::ContentType::Text),
                            i18n::t(TextKey::TypeFilterText),
                        );
                        ui.selectable_value(
                            &mut search_filter.content_type,
                            Some(crate::clipboard::types::ContentType::Image),
                            i18n::t(TextKey::TypeFilterImage),
                        );
                    });

                ui.separator();

                // Favorites filter
                ui.checkbox(&mut search_filter.favorites_only, "⭐ Favorites");
            });

            ui.separator();

            // Error message display
            if let Some(error) = error_message {
                ui.colored_label(egui::Color32::RED, error);
                ui.separator();
            }

            // Copy feedback display
            if let Some(feedback) = copy_feedback {
                ui.colored_label(egui::Color32::GREEN, feedback);
                ui.separator();
            }

            // Item list
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if items.is_empty() {
                        ui.centered_and_justified(|ui| {
                            ui.label(i18n::t(TextKey::NoRecords));
                        });
                    } else {
                        for (index, item) in items.iter().enumerate() {
                            if let Some(item_action) = self.show_item(ui, item, index) {
                                action = Some(item_action);
                            }
                        }
                    }
                });

            // Status bar
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{} {}",
                    items.len(),
                    i18n::t(TextKey::RecordsCount)
                ));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(i18n::t(TextKey::AppVersion));
                });
            });
        });

        action
    }

    fn show_item(
        &mut self,
        ui: &mut egui::Ui,
        item: &ClipboardItem,
        index: usize,
    ) -> Option<ItemAction> {
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

        // Draw item content with responsive layout
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect.shrink(8.0)), |ui| {
            let available_width = ui.available_width();
            let is_narrow = available_width < 500.0; // 判断是否为窄屏模式

            ui.horizontal(|ui| {
                // Content type icon and thumbnail
                let icon_size = if is_narrow { 32.0 } else { 48.0 };
                match &item.content {
                    ClipboardContent::Text(_) => {
                        ui.label("📄");
                    }
                    ClipboardContent::Image(image_data) => {
                        // 显示图片缩略图
                        if let Ok(image) = image::load_from_memory(&image_data.data) {
                            let thumbnail_size = icon_size as u32;
                            let thumbnail = image.thumbnail(thumbnail_size, thumbnail_size);
                            let rgba_image = thumbnail.to_rgba8();
                            let size = [thumbnail.width() as usize, thumbnail.height() as usize];
                            let pixels = rgba_image.as_flat_samples();

                            let color_image =
                                egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

                            let texture = ui.ctx().load_texture(
                                format!("thumbnail_{}", item.content_hash),
                                color_image,
                                egui::TextureOptions::default(),
                            );

                            ui.add(
                                egui::Image::from_texture(&texture)
                                    .max_size(egui::Vec2::new(icon_size, icon_size)),
                            );
                        } else {
                            ui.label("🖼️");
                        }
                    }
                }

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        // Content preview with responsive length
                        let preview_length = if is_narrow { 25 } else { 50 };
                        let preview = item.preview(preview_length);
                        ui.label(egui::RichText::new(preview).strong());

                        // Favorite indicator
                        if item.is_favorite {
                            ui.label("⭐");
                        }
                    });

                    // Time and size information with responsive formatting
                    ui.horizontal(|ui| {
                        // 响应式时间格式
                        let time_format = if is_narrow {
                            "%H:%M"
                        } else {
                            i18n::t(TextKey::TimeFormat)
                        };
                        ui.label(format!("{}", item.created_at.format(time_format)));
                        ui.separator();

                        match &item.content {
                            ClipboardContent::Text(_) => {
                                if is_narrow {
                                    ui.label(format!("{} chars", item.content_size));
                                } else {
                                    ui.label(format!(
                                        "{} {}",
                                        item.content_size,
                                        i18n::t(TextKey::CharactersCount)
                                    ));
                                }
                            }
                            ClipboardContent::Image(_) => {
                                ui.label(format!("{} bytes", item.content_size));
                            }
                        }

                        if item.access_count > 0 {
                            ui.separator();
                            if is_narrow {
                                ui.label(format!("{}x", item.access_count));
                            } else {
                                ui.label(format!(
                                    "{} {}",
                                    item.access_count,
                                    i18n::t(TextKey::UsedTimes)
                                ));
                            }
                        }
                    });
                });

                // 响应式按钮布局
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // 根据窗口宽度决定按钮样式
                    if is_narrow {
                        // 窄屏模式：使用图标按钮
                        if ui.button("🗑").on_hover_text("Delete").clicked() {
                            if let Some(id) = item.id {
                                action = Some(ItemAction::Delete(id));
                            }
                        }

                        if let Some(id) = item.id {
                            let favorite_icon = if item.is_favorite { "⭐" } else { "☆" };
                            if ui
                                .button(favorite_icon)
                                .on_hover_text("Toggle Favorite")
                                .clicked()
                            {
                                action = Some(ItemAction::ToggleFavorite(id));
                            }
                        }
                    } else {
                        // 宽屏模式：使用文字按钮
                        if ui.button(i18n::t(TextKey::Delete)).clicked() {
                            if let Some(id) = item.id {
                                action = Some(ItemAction::Delete(id));
                            }
                        }

                        if let Some(id) = item.id {
                            let favorite_icon = if item.is_favorite { "⭐" } else { "☆" };
                            if ui.button(favorite_icon).clicked() {
                                action = Some(ItemAction::ToggleFavorite(id));
                            }
                        }
                    }
                });
            });
        });

        // Handle click events
        if response.clicked() {
            self.selected_item = Some(index);

            // 单击时只更新访问记录，不复制
            if let Some(id) = item.id {
                action = Some(ItemAction::UpdateAccess(id));
            }
        }

        // Handle double-click events for copying
        if response.double_clicked() {
            log::info!("Double-click detected on item {}", index);

            match &item.content {
                ClipboardContent::Text(text) => {
                    log::info!("Double-click copy text: {} characters", text.len());
                    action = Some(ItemAction::DoubleClickCopy(text.clone()));
                }
                ClipboardContent::Image(_) => {
                    // 对于图片，我们可以复制一个描述或者实际的图片数据
                    // 这里先复制描述信息
                    let description = format!(
                        "Image ({}x{}, {} bytes)",
                        item.get_image_data().map(|img| img.width).unwrap_or(0),
                        item.get_image_data().map(|img| img.height).unwrap_or(0),
                        item.content_size
                    );
                    log::info!("Double-click copy image description: {}", description);
                    action = Some(ItemAction::DoubleClickCopy(description));
                }
            }
        }

        // Right-click context menu
        response.context_menu(|ui| {
            if ui.button(i18n::t(TextKey::ContextCopy)).clicked() {
                if let Some(text) = item.get_text_content() {
                    action = Some(ItemAction::Copy(text.to_string()));
                }
                ui.close();
            }
            if ui.button(i18n::t(TextKey::ContextDelete)).clicked() {
                if let Some(id) = item.id {
                    action = Some(ItemAction::Delete(id));
                }
                ui.close();
            }
            if let Some(id) = item.id {
                ui.separator();
                let favorite_text = if item.is_favorite {
                    i18n::t(TextKey::Unfavorite)
                } else {
                    i18n::t(TextKey::Favorite)
                };
                if ui.button(favorite_text).clicked() {
                    action = Some(ItemAction::ToggleFavorite(id));
                    ui.close();
                }
            }
        });

        action
    }
}
