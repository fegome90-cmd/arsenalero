#![forbid(unsafe_code)]

mod schema;
mod server;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::args().any(|arg| arg == "--version" || arg == "-V") {
        println!("arsenalero {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    server::run_stdio().await
}
