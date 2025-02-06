// main.rs
mod handlers;
mod server;

#[tokio::main]
async fn main() {
  server::main().await;
}
