use std::fs;
use std::path::Path;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    url: String,
}

pub async fn execute() {
    println!("Syncing repositories...");

    let repos = load_repositories();
    if repos.is_empty() {
        eprintln!("No repositories found. Use `addrepo` to add a repository.");
        return;
    }

    for repo in repos {
        println!("Syncing repository: {}", repo.url);
        let metadata_url = format!("{}/metadata.json", repo.url);

        match fetch_metadata(&metadata_url).await {
            Ok(metadata) => {
                let repo_name = repo.url.replace("://", "_").replace("/", "_");
                let metadata_filename = format!("{}_metadata.json", repo_name);
                if let Err(err) = save_metadata(&metadata_filename, &metadata) {
                    eprintln!("Failed to save metadata for {}: {}", repo.url, err);
                } else {
                    println!("Successfully synced repository: {}", repo.url);
                }
            }
            Err(err) => eprintln!("Failed to fetch metadata for {}: {}", repo.url, err),
        }
    }
}

async fn fetch_metadata(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let metadata = response.text().await?;
    Ok(metadata)
}

fn save_metadata(filename: &str, metadata: &str) -> std::io::Result<()> {
    let metadata_file = Path::new(filename);
    fs::write(metadata_file, metadata)
}

fn load_repositories() -> Vec<Repository> {
    fs::read_to_string("repositories.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_default())
        .unwrap_or_default()
}
