#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // 在 Windows 上隐藏控制台

mod app;
mod clipboard;
mod config;
mod error;
mod i18n;
mod storage;
mod ui;

use app::ClipManagerApp;
use config::settings::AppConfig;
use eframe::egui;
use ui::FontManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    // Initialize internationalization
    i18n::init();

    log::info!("Starting ClipManager");

    // 设置应用程序选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_min_inner_size([400.0, 300.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    // Run the application
    eframe::run_native(
        i18n::t(i18n::TextKey::AppTitle),
        options,
        Box::new(|cc| {
            // Setup fonts
            setup_fonts(&cc.egui_ctx);

            match ClipManagerApp::new(cc) {
                Ok(app) => {
                    // 主题会在应用程序的update方法中应用
                    log::info!("Application initialized successfully");
                    Ok(Box::new(app))
                }
                Err(e) => {
                    log::error!("Application initialization failed: {}", e);
                    Err(Box::new(e))
                }
            }
        }),
    )?;

    Ok(())
}

fn load_icon() -> egui::IconData {
    // TODO: Load actual application icon
    // For now, return empty icon data
    egui::IconData {
        rgba: vec![0; 32 * 32 * 4],
        width: 32,
        height: 32,
    }
}

fn setup_fonts(ctx: &egui::Context) {
    log::info!("Setting up fonts for egui");

    // 加载配置
    let config = AppConfig::load().unwrap_or_default();

    // 创建字体管理器并发现系统字体
    let mut font_manager = FontManager::new();
    if let Err(e) = font_manager.discover_fonts() {
        log::warn!("Failed to discover fonts: {}", e);
    }

    // 创建字体定义
    let mut fonts = egui::FontDefinitions::default();

    // 清除默认字体，我们将添加自己的字体
    fonts.font_data.clear();
    fonts.families.clear();

    // 添加系统字体
    let loaded_fonts = font_manager.get_loaded_fonts();
    let mut font_families = Vec::new();

    // 首先尝试用户偏好字体
    for font_name in &config.font.preferred_fonts {
        if let Some(font_data) = loaded_fonts.get(font_name) {
            let font_key = format!("system_{}", font_name);
            fonts.font_data.insert(
                font_key.clone(),
                std::sync::Arc::new(egui::FontData::from_owned(font_data.clone())),
            );
            font_families.push(font_key);
            log::info!("Added preferred font: {}", font_name);
        }
    }

    // 添加发现的其他字体作为回退
    for (font_name, font_data) in loaded_fonts {
        let font_key = format!("system_{}", font_name);
        if !fonts.font_data.contains_key(&font_key) {
            fonts.font_data.insert(
                font_key.clone(),
                std::sync::Arc::new(egui::FontData::from_owned(font_data.clone())),
            );
            font_families.push(font_key);
            log::debug!("Added fallback font: {}", font_name);
        }
    }

    // 如果没有找到任何系统字体，使用egui默认字体
    if font_families.is_empty() {
        log::warn!("No system fonts found, using egui defaults");
        fonts = egui::FontDefinitions::default();
    } else {
        // 设置字体族
        fonts
            .families
            .insert(egui::FontFamily::Proportional, font_families.clone());
        fonts
            .families
            .insert(egui::FontFamily::Monospace, font_families);
    }

    log::info!("Font setup completed with {} fonts", fonts.font_data.len());
    ctx.set_fonts(fonts);
}
