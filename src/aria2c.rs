use anyhow::{Context, Result};
use std::process::{Command, ExitStatus, Stdio};
use tracing::{debug, info};

use crate::config::Config;

/// Execute aria2c with the given arguments
pub fn execute(args: &[String], config: &Config) -> Result<ExitStatus> {
    info!("Executing aria2c with arguments: {:?}", args);
    debug!("Using aria2c path: {}", config.aria2c_path);

    Command::new(&config.aria2c_path)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context(format!(
            "Failed to execute aria2c at path: {}",
            config.aria2c_path
        ))
}
