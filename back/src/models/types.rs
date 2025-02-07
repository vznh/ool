// types.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct PullRequestStats {
  pub repository: String,
  pub merge_rate: f64,
  pub pr_frequency: f64,
  pub avg_merge_time: f64,
  pub is_active: bool,
}
