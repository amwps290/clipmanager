//! Internationalization module for ClipManager
//!
//! This module provides centralized text management and will support
//! multiple languages in future versions.

pub mod texts;

use std::collections::HashMap;

/// Language identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    English,
    Chinese,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

/// Text key identifier for internationalization
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextKey {
    // Application
    AppTitle,
    AppVersion,

    // Main Window
    SearchPlaceholder,
    TypeFilterAll,
    TypeFilterText,
    TypeFilterImage,
    ClearAll,
    Settings,

    // Item List
    NoRecords,
    RecordsCount,
    CharactersCount,
    UsedTimes,

    // Actions
    Copy,
    Delete,
    Favorite,
    Unfavorite,

    // Status Messages
    LoadDataFailed,
    CopyFailed,
    DeleteFailed,
    ClearFailed,
    UpdateAccessFailed,

    // Database
    ContentExists,
    DatabaseError,
    ClipboardError,
    IoError,
    SerializationError,
    ConfigError,
    ContentTooLarge,
    UnsupportedContentType,

    // Time
    TimeFormat,

    // Context Menu
    ContextCopy,
    ContextDelete,
}

/// Internationalization manager
pub struct I18n {
    current_language: Language,
    texts: HashMap<Language, HashMap<TextKey, &'static str>>,
}

impl I18n {
    pub fn new() -> Self {
        let mut i18n = Self {
            current_language: Language::default(),
            texts: HashMap::new(),
        };

        i18n.load_texts();
        i18n
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    pub fn get_language(&self) -> Language {
        self.current_language
    }

    pub fn text(&self, key: TextKey) -> &str {
        self.texts
            .get(&self.current_language)
            .and_then(|lang_texts| lang_texts.get(&key))
            .unwrap_or(&"[MISSING TEXT]")
    }

    fn load_texts(&mut self) {
        // Load English texts
        let english_texts = texts::get_english_texts();
        self.texts.insert(Language::English, english_texts);

        // Load Chinese texts (for future use)
        let chinese_texts = texts::get_chinese_texts();
        self.texts.insert(Language::Chinese, chinese_texts);
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
    }
}

/// Global instance for easy access
static mut GLOBAL_I18N: Option<I18n> = None;
static mut I18N_INITIALIZED: bool = false;

/// Initialize the global i18n instance
pub fn init() {
    unsafe {
        if !I18N_INITIALIZED {
            GLOBAL_I18N = Some(I18n::new());
            I18N_INITIALIZED = true;
        }
    }
}

/// Get text for the given key using the global i18n instance
pub fn t(key: TextKey) -> &'static str {
    unsafe {
        if !I18N_INITIALIZED {
            init();
        }

        GLOBAL_I18N
            .as_ref()
            .map(|i18n| i18n.text(key))
            .unwrap_or("[I18N NOT INITIALIZED]")
    }
}

/// Set the global language
pub fn set_language(language: Language) {
    unsafe {
        if !I18N_INITIALIZED {
            init();
        }

        if let Some(ref mut i18n) = GLOBAL_I18N {
            i18n.set_language(language);
        }
    }
}

/// Get the current global language
pub fn get_language() -> Language {
    unsafe {
        if !I18N_INITIALIZED {
            init();
        }

        GLOBAL_I18N
            .as_ref()
            .map(|i18n| i18n.get_language())
            .unwrap_or_default()
    }
}
