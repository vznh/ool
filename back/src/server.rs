use crate::handlers::maintainers::get_recent_commits_handler;
use crate::handlers::toprepos::get_top_repos_handler;
use axum::{response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;

async fn github_full_repo_refresher() -> impl IntoResponse {}

pub async fn main() {
  let app = Router::new()
    .route("/g", get(github_full_repo_refresher))
    .route("/m/:repository/:username", get(get_recent_commits_handler))
    .route("/top-repos", get(get_top_repos_handler)); // ✅ Added route for top repos

  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  println!("Successfully listening on {}", addr);
  axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();
}
