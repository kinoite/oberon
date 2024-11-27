use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oberon")]
#[command(version = "1.0")]
#[command(about = "Lightweight, fast, and configurable package manager", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Install {
        package: String,
    },
    Remove {
        package: String,
    },
    Update,
    Publish,
}
