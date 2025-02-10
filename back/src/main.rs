// main.rs
use axum::{response::IntoResponse, routing::get, Router};
use back::routes::parse_if_active::pia_handler;
use reqwest::Client;
use std::net::SocketAddr;
use std::sync::Arc;

async fn github_full_repo_refresher() -> impl IntoResponse {}

pub async fn serve() {
  let client = Arc::new(Client::new());
  let app = Router::new()
    .route("/g", get(github_full_repo_refresher))
    .route("/pia/:repository_name/:owner_username", get(pia_handler))
  ;

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Successfully listening on {}. You can now make requests.", addr);
  axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}

#[tokio::main]
async fn main() {
  serve().await;
}
