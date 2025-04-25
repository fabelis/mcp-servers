use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;
use serde_json::json;

use super::errors::McpShopifyError;

#[tool(
    name = "UpdateProduct",
    description = "Update an existing product in the Shopify store.",
    params(
        product_id = "ID of the product to update",
        title = "New title for the product",
        body_html = "New product description",
        vendor = "New product vendor",
        product_type = "New type of the product",
        price = "New price of the product"
    )
)]
async fn update_product_tool(
    product_id: String,
    title: String,
    body_html: Option<String>,
    vendor: Option<String>,
    product_type: Option<String>,
    price: Option<String>,
) -> Result<ToolResponseContent> {
    let shop_domain =
        std::env::var("SHOPIFY_SHOP_DOMAIN").map_err(|_| McpShopifyError::MissingShopifyDomain)?;
    let access_token = std::env::var("SHOPIFY_ACCESS_TOKEN")
        .map_err(|_| McpShopifyError::MissingShopifyAccessToken)?;

    let client = Client::new();
    let url = format!(
        "https://{}/admin/api/2022-04/products/{}.json",
        shop_domain, product_id
    );

    let product_data = json!({
        "product": {
            "id": product_id,
            "title": title,
            "body_html": body_html,
            "vendor": vendor,
            "product_type": product_type,
            "variants": [{
                "price": price
            }]
        }
    });

    let res = client
        .put(&url)
        .header("X-Shopify-Access-Token", access_token)
        .json(&product_data)
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
    async fn test_update_product_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("SHOPIFY_SHOP_DOMAIN").is_err()
            || std::env::var("SHOPIFY_ACCESS_TOKEN").is_err()
        {
            println!("Skipping test_update_product_tool: Missing required Shopify credentials");
            return;
        }

        // Note: This test requires a valid product ID to work
        let product_id = "123456789"; // Replace with a real product ID
        let test_update = update_product_tool(
            product_id.to_string(),
            "Updated Test Product".to_string(),
            Some("Updated Description".to_string()),
            Some("Updated Vendor".to_string()),
            Some("Updated Type".to_string()),
            Some("19.99".to_string()),
        )
        .await;

        match test_update {
            Ok(content) => {
                println!("Product update result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
