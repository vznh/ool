// main.rs
use axum::{
  response::IntoResponse,
  routing::get,
  Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use reqwest::Client;
use back::routes::parse_if_active::pia_handler;
use back::handlers::maintainers::{
  get_freq_of_merged_pull_requests_handler, get_recent_commits_handler,
};
use back::handlers::toprepos::get_top_repos_handler;

async fn github_full_repo_refresher() -> impl IntoResponse {}

pub async fn serve() {
  let client = Arc::new(Client::new());
  let app = Router::new()
    .route("/g", get(github_full_repo_refresher))
    .route("/pia/:repository/:owner", get(pia_handler))
    .route("/m/:repository/:username", get(get_recent_commits_handler))
    .route("/pr/:repository/:owner", get(get_freq_of_merged_pull_requests_handler))
    .route("/top-repos", get(get_top_repos_handler));

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Successfully listening on {}. You can now make requests.", addr);
  axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}


#[tokio::main]
async fn main() {
  serve().await;
}
