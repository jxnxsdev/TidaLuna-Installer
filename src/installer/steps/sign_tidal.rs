use crate::installer::step::{InstallStep, StepResult, SubLog};
use crate::utils::fs_helpers::get_tidal_directory;
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

                let sign_target = match get_tidal_directory().await {
                    Ok(resources_path) => resources_path
                        .parent()
                        .and_then(|contents| contents.parent())
                        .map(|app_bundle| app_bundle.to_path_buf())
                        .unwrap_or_else(|| std::path::PathBuf::from("/Applications/TIDAL.app")),
                    Err(_) => std::path::PathBuf::from("/Applications/TIDAL.app"),
                };

                sublog_callback(SubLog {
                    message: format!("Using codesign target: {:?}", sign_target),
                });

                let output = Command::new("codesign")
                    .args([
                        "--force",
                        "--deep",
                        "--sign",
                        "-",
                        sign_target.to_string_lossy().as_ref(),
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
