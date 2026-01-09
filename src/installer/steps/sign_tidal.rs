use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::process::Command;

/// Step to sign TIDAL on macOS
pub struct SignTidalStep;

#[async_trait]
impl InstallStep for SignTidalStep {
    fn name(&self) -> &str {
        "Sign TIDAL"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let os = std::env::consts::OS;

        match os {
            "windows" => {
                sublog_callback(SubLog {
                    message: "No need to sign TIDAL on Windows, skipping...".into(),
                });
                StepResult {
                    success: true,
                    message: "Signing skipped on Windows".into(),
                }
            }
            "linux" => {
                sublog_callback(SubLog {
                    message: "No need to sign TIDAL on Linux, skipping...".into(),
                });
                StepResult {
                    success: true,
                    message: "Signing skipped on Linux".into(),
                }
            }
            "macos" => {
                sublog_callback(SubLog {
                    message: "Signing TIDAL on macOS...".into(),
                });

                let output = Command::new("codesign")
                    .args([
                        "--force",
                        "--deep",
                        "--sign",
                        "-",
                        "/Applications/TIDAL.app",
                    ])
                    .output();

                match output {
                    Ok(out) => {
                        if !out.stdout.is_empty() {
                            sublog_callback(SubLog {
                                message: String::from_utf8_lossy(&out.stdout).to_string(),
                            });
                        }

                        if !out.status.success() {
                            if !out.stderr.is_empty() {
                                sublog_callback(SubLog {
                                    message: String::from_utf8_lossy(&out.stderr).to_string(),
                                });
                            }
                            return StepResult {
                                success: false,
                                message: "Error signing TIDAL on macOS".into(),
                            };
                        }

                        sublog_callback(SubLog {
                            message: "TIDAL signed successfully on macOS".into(),
                        });
                        StepResult {
                            success: true,
                            message: "Signing completed successfully".into(),
                        }
                    }
                    Err(err) => StepResult {
                        success: false,
                        message: format!("Failed to execute codesign: {}", err),
                    },
                }
            }
            _ => StepResult {
                success: false,
                message: "Unsupported operating system".into(),
            },
        }
    }
}
