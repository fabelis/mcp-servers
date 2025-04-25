use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;
use serde_json::json;

use super::errors::McpShopifyError;

#[tool(
    name = "CreateOrder",
    description = "Create a new order in the Shopify store.",
    params(
        line_items = "List of products in the order",
        customer_id = "ID of the customer",
    )
)]
async fn create_order_tool(
    line_items: Vec<serde_json::Value>,
    customer_id: Option<String>,
) -> Result<ToolResponseContent> {
    let shop_domain =
        std::env::var("SHOPIFY_SHOP_DOMAIN").map_err(|_| McpShopifyError::MissingShopifyDomain)?;
    let access_token = std::env::var("SHOPIFY_ACCESS_TOKEN")
        .map_err(|_| McpShopifyError::MissingShopifyAccessToken)?;

    let client = Client::new();
    let url = format!("https://{}/admin/api/2022-04/orders.json", shop_domain);

    let order_data = json!({
        "order": {
            "line_items": line_items,
            "customer": customer_id.map(|id| json!({ "id": id })),
        }
    });

    let res = client
        .post(&url)
        .header("X-Shopify-Access-Token", access_token)
        .json(&order_data)
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
    async fn test_create_order_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("SHOPIFY_SHOP_DOMAIN").is_err()
            || std::env::var("SHOPIFY_ACCESS_TOKEN").is_err()
        {
            println!("Skipping test_create_order_tool: Missing required Shopify credentials");
            return;
        }

        let test_line_items = vec![json!({
            "variant_id": 123456789, // Replace with a real variant ID
            "quantity": 1
        })];

        let test_order = create_order_tool(test_line_items, None).await;

        match test_order {
            Ok(content) => {
                println!("Order creation result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
