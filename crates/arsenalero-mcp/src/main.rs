#![forbid(unsafe_code)]

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run_stdio().await
}
