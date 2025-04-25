use super::errors::McpTwitterError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use twitter_v2::{TwitterApi, authorization::Oauth1aToken, query};

#[tool(
    name = "SearchTweets",
    description = "Search tweets from Twitter. Required API and ACCESS values can be valued as 'null' if the info is not available.",
    params(
        query = "Search query for Twitter search",
        count = "The max count of tweets to be fetched",
        sort_order = "The Twitter sort method used for the search"
    )
)]
async fn search_tweets_tool(
    query: String,
    count: Option<f64>,
    sort_order: Option<String>,
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

    if query.is_empty() {
        return Err(McpTwitterError::InvalidQuery("Search cannot be empty".to_string()).into());
    }

    let mut builder = api.get_tweets_search_recent(query);

    if let Some(c) = count {
        if c < 0.0 {
            return Err(McpTwitterError::InvalidCount(c).into());
        }
        builder.max_results(c as usize);
    }

    if let Some(s) = sort_order {
        match s.as_str() {
            "recency" => builder.sort_order(query::SortOrder::Recency),
            "relevancy" => builder.sort_order(query::SortOrder::Relevancy),
            invalid => return Err(McpTwitterError::InvalidSortOrder(invalid.to_string()).into()),
        };
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
    async fn test_search_tweets_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("TWITTER_API_KEY").is_err()
            || std::env::var("TWITTER_API_SECRET").is_err()
            || std::env::var("TWITTER_ACCESS_TOKEN").is_err()
            || std::env::var("TWITTER_ACCESS_TOKEN_SECRET").is_err()
        {
            println!("Skipping test_search_tool: Missing required Twitter credentials");
            return;
        }

        match search_tweets_tool(
            "rust programming".to_string(),
            Some(5.0),
            Some("recency".to_string()),
        )
        .await
        {
            Ok(content) => {
                println!("Search result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
