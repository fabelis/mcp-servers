use super::errors::McpReplicateError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::{Client, header::AUTHORIZATION};
use serde_json::Value;

#[tool(
    name = "GetPrediction",
    description = "Check status and retrieve outputs of a Replicate prediction.",
    params(prediction_id = "ID returned by GenerateImage")
)]
async fn get_prediction_tool(prediction_id: String) -> Result<ToolResponseContent> {
    let token =
        std::env::var("REPLICATE_API_TOKEN").map_err(|_| McpReplicateError::MissingToken)?;
    let client = Client::new();
    let url = format!("https://api.replicate.com/v1/predictions/{}", prediction_id);

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
    async fn test_get_prediction_tool() {
        dotenv().ok();

        if std::env::var("REPLICATE_PREDICTION_ID").is_err() {
            println!("Skipping test_get_prediction_tool: No prediction ID available");
            return;
        }

        let prediction_id = std::env::var("REPLICATE_PREDICTION_ID").unwrap();

        match get_prediction_tool(prediction_id.to_string()).await {
            Ok(content) => {
                println!("Prediction status: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
