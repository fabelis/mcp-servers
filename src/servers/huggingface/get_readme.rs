use super::errors::McpHuggingFaceError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

#[tool(
    name = "GetReadme",
    description = "Retrieve README.md file from a HuggingFace model.",
    params(model_id = "ID of the model to retrieve")
)]
async fn get_readme_tool(model_id: String) -> Result<ToolResponseContent> {
    let client = Client::new();
    let url = format!("https://huggingface.co/{}/raw/main/README.md", model_id);

    let res = client
        .get(&url)
        .send()
        .await
        .map_err(McpHuggingFaceError::HttpError)?;

    let readme = res.text().await.map_err(McpHuggingFaceError::HttpError)?;

    Ok(tool_text_content!(readme))
}

#[cfg(test)]
#[cfg(feature = "huggingface")]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_readme_tool() {
        match get_readme_tool("HiDream-ai/HiDream-I1-Full".to_string()).await {
            Ok(content) => {
                println!("{:#?}", content);
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        }
    }
}
