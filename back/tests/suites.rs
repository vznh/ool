// tests/suites.rs
mod basic;
use basic::handlers::tests::{
  test_get_freq_of_merged_pull_requests_success,
  test_get_recent_commits_success,
  test_get_top_repos_success
};

async fn test_all_handlers() {
  println!("func @get_top_repos_success... start");
  test_get_top_repos_success().await;
  println!("func @get_top_repos_success... successful");

  println!("func @get_recent_commits_success()... start");
  test_get_recent_commits_success().await;
  println!("func @get_recent_commits_success()... successful");

  println!("func @get_freq_of_merged_pull_requests_success()... start");
  test_get_freq_of_merged_pull_requests_success().await;
  println!("func @get_freq_of_merged_pull_requests_success()... successful");
}

#[tokio::test]
async fn main() {
  // handlers/
  println!("* Now beginning handlers test suite...");
  test_all_handlers().await;
  println!("* Successfully ran handlers test suite.");

}
