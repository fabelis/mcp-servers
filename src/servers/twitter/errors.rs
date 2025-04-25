use serde_json::Error as JsonError;
use thiserror::Error;
use twitter_v2::Error as TwitterError;

#[derive(Debug, Error)]
pub enum McpTwitterError {
    #[error("Failed to serialize response: {0}")]
    ResponseSerializeError(JsonError),
    #[error("Missing Twitter API Key")]
    MissingTwitterApiKey,
    #[error("Missing Twitter API Secret")]
    MissingTwitterApiSecret,
    #[error("Missing Twitter Access Token")]
    MissingTwitterAccessToken,
    #[error("Missing Twitter Access Token Secret")]
    MissingTwitterAccessTokenSecret,
    #[error("Twitter API error: {0}")]
    TwitterApiError(TwitterError),
    #[error("Invalid latest id: {0}")]
    InvalidLatestID(f64),
    #[error("Invalid count: {0}")]
    InvalidCount(f64),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Tweet not found")]
    TweetNotFound,
    #[error("Tweets not found")]
    TweetsNotFound,
    #[error("Invalid reply id: {0}")]
    InvalidReplyID(f64),
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    #[error("Invalid sort order: {0}")]
    InvalidSortOrder(String),
    #[error("User not found")]
    UserNotFound,
}
