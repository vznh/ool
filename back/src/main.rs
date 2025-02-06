// main.rs
mod server;

#[tokio::main]
async fn main() {
  server::main().await;
}
