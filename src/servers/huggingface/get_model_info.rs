use super::errors::McpHuggingFaceError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;
use serde_json::Value;

#[tool(
    name = "GetModelInfo",
    description = "Fetch metadata for a HuggingFace model (author, tags, license, description).",
    params(model_id = "ID of the model, e.g. 'HiDream-ai/HiDream-I1-Full'")
)]
async fn get_model_info_tool(model_id: String) -> Result<ToolResponseContent> {
    let client = Client::new();
    let url = format!("https://huggingface.co/api/models/{}", model_id);

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(McpHuggingFaceError::HttpError)?;
    let json: Value = res.json().await.map_err(McpHuggingFaceError::HttpError)?;

    let summary = serde_json::json!({
        "modelId": model_id,
        "author": json.get("author").and_then(Value::as_str).unwrap_or(""),
        "tags": json.get("tags").unwrap_or(&Value::Null),
        "license": json.get("license").and_then(Value::as_str).unwrap_or(""),
        "pipeline": json.get("pipeline_tag").and_then(Value::as_str).unwrap_or(""),
        "cardData": json.get("cardData").unwrap_or(&Value::Null)
    });

    Ok(tool_text_content!(summary.to_string()))
}

#[cfg(test)]
#[cfg(feature = "huggingface")]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_model_info_tool() {
        match get_model_info_tool("HiDream-ai/HiDream-I1-Full".to_string()).await {
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
