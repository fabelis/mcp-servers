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
    name = "EditImageWithMask",
    description = "Edit an image via Replicate by providing an image URL, prompt, and optional mask.",
    params(
        image = "URL of the image to edit",
        mask = "URL of the mask image defining areas to edit",
        prompt = "Text prompt for guiding the edit",
    )
)]
async fn edit_image_with_mask_tool(
    image: String,
    mask: String,
    prompt: String,
) -> Result<ToolResponseContent> {
    let token =
        std::env::var("REPLICATE_API_TOKEN").map_err(|_| McpReplicateError::MissingToken)?;
    let client = Client::new();

    // Create the prediction URL
    let url = "https://api.replicate.com/v1/models/black-forest-labs/flux-fill-pro/predictions"
        .to_string();

    // Create the input payload
    let input = serde_json::json!({
        "image": image,
        "prompt": prompt,
        "mask": mask
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
        .post(&url)
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
    async fn test_edit_image_with_mask_tool() {
        dotenv().ok();

        if std::env::var("REPLICATE_API_TOKEN").is_err() {
            println!("Skipping test_edit_image_with_mask_tool: No API token available");
            return;
        }

        match edit_image_with_mask_tool(
            "https://replicate.delivery/pbxt/M0gpKVE9wmEtOQFNDOpwz1uGs0u6nK2NcE85IihwlN0ZEnMF/kill-bill-poster.jpg".to_string(),
            "https://replicate.delivery/pbxt/M0gpLCYdCLbnhcz95Poy66q30XW9VSCN65DoDQ8IzdzlQonw/kill-bill-mask.png".to_string(),
            "movie poster says \"FLUX FILL\"".to_string(),
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
