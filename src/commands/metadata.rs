use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub packages: HashMap<String, Package>,
}

pub fn load_metadata(file_path: &str) -> Result<Metadata, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let metadata: Metadata = toml::from_str(&content)?;
    Ok(metadata)
}
