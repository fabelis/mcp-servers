use thiserror::Error;

#[derive(Debug, Error)]
pub enum McpArvixError {
    #[error("HTTP request error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("API error: {0}")]
    ApiError(String),
}
