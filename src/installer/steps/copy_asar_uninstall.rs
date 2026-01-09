use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

/// Step to restore original ASAR during uninstallation
pub struct CopyAsarUninstallStep {
    pub overwrite_path: Option<PathBuf>,
}

#[async_trait]
impl InstallStep for CopyAsarUninstallStep {
    fn name(&self) -> &str {
        "Restore original ASAR"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let tidal_path = if let Some(path) = &self.overwrite_path {
            path.clone()
        } else {
            match crate::utils::fs_helpers::get_tidal_directory().await {
                Ok(p) => p,
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

        let original_asar = tidal_path.join("original.asar");
        let app_asar = tidal_path.join("app.asar");

        if !original_asar.exists() {
            sublog_callback(SubLog {
                message: "original.asar not found, cannot restore app.asar".into(),
            });
            return StepResult {
                success: false,
                message: "original.asar missing".into(),
            };
        }

        sublog_callback(SubLog {
            message: "Restoring original app.asar...".into(),
        });

        if app_asar.exists() {
            if let Err(err) = fs::remove_file(&app_asar).await {
                sublog_callback(SubLog {
                    message: format!("Failed to remove existing app.asar: {}", err),
                });
                return StepResult {
                    success: false,
                    message: format!("Failed to remove app.asar: {}", err),
                };
            }
        }

        if let Err(err) = fs::copy(&original_asar, &app_asar).await {
            sublog_callback(SubLog {
                message: format!("Failed to restore app.asar: {}", err),
            });
            return StepResult {
                success: false,
                message: format!("Failed to restore app.asar: {}", err),
            };
        }

        sublog_callback(SubLog {
            message: "original.asar restored to app.asar successfully".into(),
        });

        StepResult {
            success: true,
            message: "ASAR restored successfully".into(),
        }
    }
}
