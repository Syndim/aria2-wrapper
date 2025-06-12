use crate::config::{Config, is_url};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Process an input file containing URLs, applying replacements
pub fn process_input_file(input_path: &Path, config: &Config) -> Result<PathBuf> {
    // Read the original file
    let content = fs::read_to_string(input_path)
        .context(format!("Failed to read input file: {:?}", input_path))?;

    // Process each line applying URL replacements
    let processed_content: Vec<String> = content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if is_url(trimmed) {
                config.apply_url_replacements(trimmed)
            } else {
                trimmed.to_string()
            }
        })
        .collect();

    // Create a temporary file with the processed content
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join(format!("aria2_wrapper_{}.txt", uuid::Uuid::new_v4()));

    fs::write(&temp_file_path, processed_content.join("\n")).context(format!(
        "Failed to write temporary file: {:?}",
        temp_file_path
    ))?;

    Ok(temp_file_path)
}
