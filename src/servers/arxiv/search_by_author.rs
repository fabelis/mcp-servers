use super::errors::McpArxivError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

#[tool(
    name = "SearchByAuthor",
    description = "Search for papers by a specific author on ArXiv.",
    params(
        author = "Author name (e.g. 'Einstein, Albert')",
        start = "Starting index for results (default: 0)",
        max_results = "Maximum number of results to return (default: 10)"
    )
)]
pub async fn search_by_author_tool(
    author: String,
    start: Option<u32>,
    max_results: Option<u32>,
) -> Result<ToolResponseContent> {
    let client = Client::new();
    let base_url = "http://export.arxiv.org/api/query";

    // Format the author query
    let search_query = format!("au:\"{}\"", author);

    // Build query parameters
    let params = [
        ("search_query", &search_query),
        ("start", &start.unwrap_or(0).to_string()),
        ("max_results", &max_results.unwrap_or(10).to_string()),
    ];

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
    async fn test_search_by_author_tool() {
        match search_by_author_tool("Alekou, A".to_string(), Some(0), Some(5)).await {
            Ok(content) => {
                println!("Author search result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
