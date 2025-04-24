use super::*;
use mcp_core::protocol::Protocol;
use mcp_core::server::Server;
use mcp_core::types::ServerCapabilities;
use serde_json::json;

pub fn protocol() -> Protocol {
    Server::builder("Arvix".to_string(), "0.1.0".to_string())
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        })
        .register_tool(GetPaperByIdTool::tool(), GetPaperByIdTool::call())
        .register_tool(SearchPapersTool::tool(), SearchPapersTool::call())
        .register_tool(ListRecordsTool::tool(), ListRecordsTool::call())
        .register_tool(SearchByAuthorTool::tool(), SearchByAuthorTool::call())
        .register_tool(ExtractPaperTextTool::tool(), ExtractPaperTextTool::call())
        .build()
}
