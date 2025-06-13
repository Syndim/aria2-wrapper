use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use regex::Regex;
use tracing::warn;

use crate::config::Config;

/// Check if the string is a URL
pub fn is_url(s: &str) -> bool {
    s.starts_with("http://")
        || s.starts_with("https://")
        || s.starts_with("ftp://")
        || s.starts_with("magnet:")
}

/// Apply URL replacement rules to the given URL
pub fn apply_url_replacements(config: &Config, url_str: &str) -> String {
    let mut result = url_str.to_string();

    for rule in &config.replacements {
        match Regex::new(&rule.pattern) {
            Ok(regex) => {
                result = regex.replace_all(&result, &rule.replacement).to_string();
            }
            Err(e) => {
                warn!("Invalid regex pattern '{}': {}", rule.pattern, e);
            }
        }
    }

    result
}

/// Process an input file containing URLs, applying replacements
pub fn replace_urls_in_file(input_path: &Path, config: &Config) -> Result<()> {
    // Read the original file
    let content = fs::read_to_string(input_path)
        .context(format!("Failed to read input file: {:?}", input_path))?;

    // Process each line applying URL replacements
    let processed_content: Vec<String> = content
        .lines()
        .map(|line| {
            if is_url(line.trim()) {
                apply_url_replacements(config, line)
            } else {
                line.to_string()
            }
        })
        .collect();

    fs::write(input_path, processed_content.join("\n"))
        .context(format!("Failed to write to file: {:?}", input_path))?;

    Ok(())
}
