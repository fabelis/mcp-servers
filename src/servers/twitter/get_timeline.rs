use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use twitter_v2::{TwitterApi, authorization::Oauth1aToken, id::NumericId};

use super::errors::McpTwitterError;

#[tool(
    name = "GetTimeline",
    description = "Fetches the user's timeline from Twitter. Required API and ACCESS values can be valued as 'null' if the info is not available.",
    params(
        count = "The max count of tweets to be fetched",
        latest_id = "The Tweet ID to fetch tweets after"
    )
)]
async fn get_timeline_tool(
    count: Option<f64>,
    latest_id: Option<f64>,
) -> Result<ToolResponseContent> {
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

    let user = api
        .get_users_me()
        .send()
        .await
        .map_err(McpTwitterError::TwitterApiError)?
        .into_data()
        .ok_or_else(|| McpTwitterError::UserNotFound)?;

    let mut builder = api.get_user_tweets(user.id);

    if let Some(c) = count {
        if c < 0.0 {
            return Err(McpTwitterError::InvalidCount(c).into());
        }
        builder.max_results(c as usize);
    }

    if let Some(id) = latest_id {
        if (id as u64) == 0 {
            return Err(McpTwitterError::InvalidLatestID(id).into());
        }
        builder.since_id(NumericId::new(id as u64));
    }

    let tweets = builder
        .send()
        .await
        .map_err(McpTwitterError::TwitterApiError)?
        .into_data()
        .ok_or_else(|| McpTwitterError::TweetsNotFound)?;

    Ok(tool_text_content!(
        serde_json::to_string(&tweets).map_err(McpTwitterError::ResponseSerializeError)?
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_timeline_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("TWITTER_API_KEY").is_err()
            || std::env::var("TWITTER_API_SECRET").is_err()
            || std::env::var("TWITTER_ACCESS_TOKEN").is_err()
            || std::env::var("TWITTER_ACCESS_TOKEN_SECRET").is_err()
        {
            println!("Skipping test_get_timeline_tool: Missing required Twitter credentials");
            return;
        }

        match get_timeline_tool(Some(5.0), None).await {
            Ok(content) => {
                println!("Timeline result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
