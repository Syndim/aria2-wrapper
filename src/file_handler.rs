use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::Config;
use crate::utils;

/// Process an input file containing URLs, applying replacements
pub fn process_input_file(input_path: &Path, config: &Config) -> Result<()> {
    // Read the original file
    let content = fs::read_to_string(input_path)
        .context(format!("Failed to read input file: {:?}", input_path))?;

    // Process each line applying URL replacements
    let processed_content: Vec<String> = content
        .lines()
        .map(|line| {
            if utils::is_url(line.trim()) {
                utils::apply_url_replacements(config, line)
            } else {
                line.to_string()
            }
        })
        .collect();

    fs::write(input_path, processed_content.join("\n"))
        .context(format!("Failed to write to file: {:?}", input_path))?;

    Ok(())
}
