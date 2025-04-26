use super::errors::McpArxivError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

#[tool(
    name = "ListRecords",
    description = "Bulk harvest metadata from ArXiv using OAI-PMH interface.",
    params(
        from = "Start date for records (YYYY-MM-DD)",
        until = "End date for records (YYYY-MM-DD)",
        metadata_prefix = "Metadata format (default: oai_dc)",
        set = "Optional set identifier to filter records"
    )
)]
pub async fn list_records_tool(
    from: String,
    until: String,
    metadata_prefix: Option<String>,
    set: Option<String>,
) -> Result<ToolResponseContent> {
    let client = Client::new();
    let base_url = "http://export.arxiv.org/oai2";

    // Build query parameters
    let metadata_prefix_str = metadata_prefix.unwrap_or_else(|| "oai_dc".to_string());
    let mut params = vec![
        ("verb", "ListRecords"),
        ("from", &from),
        ("until", &until),
        ("metadataPrefix", &metadata_prefix_str),
    ];

    if let Some(set_id) = &set {
        params.push(("set", set_id));
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
        return Err(McpArxivError::ApiError(format!("OAI-PMH API error: {}", error_text)).into());
    }

    // Parse the response
    let result = res.text().await.map_err(McpArxivError::HttpError)?;

    Ok(tool_text_content!(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_records_tool() {
        match list_records_tool(
            "2024-01-01".to_string(),
            "2024-01-01".to_string(),
            Some("oai_dc".to_string()),
            None,
        )
        .await
        {
            Ok(content) => {
                println!("OAI-PMH result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
