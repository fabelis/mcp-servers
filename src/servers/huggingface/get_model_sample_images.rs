use super::errors::McpHuggingFaceError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

#[tool(
    name = "GetModelSampleImages",
    description = "Extract and return all image URLs found in the model's README.md.",
    params(model_id = "ID of the model, e.g. 'huggingface/CodeBERT'")
)]
async fn get_model_sample_images_tool(model_id: String) -> Result<ToolResponseContent> {
    let client = Client::new();
    let url = format!("https://huggingface.co/{}/raw/main/README.md", model_id);

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(McpHuggingFaceError::HttpError)?;
    let readme = res.text().await.map_err(McpHuggingFaceError::HttpError)?;

    if readme == "Invalid username or password." {
        return Err(
            McpHuggingFaceError::ApiError("Invalid username or password".to_string()).into(),
        );
    }

    // Extract image references from the README
    let image_links: Vec<String> = readme
        .lines()
        .filter_map(|line| {
            // Look for markdown image syntax: ![alt text](image_path)
            if let Some(start) = line.find("![") {
                if let Some(mid) = line[start..].find("](") {
                    if let Some(end) = line[start + mid + 2..].find(')') {
                        let image_path = &line[start + mid + 2..start + mid + 2 + end];
                        // Convert relative path to full HuggingFace URL
                        let full_url = if image_path.starts_with("http") {
                            image_path.to_string()
                        } else {
                            format!(
                                "https://huggingface.co/{}/resolve/main/{}",
                                model_id, image_path
                            )
                        };
                        return Some(full_url);
                    }
                }
            }
            None
        })
        .collect();

    let out = serde_json::to_string(&image_links)
        .map_err(|e| McpHuggingFaceError::JsonParseError(e.to_string()))?;
    Ok(tool_text_content!(out))
}

#[cfg(test)]
#[cfg(feature = "huggingface")]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_model_sample_images_tool() {
        match get_model_sample_images_tool("HiDream-ai/HiDream-I1-Full".to_string()).await {
            Ok(content) => {
                println!("Sample images: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
