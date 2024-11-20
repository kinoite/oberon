use crate::commands::metadata::load_metadata;
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

pub async fn execute(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: install <package>");
        return;
    }

    let package_name = &args[0];
    let metadata_file = "metadata.toml";

    match load_metadata(metadata_file) {
        Ok(metadata) => {
            let package = metadata.packages.values().find(|pkg| pkg.name == *package_name);
            match package {
                Some(pkg) => {
                    println!("Installing '{}'", pkg.name);
                    println!("Version: {}", pkg.version);
                    println!("Dependencies: {:?}", pkg.dependencies);

                    let mut installed_packages = load_installed_packages();

                    for dep in &pkg.dependencies {
                        if !installed_packages.contains_key(dep) {
                            println!("Installing dependency: {}", dep);
                        }
                    }

                    println!("Downloading and installing package '{}'", pkg.name);

                    installed_packages.insert(
                        pkg.name.clone(),
                        InstalledPackage {
                            name: pkg.name.clone(),
                            version: pkg.version.clone(),
                        },
                    );

                    if let Err(err) = save_installed_packages(&installed_packages) {
                        eprintln!("Failed to save installed packages: {}", err);
                    } else {
                        println!("Package '{}' installed successfully.", pkg.name);
                    }
                }
                None => eprintln!("Package '{}' not found in metadata.", package_name),
            }
        }
        Err(err) => eprintln!("Failed to read metadata: {}", err),
    }
}
