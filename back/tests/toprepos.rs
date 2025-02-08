// tests/toprepos.rs
use back::handlers::toprepos::*;
use tokio;

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_get_top_repos_success() {
    match get_top_repos().await {
      Ok(data) => println!("✅ Successfully fetched top repos: {:?}", data),
      Err(err) => println!("😭 Error arose: {}", err),
    }
  }
}
