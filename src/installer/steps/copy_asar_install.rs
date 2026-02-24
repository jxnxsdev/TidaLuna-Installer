use crate::installer::step::{InstallStep, StepResult, SubLog};
use crate::utils::fs_helpers::get_tidal_directory;
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs;

pub struct CopyAsarInstallStep {
    pub overwrite_path: Option<PathBuf>,
}

#[async_trait]
impl InstallStep for CopyAsarInstallStep {
    fn name(&self) -> &str {
        "Copy ASAR for Installation"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let tidal_path = if let Some(p) = &self.overwrite_path {
            p.clone()
        } else {
            match get_tidal_directory().await {
                Ok(p) if !p.as_os_str().is_empty() => p,
                _ => {
                    return StepResult {
                        success: false,
                        message: "Tidal is not installed or path could not be found".into(),
                    }
                }
            }
        };

        sublog_callback(SubLog {
            message: format!("Using Tidal path: {:?}", tidal_path),
        });

        if !tidal_path.exists() {
            return StepResult {
                success: false,
                message: "Tidal path does not exist".into(),
            };
        }

        let mut has_asar = false;
        if let Ok(mut entries) = fs::read_dir(&tidal_path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                if let Ok(file_type) = entry.file_type().await {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "asar" {
                                has_asar = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if !has_asar {
            return StepResult {
                success: false,
                message: "No .asar files found — Tidal is not installed correctly".into(),
            };
        }

        let original_asar = tidal_path.join("original.asar");
        let app_asar = tidal_path.join("app.asar");

        if !original_asar.exists() {
            if !app_asar.exists() {
                return StepResult {
                    success: false,
                    message: "app.asar not found. Tidal installation may be corrupt.".into(),
                };
            }
            sublog_callback(SubLog {
                message: "Creating original.asar backup".into(),
            });
            if let Err(e) = fs::copy(&app_asar, &original_asar).await {
                return StepResult {
                    success: false,
                    message: format!("Failed to backup app.asar: {}", e),
                };
            }
        }

        if app_asar.exists() {
            if let Err(e) = fs::remove_file(&app_asar).await {
                return StepResult {
                    success: false,
                    message: format!("Failed to delete existing app.asar: {}", e),
                };
            }
        }

        sublog_callback(SubLog {
            message: "app.asar copied to original.asar successfully".into(),
        });

        StepResult {
            success: true,
            message: "Copy ASAR step completed".into(),
        }
    }
}
