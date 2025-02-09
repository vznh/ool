async fn get_maintainers(client: &Client, repository_name: &str, owner: &str) -> HashSet<String> {
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

async fn get_pr_issue_comments(
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
