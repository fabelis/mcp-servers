use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

use super::errors::McpShopifyError;

#[tool(
    name = "DeleteProduct",
    description = "Delete a product from the Shopify store.",
    params(product_id = "ID of the product to delete")
)]
async fn delete_product_tool(product_id: String) -> Result<ToolResponseContent> {
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
        .delete(&url)
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
    async fn test_delete_product_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("SHOPIFY_SHOP_DOMAIN").is_err()
            || std::env::var("SHOPIFY_ACCESS_TOKEN").is_err()
        {
            println!("Skipping test_delete_product_tool: Missing required Shopify credentials");
            return;
        }

        // Note: This test requires a valid product ID to work
        let product_id = "123456789"; // Replace with a real product ID
        match delete_product_tool(product_id.to_string()).await {
            Ok(content) => {
                println!("Product deletion result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
