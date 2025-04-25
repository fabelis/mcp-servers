use super::*;
use mcp_core::protocol::Protocol;
use mcp_core::server::Server;
use mcp_core::types::ServerCapabilities;
use serde_json::json;

pub fn protocol() -> Protocol {
    Server::builder("Twitter".to_string(), "0.1.0".to_string())
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        })
        .register_tool(GetMentionsTool::tool(), GetMentionsTool::call())
        .register_tool(GetTimelineTool::tool(), GetTimelineTool::call())
        .register_tool(PostTweetTool::tool(), PostTweetTool::call())
        .register_tool(ReplyToTweetTool::tool(), ReplyToTweetTool::call())
        .register_tool(SearchTweetsTool::tool(), SearchTweetsTool::call())
        .build()
}
