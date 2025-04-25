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
    name = "GenerateImage",
    description = "Generate an image via Replicate by prompting the model.",
    params(
        model_id = "ID of the model, e.g. 'black-forest-labs/flux-dev-lora'",
        prompt = "Text prompt for generation",
        lora_weights = "Optional LoRA weights to apply, e.g. 'fofr/flux-80s-cyberpunk'"
    )
)]
async fn generate_image_tool(
    model_id: String,
    prompt: String,
    lora_weights: Option<String>,
) -> Result<ToolResponseContent> {
    let token =
        std::env::var("REPLICATE_API_TOKEN").map_err(|_| McpReplicateError::MissingToken)?;
    let client = Client::new();

    // Create the prediction URL
    let url = format!(
        "https://api.replicate.com/v1/models/{}/predictions",
        model_id
    );

    // Create the input payload
    let mut input = serde_json::json!({
        "prompt": prompt
    });

    // Add lora_weights if provided
    if let Some(weights) = lora_weights {
        if let serde_json::Value::Object(ref mut map) = input {
            map.insert(
                "lora_weights".to_string(),
                serde_json::Value::String(weights),
            );
        }
    }

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
    async fn test_generate_image_tool() {
        dotenv().ok();

        if std::env::var("REPLICATE_API_TOKEN").is_err() {
            println!("Skipping test_generate_image_tool: No API token available");
            return;
        }

        match generate_image_tool(
            "black-forest-labs/flux-dev-lora".to_string(),
            "style of 80s cyberpunk, a portrait photo".to_string(),
            Some("fofr/flux-80s-cyberpunk".to_string()),
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
