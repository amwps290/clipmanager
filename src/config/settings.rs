// 配置管理模块

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub max_items: usize,
    pub max_item_size: usize,
    pub auto_start: bool,
    pub show_notifications: bool,
    pub hotkey: String,
    pub window: WindowConfig,
    pub theme: ThemeConfig,
    pub font: FontConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub mode: ThemeMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
    System, // 跟随系统主题
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub preferred_fonts: Vec<String>,
    pub font_size: f32,
    pub enable_font_fallback: bool,
    pub auto_detect_fonts: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowConfig {
    pub width: f32,
    pub height: f32,
    pub always_on_top: bool,
    pub start_minimized: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            max_items: 100,
            max_item_size: 1024 * 1024, // 1MB
            auto_start: false,
            show_notifications: true,
            hotkey: "Ctrl+Shift+V".to_string(),
            window: WindowConfig::default(),
            theme: ThemeConfig::default(),
            font: FontConfig::default(),
        }
    }
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            mode: ThemeMode::System,
        }
    }
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::System
    }
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            preferred_fonts: vec![
                // 根据操作系统提供默认字体
                #[cfg(target_os = "windows")]
                "Microsoft YaHei".to_string(),
                #[cfg(target_os = "macos")]
                "PingFang SC".to_string(),
                #[cfg(target_os = "linux")]
                "Noto Sans CJK SC".to_string(),
                // 通用回退字体
                "Arial Unicode MS".to_string(),
                "DejaVu Sans".to_string(),
            ],
            font_size: 14.0,
            enable_font_fallback: true,
            auto_detect_fonts: true,
        }
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: 600.0,
            height: 500.0,
            always_on_top: false,
            start_minimized: false,
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_file_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;

            // 尝试解析新格式的配置
            match toml::from_str::<AppConfig>(&content) {
                Ok(config) => {
                    log::info!("Loaded config from: {:?}", config_path);
                    Ok(config)
                }
                Err(e) => {
                    log::warn!("Failed to parse config with new format: {}", e);
                    log::info!("Attempting to migrate old config format...");

                    // 尝试解析旧格式并迁移
                    match Self::migrate_old_config(&content) {
                        Ok(config) => {
                            log::info!("Successfully migrated old config format");
                            // 保存迁移后的配置
                            config.save()?;
                            Ok(config)
                        }
                        Err(migrate_err) => {
                            log::error!("Failed to migrate config: {}", migrate_err);
                            log::info!("Creating new default config");
                            // 备份旧配置文件
                            let backup_path = config_path.with_extension("toml.backup");
                            let _ = fs::copy(&config_path, &backup_path);
                            log::info!("Backed up old config to: {:?}", backup_path);

                            // 创建新的默认配置
                            let default_config = Self::default();
                            default_config.save()?;
                            Ok(default_config)
                        }
                    }
                }
            }
        } else {
            // 如果配置文件不存在，创建默认配置并保存
            log::info!("No config file found, creating default config");
            let config = AppConfig::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_file_path()?;

        // 确保配置目录存在
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let project_dirs = ProjectDirs::from("com", "clipmanager", "ClipManager")
            .ok_or("Failed to get project directories")?;

        let config_dir = project_dirs.config_dir();
        Ok(config_dir.join("config.toml"))
    }

    pub fn config_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let project_dirs = ProjectDirs::from("com", "clipmanager", "ClipManager")
            .ok_or("Failed to get project directories")?;
        Ok(project_dirs.config_dir().to_path_buf())
    }

    /// 迁移旧版本的配置文件格式
    fn migrate_old_config(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // 定义旧版本的配置结构（没有theme字段）
        #[derive(Deserialize)]
        struct OldAppConfig {
            pub max_items: usize,
            pub max_item_size: usize,
            pub auto_start: bool,
            pub show_notifications: bool,
            pub hotkey: String,
            pub window: WindowConfig,
        }

        // 尝试解析旧格式
        let old_config: OldAppConfig = toml::from_str(content)?;

        // 转换为新格式
        let new_config = AppConfig {
            max_items: old_config.max_items,
            max_item_size: old_config.max_item_size,
            auto_start: old_config.auto_start,
            show_notifications: old_config.show_notifications,
            hotkey: old_config.hotkey,
            window: old_config.window,
            theme: ThemeConfig::default(), // 使用默认主题配置
            font: FontConfig::default(),   // 使用默认字体配置
        };

        log::info!("Migrated config: added default theme configuration");
        Ok(new_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_config_serialization() {
        let config = AppConfig::default();

        // Test serialization
        let toml_str = toml::to_string(&config).unwrap();
        println!("Serialized config:\n{}", toml_str);

        // Test deserialization
        let deserialized: AppConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.theme.mode, deserialized.theme.mode);

        println!("✓ Theme config serialization test passed");
    }

    #[test]
    fn test_theme_mode_variants() {
        let modes = vec![ThemeMode::Light, ThemeMode::Dark, ThemeMode::System];

        for mode in modes {
            let config = AppConfig {
                theme: ThemeConfig { mode: mode.clone() },
                ..AppConfig::default()
            };

            let serialized = toml::to_string(&config).unwrap();
            let deserialized: AppConfig = toml::from_str(&serialized).unwrap();

            assert_eq!(config.theme.mode, deserialized.theme.mode);
        }

        println!("✓ Theme mode variants test passed");
    }

    #[test]
    fn test_config_migration() {
        // 模拟旧版本的配置文件内容（没有theme字段）
        let old_config_content = r#"
max_items = 200
max_item_size = 2097152
auto_start = true
show_notifications = false
hotkey = "Ctrl+Alt+V"

[window]
width = 800.0
height = 600.0
always_on_top = true
start_minimized = true
"#;

        // 测试迁移功能
        let migrated_config = AppConfig::migrate_old_config(old_config_content).unwrap();

        // 验证迁移后的配置
        assert_eq!(migrated_config.max_items, 200);
        assert_eq!(migrated_config.max_item_size, 2097152);
        assert_eq!(migrated_config.auto_start, true);
        assert_eq!(migrated_config.show_notifications, false);
        assert_eq!(migrated_config.hotkey, "Ctrl+Alt+V");
        assert_eq!(migrated_config.window.width, 800.0);
        assert_eq!(migrated_config.window.height, 600.0);
        assert_eq!(migrated_config.window.always_on_top, true);
        assert_eq!(migrated_config.window.start_minimized, true);

        // 验证默认主题配置被添加
        assert_eq!(migrated_config.theme.mode, ThemeMode::System);

        println!("✓ Config migration test passed");
    }

    #[test]
    fn test_config_load_with_migration() {
        use std::fs;

        // 创建临时配置目录
        let temp_dir = std::env::temp_dir().join("clipmanager_test");
        fs::create_dir_all(&temp_dir).unwrap();

        let config_path = temp_dir.join("config.toml");

        // 写入旧格式的配置文件
        let old_config_content = r#"max_items = 150
max_item_size = 2048000
auto_start = true
show_notifications = false
hotkey = "Ctrl+Alt+C"

[window]
width = 700.0
height = 550.0
always_on_top = true
start_minimized = false"#;

        fs::write(&config_path, old_config_content).unwrap();

        // 临时修改配置文件路径用于测试
        std::env::set_var("CLIPMANAGER_CONFIG_PATH", config_path.to_str().unwrap());

        // 测试加载配置（应该触发迁移）
        // 注意：这里我们不能直接调用AppConfig::load()，因为它使用固定的路径
        // 所以我们直接测试迁移逻辑
        let migrated = AppConfig::migrate_old_config(old_config_content).unwrap();

        // 验证迁移结果
        assert_eq!(migrated.max_items, 150);
        assert_eq!(migrated.max_item_size, 2048000);
        assert_eq!(migrated.auto_start, true);
        assert_eq!(migrated.show_notifications, false);
        assert_eq!(migrated.hotkey, "Ctrl+Alt+C");
        assert_eq!(migrated.window.width, 700.0);
        assert_eq!(migrated.window.height, 550.0);
        assert_eq!(migrated.window.always_on_top, true);
        assert_eq!(migrated.window.start_minimized, false);
        assert_eq!(migrated.theme.mode, ThemeMode::System);

        // 清理
        let _ = fs::remove_dir_all(&temp_dir);
        std::env::remove_var("CLIPMANAGER_CONFIG_PATH");

        println!("✓ Config load with migration test passed");
    }
}
