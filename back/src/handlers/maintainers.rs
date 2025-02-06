// maintainers.rs
use reqwest;
use serde_json::Value;
use std::error::Error;
use tokio;

async fn get_recent_commits(repository_name: &str, username: &str) -> Result<(), Box<dyn Error>> {
  let url = format!("https://api.github.com/repos/{}/{}/commits", repository_name, username);
  // follow-up: so we can query repos with ...url/repos/<repo_name>/<user>/commits for a specific repository
  // such as the main maintainer? if so, how can we also determine the main maintainer?

  let client = reqwest::Client::new();
  let res =
    client.get(url).header("User-Agent", "ool").send().await?.json::<Value>().await?;

  let empty_vec = vec![];
  let commits = res.as_array().unwrap_or(&empty_vec);

  for commit in commits.iter().take(5) {
    let date = commit["commit"]["author"]["date"].as_str().unwrap_or("Unknown date");
    let message = commit["commit"]["message"].as_str().unwrap_or("No message");
    println!("Date: {} | Message: {}", date, message);
  }

  Ok(())
}

#[tokio::test]
async fn test_get_recent_commits_success() {
  match get_recent_commits(
    "rust-lang", "rust"
  ).await {
    Ok(_) => println!("✅ Successfully fetched commits."),
    Err(err) => println!("😭 Error arose: {}", err),
  }
}
