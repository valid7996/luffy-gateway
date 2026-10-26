use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
