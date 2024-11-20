use crate::commands::metadata::{load_metadata, Package};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct InstalledPackage {
    name: String,
    version: String,
}

fn load_installed_packages() -> HashMap<String, InstalledPackage> {
    match fs::read_to_string("installed_packages.json") {
        Ok(data) => serde_json::from_str(&data).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

fn save_installed_packages(installed_packages: &HashMap<String, InstalledPackage>) -> std::io::Result<()> {
    let data = serde_json::to_string_pretty(installed_packages)?;
    fs::write("installed_packages.json", data)
}

fn find_outdated_packages(
    installed: &HashMap<String, InstalledPackage>,
    available: &HashMap<String, Package>,
) -> Vec<&Package> {
    available
        .values()
        .filter(|pkg| {
            installed.get(&pkg.name).map_or(true, |inst_pkg| inst_pkg.version < pkg.version)
        })
        .collect()
}

pub async fn execute() {
    let metadata_file = "metadata.toml";

    println!("Checking for updates...");

    let available_packages = match load_metadata(metadata_file) {
        Ok(metadata) => metadata.packages,
        Err(err) => {
            eprintln!("Failed to read metadata: {}", err);
            return;
        }
    };

    let mut installed_packages = load_installed_packages();

    let outdated_packages = find_outdated_packages(&installed_packages, &available_packages);

    if outdated_packages.is_empty() {
        println!("All packages are up-to-date!");
        return;
    }

    println!("The following packages have updates available:");
    for pkg in &outdated_packages {
        println!(
            "  {}: Installed {}, Available {}",
            pkg.name,
            installed_packages
                .get(&pkg.name)
                .map(|p| &p.version)
                .unwrap_or(&"None".to_string()),
            pkg.version
        );
    }

    for pkg in outdated_packages {
        println!("Updating package: {}", pkg.name);

        println!("Downloading {} version {}", pkg.name, pkg.version);
        println!("Installing {}...", pkg.name);

        installed_packages.insert(
            pkg.name.clone(),
            InstalledPackage {
                name: pkg.name.clone(),
                version: pkg.version.clone(),
            },
        );

        println!("Package {} updated to version {}", pkg.name, pkg.version);
    }

    if let Err(err) = save_installed_packages(&installed_packages) {
        eprintln!("Failed to save updated installed packages: {}", err);
    } else {
        println!("Installed packages list updated successfully.");
    }
}
