use super::errors::McpReplicateError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::{Client, header::AUTHORIZATION};

#[tool(
    name = "WhoAmI",
    description = "Get details for the current Replicate API token."
)]
async fn whoami_tool() -> Result<ToolResponseContent> {
    let token =
        std::env::var("REPLICATE_API_TOKEN").map_err(|_| McpReplicateError::MissingToken)?;
    let client = Client::new();
    let url = "https://api.replicate.com/v1/user";

    let res = client
        .get(url)
        .header(AUTHORIZATION, format!("Token {}", token))
        .send()
        .await
        .map_err(McpReplicateError::HttpError)?;
    let info = res.text().await.map_err(McpReplicateError::HttpError)?;

    Ok(tool_text_content!(info))
}

#[cfg(test)]
#[cfg(feature = "replicate")]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_whoami_tool() {
        dotenv().ok();

        if std::env::var("REPLICATE_API_TOKEN").is_err() {
            println!("Skipping test_whoami_tool: No API token available");
            return;
        }

        match whoami_tool().await {
            Ok(content) => {
                println!("User info: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
