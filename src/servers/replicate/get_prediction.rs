use super::errors::McpReplicateError;
use anyhow::Result;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use mcp_core::{tool_image_content, tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::{Client, header::AUTHORIZATION};
use serde_json::Value;
use url::Url;

#[tool(
    name = "GetPrediction",
    description = "Check status and retrieve outputs of a Replicate prediction.",
    params(prediction_id = "ID returned by GenerateImage")
)]
async fn get_prediction_tool(prediction_id: String) -> Result<Vec<ToolResponseContent>> {
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

    println!("Prediction response: {:#?}", json);

    // Extract the image URL from the output array
    let image_url = if let Some(url_str) = json["output"].as_str() {
        // Handle case where output is directly a string
        url_str
    } else {
        // Handle case where output is an array of strings
        json["output"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|url| url.as_str())
            .ok_or_else(|| McpReplicateError::InvalidResponse("No image URL found".into()))?
    };

    // Extract extension and determine mime type
    let mime_type = Url::parse(image_url)
        .ok()
        .and_then(|url| url.path().split('.').last().map(|s| s.to_string()))
        .map(|ext| match ext.as_str() {
            "webp" => "image/webp",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            _ => "application/octet-stream",
        })
        .unwrap_or("application/octet-stream")
        .to_string();

    // Fetch the image data
    let image_response = client
        .get(image_url)
        .send()
        .await
        .map_err(McpReplicateError::HttpError)?;

    let image_data = image_response
        .bytes()
        .await
        .map_err(McpReplicateError::HttpError)?;

    Ok(vec![
        tool_image_content!(STANDARD.encode(image_data), mime_type),
        tool_text_content!(json.to_string()),
    ])
}

#[cfg(test)]
#[cfg(feature = "replicate")]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_prediction_tool() {
        dotenv().ok();

        if std::env::var("TEST_REPLICATE_PREDICTION_ID").is_err() {
            println!("Skipping test_get_prediction_tool: No prediction ID available");
            return;
        }

        let prediction_id = std::env::var("TEST_REPLICATE_PREDICTION_ID").unwrap();

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
