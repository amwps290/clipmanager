use font_kit::family_name::FamilyName;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use std::collections::HashMap;

/// 字体管理器，负责发现和加载系统字体
pub struct FontManager {
    system_source: SystemSource,
    font_cache: HashMap<String, Vec<u8>>,
    preferred_fonts: Vec<String>,
}

/// 支持的语言类型
#[derive(Debug, Clone, PartialEq)]
pub enum LanguageScript {
    Latin,      // 英文、法文、德文等
    CJK,        // 中文、日文、韩文
    Arabic,     // 阿拉伯文
    Cyrillic,   // 俄文
    Thai,       // 泰文
    Devanagari, // 印地文
}

impl FontManager {
    pub fn new() -> Self {
        log::info!("Initializing font manager");
        Self {
            system_source: SystemSource::new(),
            font_cache: HashMap::new(),
            preferred_fonts: Self::get_default_font_preferences(),
        }
    }

    /// 获取默认字体偏好列表
    fn get_default_font_preferences() -> Vec<String> {
        let mut fonts = Vec::new();

        // 根据操作系统添加默认字体
        #[cfg(target_os = "windows")]
        {
            fonts.extend_from_slice(&[
                "Microsoft YaHei".to_string(),  // 微软雅黑
                "SimSun".to_string(),           // 宋体
                "SimHei".to_string(),           // 黑体
                "Malgun Gothic".to_string(),    // 韩文
                "Yu Gothic".to_string(),        // 日文
                "Segoe UI".to_string(),         // 英文
                "Arial Unicode MS".to_string(), // Unicode支持
            ]);
        }

        #[cfg(target_os = "macos")]
        {
            fonts.extend_from_slice(&[
                "PingFang SC".to_string(),               // 苹方简体
                "PingFang TC".to_string(),               // 苹方繁体
                "Hiragino Sans GB".to_string(),          // 冬青黑体
                "Apple SD Gothic Neo".to_string(),       // 韩文
                "Hiragino Kaku Gothic ProN".to_string(), // 日文
                "SF Pro Display".to_string(),            // 英文
                "Arial Unicode MS".to_string(),          // Unicode支持
            ]);
        }

        #[cfg(target_os = "linux")]
        {
            fonts.extend_from_slice(&[
                "Noto Sans CJK SC".to_string(),    // 思源黑体简体
                "Noto Sans CJK TC".to_string(),    // 思源黑体繁体
                "Noto Sans CJK JP".to_string(),    // 思源黑体日文
                "Noto Sans CJK KR".to_string(),    // 思源黑体韩文
                "Source Han Sans SC".to_string(),  // 思源黑体简体（Adobe版本）
                "WenQuanYi Micro Hei".to_string(), // 文泉驿微米黑
                "Droid Sans Fallback".to_string(), // Android回退字体
                "DejaVu Sans".to_string(),         // 英文
                "Liberation Sans".to_string(),     // 英文
            ]);
        }

        // 通用回退字体
        fonts.extend_from_slice(&[
            "Arial".to_string(),
            "Helvetica".to_string(),
            "sans-serif".to_string(),
        ]);

        log::info!("Default font preferences: {:?}", fonts);
        fonts
    }

    /// 发现并加载系统中可用的字体
    pub fn discover_fonts(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Discovering system fonts...");

        for font_name in &self.preferred_fonts.clone() {
            if let Ok(font_data) = self.load_font_by_name(font_name) {
                log::info!("Successfully loaded font: {}", font_name);
                self.font_cache.insert(font_name.clone(), font_data);
            } else {
                log::debug!("Font not found: {}", font_name);
            }
        }

        log::info!(
            "Font discovery completed. Loaded {} fonts",
            self.font_cache.len()
        );
        Ok(())
    }

    /// 根据字体名称加载字体数据
    fn load_font_by_name(&self, font_name: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // 尝试通过家族名称查找字体
        let family_name = FamilyName::Title(font_name.to_string());

        match self
            .system_source
            .select_best_match(&[family_name], &Properties::new())
        {
            Ok(handle) => {
                match handle.load() {
                    Ok(font) => {
                        // 获取字体数据
                        match font.copy_font_data() {
                            Some(font_data) => Ok(font_data.to_vec()),
                            None => Err("Failed to copy font data".into()),
                        }
                    }
                    Err(e) => {
                        log::debug!("Failed to load font {}: {}", font_name, e);
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                log::debug!("Font {} not found: {}", font_name, e);
                Err(Box::new(e))
            }
        }
    }

    /// 获取已加载的字体数据
    pub fn get_loaded_fonts(&self) -> &HashMap<String, Vec<u8>> {
        &self.font_cache
    }

    /// 检测文本需要的字体类型
    pub fn detect_script(text: &str) -> Vec<LanguageScript> {
        let mut scripts = Vec::new();
        let mut has_latin = false;
        let mut has_cjk = false;
        let mut has_arabic = false;
        let mut has_cyrillic = false;

        for ch in text.chars() {
            match ch {
                // 拉丁字母
                'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    if !has_latin {
                        scripts.push(LanguageScript::Latin);
                        has_latin = true;
                    }
                }
                // 中日韩字符
                '\u{4E00}'..='\u{9FFF}' |  // CJK统一汉字
                '\u{3400}'..='\u{4DBF}' |  // CJK扩展A
                '\u{20000}'..='\u{2A6DF}' | // CJK扩展B
                '\u{3040}'..='\u{309F}' |  // 平假名
                '\u{30A0}'..='\u{30FF}' |  // 片假名
                '\u{AC00}'..='\u{D7AF}' => { // 韩文音节
                    if !has_cjk {
                        scripts.push(LanguageScript::CJK);
                        has_cjk = true;
                    }
                }
                // 阿拉伯文
                '\u{0600}'..='\u{06FF}' |  // 阿拉伯文
                '\u{0750}'..='\u{077F}' => { // 阿拉伯文补充
                    if !has_arabic {
                        scripts.push(LanguageScript::Arabic);
                        has_arabic = true;
                    }
                }
                // 西里尔字母（俄文等）
                '\u{0400}'..='\u{04FF}' => {
                    if !has_cyrillic {
                        scripts.push(LanguageScript::Cyrillic);
                        has_cyrillic = true;
                    }
                }
                _ => {}
            }
        }

        if scripts.is_empty() {
            scripts.push(LanguageScript::Latin); // 默认
        }

        scripts
    }

    /// 根据脚本类型获取推荐字体
    pub fn get_fonts_for_script(&self, script: &LanguageScript) -> Vec<String> {
        match script {
            LanguageScript::CJK => {
                vec![
                    "Noto Sans CJK SC".to_string(),
                    "Microsoft YaHei".to_string(),
                    "PingFang SC".to_string(),
                    "Source Han Sans SC".to_string(),
                    "WenQuanYi Micro Hei".to_string(),
                ]
            }
            LanguageScript::Arabic => {
                vec![
                    "Noto Sans Arabic".to_string(),
                    "Arial Unicode MS".to_string(),
                ]
            }
            LanguageScript::Cyrillic => {
                vec![
                    "Noto Sans".to_string(),
                    "DejaVu Sans".to_string(),
                    "Liberation Sans".to_string(),
                ]
            }
            _ => {
                vec![
                    "Segoe UI".to_string(),
                    "SF Pro Display".to_string(),
                    "DejaVu Sans".to_string(),
                    "Liberation Sans".to_string(),
                ]
            }
        }
    }

    /// 设置用户偏好字体
    pub fn set_preferred_fonts(&mut self, fonts: Vec<String>) {
        log::info!("Setting preferred fonts: {:?}", fonts);
        self.preferred_fonts = fonts;
    }

    /// 获取当前偏好字体列表
    pub fn get_preferred_fonts(&self) -> &Vec<String> {
        &self.preferred_fonts
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_detection() {
        // 测试中文检测
        let chinese_text = "你好世界";
        let scripts = FontManager::detect_script(chinese_text);
        assert!(scripts.contains(&LanguageScript::CJK));

        // 测试英文检测
        let english_text = "Hello World";
        let scripts = FontManager::detect_script(english_text);
        assert!(scripts.contains(&LanguageScript::Latin));

        // 测试混合文本
        let mixed_text = "Hello 世界 123";
        let scripts = FontManager::detect_script(mixed_text);
        assert!(scripts.contains(&LanguageScript::Latin));
        assert!(scripts.contains(&LanguageScript::CJK));

        // 测试阿拉伯文
        let arabic_text = "مرحبا";
        let scripts = FontManager::detect_script(arabic_text);
        assert!(scripts.contains(&LanguageScript::Arabic));

        // 测试俄文
        let cyrillic_text = "Привет";
        let scripts = FontManager::detect_script(cyrillic_text);
        assert!(scripts.contains(&LanguageScript::Cyrillic));

        println!("✓ Script detection test passed");
    }

    #[test]
    fn test_font_manager_creation() {
        let font_manager = FontManager::new();
        assert!(!font_manager.preferred_fonts.is_empty());
        println!("✓ Font manager creation test passed");
    }

    #[test]
    fn test_font_discovery() {
        let mut font_manager = FontManager::new();

        // 尝试发现字体（可能在某些环境中失败，这是正常的）
        match font_manager.discover_fonts() {
            Ok(_) => {
                println!("✓ Font discovery succeeded");
                println!("Found {} fonts", font_manager.get_loaded_fonts().len());
            }
            Err(e) => {
                println!(
                    "⚠ Font discovery failed (this may be normal in test environment): {}",
                    e
                );
            }
        }

        println!("✓ Font discovery test completed");
    }

    #[test]
    fn test_fonts_for_script() {
        let font_manager = FontManager::new();

        let cjk_fonts = font_manager.get_fonts_for_script(&LanguageScript::CJK);
        assert!(!cjk_fonts.is_empty());

        let latin_fonts = font_manager.get_fonts_for_script(&LanguageScript::Latin);
        assert!(!latin_fonts.is_empty());

        println!("✓ Fonts for script test passed");
    }

    #[test]
    fn test_multilingual_text_detection() {
        // 测试各种语言的文本检测
        let test_cases = vec![
            ("Hello World", vec![LanguageScript::Latin]),
            ("你好世界", vec![LanguageScript::CJK]),
            ("こんにちは", vec![LanguageScript::CJK]),
            ("안녕하세요", vec![LanguageScript::CJK]),
            ("مرحبا", vec![LanguageScript::Arabic]),
            ("Привет", vec![LanguageScript::Cyrillic]),
            (
                "Hello 世界",
                vec![LanguageScript::Latin, LanguageScript::CJK],
            ),
            (
                "Test 测试 مرحبا",
                vec![
                    LanguageScript::Latin,
                    LanguageScript::CJK,
                    LanguageScript::Arabic,
                ],
            ),
        ];

        for (text, expected_scripts) in test_cases {
            let detected = FontManager::detect_script(text);
            println!("Text: '{}' -> Scripts: {:?}", text, detected);

            for expected in &expected_scripts {
                assert!(
                    detected.contains(expected),
                    "Expected script {:?} not found in {:?} for text '{}'",
                    expected,
                    detected,
                    text
                );
            }
        }

        println!("✓ Multilingual text detection test passed");
    }

    #[test]
    fn test_complex_unicode_text() {
        // 测试复杂的Unicode文本，包括表情符号
        let complex_texts = vec![
            "🎉 Hello 世界 🌍",
            "Test 测试 ✅ Success 成功 🚀",
            "Emoji: 😀😃😄😁😆😅😂🤣",
            "Math: ∑∏∫∆∇∂∞±≤≥≠≈",
            "Symbols: ♠♣♥♦♪♫♬♭♮♯",
        ];

        for text in complex_texts {
            println!("Testing complex text: {}", text);

            // 应该能够检测脚本而不panic
            let scripts = FontManager::detect_script(text);
            println!("  Detected scripts: {:?}", scripts);

            // 至少应该检测到拉丁文（因为有英文字符）
            assert!(
                !scripts.is_empty(),
                "Should detect at least one script for: {}",
                text
            );
        }

        println!("✓ Complex Unicode text test passed");
    }

    #[test]
    fn test_font_preferences() {
        let mut font_manager = FontManager::new();

        // 测试默认偏好
        let default_prefs = font_manager.get_preferred_fonts();
        assert!(!default_prefs.is_empty());
        println!("Default preferences: {:?}", default_prefs);

        // 测试设置自定义偏好
        let custom_fonts = vec!["Custom Font 1".to_string(), "Custom Font 2".to_string()];
        font_manager.set_preferred_fonts(custom_fonts.clone());

        let updated_prefs = font_manager.get_preferred_fonts();
        assert_eq!(updated_prefs, &custom_fonts);

        println!("✓ Font preferences test passed");
    }
}
