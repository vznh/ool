use reqwest;
use serde_json::Value;
use std::error::Error;
use tokio;

async fn get_recent_commits() -> Result<(), Box<dyn Error>> {
    let url = "https://api.github.com/repos/CarmineOptions/konoha/commits";
    let client = reqwest::Client::new();

    let res = client.get(url)
        .header("User-Agent", "YourAppName") 
        .send()
        .await?
        .json::<Value>()
        .await?;

    let empty_vec = vec![]; // Define an empty vector with a longer lifetime
    let commits = res.as_array().unwrap_or(&empty_vec);
            
    for commit in commits.iter().take(5) {
        let date = commit["commit"]["author"]["date"].as_str().unwrap_or("Unknown date");
        let message = commit["commit"]["message"].as_str().unwrap_or("No message");
        println!("Date: {} | Message: {}", date, message);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    match get_recent_commits().await {
        Ok(_) => println!("✅ Successfully fetched commits."),
        Err(err) => eprintln!("❌ Error: {}", err),
    }
}
