use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Option<std::collections::HashMap<String, Dependency>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
    pub version: String,
    pub optional: Option<bool>,
}

impl Package {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let package: Package = toml::from_str(&content)?;
        Ok(package)
    }
}
