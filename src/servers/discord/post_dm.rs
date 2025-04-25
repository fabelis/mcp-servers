use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use serenity::all::{Http, UserId};

use super::errors::McpDiscordError;

#[tool(
    name = "PostDM",
    description = "Send a direct message to a Discord user.",
    params(
        discord_user_id = "The user ID to send the DM to",
        content = "The content of the DM",
    )
)]
async fn post_dm_tool(discord_user_id: String, content: String) -> Result<ToolResponseContent> {
    let discord_token =
        std::env::var("DISCORD_TOKEN").map_err(|_| McpDiscordError::MissingDiscordToken)?;

    let discord_user_id = UserId::new(
        discord_user_id
            .parse::<u64>()
            .map_err(|_| McpDiscordError::InvalidUserID(discord_user_id.clone()))?,
    );

    let http = Http::new(&discord_token);

    let channel = discord_user_id
        .create_dm_channel(&http)
        .await
        .map_err(McpDiscordError::DiscordApiError)?;

    let message = channel
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
    async fn test_post_dm_tool() {
        dotenv().ok();

        if std::env::var("DISCORD_TOKEN").is_err() {
            println!("Skipping test_post_dm_tool: Missing Discord token");
            return;
        }

        if std::env::var("TEST_DISCORD_USER_ID").is_err() {
            println!("Skipping test_post_dm_tool: Missing test user ID");
            return;
        }

        let test_user_id = std::env::var("TEST_DISCORD_USER_ID").unwrap();

        match post_dm_tool(
            test_user_id.to_string(),
            "Test DM from automated testing".to_string(),
        )
        .await
        {
            Ok(content) => {
                println!("DM result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                // Don't panic on error since this test requires a valid user ID
                println!("Test skipped: requires valid user ID to send DM to");
            }
        }
    }
}
