use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use twitter_v2::{TwitterApi, authorization::Oauth1aToken};

use super::errors::McpTwitterError;

#[tool(
    name = "PostTweet",
    description = "Post a tweet to Twitter. Required API and ACCESS values can be valued as 'null' if the info is not available.",
    params(tweet = "Text to post on Twitter")
)]
async fn post_tweet_tool(tweet: String) -> Result<ToolResponseContent> {
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

    if tweet.is_empty() {
        return Err(McpTwitterError::InvalidInput("Tweet cannot be empty".to_string()).into());
    }

    let tweet = api
        .post_tweet()
        .text(tweet)
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
    async fn test_post_tweet_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("TWITTER_API_KEY").is_err()
            || std::env::var("TWITTER_API_SECRET").is_err()
            || std::env::var("TWITTER_ACCESS_TOKEN").is_err()
            || std::env::var("TWITTER_ACCESS_TOKEN_SECRET").is_err()
        {
            println!("Skipping test_post_tweet_tool: Missing required Twitter credentials");
            return;
        }

        match post_tweet_tool("Test tweet from automated testing".to_string()).await {
            Ok(content) => {
                println!("Tweet result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
