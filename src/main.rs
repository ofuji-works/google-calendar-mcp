use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::ServiceExt;

#[derive(derive_new::new, Clone)]
pub struct UILibrary;

#[rmcp::tool(tool_box)]
impl UILibrary {
    #[rmcp::tool(description = "Get the list of available UI components")]
    pub fn get_component_list(&self) -> String {
        "Button".to_string()
    }

    #[rmcp::tool(description = "Get the source code of a specific UI component")]
    pub fn get_component_source_code(&self) -> String {
        "A clickable button".to_string()
    }
}

#[rmcp::tool(tool_box)]
impl rmcp::ServerHandler for UILibrary {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("ui mcp".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting UI Library MCP...");

    let service = UILibrary::new().serve(rmcp::transport::stdio()).await?;

    service.waiting().await?;

    Ok(())
}
