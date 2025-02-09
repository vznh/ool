// types.rs
use serde::Serialize;

#[derive(Serialize)]
pub struct PiaResponse {
  // generic
  pub success: bool,
  pub error: Option<String>,

  // should also include specificities?
  pub is_active: bool,
  // more? can't think of any rn
}

#[derive(Serialize)]
pub struct PullRequestStats {
  pub repository: String,
  pub merge_rate: f64,
  pub pr_frequency: f64,
  pub avg_merge_time: f64,
  pub is_active: bool,
}
