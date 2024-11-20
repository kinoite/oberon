use std::fs;

pub fn execute(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: chooserepo <repository_name>");
        return;
    }

    let repo_name = &args[0];
    let repos = load_repositories();

    if !repos.contains(repo_name) {
        eprintln!("Repository '{}' not found. Add it first using `addrepo`.", repo_name);
        return;
    }

    fs::write("current_repo.txt", repo_name).unwrap();
    println!("Repository '{}' selected successfully.", repo_name);
}

fn load_repositories() -> Vec<String> {
    fs::read_to_string("repositories.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_default())
        .unwrap_or_default()
}
