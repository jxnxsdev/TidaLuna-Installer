use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

/// Step to uninstall TidaLuna / Neptune
pub struct UninstallStep {
    pub overwrite_path: Option<PathBuf>,
}

#[async_trait]
impl InstallStep for UninstallStep {
    fn name(&self) -> &str {
        "Uninstall TidaLuna"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let tidal_path = if let Some(path) = &self.overwrite_path {
            path.clone()
        } else {
            match crate::utils::fs_helpers::get_tidal_directory().await {
                Ok(p) if !p.as_os_str().is_empty() => p,
                Ok(_) => {
                    sublog_callback(SubLog {
                        message: "Tidal path could not be resolved".into(),
                    });
                    return StepResult {
                        success: false,
                        message: "Invalid Tidal path".into(),
                    };
                }
                Err(err) => {
                    sublog_callback(SubLog {
                        message: format!("Tidal is not installed: {}", err),
                    });
                    return StepResult {
                        success: false,
                        message: "Invalid Tidal path".into(),
                    };
                }
            }
        };

        if !tidal_path.exists() {
            sublog_callback(SubLog {
                message: "Tidal is not installed, skipping uninstallation".into(),
            });
            return StepResult {
                success: false,
                message: "Invalid Tidal path".into(),
            };
        }

        sublog_callback(SubLog {
            message: "Uninstalling TidaLuna / Neptune...".into(),
        });

        let luna_dir = tidal_path.join("app");

        if !luna_dir.exists() {
            sublog_callback(SubLog {
                message: "TidaLuna / Neptune is not installed, skipping uninstallation...".into(),
            });
            return StepResult {
                success: true,
                message: "Nothing to uninstall".into(),
            };
        }

        match fs::remove_dir_all(&luna_dir).await {
            Ok(_) => {
                sublog_callback(SubLog {
                    message: "TidaLuna / Neptune uninstalled successfully".into(),
                });
                StepResult {
                    success: true,
                    message: "Uninstallation completed successfully".into(),
                }
            }
            Err(err) => {
                sublog_callback(SubLog {
                    message: format!("Error uninstalling TidaLuna / Neptune: {}", err),
                });
                StepResult {
                    success: false,
                    message: format!("Failed to uninstall: {}", err),
                }
            }
        }
    }
}
