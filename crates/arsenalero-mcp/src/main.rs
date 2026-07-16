#![forbid(unsafe_code)]

mod schema;
mod server;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run_stdio().await
}
