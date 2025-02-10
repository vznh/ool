// maintainers.rs
use crate::models::types::GenericResponse;
use chrono::{DateTime, Duration, Utc};
use reqwest::{self, Client};
use serde_json::Value;
use std::collections::HashSet;
use std::error::Error;

// Signs of an active repository
/*
 * Frequent commit activity - done
 * Responds to issues - need
 * High rate of merged PRs - done
 * Timely PR reviews - need
 * Balance of opened/closed issues - need
 * Comments on issues & prs - need, needs to be author
 * Documentation is maintained - needs mod (does it have updates?)
 * ... all which indicate there is consistent activity over-time.
 */

pub async fn get_recent_commits(
  client: &Client,
  repository_name: &str,
  username: &str,
) -> Result<GenericResponse, Box<dyn Error>> {
  let url = format!("https://api.github.com/repos/{}/{}/commits", repository_name, username);

  let res = client.get(&url).header("User-Agent", "ool").send().await?.json::<Value>().await?;

  let empty_vec = vec![];
  let commits = res.as_array().unwrap_or(&empty_vec);

  // Ensure we have enough commits to analyze
  if commits.len() < 2 {
    return Ok(GenericResponse {
      success: false, // Call succeeded, but didn't actually
      error: Some("There was an error processing recent commits.".into()),
      trait_ret: false, // Not enough data to determine activity
    });
  }

  // Parse commit dates and calculate time gaps
  let mut commit_dates: Vec<DateTime<Utc>> = vec![];
  for commit in commits.iter() {
    if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
      if let Ok(date) = date_str.parse::<DateTime<Utc>>() {
        commit_dates.push(date);
      }
    }
  }

  // Sort dates in descending order (most recent first)
  commit_dates.sort_by(|a, b| b.cmp(a));

  let mut active = true;
  for window in commit_dates.windows(2) {
    if let [newer, older] = window {
      let gap = *newer - *older;
      if gap > Duration::days(20) {
        active = false;
        break;
      }
    }
  }

  Ok(GenericResponse {
    success: true,
    error: None,
    trait_ret: active, // true if the repository is active, false otherwise
  })
}

pub async fn get_freq_of_merged_pull_requests(
  client: &Client,
  repository_name: &str,
  owner: &str,
) -> Result<GenericResponse, Box<dyn Error>> {
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

  Ok(GenericResponse { success: true, error: None, trait_ret: is_active })
}

pub async fn get_maintainers(
  client: &Client,
  repository_name: &str,
  owner: &str,
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

pub async fn has_readme_maintenance(
  client: &Client,
  repository_name: &str,
  owner_username: &str,
) -> Result<GenericResponse, Box<dyn Error>> {
  let md_url = format!(
    "https://api.github.com/repos/{}/{}/contents/README.md",
    owner_username, repository_name
  );

  let md_res = client.get(&md_url).header("User-Agent", "ool").send().await;

  if md_res.is_err() {
    return Ok(GenericResponse {
      success: false,
      error: Some("No README.md or README was found in the repository.".into()),
      trait_ret: false,
    });
  }
  unimplemented!();
}
