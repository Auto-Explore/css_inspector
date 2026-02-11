use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidatorError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Error)]
pub enum UrlDecodePlusError {
    #[error("incomplete percent-escape")]
    IncompletePercentEscape,
    #[error("invalid hex digit: {0:?}")]
    InvalidHexDigit(u8),
    #[error("invalid utf-8 in decoded string: {0}")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
}
