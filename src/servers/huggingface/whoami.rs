use super::errors::McpHuggingFaceError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

#[tool(
    name = "WhoAmI",
    description = "Retrieve the username, email, and orgs for the current HF API token"
)]
async fn whoami_tool() -> Result<ToolResponseContent> {
    let token = std::env::var("HF_API_TOKEN").map_err(|_| McpHuggingFaceError::MissingToken)?;
    let client = Client::new();
    let url = "https://huggingface.co/api/whoami-v2";

    let res = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(McpHuggingFaceError::HttpError)?;
    let info = res.text().await.map_err(McpHuggingFaceError::HttpError)?;

    Ok(tool_text_content!(info))
}

#[cfg(test)]
#[cfg(feature = "huggingface")]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_whoami_tool() {
        dotenv().ok();

        if std::env::var("HF_API_TOKEN").is_err() {
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
