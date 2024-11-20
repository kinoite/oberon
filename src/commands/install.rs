use std::fs;
use std::path::Path;
use tokio::fs as async_fs;

pub async fn execute(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: install <package>");
        return;
    }

    let package_name = &args[0];
    let repo_url = match fs::read_to_string("current_repo.txt") {
        Ok(repo) => repo.trim().to_string(),
        Err(_) => {
            eprintln!("No repository selected. Use `chooserepo` to select a repository.");
            return;
        }
    };

    let package_url = format!("{}/{}.tar.gz", repo_url, package_name);
    println!("Downloading package from: {}", package_url);

    let response = match reqwest::get(&package_url).await {
        Ok(resp) => resp.bytes().await.unwrap(),
        Err(err) => {
            eprintln!("Failed to download package: {}", err);
            return;
        }
    };

    let package_file = format!("{}.tar.gz", package_name);
    if let Err(err) = async_fs::write(&package_file, response).await {
        eprintln!("Failed to save package: {}", err);
        return;
    }

    println!("Extracting package...");
    if let Err(err) = extract_package(&package_file, package_name) {
        eprintln!("Failed to extract package: {}", err);
        return;
    }

    println!("Package '{}' installed successfully.", package_name);
    let _ = async_fs::remove_file(&package_file).await;
}

fn extract_package(file_path: &str, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tar_gz = fs::File::open(file_path)?;
    let decompressed = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(decompressed);
    archive.unpack(output_dir)?;
    Ok(())
}
