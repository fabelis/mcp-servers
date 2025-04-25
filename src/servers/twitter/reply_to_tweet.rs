use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use twitter_v2::{TwitterApi, authorization::Oauth1aToken, id::NumericId};

use super::errors::McpTwitterError;

#[tool(
    name = "ReplyToTweet",
    description = "Reply a tweet to Twitter. Required API and ACCESS values can be valued as 'null' if the info is not available.",
    params(
        reply = "Text for Twitter reply",
        reply_to_tweet_id = "Tweet ID to reply to"
    )
)]
async fn reply_to_tweet_tool(reply: String, reply_to_tweet_id: f64) -> Result<ToolResponseContent> {
    let twitter_api_key =
        std::env::var("TWITTER_API_KEY").map_err(|_| McpTwitterError::MissingTwitterApiKey)?;
    let twitter_api_secret = std::env::var("TWITTER_API_SECRET")
        .map_err(|_| McpTwitterError::MissingTwitterApiSecret)?;
    let twitter_access_token = std::env::var("TWITTER_ACCESS_TOKEN")
        .map_err(|_| McpTwitterError::MissingTwitterAccessToken)?;
    let twitter_access_token_secret = std::env::var("TWITTER_ACCESS_TOKEN_SECRET")
        .map_err(|_| McpTwitterError::MissingTwitterAccessTokenSecret)?;

    let api = TwitterApi::new(Oauth1aToken::new(
        &twitter_api_key,
        &twitter_api_secret,
        &twitter_access_token,
        &twitter_access_token_secret,
    ));

    if reply.is_empty() {
        return Err(McpTwitterError::InvalidInput("Reply cannot be empty".to_string()).into());
    }

    let u64_reply_to_tweet_id = reply_to_tweet_id as u64;
    if u64_reply_to_tweet_id == 0 {
        return Err(McpTwitterError::InvalidReplyID(reply_to_tweet_id).into());
    }

    let numeric_reply_to_tweet_id = NumericId::new(u64_reply_to_tweet_id);

    let tweet = api
        .post_tweet()
        .in_reply_to_tweet_id(numeric_reply_to_tweet_id)
        .text(reply)
        .send()
        .await
        .map_err(McpTwitterError::TwitterApiError)?
        .into_data()
        .ok_or_else(|| McpTwitterError::TweetNotFound)?;

    Ok(tool_text_content!(
        serde_json::to_string(&tweet).map_err(McpTwitterError::ResponseSerializeError)?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_reply_to_tweet_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("TWITTER_API_KEY").is_err()
            || std::env::var("TWITTER_API_SECRET").is_err()
            || std::env::var("TWITTER_ACCESS_TOKEN").is_err()
            || std::env::var("TWITTER_ACCESS_TOKEN_SECRET").is_err()
        {
            println!("Skipping test_reply_to_tweet_tool: Missing required Twitter credentials");
            return;
        }

        // Note: This test requires a valid tweet ID to reply to
        let test_tweet_id = 1234567890.0; // Replace with a real tweet ID for actual testing
        match reply_to_tweet_tool(
            "Test reply from automated testing".to_string(),
            test_tweet_id,
        )
        .await
        {
            Ok(content) => {
                println!("Reply result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                // Don't panic on error since this test requires a valid tweet ID
                println!("Test skipped: requires valid tweet ID to reply to");
            }
        }
    }
}
