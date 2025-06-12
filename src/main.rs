mod aria2c;
mod config;
mod file_handler;
mod utils;

use std::path::PathBuf;

use anyhow::Result;
use tracing::{error, info, warn};

use crate::aria2c::execute;
use crate::config::Config;
use crate::file_handler::process_input_file;

fn patch_parameters(args: &[String], config: &Config) -> Vec<String> {
    let mut modified_args = Vec::new();
    let mut i = 0;

    while i < args.len() {
        let arg = &args[i];

        if arg == "-i" {
            modified_args.push(arg.clone());

            // Check if there's a value after -i
            if i + 1 < args.len() {
                info!("Processing input file: {}", args[i + 1]);
                i += 1;
                let input_file = PathBuf::from(&args[i]);

                // Process the input file
                if let Err(e) = process_input_file(&input_file, config) {
                    warn!("Failed to process input file: {}", e);
                }

                modified_args.push(arg.clone());
            }
        } else if arg.starts_with("--input-file=") {
            let file_path = arg.trim_start_matches("--input-file=");
            let input_file = PathBuf::from(file_path);

            // Process the input file
            if let Err(e) = process_input_file(&input_file, config) {
                warn!("Failed to process input file: {}", e);
            }

            modified_args.push(arg.clone());
        } else if utils::is_url(arg) {
            let modified_url = utils::apply_url_replacements(config, arg);
            modified_args.push(modified_url);
        } else {
            modified_args.push(arg.clone());
        }

        i += 1;
    }

    modified_args
}

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting aria2-wrapper");

    // Get command line arguments (excluding the program name)
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Set default config path in user's home directory
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let config_path = home_dir
        .join(".config")
        .join("aria2-wrapper")
        .join("config.toml");

    let config = match Config::from_file(&config_path) {
        Ok(config) => config,
        Err(e) => {
            warn!(
                "Config file not found or invalid at {:?}, using empty config: {}",
                config_path, e
            );
            Config::empty()
        }
    };

    let modified_args = patch_parameters(&args, &config);
    let status = execute(&modified_args, &config)?;

    if !status.success() {
        error!("aria2c exited with status: {}", status);
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
