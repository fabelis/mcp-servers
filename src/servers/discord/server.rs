use super::*;
use mcp_core::protocol::Protocol;
use mcp_core::server::Server;
use mcp_core::types::ServerCapabilities;
use serde_json::json;

pub fn protocol() -> Protocol {
    Server::builder("Discord".to_string(), "0.1.0".to_string())
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        })
        .register_tool(AddReactionTool::tool(), AddReactionTool::call())
        .register_tool(AssignRoleTool::tool(), AssignRoleTool::call())
        .register_tool(
            GetChannelMessagesTool::tool(),
            GetChannelMessagesTool::call(),
        )
        .register_tool(PostDmTool::tool(), PostDmTool::call())
        .register_tool(PostMessageTool::tool(), PostMessageTool::call())
        .register_tool(PostWebhookTool::tool(), PostWebhookTool::call())
        .build()
}
