use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use pdf_extract::extract_text;
use reqwest;
use std::fs;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use url::Url;

use super::errors::McpArvixError;

#[tool(
    name = "ExtractPaperText",
    description = "Extract text from an arXiv paper PDF.",
    params(paper_url = "The arXiv paper URL or ID")
)]
pub async fn extract_paper_text_tool(paper_url: String) -> Result<ToolResponseContent> {
    // Extract arXiv ID from URL or use directly
    let arxiv_id = if paper_url.contains("arxiv.org") {
        let url = Url::parse(&paper_url).map_err(|e| McpArvixError::ApiError(e.to_string()))?;
        url.path_segments()
            .ok_or_else(|| McpArvixError::ApiError("Invalid URL".to_string()))?
            .last()
            .ok_or_else(|| McpArvixError::ApiError("No paper ID found".to_string()))?
            .to_string()
    } else {
        paper_url
    };

    // Construct PDF URL
    let pdf_url = format!("https://arxiv.org/pdf/{}.pdf", arxiv_id);

    // Create temp directory if it doesn't exist
    let temp_dir = PathBuf::from("temp");
    if !temp_dir.exists() {
        fs::create_dir(&temp_dir).map_err(|e| McpArvixError::ApiError(e.to_string()))?;
    }

    // Download PDF
    let pdf_path = temp_dir.join(format!("{}.pdf", arxiv_id));
    let response = reqwest::get(&pdf_url)
        .await
        .map_err(McpArvixError::HttpError)?;

    if !response.status().is_success() {
        return Err(McpArvixError::ApiError(format!(
            "Failed to download PDF: {}",
            response.status()
        ))
        .into());
    }

    let mut file = File::create(&pdf_path)
        .await
        .map_err(|e| McpArvixError::ApiError(e.to_string()))?;
    let content = response.bytes().await.map_err(McpArvixError::HttpError)?;
    file.write_all(&content)
        .await
        .map_err(|e| McpArvixError::ApiError(e.to_string()))?;

    // Extract text
    let text = extract_text(&pdf_path).map_err(|e| McpArvixError::ApiError(e.to_string()))?;

    // Clean up
    fs::remove_file(&pdf_path).map_err(|e| McpArvixError::ApiError(e.to_string()))?;

    Ok(tool_text_content!(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_extract_paper_features_tool() {
        // Test with a known arXiv paper
        match extract_paper_text_tool("https://arxiv.org/abs/2401.00001".to_string()).await {
            Ok(content) => {
                println!("Extracted content: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
