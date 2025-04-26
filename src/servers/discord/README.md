# Discord Server

**Version:** 0.1.0  
**Total Tools:** 6

<details>
<summary><strong>AddReaction</strong></summary>

**Description:** Add a reaction to a message in a Discord channel.

**Parameters:**
- `discord_channel_id`: ID of the channel containing the message
- `discord_message_id`: ID of the message to react to
- `reaction`: Emoji to add as a reaction (e.g., 👍)

</details>

<details>
<summary><strong>AssignRole</strong></summary>

**Description:** Assign a role to a Discord user.

**Parameters:**
- `guild_id`: The guild (server) ID
- `role_id`: The role ID to assign
- `user_id`: The user ID to assign the role to
- `reason`: The reason for the role assignment

</details>

<details>
<summary><strong>GetChannelMessages</strong></summary>

**Description:** Get messages from a Discord channel.

**Parameters:**
- `channel_id`: The channel ID to get messages from
- `limit`: The maximum number of messages to get

</details>

<details>
<summary><strong>PostDM</strong></summary>

**Description:** Send a direct message to a Discord user.

**Parameters:**
- `content`: The content of the DM
- `discord_user_id`: The user ID to send the DM to

</details>

<details>
<summary><strong>PostMessage</strong></summary>

**Description:** Post a message to Discord.

**Parameters:**
- `content`: Content to send to the channel
- `discord_channel_id`: Discord channel ID to send content to

</details>

<details>
<summary><strong>PostWebhook</strong></summary>

**Description:** Post a message using a Discord webhook.

**Parameters:**
- `discord_webhook_url`: The URL of the webhook
- `content`: Content to send via the webhook

</details> 