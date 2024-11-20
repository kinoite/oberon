use std::fs;
use std::path::Path;

pub fn execute(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: remove <package>");
        return;
    }

    let package_name = &args[0];
    let package_dir = format!("./{}", package_name);

    if !Path::new(&package_dir).exists() {
        eprintln!("Package '{}' is not installed.", package_name);
        return;
    }

    if let Err(err) = fs::remove_dir_all(&package_dir) {
        eprintln!("Failed to remove package '{}': {}", package_name, err);
        return;
    }

    println!("Package '{}' removed successfully.", package_name);
}
