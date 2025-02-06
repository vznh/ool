// server.rs
use axum::{response::IntoResponse, routing::get, Json, Router};
use serde_json::json;
use std::net::SocketAddr;
// use reqwest;
// use handlers::base::github_full_repo_refresher;

async fn github_full_repo_refresher() -> impl IntoResponse {
  


}
async fn test_func() -> impl IntoResponse {
  Json(json!({ "message": "Rust server is running!" }));
}

pub async fn main() {
  let app = Router::new().route("/g", get(github_full_repo_refresher)).route("/", get(test_func));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Successfully listening on {}", addr);
  axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}
