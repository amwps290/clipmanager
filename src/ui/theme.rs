use crate::config::settings::{ThemeConfig, ThemeMode};
use eframe::egui;

pub struct ThemeManager {
    current_mode: ThemeMode,
}

impl ThemeManager {
    pub fn new(config: &ThemeConfig) -> Self {
        Self {
            current_mode: config.mode.clone(),
        }
    }

    pub fn apply_theme(&self, ctx: &egui::Context) {
        log::debug!("Applying theme: {:?}", self.current_mode);

        match self.current_mode {
            ThemeMode::Light => {
                ctx.set_visuals(egui::Visuals::light());
                log::info!("Applied light theme");
            }
            ThemeMode::Dark => {
                ctx.set_visuals(egui::Visuals::dark());
                log::info!("Applied dark theme");
            }
            ThemeMode::System => {
                // 检测系统主题偏好
                let is_dark = self.detect_system_dark_mode();
                if is_dark {
                    ctx.set_visuals(egui::Visuals::dark());
                    log::info!("Applied dark theme (system preference)");
                } else {
                    ctx.set_visuals(egui::Visuals::light());
                    log::info!("Applied light theme (system preference)");
                }
            }
        }

        // 应用自定义样式
        self.apply_custom_style(ctx);
    }

    pub fn set_theme(&mut self, mode: ThemeMode, ctx: &egui::Context) {
        log::info!("Switching theme from {:?} to {:?}", self.current_mode, mode);
        self.current_mode = mode;
        self.apply_theme(ctx);
    }

    pub fn get_current_mode(&self) -> &ThemeMode {
        &self.current_mode
    }

    pub fn toggle_theme(&mut self, ctx: &egui::Context) -> ThemeMode {
        let new_mode = match self.current_mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::System => {
                // 如果当前是系统模式，切换到相反的模式
                if self.detect_system_dark_mode() {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                }
            }
        };

        log::info!("Toggling theme to {:?}", new_mode);
        self.set_theme(new_mode.clone(), ctx);
        new_mode
    }

    fn detect_system_dark_mode(&self) -> bool {
        // 在Linux环境中检测系统主题
        // 这是一个简化的实现，实际应用中可能需要更复杂的检测逻辑

        // 尝试读取环境变量
        if let Ok(theme) = std::env::var("GTK_THEME") {
            return theme.to_lowercase().contains("dark");
        }

        // 尝试检测GNOME设置
        if let Ok(output) = std::process::Command::new("gsettings")
            .args(&["get", "org.gnome.desktop.interface", "gtk-theme"])
            .output()
        {
            if let Ok(theme) = String::from_utf8(output.stdout) {
                return theme.to_lowercase().contains("dark");
            }
        }

        // 默认返回false（亮色主题）
        false
    }

    fn apply_custom_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        // 自定义字体大小
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
        );

        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
        );

        // 自定义间距
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        style.spacing.menu_margin = egui::Margin::same(8);

        ctx.set_style(style);
    }

    pub fn get_theme_icon(&self) -> &'static str {
        match self.current_mode {
            ThemeMode::Light => "🌞",
            ThemeMode::Dark => "🌙",
            ThemeMode::System => "🖥️",
        }
    }

    pub fn get_theme_name(&self) -> &'static str {
        match self.current_mode {
            ThemeMode::Light => "Light",
            ThemeMode::Dark => "Dark",
            ThemeMode::System => "System",
        }
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            current_mode: ThemeMode::System,
        }
    }
}
