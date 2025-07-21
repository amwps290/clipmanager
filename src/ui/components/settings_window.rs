use crate::config::settings::{AppConfig, ThemeMode};
use crate::i18n::{self, TextKey};
use eframe::egui;

pub struct SettingsWindow {
    pub open: bool,
    config: AppConfig,
    temp_config: AppConfig,
}

impl SettingsWindow {
    pub fn new(config: AppConfig) -> Self {
        Self {
            open: false,
            temp_config: config.clone(),
            config,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) -> Option<AppConfig> {
        let mut result = None;
        let mut should_close = false;

        egui::Window::new(i18n::t(TextKey::Settings))
            .open(&mut self.open)
            .default_size([400.0, 500.0])
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // General Settings
                    ui.heading("General Settings");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Max items:");
                        ui.add(
                            egui::DragValue::new(&mut self.temp_config.max_items).range(10..=10000),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("Max item size (MB):");
                        let mut size_mb = self.temp_config.max_item_size as f64 / (1024.0 * 1024.0);
                        if ui
                            .add(
                                egui::DragValue::new(&mut size_mb)
                                    .range(0.1..=100.0)
                                    .speed(0.1),
                            )
                            .changed()
                        {
                            self.temp_config.max_item_size = (size_mb * 1024.0 * 1024.0) as usize;
                        }
                    });

                    ui.checkbox(&mut self.temp_config.auto_start, "Auto start with system");
                    ui.checkbox(
                        &mut self.temp_config.show_notifications,
                        "Show notifications",
                    );

                    ui.add_space(10.0);

                    // Theme Settings
                    ui.heading("Theme Settings");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Theme mode:");
                        egui::ComboBox::from_label("")
                            .selected_text(format!("{:?}", self.temp_config.theme.mode))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.temp_config.theme.mode,
                                    ThemeMode::Light,
                                    "Light",
                                );
                                ui.selectable_value(
                                    &mut self.temp_config.theme.mode,
                                    ThemeMode::Dark,
                                    "Dark",
                                );
                                ui.selectable_value(
                                    &mut self.temp_config.theme.mode,
                                    ThemeMode::System,
                                    "System",
                                );
                            });
                    });

                    ui.add_space(10.0);

                    // Hotkey Settings
                    ui.heading("Hotkey Settings");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Global hotkey:");
                        ui.text_edit_singleline(&mut self.temp_config.hotkey);
                    });

                    ui.add_space(10.0);

                    // Font Settings
                    ui.heading("Font Settings");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Font size:");
                        ui.add(
                            egui::Slider::new(&mut self.temp_config.font.font_size, 8.0..=24.0)
                                .suffix("px"),
                        );
                    });

                    ui.checkbox(
                        &mut self.temp_config.font.enable_font_fallback,
                        "Enable font fallback",
                    );
                    ui.checkbox(
                        &mut self.temp_config.font.auto_detect_fonts,
                        "Auto-detect system fonts",
                    );

                    ui.label("Preferred fonts (one per line):");
                    let mut font_text = self.temp_config.font.preferred_fonts.join("\n");
                    if ui.text_edit_multiline(&mut font_text).changed() {
                        self.temp_config.font.preferred_fonts = font_text
                            .lines()
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }

                    ui.add_space(10.0);

                    // Window Settings
                    ui.heading("Window Settings");
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.label("Window width:");
                        ui.add(
                            egui::DragValue::new(&mut self.temp_config.window.width)
                                .range(300.0..=2000.0),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label("Window height:");
                        ui.add(
                            egui::DragValue::new(&mut self.temp_config.window.height)
                                .range(200.0..=1500.0),
                        );
                    });

                    ui.checkbox(&mut self.temp_config.window.always_on_top, "Always on top");
                    ui.checkbox(
                        &mut self.temp_config.window.start_minimized,
                        "Start minimized",
                    );

                    ui.add_space(20.0);

                    // Buttons
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            self.config = self.temp_config.clone();
                            result = Some(self.config.clone());
                            should_close = true;
                        }

                        if ui.button("Cancel").clicked() {
                            self.temp_config = self.config.clone();
                            should_close = true;
                        }

                        if ui.button("Reset to Default").clicked() {
                            self.temp_config = AppConfig::default();
                        }
                    });
                });
            });

        if should_close {
            self.open = false;
        }

        result
    }

    pub fn open(&mut self) {
        self.open = true;
        self.temp_config = self.config.clone();
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn update_config(&mut self, config: AppConfig) {
        self.config = config;
        if !self.open {
            self.temp_config = self.config.clone();
        }
    }
}
