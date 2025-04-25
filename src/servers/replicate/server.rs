use super::*;
use mcp_core::protocol::Protocol;
use mcp_core::server::Server;
use mcp_core::types::ServerCapabilities;
use serde_json::json;

pub fn protocol() -> Protocol {
    Server::builder("Replicate".to_string(), "0.1.0".to_string())
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        })
        .register_tool(ListModelsTool::tool(), ListModelsTool::call())
        .register_tool(GenerateImageTool::tool(), GenerateImageTool::call())
        .register_tool(EditImageTool::tool(), EditImageTool::call())
        .register_tool(EditImageWithMaskTool::tool(), EditImageWithMaskTool::call())
        .register_tool(GetPredictionTool::tool(), GetPredictionTool::call())
        .register_tool(GetModelInfoTool::tool(), GetModelInfoTool::call())
        .register_tool(WhoamiTool::tool(), WhoamiTool::call())
        .build()
}
