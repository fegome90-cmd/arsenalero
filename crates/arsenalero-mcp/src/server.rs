use rmcp::{
    ServerHandler, ServiceExt,
    model::{Implementation, ServerCapabilities, ServerInfo},
    transport::stdio,
};

/// Minimal MCP server used only to establish the bootstrap protocol boundary.
#[derive(Clone, Debug, Default)]
pub struct ArsenaleroServer;

impl ServerHandler for ArsenaleroServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::new(ServerCapabilities::builder().enable_tools().build());
        info.server_info = Implementation::new("arsenalero", env!("CARGO_PKG_VERSION"));
        info.instructions = Some(
            "Bootstrap scaffold. No domain tools, resources, prompts, sampling, or roots are available."
                .to_owned(),
        );
        info
    }
}

/// Serves MCP over standard input/output until the transport closes.
pub async fn run_stdio() -> Result<(), Box<dyn std::error::Error>> {
    let service = ArsenaleroServer.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
