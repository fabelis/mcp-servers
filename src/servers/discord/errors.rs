use serde_json::Error as JsonError;
use serenity::all::Error as DiscordError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum McpDiscordError {
    #[error("Failed to serialize response: {0}")]
    ResponseSerializeError(JsonError),
    #[error("Invalid content: {0}")]
    InvalidContent(String),
    #[error("Invalid WebhookURL: {0}")]
    InvalidWebhookURL(DiscordError),
    #[error("Invalid UserID: {0}")]
    InvalidUserID(String),
    #[error("Invalid ChannelID: {0}")]
    InvalidChannelID(String),
    #[error("Invalid MessageID: {0}")]
    InvalidMessageID(String),
    #[error("Discord API error: {0}")]
    DiscordApiError(DiscordError),
    #[error("Invalid RoleID: {0}")]
    InvalidRoleID(String),
    #[error("Invalid GuildID: {0}")]
    InvalidGuildID(String),
    #[error("Missing Discord Token")]
    MissingDiscordToken,
}
