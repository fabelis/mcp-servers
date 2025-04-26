use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use reqwest::Client;
use serde_json::json;

use super::errors::McpShopifyError;

#[tool(
    name = "AddProductMedia",
    description = "Attach media (images, videos, or models) to an existing Shopify product using GraphQL.",
    params(
        product_id = "The Shopify product GID (e.g., gid://shopify/Product/1234567890).",
        image_url = "URL of the product image",
        image_alt = "Alt text for the image",
    )
)]
async fn add_product_image_tool(
    product_id: String,
    image_url: String,
    image_alt: String,
) -> Result<ToolResponseContent> {
    let shop_domain =
        std::env::var("SHOPIFY_SHOP_DOMAIN").map_err(|_| McpShopifyError::MissingShopifyDomain)?;
    let access_token = std::env::var("SHOPIFY_ACCESS_TOKEN")
        .map_err(|_| McpShopifyError::MissingShopifyAccessToken)?;

    let client = Client::new();
    let graphql_url = format!("https://{}/admin/api/2025-04/graphql.json", shop_domain);

    // Create media input for the image
    let media_input = json!([
        {
            "alt": image_alt,
            "mediaContentType": "IMAGE",
            "originalSource": image_url
        }
    ]);

    let mutation = r#"
        mutation productCreateMedia($media: [CreateMediaInput!]!, $productId: ID!) {
            productCreateMedia(media: $media, productId: $productId) {
                media {
                    alt
                    mediaContentType
                    status
                }
                mediaUserErrors {
                    field
                    message
                }
                product {
                    id
                    title
                }
            }
        }
    "#;

    let graphql_body = json!({
        "query": mutation,
        "variables": {
            "productId": product_id,
            "media": media_input
        }
    });

    let res = client
        .post(&graphql_url)
        .header("X-Shopify-Access-Token", &access_token)
        .header("Content-Type", "application/json")
        .json(&graphql_body)
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
    async fn test_add_product_image_tool() {
        dotenv().ok();

        // Check if any required env var is missing
        if std::env::var("SHOPIFY_SHOP_DOMAIN").is_err()
            || std::env::var("SHOPIFY_ACCESS_TOKEN").is_err()
        {
            println!("Skipping test_add_product_image_tool: Missing required Shopify credentials");
            return;
        }

        let test_product_id = "gid://shopify/Product/9829406277917";
        let test_image_url = "https://cdn.shopify.com/s/files/1/0070/7032/files/logohashtag.png";
        let test_image_alt = "Test Product Image";

        let test_result = add_product_image_tool(
            test_product_id.to_string(),
            test_image_url.to_string(),
            test_image_alt.to_string(),
        )
        .await;

        match test_result {
            Ok(content) => {
                println!("Media addition result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
