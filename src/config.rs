use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
}

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    #[allow(dead_code)] // Will be used for project metadata in future
    pub author: Option<String>,
}

pub fn load_config(path: &Path) -> Result<Config, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    toml::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))
}
