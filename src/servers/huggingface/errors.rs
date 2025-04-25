use thiserror::Error;

#[derive(Debug, Error)]
pub enum McpHuggingFaceError {
    #[error("HTTP request error: {0}")]
    HttpError(reqwest::Error),
    #[error("Missing token")]
    MissingToken,
    #[error("Failed to parse JSON: {0}")]
    JsonParseError(String),
    #[error("API error: {0}")]
    ApiError(String),
    // #[error("Failed to parse image: {0}")]
    // ImageParseError(String),
}
