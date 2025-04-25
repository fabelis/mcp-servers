use super::errors::McpReplicateError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::{Client, header::AUTHORIZATION};
use serde_json::Value;

#[tool(
    name = "GetModelInfo",
    description = "Fetch metadata for a Replicate model (owner/name).",
    params(model_id = "ID of the model, e.g. 'stability-ai/stable-diffusion'")
)]
async fn get_model_info_tool(model_id: String) -> Result<ToolResponseContent> {
    let token =
        std::env::var("REPLICATE_API_TOKEN").map_err(|_| McpReplicateError::MissingToken)?;
    let client = Client::new();
    let url = format!("https://api.replicate.com/v1/models/{}", model_id);

    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Token {}", token))
        .send()
        .await
        .map_err(McpReplicateError::HttpError)?;
    let json: Value = res.json().await.map_err(McpReplicateError::HttpError)?;

    Ok(tool_text_content!(json.to_string()))
}

#[cfg(test)]
#[cfg(feature = "replicate")]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_model_info_tool() {
        dotenv().ok();

        if std::env::var("REPLICATE_API_TOKEN").is_err() {
            println!("Skipping test_get_model_info_tool: No API token available");
            return;
        }

        match get_model_info_tool("black-forest-labs/flux-dev-lora".to_string()).await {
            Ok(content) => {
                println!("Model info: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
