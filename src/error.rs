use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipManagerError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Clipboard operation error: {0}")]
    Clipboard(#[from] arboard::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {message}")]
    Config { message: String },

    #[error("Content too large: {size} bytes, maximum allowed {max_size} bytes")]
    ContentTooLarge { size: usize, max_size: usize },

    #[error("Unsupported content type")]
    UnsupportedContentType,
}

pub type Result<T> = std::result::Result<T, ClipManagerError>;
