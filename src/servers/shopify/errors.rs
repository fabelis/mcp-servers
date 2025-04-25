use reqwest::Error as HttpError;
use serde_json::Error as JsonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum McpShopifyError {
    #[error("Failed to serialize response: {0}")]
    ResponseSerializeError(JsonError),
    #[error("HTTP error: {0}")]
    HttpError(HttpError),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Missing Shopify Domain")]
    MissingShopifyDomain,
    #[error("Missing Shopify Access Token")]
    MissingShopifyAccessToken,
    #[error("Invalid Product ID: {0}")]
    InvalidProductId(String),
    #[error("Invalid Order ID: {0}")]
    InvalidOrderId(String),
    #[error("Invalid Customer ID: {0}")]
    InvalidCustomerId(String),
}
