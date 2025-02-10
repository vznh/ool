use crate::handlers::maintainers::{get_freq_of_merged_pull_requests, has_readme_maintenance};
// parse_if_active.rs
// Checks if a repository is active using these guidelines:
/*
 * Frequent commit activity - need mod
 * Responds to issues - need
 * High rate of merged PRs - check
 * Timely PR reviews - need
 * Balance of opened/closed issues - need
 * Comments on issues & prs - need, needs to be author
 * Documentation is maintained - need
 * ... all which indicate there is consistent activity over-time.
 */
use crate::{handlers::maintainers::get_recent_commits, models::types::PiaResponse};
use axum::{
  extract::{Extension, Path},
  response::IntoResponse,
};
use reqwest::Client;
use std::error::Error;
use std::sync::Arc;

// aggregate all checks and possibly try to combine, but do not run at a high volume
// e.g. top picks for you, first_name
pub async fn pia(
  client: Arc<Client>,
  repository_name: &str,
  owner_username: &str,
) -> Result<PiaResponse, Box<dyn Error>> {
  let mut ct = 0; // increment every time, and place into

  match get_recent_commits(&client, repository_name, owner_username).await {
    Ok(response) => {
      if response.trait_ret {
        ct += 1
      }
      println!("Commits were a factor to contribution.");
    }
    Err(e) => {
      eprintln!("There was a thrown error fetching commits: {:?}", e.to_string());
    }
  }

  match get_freq_of_merged_pull_requests(&client, repository_name, owner_username).await {
    Ok(response) => {
      if response.trait_ret {
        ct += 1;
      }
      println!("Frequency of merged PRs were a factor to contribution.");
    }
    Err(e) => {
      eprintln!("There was a thrown error fetching frequency of merged PRs. {}", e.to_string());
    }
  }

  match has_readme_maintenance(&client, repository_name, owner_username).await {
    Ok(response) => {
      if response.trait_ret {
        ct += 1
      }
      println!("The repository containing a README was a factor to contribution.");
    }
    Err(e) => {
      eprintln!("There was a thrown error fetching the maintenance status of the repository's README. Specific error details: {}", e.to_string());
    }
  }

  Ok(PiaResponse { success: true, error: None, is_active: ct >= 8 })
}

pub async fn pia_handler(
  Path((repository_name, username)): Path<(String, String)>,
  Extension(client): Extension<Arc<Client>>,
) -> impl IntoResponse {
  match pia(client, repository_name.as_str(), username.as_str()).await {
    Ok(_) => {
      unimplemented!()
    }
    Err(_) => {
      unimplemented!()
    }
  }
}
