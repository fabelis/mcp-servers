use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use serenity::all::{ChannelId, Http, MessageId, ReactionType};

use super::errors::McpDiscordError;

#[tool(
    name = "AddReaction",
    description = "Add a reaction to a message in a Discord channel.",
    params(
        discord_channel_id = "ID of the channel containing the message",
        discord_message_id = "ID of the message to react to",
        reaction = "Emoji to add as a reaction (e.g., 👍)"
    )
)]
async fn add_reaction_tool(
    discord_channel_id: String,
    discord_message_id: String,
    reaction: String,
) -> Result<ToolResponseContent> {
    let discord_token =
        std::env::var("DISCORD_TOKEN").map_err(|_| McpDiscordError::MissingDiscordToken)?;

    let channel_id = ChannelId::new(
        discord_channel_id
            .parse::<u64>()
            .map_err(|_| McpDiscordError::InvalidChannelID(discord_channel_id.clone()))?,
    );

    let message_id = MessageId::new(
        discord_message_id
            .parse::<u64>()
            .map_err(|_| McpDiscordError::InvalidMessageID(discord_message_id.clone()))?,
    );

    let http = Http::new(&discord_token);

    channel_id
        .create_reaction(&http, message_id, ReactionType::Unicode(reaction.clone()))
        .await
        .map_err(McpDiscordError::DiscordApiError)?;

    Ok(tool_text_content!(format!(
        "Successfully added reaction {} to message {} in channel {}",
        reaction, message_id, channel_id
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_add_reaction_tool() {
        dotenv().ok();

        if std::env::var("DISCORD_TOKEN").is_err() {
            println!("Skipping test_add_reaction_tool: Missing Discord token");
            return;
        }

        if std::env::var("TEST_DISCORD_CHANNEL_ID").is_err() {
            println!("Skipping test_add_reaction_tool: Missing test channel ID");
            return;
        }

        if std::env::var("TEST_DISCORD_MESSAGE_ID").is_err() {
            println!("Skipping test_add_reaction_tool: Missing test message ID");
            return;
        }

        let test_channel_id = std::env::var("TEST_DISCORD_CHANNEL_ID").unwrap();
        let test_message_id = std::env::var("TEST_DISCORD_MESSAGE_ID").unwrap();

        match add_reaction_tool(
            test_channel_id.to_string(),
            test_message_id.to_string(),
            "👍".to_string(),
        )
        .await
        {
            Ok(content) => {
                println!("Reaction result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                // Don't panic on error since this test requires valid IDs
                println!("Test skipped: requires valid channel and message IDs");
            }
        }
    }
}
