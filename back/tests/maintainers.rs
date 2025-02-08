// tests/maintainers.rs
use back::handlers::maintainers::*;
use reqwest::Client;
use tokio;

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_get_recent_commits_success() {
    let client = Client::new();
    match get_recent_commits(&client, "rust-lang", "rust").await {
      Ok(_) => println!("✅ Successfully fetched commits."),

      Err(err) => println!("😭 Error arose: {}", err),
    }
  }

  #[tokio::test]
  async fn test_get_freq_of_merged_pull_requests_success() {
    let client = Client::new();
    match get_freq_of_merged_pull_requests(&client, "rust-lang", "rust").await {
      Ok(_) => println!("✅ Successfully analyzed repository activity."),
      Err(err) => println!("😭 Error arose: {}", err),
    }
  }
}
