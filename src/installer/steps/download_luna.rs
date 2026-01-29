use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct DownloadLunaStep {
    pub download_url: String,
}

#[async_trait]
impl InstallStep for DownloadLunaStep {
    fn name(&self) -> &str {
        "Download Luna"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let temp_dir = std::env::temp_dir().join("TidaLunaInstaller");
        sublog_callback(SubLog {
            message: format!("Using temporary directory: {:?}", temp_dir),
        });

        if let Err(err) = tokio::fs::create_dir_all(&temp_dir).await {
            return StepResult {
                success: false,
                message: format!("Failed to create temporary directory: {}", err),
            };
        }

        let zip_path = temp_dir.join("Luna.zip");
        sublog_callback(SubLog {
            message: "Downloading Luna...".into(),
        });

        let response = match reqwest::get(&self.download_url).await {
            Ok(resp) => resp,
            Err(err) => {
                return StepResult {
                    success: false,
                    message: format!("Failed to send download request: {}", err),
                };
            }
        };

        if !response.status().is_success() {
            return StepResult {
                success: false,
                message: format!("Failed to download Luna, HTTP status: {}", response.status()),
            };
        }

        let bytes = match response.bytes().await {
            Ok(b) => b,
            Err(err) => {
                return StepResult {
                    success: false,
                    message: format!("Failed to read download response: {}", err),
                };
            }
        };

        match File::create(&zip_path).await {
            Ok(mut file) => {
                if let Err(err) = file.write_all(&bytes).await {
                    return StepResult {
                        success: false,
                        message: format!("Failed to write Luna.zip: {}", err),
                    };
                }
            }
            Err(err) => {
                return StepResult {
                    success: false,
                    message: format!("Failed to create Luna.zip: {}", err),
                };
            }
        }

        sublog_callback(SubLog {
            message: format!("Luna downloaded successfully to {:?}", zip_path),
        });

        StepResult {
            success: true,
            message: "Download completed successfully".into(),
        }
    }
}
