// toprepos.rs
use axum::response::IntoResponse;
use reqwest;
use std::error::Error;

pub async fn get_top_repos() -> Result<(), Box<dyn Error>> {
  let url = "https://api.github.com/search/repositories?q=topic:good-first-issue&sort=stars&order=desc&per_page=10";

  let client = reqwest::Client::new();
  let res = client
      .get(url)
      .header("User-Agent", "ool") // GitHub requires a User-Agent header
      .send()
      .await?
      .json::<serde_json::Value>()
      .await?;

  if let Some(repos) = res.get("items").and_then(|v| v.as_array()) {
      for repo in repos.iter().take(10) {
          let name = repo
              .get("name")
              .and_then(|v| v.as_str())
              .unwrap_or("Unknown repo");

          let owner = repo
              .get("owner")
              .and_then(|v| v.get("login"))
              .and_then(|v| v.as_str())
              .unwrap_or("Unknown owner");

          let stars = repo
              .get("stargazers_count")
              .and_then(|v| v.as_i64())
              .unwrap_or(0);

          let url = repo
              .get("html_url")
              .and_then(|v| v.as_str())
              .unwrap_or("No URL");

          println!("Repo: {} | Owner: {} | Stars: {} | URL: {}", name, owner, stars, url);
          println!("--------------------------------------------");
      }
  } else {
      println!("No repositories found. Please retry.");
  }

  Ok(())
}


pub async fn get_top_repos_handler() -> impl IntoResponse {
  match get_top_repos().await {
    Ok(_) => (axum::http::StatusCode::OK, "Successfully fetched top repos.").into_response(),
    Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
  }
}
