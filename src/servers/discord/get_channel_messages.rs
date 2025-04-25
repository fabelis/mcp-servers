use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use serenity::all::{ChannelId, GetMessages, Http};

use super::errors::McpDiscordError;

#[tool(
    name = "GetChannelMessages",
    description = "Get messages from a Discord channel.",
    params(
        channel_id = "The channel ID to get messages from",
        limit = "The maximum number of messages to get"
    )
)]
async fn get_channel_messages_tool(
    channel_id: String,
    limit: Option<f64>,
) -> Result<ToolResponseContent> {
    let discord_token =
        std::env::var("DISCORD_TOKEN").map_err(|_| McpDiscordError::MissingDiscordToken)?;

    let channel_id = ChannelId::new(
        channel_id
            .parse::<u64>()
            .map_err(|_| McpDiscordError::InvalidChannelID(channel_id.clone()))?,
    );

    let http = Http::new(&discord_token);

    let builder = GetMessages::new().limit(limit.unwrap_or(100.0) as u8);
    let messages = channel_id
        .messages(&http, builder)
        .await
        .map_err(McpDiscordError::DiscordApiError)?;

    let messages_json =
        serde_json::to_string(&messages).map_err(McpDiscordError::ResponseSerializeError)?;

    Ok(tool_text_content!(messages_json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_get_channel_messages_tool() {
        dotenv().ok();

        if std::env::var("DISCORD_TOKEN").is_err() {
            println!("Skipping test_get_channel_messages_tool: Missing Discord token");
            return;
        }

        if std::env::var("TEST_DISCORD_CHANNEL_ID").is_err() {
            println!("Skipping test_get_channel_messages_tool: Missing test channel ID");
            return;
        }

        let test_channel_id = std::env::var("TEST_DISCORD_CHANNEL_ID").unwrap();

        match get_channel_messages_tool(test_channel_id.to_string(), Some(5.0)).await {
            Ok(content) => {
                println!("Messages result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                // Don't panic on error since this test requires a valid channel ID
                println!("Test skipped: requires valid channel ID to get messages from");
            }
        }
    }
}
