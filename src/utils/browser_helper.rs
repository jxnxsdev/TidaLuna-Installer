use anyhow::{anyhow, Result};
use std::process::Command;

/// Opens a URL in the default web browser.
pub fn open_url(url: &str) -> Result<()> {
    let status = match std::env::consts::OS {
        "windows" => {
            Command::new("cmd")
                .args(["/C", "start", "", url])
                .status()
        }
        "macos" => {
            Command::new("open")
                .arg(url)
                .status()
        }
        "linux" => {
            Command::new("xdg-open")
                .arg(url)
                .status()
        }
        other => {
            return Err(anyhow!("Unsupported OS: {}", other));
        }
    }?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to open URL: {}", url))
    }
}