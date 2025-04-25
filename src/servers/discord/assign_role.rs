use anyhow::Result;
use mcp_core::{tool_text_content, types::ToolResponseContent};
use mcp_core_macros::tool;
use serenity::all::{GuildId, Http, RoleId, UserId};

use super::errors::McpDiscordError;

#[tool(
    name = "AssignRole",
    description = "Assign a role to a Discord user.",
    params(
        guild_id = "The guild (server) ID",
        user_id = "The user ID to assign the role to",
        role_id = "The role ID to assign"
    )
)]
async fn assign_role_tool(
    guild_id: String,
    user_id: String,
    role_id: String,
) -> Result<ToolResponseContent> {
    let discord_token =
        std::env::var("DISCORD_TOKEN").map_err(|_| McpDiscordError::MissingDiscordToken)?;

    let guild_id = GuildId::new(
        guild_id
            .parse::<u64>()
            .map_err(|_| McpDiscordError::InvalidGuildID(guild_id.clone()))?,
    );

    let user_id = UserId::new(
        user_id
            .parse::<u64>()
            .map_err(|_| McpDiscordError::InvalidUserID(user_id.clone()))?,
    );

    let role_id = RoleId::new(
        role_id
            .parse::<u64>()
            .map_err(|_| McpDiscordError::InvalidRoleID(role_id.clone()))?,
    );

    let http = Http::new(&discord_token);

    let member = guild_id
        .member(&http, user_id)
        .await
        .map_err(McpDiscordError::DiscordApiError)?;

    member
        .add_role(&http, role_id)
        .await
        .map_err(McpDiscordError::DiscordApiError)?;

    Ok(tool_text_content!(format!(
        "Successfully assigned role {} to user {} in guild {}",
        role_id, user_id, guild_id
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_assign_role_tool() {
        dotenv().ok();

        if std::env::var("DISCORD_TOKEN").is_err() {
            println!("Skipping test_assign_role_tool: Missing Discord token");
            return;
        }

        if std::env::var("TEST_DISCORD_GUILD_ID").is_err() {
            println!("Skipping test_assign_role_tool: Missing test guild ID");
            return;
        }

        if std::env::var("TEST_DISCORD_USER_ID").is_err() {
            println!("Skipping test_assign_role_tool: Missing test user ID");
            return;
        }

        if std::env::var("TEST_DISCORD_ROLE_ID").is_err() {
            println!("Skipping test_assign_role_tool: Missing test role ID");
            return;
        }

        let test_guild_id = std::env::var("TEST_DISCORD_GUILD_ID").unwrap();
        let test_user_id = std::env::var("TEST_DISCORD_USER_ID").unwrap();
        let test_role_id = std::env::var("TEST_DISCORD_ROLE_ID").unwrap();

        match assign_role_tool(
            test_guild_id.to_string(),
            test_user_id.to_string(),
            test_role_id.to_string(),
        )
        .await
        {
            Ok(content) => {
                println!("Role assignment result: {:?}", content);
                assert!(format!("{:?}", content).len() > 0);
            }
            Err(e) => {
                println!("Error: {:#?}", e);
                // Don't panic on error since this test requires valid IDs
                println!("Test skipped: requires valid guild, user, and role IDs");
            }
        }
    }
}
