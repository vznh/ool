// main.rs
mod handlers;
mod models;
mod server;

#[tokio::main]
async fn main() {
  server::main().await;
}
