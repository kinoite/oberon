mod cli;
mod config;
mod metadata;
mod installer;

use clap::Parser;
use cli::{Cli, Commands};
use metadata::Package;
use std::fs;
use std::path::PathBuf;

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    let config_path = "oberon.toml";
    let config = config::Config::load(config_path).unwrap_or_else(|_| {
        config::Config {
            cache_dir: PathBuf::from("~/.oberon/cache"),
            registry_url: String::from("https://registry.oberon.dev"),
            logging: config::Logging {
                level: String::from("info"),
            },
        }
    });

    match &cli.command {
        Commands::Init => {
            let content = r#"
[package]
name = "new-package"
version = "0.1.0"
description = "A new Oberon package"
dependencies = {}
            "#;
            fs::write("package.toml", content).expect("Failed to write package.toml");
            println!("Initialized new Oberon package in package.toml");
        }
        Commands::Install { package } => {
            let cache_dir = &config.cache_dir;
            let install_dir = PathBuf::from("~/.oberon/installed");
            let registry_url = &config.registry_url;
            installer::install_package(package, "1.0.0", registry_url, cache_dir, &install_dir)
                .unwrap_or_else(|e| eprintln!("Failed to install package: {}", e));
        }
        Commands::Remove { package } => {
            let install_dir = PathBuf::from("~/.oberon/installed").join(package);
            if install_dir.exists() {
                fs::remove_dir_all(&install_dir).unwrap_or_else(|e| eprintln!("Failed to remove package: {}", e));
                println!("Removed package: {}", package);
            } else {
                eprintln!("Package not found: {}", package);
            }
        }
        Commands::Update => {
            println!("Update is not yet implemented.");
        }
        Commands::Publish => {
            let metadata = Package::load("package.toml").expect("Failed to load package metadata");
            println!("Publishing package: {} v{}", metadata.name, metadata.version);
        }
    }
}
