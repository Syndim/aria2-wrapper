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
