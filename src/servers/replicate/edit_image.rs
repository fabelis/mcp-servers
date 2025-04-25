use super::errors::McpReplicateError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::{
    Client,
    header::{AUTHORIZATION, CONTENT_TYPE, HeaderValue},
};
use serde_json::Value;

#[tool(
    name = "EditImage",
    description = "Edit an image with a prompt.",
    params(
        image = "URL of the image to edit",
        prompt = "Text prompt for guiding the edit",
        steps = "Number of steps for image generation (default: 28)",
        guidance = "Guidance scale for the model (default: 25)"
    )
)]
async fn edit_image_tool(
    image: String,
    prompt: String,
    steps: Option<u32>,
    guidance: Option<u32>,
) -> Result<ToolResponseContent> {
    let token =
        std::env::var("REPLICATE_API_TOKEN").map_err(|_| McpReplicateError::MissingToken)?;
    let client = Client::new();

    // Fixed model URL for flux-canny-pro
    let url = "https://api.replicate.com/v1/models/black-forest-labs/flux-canny-pro/predictions";

    // Create the input payload with defaults
    let input = serde_json::json!({
        "control_image": image,
        "prompt": prompt,
        "steps": steps.unwrap_or(28),
        "guidance": guidance.unwrap_or(25)
    });

    // Create the full payload
    let payload = serde_json::json!({
        "input": input
    });

    // Create the request with the "Prefer: wait" header
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Token {}", token))?,
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("Prefer", HeaderValue::from_static("wait"));

    // Send the request
    let res = client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await
        .map_err(McpReplicateError::HttpError)?;

    // Check if the response is successful
    if !res.status().is_success() {
        let error_text = res.text().await.map_err(McpReplicateError::HttpError)?;
        return Err(
            McpReplicateError::ContentParseError(format!("API error: {}", error_text)).into(),
        );
    }

    // Parse the response
    let result: Value = res.json().await.map_err(McpReplicateError::HttpError)?;

    Ok(tool_text_content!(result.to_string()))
}

#[cfg(test)]
#[cfg(feature = "replicate")]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_edit_image_tool() {
        dotenv().ok();

        if std::env::var("REPLICATE_API_TOKEN").is_err() {
            println!("Skipping test_edit_image_tool: No API token available");
            return;
        }

        match edit_image_tool(
            "https://replicate.delivery/pbxt/M0j11UQhwUWoxUQ9hJCOaALsAHTeoPZcGGtUf6n3BJxtKHul/output-14.webp".to_string(),
            "a photo of a red car on a city street".to_string(),
            Some(28),
            Some(25),
        )
        .await
        {
            Ok(content) => {
                println!("Prediction result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
