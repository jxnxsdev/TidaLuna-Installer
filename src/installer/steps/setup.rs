use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;
use crate::installer::step::{InstallStep, StepResult, SubLog};
use crate::utils::fs_helpers::{get_tidal_directory, has_tidal_app_asar};

/// SetupStep: creates temporary directory and checks if Tidal is installed
pub struct SetupStep {
    /// Optional override path (like `options.overwrite_path`)
    pub overwrite_path: Option<PathBuf>,
}

#[async_trait]
impl InstallStep for SetupStep {
    fn name(&self) -> &str {
        "Setup"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let tmp_dir = std::env::temp_dir().join("TidaLunaInstaller");
        sublog_callback(SubLog {
            message: format!("Getting system temporary directory: {:?}", tmp_dir),
        });

        if let Err(err) = fs::create_dir_all(&tmp_dir).await {
            return StepResult {
                success: false,
                message: format!("Failed to create temporary directory: {}", err),
            };
        }
        sublog_callback(SubLog {
            message: format!("Temporary directory created: {:?}", tmp_dir),
        });

        sublog_callback(SubLog {
            message: "Checking if Tidal is installed".into(),
        });

        let tidal_path: PathBuf = match &self.overwrite_path {
            Some(p) => p.clone(),
            None => match get_tidal_directory().await {
                Ok(p) if !p.as_os_str().is_empty() => p,
                _ => {
                    return StepResult {
                        success: false,
                        message: "Tidal is not installed or path could not be found".into(),
                    }
                }
            },
        };

        if !tidal_path.exists() {
            return StepResult {
                success: false,
                message: format!("Tidal path does not exist: {:?}", tidal_path),
            };
        }

        if !has_tidal_app_asar(&tidal_path) {
            return StepResult {
                success: false,
                message: "app.asar not found — Tidal is not installed correctly".into(),
            };
        }

        sublog_callback(SubLog {
            message: "Tidal is installed and valid".into(),
        });

        StepResult {
            success: true,
            message: "Setup step completed successfully".into(),
        }
    }
}
