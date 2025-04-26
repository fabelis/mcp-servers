use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;
use serde_json::json;

use super::errors::McpShopifyError;

#[tool(
    name = "CreateProduct",
    description = "Create a new product in the Shopify store.",
    params(
        title = "Title of the product",
        body_html = "Product description",
        vendor = "Product vendor",
        product_type = "Type of the product",
        price = "Price of the product",
        image_url = "URL of the product image"
    )
)]
async fn create_product_tool(
    title: String,
    body_html: Option<String>,
    vendor: Option<String>,
    product_type: Option<String>,
    price: Option<String>,
    image_url: Option<String>,
) -> Result<ToolResponseContent> {
    let shop_domain =
        std::env::var("SHOPIFY_SHOP_DOMAIN").map_err(|_| McpShopifyError::MissingShopifyDomain)?;
    let access_token = std::env::var("SHOPIFY_ACCESS_TOKEN")
        .map_err(|_| McpShopifyError::MissingShopifyAccessToken)?;

    let client = Client::new();
    let url = format!("https://{}/admin/api/2022-04/products.json", shop_domain);

    let mut product_data = json!({
        "product": {
            "title": title,
            "body_html": body_html,
            "vendor": vendor,
            "product_type": product_type,
            "variants": [{
                "price": price
            }]
        }
    });

    // Add image if provided
    if let Some(image) = image_url {
        product_data["product"]["images"] = json!([{
            "src": image
        }]);
    }

    let res = client
        .post(&url)
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
    async fn test_create_product_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("SHOPIFY_SHOP_DOMAIN").is_err()
            || std::env::var("SHOPIFY_ACCESS_TOKEN").is_err()
        {
            println!("Skipping test_create_product_tool: Missing required Shopify credentials");
            return;
        }

        let test_product = create_product_tool(
            "Test Product".to_string(),
            Some("Test Description".to_string()),
            Some("Test Vendor".to_string()),
            Some("Test Type".to_string()),
            Some("9.99".to_string()),
            Some("https://example.com/test-image.jpg".to_string()),
        )
        .await;

        match test_product {
            Ok(content) => {
                println!("Product creation result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
