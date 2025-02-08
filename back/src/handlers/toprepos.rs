// toprepos.rs
use axum::response::IntoResponse;
use reqwest;
use serde_json::Value;
use std::error::Error;

pub async fn get_top_repos() -> Result<(), Box<dyn Error>> {
  let url = "https://api.github.com/search/repositories?q=topic:good-first-issue&sort=stars&order=desc&per_page=10";

  let client = reqwest::Client::new();
  let res = client
    .get(url)
    .header("User-Agent", "ool") // Github Requires
    .send()
    .await?
    .json::<Value>()
    .await?;

  let empty_vec = vec![];
  let repos = res["items"].as_array().unwrap_or(&empty_vec);

  for repo in repos.iter().take(10) {
    let name = repo["name"].as_str().unwrap_or("Unknown repo");
    let owner = repo["owner"]["login"].as_str().unwrap_or("Unknown owner");
    let stars = repo["stargazers_count"].as_i64().unwrap_or(0);
    let url = repo["html_url"].as_str().unwrap_or("No URL");

    println!("Repo: {} | Owner: {} | Stars: {} | URL: {}", name, owner, stars, url);
    println!("--------------------------------------------");
  }

  Ok(())
}

pub async fn get_top_repos_handler() -> impl IntoResponse {
  match get_top_repos().await {
    Ok(_) => (axum::http::StatusCode::OK, "Successfully fetched top repos.").into_response(),
    Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
  }
}
