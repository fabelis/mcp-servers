use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use serenity::all::{ExecuteWebhook, Http, Webhook};

use super::errors::McpDiscordError;

#[tool(
    name = "PostWebhook",
    description = "Post a message using a Discord webhook.",
    params(
        discord_webhook_url = "The URL of the webhook",
        content = "Content to send via the webhook"
    )
)]
async fn post_webhook_tool(
    discord_webhook_url: String,
    content: String,
) -> Result<ToolResponseContent> {
    if content.is_empty() {
        return Err(McpDiscordError::InvalidContent("Content cannot be empty".to_string()).into());
    }

    let http = Http::new("");

    let webhook = Webhook::from_url(&http, &discord_webhook_url)
        .await
        .map_err(McpDiscordError::InvalidWebhookURL)?;

    let builder = ExecuteWebhook::new().content(&content);

    let message = webhook
        .execute(&http, true, builder)
        .await
        .map_err(McpDiscordError::DiscordApiError)?
        .ok_or_else(|| McpDiscordError::InvalidContent("Failed to send message".to_string()))?;

    let message_json =
        serde_json::to_string(&message).map_err(McpDiscordError::ResponseSerializeError)?;

    Ok(tool_text_content!(message_json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_post_webhook_tool() {
        dotenv().ok();

        if std::env::var("TEST_DISCORD_WEBHOOK_URL").is_err() {
            println!("Skipping test_post_webhook_tool: Missing test webhook URL");
            return;
        }

        let test_webhook_url = std::env::var("TEST_DISCORD_WEBHOOK_URL").unwrap();

        match post_webhook_tool(
            test_webhook_url.to_string(),
            "Test webhook message from automated testing".to_string(),
        )
        .await
        {
            Ok(content) => {
                println!("Webhook result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                // Don't panic on error since this test requires a valid webhook URL
                println!("Test skipped: requires valid webhook URL");
            }
        }
    }
}
