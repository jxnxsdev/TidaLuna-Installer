use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

pub struct ReinstallCleanupStep {
    pub overwrite_path: Option<PathBuf>,
}

#[async_trait]
impl InstallStep for ReinstallCleanupStep {
    fn name(&self) -> &str {
        "Reinstall Cleanup"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let tidal_path = if let Some(path) = &self.overwrite_path {
            path.clone()
        } else {
            match crate::utils::fs_helpers::get_tidal_directory().await {
                Ok(path) => path,
                Err(err) => {
                    sublog_callback(SubLog {
                        message: format!("Skipping reinstall cleanup: could not resolve TIDAL path ({})", err),
                    });
                    return StepResult {
                        success: true,
                        message: "Reinstall cleanup skipped".into(),
                    };
                }
            }
        };

        if !tidal_path.exists() {
            sublog_callback(SubLog {
                message: "Skipping reinstall cleanup: TIDAL path does not exist".into(),
            });
            return StepResult {
                success: true,
                message: "Reinstall cleanup skipped".into(),
            };
        }

        let original_asar = tidal_path.join("original.asar");
        let app_asar = tidal_path.join("app.asar");
        let luna_dir = tidal_path.join("app");

        if original_asar.exists() {
            sublog_callback(SubLog {
                message: "Attempting to restore original app.asar before reinstall".into(),
            });

            if app_asar.exists() {
                if let Err(err) = fs::remove_file(&app_asar).await {
                    sublog_callback(SubLog {
                        message: format!("Warning: failed removing app.asar before restore: {}", err),
                    });
                }
            }

            if let Err(err) = fs::copy(&original_asar, &app_asar).await {
                sublog_callback(SubLog {
                    message: format!("Warning: failed restoring app.asar from original.asar: {}", err),
                });
            }
        } else {
            sublog_callback(SubLog {
                message: "original.asar not found, skipping ASAR restore for reinstall".into(),
            });
        }

        if luna_dir.exists() {
            sublog_callback(SubLog {
                message: "Removing existing TidaLuna app directory before reinstall".into(),
            });
            if let Err(err) = fs::remove_dir_all(&luna_dir).await {
                sublog_callback(SubLog {
                    message: format!("Warning: failed removing existing app directory: {}", err),
                });
            }
        } else {
            sublog_callback(SubLog {
                message: "No existing TidaLuna app directory found; cleanup not needed".into(),
            });
        }

        StepResult {
            success: true,
            message: "Reinstall cleanup completed".into(),
        }
    }
}
