use super::errors::McpHuggingFaceError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

#[tool(
    name = "SearchModels",
    description = "Search for HuggingFace models matching a keyword (filtered to text‑to‑image).",
    params(
        keyword = "Term to search for",
        limit = "Max results to return (default 10)"
    )
)]
async fn search_models_tool(keyword: String, limit: Option<usize>) -> Result<ToolResponseContent> {
    let client = Client::new();
    let limit = limit.unwrap_or(10);
    let url = format!(
        "https://huggingface.co/api/models?search={}&pipeline_tag=text-to-image&limit={}",
        urlencoding::encode(&keyword),
        limit
    );

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(McpHuggingFaceError::HttpError)?;
    let body = res.text().await.map_err(McpHuggingFaceError::HttpError)?;

    Ok(tool_text_content!(body))
}

#[cfg(test)]
#[cfg(feature = "huggingface")]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_models_tool() {
        match search_models_tool("stable diffusion".to_string(), Some(5)).await {
            Ok(content) => {
                println!("Search results: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
