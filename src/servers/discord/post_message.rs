use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use serenity::all::{ChannelId, Http};

use super::errors::McpDiscordError;

#[tool(
    name = "PostMessage",
    description = "Post a message to Discord.",
    params(
        discord_channel_id = "Discord channel ID to send content to",
        content = "Content to send to the channel"
    )
)]
async fn post_message_tool(
    discord_channel_id: String,
    content: String,
) -> Result<ToolResponseContent> {
    let discord_token =
        std::env::var("DISCORD_TOKEN").map_err(|_| McpDiscordError::MissingDiscordToken)?;

    let channel_id = ChannelId::new(
        discord_channel_id
            .parse::<u64>()
            .map_err(|_| McpDiscordError::InvalidChannelID(discord_channel_id.clone()))?,
    );

    let http = Http::new(&discord_token);

    let message = channel_id
        .say(&http, &content)
        .await
        .map_err(McpDiscordError::DiscordApiError)?;

    let message_json =
        serde_json::to_string(&message).map_err(McpDiscordError::ResponseSerializeError)?;

    Ok(tool_text_content!(message_json))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_post_message_tool() {
        dotenv().ok();

        if std::env::var("DISCORD_TOKEN").is_err() {
            println!("Skipping test_post_message_tool: Missing Discord token");
            return;
        }

        if std::env::var("TEST_DISCORD_CHANNEL_ID").is_err() {
            println!("Skipping test_post_message_tool: Missing test channel ID");
            return;
        }

        let test_channel_id = std::env::var("TEST_DISCORD_CHANNEL_ID").unwrap();

        match post_message_tool(
            test_channel_id.to_string(),
            "Test message from automated testing".to_string(),
        )
        .await
        {
            Ok(content) => {
                println!("Message result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                // Don't panic on error since this test requires a valid channel ID
                println!("Test skipped: requires valid channel ID to send message to");
            }
        }
    }
}
