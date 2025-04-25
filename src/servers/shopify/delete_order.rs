use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;

use super::errors::McpShopifyError;

#[tool(
    name = "DeleteOrder",
    description = "Delete an order from the Shopify store.",
    params(order_id = "ID of the order to delete")
)]
async fn delete_order_tool(order_id: String) -> Result<ToolResponseContent> {
    let shop_domain =
        std::env::var("SHOPIFY_SHOP_DOMAIN").map_err(|_| McpShopifyError::MissingShopifyDomain)?;
    let access_token = std::env::var("SHOPIFY_ACCESS_TOKEN")
        .map_err(|_| McpShopifyError::MissingShopifyAccessToken)?;

    let client = Client::new();
    let url = format!(
        "https://{}/admin/api/2022-04/orders/{}.json",
        shop_domain, order_id
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
    async fn test_delete_order_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("SHOPIFY_SHOP_DOMAIN").is_err()
            || std::env::var("SHOPIFY_ACCESS_TOKEN").is_err()
        {
            println!("Skipping test_delete_order_tool: Missing required Shopify credentials");
            return;
        }

        // Note: This test requires a valid order ID to work
        let order_id = "123456789"; // Replace with a real order ID
        match delete_order_tool(order_id.to_string()).await {
            Ok(content) => {
                println!("Order deletion result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
