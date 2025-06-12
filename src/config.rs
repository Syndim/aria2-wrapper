use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Config for URL replacements
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub replacements: Vec<UrlReplacement>,
    #[serde(default = "default_aria2c_path")]
    pub aria2c_path: String,
}

/// Default path for aria2c executable
fn default_aria2c_path() -> String {
    "aria2c".to_string()
}

/// A single URL replacement rule
#[derive(Debug, Serialize, Deserialize)]
pub struct UrlReplacement {
    pub pattern: String,
    pub replacement: String,
}

impl Config {
    /// Create a new empty config
    pub fn empty() -> Self {
        Config {
            replacements: vec![],
            aria2c_path: default_aria2c_path(),
        }
    }

    /// Load configuration from the specified file
    pub fn from_file(path: &Path) -> Result<Self> {
        let content =
            fs::read_to_string(path).context(format!("Failed to read config file: {:?}", path))?;
        let config: Config = toml::from_str(content.as_str())?;
        Ok(config)
    }
}
