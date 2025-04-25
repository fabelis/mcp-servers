use super::*;
use mcp_core::protocol::Protocol;
use mcp_core::server::Server;
use mcp_core::types::ServerCapabilities;
use serde_json::json;

pub fn protocol() -> Protocol {
    Server::builder("HuggingFace".to_string(), "0.1.0".to_string())
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        })
        .register_tool(SearchModelsTool::tool(), SearchModelsTool::call())
        .register_tool(GetModelInfoTool::tool(), GetModelInfoTool::call())
        .register_tool(
            GetModelSampleImagesTool::tool(),
            GetModelSampleImagesTool::call(),
        )
        .register_tool(GetReadmeTool::tool(), GetReadmeTool::call())
        .register_tool(WhoamiTool::tool(), WhoamiTool::call())
        .build()
}
