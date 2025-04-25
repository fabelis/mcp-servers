use thiserror::Error;

#[derive(Debug, Error)]
pub enum McpReplicateError {
    #[error("HTTP request error: {0}")]
    HttpError(reqwest::Error),
    #[error("Missing token")]
    MissingToken,
    #[error("Failed to parse Content: {0}")]
    ContentParseError(String),
    // #[error("Failed to parse JSON: {0}")]
    // JsonParseError(String),
    // #[error("Failed to parse image: {0}")]
    // ImageParseError(String),
}
