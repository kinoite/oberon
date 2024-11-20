use std::fs;
use std::path::Path;
use reqwest::Error;

pub async fn execute() {
    println!("Updating package database...");

    let repo_url = match fs::read_to_string("current_repo.txt") {
        Ok(repo) => repo.trim().to_string(),
        Err(_) => {
            eprintln!("No repository selected. Use `chooserepo` to select a repository.");
            return;
        }
    };

    let metadata_url = format!("{}/metadata.json", repo_url);
    println!("Fetching metadata from: {}", metadata_url);

    match fetch_metadata(&metadata_url).await {
        Ok(metadata) => {
            if let Err(err) = save_metadata(&metadata) {
                eprintln!("Failed to save metadata: {}", err);
            } else {
                println!("Package database updated successfully.");
            }
        }
        Err(err) => eprintln!("Failed to fetch metadata: {}", err),
    }
}

async fn fetch_metadata(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let metadata = response.text().await?;
    Ok(metadata)
}

fn save_metadata(metadata: &str) -> std::io::Result<()> {
    let metadata_file = Path::new("metadata.json");
    fs::write(metadata_file, metadata)
}
