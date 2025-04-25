use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

use super::errors::McpShopifyError;

#[tool(
    name = "GetProduct",
    description = "Fetch details of a specific product.",
    params(product_id = "ID of the product to retrieve")
)]
async fn get_product_tool(product_id: String) -> Result<ToolResponseContent> {
    let shop_domain =
        std::env::var("SHOPIFY_SHOP_DOMAIN").map_err(|_| McpShopifyError::MissingShopifyDomain)?;
    let access_token = std::env::var("SHOPIFY_ACCESS_TOKEN")
        .map_err(|_| McpShopifyError::MissingShopifyAccessToken)?;

    let client = Client::new();
    let url = format!(
        "https://{}/admin/api/2022-04/products/{}.json",
        shop_domain, product_id
    );

    let res = client
        .get(&url)
        .header("X-Shopify-Access-Token", access_token)
        .send()
        .await
        .map_err(McpShopifyError::HttpError)?;

    let body = res.text().await.map_err(McpShopifyError::HttpError)?;

    Ok(tool_text_content!(body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_product_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("SHOPIFY_SHOP_DOMAIN").is_err()
            || std::env::var("SHOPIFY_ACCESS_TOKEN").is_err()
        {
            println!("Skipping test_get_product_tool: Missing required Shopify credentials");
            return;
        }

        // Note: This test requires a valid product ID to work
        let product_id = "123456789"; // Replace with a real product ID
        match get_product_tool(product_id.to_string()).await {
            Ok(content) => {
                println!("Product result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
