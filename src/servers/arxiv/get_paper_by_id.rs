use super::errors::McpArxivError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

#[tool(
    name = "GetPaperById",
    description = "Fetch a specific paper by its ArXiv ID.",
    params(id = "The ArXiv ID (e.g. '2101.00001v2')")
)]
pub async fn get_paper_by_id_tool(id: String) -> Result<ToolResponseContent> {
    let client = Client::new();
    let base_url = "http://export.arxiv.org/api/query";

    // Build query parameters
    let params = [("id_list", id)];

    // Send the request
    let res = client
        .get(base_url)
        .query(&params)
        .send()
        .await
        .map_err(McpArxivError::HttpError)?;

    // Check if the response is successful
    if !res.status().is_success() {
        let error_text = res.text().await.map_err(McpArxivError::HttpError)?;
        return Err(McpArxivError::ApiError(format!("ArXiv API error: {}", error_text)).into());
    }

    // Parse the response
    let result = res.text().await.map_err(McpArxivError::HttpError)?;

    Ok(tool_text_content!(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_paper_by_id_tool() {
        match get_paper_by_id_tool("2101.00001v2".to_string()).await {
            Ok(content) => {
                println!("Paper result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
