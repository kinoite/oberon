use std::fs;

pub fn execute(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: addrepo <repository_url>");
        return;
    }

    let repo_url = &args[0];
    let mut repos = load_repositories();

    if repos.contains(repo_url) {
        eprintln!("Repository '{}' is already added.", repo_url);
        return;
    }

    repos.push(repo_url.clone());
    save_repositories(&repos);

    println!("Repository '{}' added successfully.", repo_url);
}

fn load_repositories() -> Vec<String> {
    fs::read_to_string("repositories.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_default())
        .unwrap_or_default()
}

fn save_repositories(repos: &[String]) {
    let data = serde_json::to_string_pretty(repos).unwrap();
    fs::write("repositories.json", data).unwrap();
}
