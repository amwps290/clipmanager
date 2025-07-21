// Windows 子系统配置：
// - Debug 模式：显示控制台，便于调试
// - Release 模式：隐藏控制台，提供更好的用户体验
// 注意：如果程序启动失败，用户可能看不到错误信息
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
    // 在 Windows 上设置更好的错误处理
    if let Err(e) = run_app() {
        show_error_message(&format!("ClipManager 启动失败: {}", e));
        return Err(e);
    }
    Ok(())
}

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with better configuration for Windows
    setup_logging()?;

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
                    show_error_message(&format!("应用程序初始化失败: {}", e));
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

/// 设置日志系统，在 Windows 上提供更好的日志输出
fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = env_logger::Builder::from_default_env();

    // 在 Windows Release 模式下，由于没有控制台，我们需要将日志写入文件
    #[cfg(all(target_os = "windows", not(debug_assertions)))]
    {
        use std::fs::OpenOptions;
        use std::io::Write;

        // 创建日志文件路径
        let log_dir = std::env::temp_dir().join("clipmanager");
        std::fs::create_dir_all(&log_dir)?;
        let log_file = log_dir.join("clipmanager.log");

        // 设置文件日志
        let target = Box::new(OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?);

        builder
            .target(env_logger::Target::Pipe(target))
            .filter_level(log::LevelFilter::Info)
            .format(|buf, record| {
                writeln!(buf, "{} [{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            });
    }

    // 在其他情况下使用标准输出
    #[cfg(not(all(target_os = "windows", not(debug_assertions))))]
    {
        builder.filter_level(log::LevelFilter::Info);
    }

    builder.init();
    Ok(())
}

/// 在 Windows 上显示错误消息对话框
fn show_error_message(message: &str) {
    #[cfg(target_os = "windows")]
    {
        // 在 Windows 上显示消息框
        use std::ffi::CString;
        use std::ptr;

        // 这里我们使用一个简单的方法来显示错误
        // 在实际应用中，您可能想要使用 winapi 或其他 Windows API 绑定
        eprintln!("错误: {}", message);

        // 尝试写入临时文件，用户可以查看
        if let Ok(temp_dir) = std::env::temp_dir().join("clipmanager_error.txt").to_str() {
            if let Ok(mut file) = std::fs::File::create(temp_dir) {
                use std::io::Write;
                let _ = writeln!(file, "ClipManager 错误报告");
                let _ = writeln!(file, "时间: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
                let _ = writeln!(file, "错误: {}", message);
                let _ = writeln!(file, "\n请将此文件发送给开发者以获取帮助。");
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        eprintln!("错误: {}", message);
    }
}
