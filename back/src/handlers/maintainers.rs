// maintainers.rs
use crate::models::types::PullRequestStats;
use axum::{extract::{Path, Extension}, response::IntoResponse, Json};
use chrono::{DateTime, Duration, Utc};
use reqwest::{self, Client};
use serde_json::{json, Value};
use std::sync::Arc;
use std::collections::HashSet;
use std::error::Error;


// Signs of an active repository
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

pub async fn get_recent_commits(
  client: &Client,
  repository_name: &str,
  username: &str
) -> Result<(), Box<dyn Error>> {
  let url = format!("https://api.github.com/repos/{}/{}/commits", repository_name, username);
  // follow-up: so we can query repos with ...url/repos/<repo_name>/<user>/commits for a specific repository
  // such as the main maintainer? if so, how can we also determine the main maintainer?

  let res = client.get(url).header("User-Agent", "ool").send().await?.json::<Value>().await?;

  let empty_vec = vec![];
  let commits = res.as_array().unwrap_or(&empty_vec);

  for commit in commits.iter().take(5) {
    let date = commit["commit"]["author"]["date"].as_str().unwrap_or("Unknown date");
    let message = commit["commit"]["message"].as_str().unwrap_or("No message");
    println!("Date: {} | Message: {}", date, message);
    println!("--------------------------------------------");
  }

  // we should have some logic determining whether this can be determined as an active..
  Ok(())
}

pub async fn get_recent_commits_handler(
  Path((repository_name, username)): Path<(String, String)>,
  Extension(client): Extension<Arc<Client>>
) -> impl IntoResponse {
  match get_recent_commits(&client, repository_name.as_str(), username.as_str()).await {
    Ok(_) => (axum::http::StatusCode::OK, "Successfully fetched commits.").into_response(),
    Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
  }
}

pub async fn get_freq_of_merged_pull_requests(
  client: &Client,
  repository_name: &str,
  owner: &str,
) -> Result<PullRequestStats, Box<dyn Error>> {
  let url = format!(
    "https://api.github.com/repos/{}/{}/pulls?state=all&per_page=100",
    owner, repository_name
  );

  let res = client.get(&url).header("User-Agent", "ool").send().await?.json::<Value>().await?;

  let empty_vec = vec![];
  let pulls = res.as_array().unwrap_or(&empty_vec);

  let mut total_prs = 0;
  let mut merged_prs = 0;
  let mut recent_prs = 0;
  let mut merge_times = vec![];
  let thirty_days_ago = Utc::now() - Duration::days(30);

  for pr in pulls {
    total_prs += 1;
    let created_at = pr["created_at"].as_str().unwrap_or("");
    let merged_at = pr["merged_at"].as_str().unwrap_or("");

    if let Ok(created_date) = created_at.parse::<DateTime<Utc>>() {
      if created_date > thirty_days_ago {
        recent_prs += 1;
      }
    }

    if !merged_at.is_empty() {
      merged_prs += 1;
      if let (Ok(created_date), Ok(merged_date)) =
        (created_at.parse::<DateTime<Utc>>(), merged_at.parse::<DateTime<Utc>>())
      {
        let merge_time = (merged_date - created_date).num_days();
        merge_times.push(merge_time as f64);
      }
    }
  }

  let merge_rate = if total_prs > 0 { merged_prs as f64 / total_prs as f64 } else { 0.0 };

  let avg_merge_time = if !merge_times.is_empty() {
    merge_times.iter().sum::<f64>() / merge_times.len() as f64
  } else {
    f64::MAX
  };

  let pr_frequency = recent_prs as f64 / 30.0;
  let is_active = merge_rate > 0.6 && pr_frequency > 1.0 && avg_merge_time < 7.0;

  Ok(PullRequestStats {
    repository: repository_name.to_string(),
    merge_rate,
    pr_frequency,
    avg_merge_time,
    is_active,
  })
}

pub async fn get_freq_of_merged_pull_requests_handler(
  Path((repository_name, owner)): Path<(String, String)>,
  Extension(client): Extension<Arc<Client>>
) -> impl IntoResponse {
  match get_freq_of_merged_pull_requests(&client, repository_name.as_str(), owner.as_str()).await {
    Ok(stats) => (
      axum::http::StatusCode::OK,
      Json(json!({
          "repository": stats.repository,
          "merge_rate": stats.merge_rate,
          "pr_frequency": stats.pr_frequency,
          "avg_merge_time": stats.avg_merge_time,
          "is_active": stats.is_active
      })),
    )
      .into_response(),
    Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
  }
}

pub async fn get_maintainers(
  client: &Client,
  repository_name: &str,
  owner: &str
) -> HashSet<String> {
  let url = format!("https://api.github.com/repos/{}/{}/collaborators", owner, repository_name);

  match client.get(&url).header("User-Agent", "ool").send().await {
    Ok(response) => match response.json::<Value>().await {
      Ok(data) => data
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|user| user["login"].as_str().map(|s| s.to_string()))
        .collect(),
      Err(_) => HashSet::new(),
    },
    Err(_) => HashSet::new(),
  }
}

pub async fn get_pr_issue_comments(
  client: &Client,
  repository_name: &str,
  owner: &str,
) -> Vec<(String, String, DateTime<Utc>)> {
  let pr_comments_url = format!(
    "https://api.github.com/repos/{}/{}/pulls/comments?per_page=100",
    owner, repository_name
  );
  let issue_comments_url = format!(
    "https://api.github.com/repos/{}/{}/issues/comments?per_page=100",
    owner, repository_name
  );

  let mut comments = vec![];

  for url in [&pr_comments_url, &issue_comments_url] {
    if let Ok(response) = client.get(url).header("User-Agent", "ool").send().await {
      if let Ok(data) = response.json::<Value>().await {
        if let Some(comment_array) = data.as_array() {
          for comment in comment_array {
            if let (Some(user), Some(body), Some(created_at)) = (
              comment["user"]["login"].as_str(),
              comment["body"].as_str(),
              comment["created_at"].as_str(),
            ) {
              if let Ok(date) = created_at.parse::<DateTime<Utc>>() {
                comments.push((user.to_string(), body.to_string(), date));
              }
            }
          }
        }
      }
    }
  }

  comments
}
