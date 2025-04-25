use super::errors::McpReplicateError;
use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::{Client, header::AUTHORIZATION};

#[tool(
    name = "ListModels",
    description = "List models from Replicate (with optional name filter).",
    params(
        name_filter = "Optional substring to match in model name",
        limit = "Max results to return (default 10)"
    )
)]
async fn list_models_tool(
    name_filter: Option<String>,
    limit: Option<usize>,
) -> Result<ToolResponseContent> {
    let token =
        std::env::var("REPLICATE_API_TOKEN").map_err(|_| McpReplicateError::MissingToken)?;
    let client = Client::new();
    let mut url = format!(
        "https://api.replicate.com/v1/models?limit={}",
        limit.unwrap_or(10)
    );
    if let Some(f) = name_filter {
        url.push_str(&format!(
            "&filter[name][contains]={}",
            urlencoding::encode(&f)
        ));
    }

    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Token {}", token))
        .send()
        .await
        .map_err(McpReplicateError::HttpError)?;
    let body = res.text().await.map_err(McpReplicateError::HttpError)?;

    Ok(tool_text_content!(body))
}

#[cfg(test)]
#[cfg(feature = "replicate")]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_list_models_tool() {
        dotenv().ok();

        if std::env::var("REPLICATE_API_TOKEN").is_err() {
            println!("Skipping test_list_models_tool: No API token available");
            return;
        }

        match list_models_tool(Some("stable-diffusion".to_string()), Some(5)).await {
            Ok(content) => {
                println!("Models list: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
