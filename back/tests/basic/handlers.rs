// basic/handlers.rs
use back::handlers::{toprepos::*, maintainers::*};
use reqwest::Client;

pub mod tests {
  use super::*;

  // toprepos.rs
  pub async fn test_get_top_repos_success() {
    match get_top_repos().await {
      Ok(data) => println!("✅ Successfully fetched top repos: {:?}", data),
      Err(err) => println!("😭 Error arose: {}", err),
    }
  }

  pub async fn test_get_recent_commits_success() {
    let client = Client::new();
    match get_recent_commits(&client, "rust-lang", "rust").await {
      Ok(_) => println!("✅ Successfully fetched commits."),

      Err(err) => println!("😭 Error arose: {}", err),
    }
  }

  pub async fn test_get_freq_of_merged_pull_requests_success() {
    let client = Client::new();
    match get_freq_of_merged_pull_requests(&client, "rust-lang", "rust").await {
      Ok(_) => println!("✅ Successfully analyzed repository activity."),
      Err(err) => println!("😭 Error arose: {}", err),
    }
  }
}
