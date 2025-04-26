use super::errors::McpArxivError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

#[tool(
    name = "SearchPapers",
    description = "Search for papers on ArXiv using various criteria.",
    params(
        query = "The search query string (e.g. 'quantum computing', 'au:\"Einstein, Albert\"', 'cat:cs.CV')",
        start = "Starting index for results (default: 0)",
        max_results = "Maximum number of results to return (default: 5)",
        sort_by = "Sort field (submittedDate, lastUpdatedDate, relevance)",
        sort_order = "Sort order (ascending, descending)"
    )
)]
pub async fn search_papers_tool(
    query: String,
    start: Option<u32>,
    max_results: Option<u32>,
    sort_by: Option<String>,
    sort_order: Option<String>,
) -> Result<ToolResponseContent> {
    let client = Client::new();
    let base_url = "http://export.arxiv.org/api/query";

    // Build query parameters
    let mut params = vec![
        ("search_query", query),
        ("start", start.unwrap_or(0).to_string()),
        ("max_results", max_results.unwrap_or(5).to_string()),
    ];

    if let Some(sort) = sort_by {
        params.push(("sortBy", sort));
    }
    if let Some(order) = sort_order {
        params.push(("sortOrder", order));
    }

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
    async fn test_search_papers_tool() {
        match search_papers_tool(
            "quantum computing".to_string(),
            Some(0),
            Some(2),
            Some("submittedDate".to_string()),
            Some("descending".to_string()),
        )
        .await
        {
            Ok(content) => {
                println!("Search result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
