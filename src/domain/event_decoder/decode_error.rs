#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("short buffer while reading {0}")]
    ShortBuffer(&'static str),
    #[error("utf8 error")]
    Utf8(#[from] std::string::FromUtf8Error),
}

pub type Result<T> = std::result::Result<T, DecodeError>;
