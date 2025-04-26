use super::*;
use mcp_core::protocol::Protocol;
use mcp_core::server::Server;
use mcp_core::types::ServerCapabilities;
use serde_json::json;

pub fn protocol() -> Protocol {
    Server::builder("Shopify".to_string(), "0.1.0".to_string())
        .capabilities(ServerCapabilities {
            tools: Some(json!({})),
            ..Default::default()
        })
        .register_tool(CreateOrderTool::tool(), CreateOrderTool::call())
        .register_tool(CreateProductTool::tool(), CreateProductTool::call())
        .register_tool(DeleteOrderTool::tool(), DeleteOrderTool::call())
        .register_tool(DeleteProductTool::tool(), DeleteProductTool::call())
        .register_tool(GetOrderTool::tool(), GetOrderTool::call())
        .register_tool(GetProductTool::tool(), GetProductTool::call())
        .register_tool(GetSalesDataTool::tool(), GetSalesDataTool::call())
        .register_tool(ListCustomersTool::tool(), ListCustomersTool::call())
        .register_tool(ListProductsTool::tool(), ListProductsTool::call())
        .register_tool(UpdateProductTool::tool(), UpdateProductTool::call())
        .register_tool(AddProductImageTool::tool(), AddProductImageTool::call())
        .build()
}
