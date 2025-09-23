#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("short buffer while reading {0}")]
    ShortBuffer(&'static str),
    #[error("utf8 error")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("invalid tag {tag} for {context}, expected 0=None or 1=Some")]
    InvalidTag {
        context: &'static str,
        tag: u32,
    },
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, DecodeError>;
