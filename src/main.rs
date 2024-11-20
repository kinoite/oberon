mod commands;

use commands::{install, remove, update, sync, addrepo, chooserepo};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: oberon <command> [options]");
        return;
    }
    
    match args[1].as_str() {
        "install" => install::execute(&args[2..]).await,
        "remove" => remove::execute(&args[2..]),
        "update" => update::execute().await,
        "upgrade" => upgrade::execute().await,
        "sync" => sync::execute().await,
        "addrepo" => addrepo::execute(&args[2..]),
        "chooserepo" => chooserepo::execute(&args[2..]),
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}
