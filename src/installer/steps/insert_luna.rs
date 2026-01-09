use async_trait::async_trait;
use tokio::fs;
use std::path::PathBuf;
use std::collections::VecDeque;

use crate::installer::step::{InstallStep, StepResult, SubLog};
use crate::utils::fs_helpers::get_tidal_directory;

/// Inserts extracted Luna files into the Tidal app directory
pub struct InsertLunaStep {
    pub overwrite_path: Option<PathBuf>,
}

#[async_trait]
impl InstallStep for InsertLunaStep {
    fn name(&self) -> &str {
        "Insert Luna"
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

        if !tidal_path.exists() {
            return StepResult {
                success: false,
                message: "Tidal path does not exist".into(),
            };
        }

        let temp_dir = std::env::temp_dir().join("TidaLunaInstaller");
        let temp_luna_dir = temp_dir.join("LunaExtracted");
        let destination_path = tidal_path.join("app");

        sublog_callback(SubLog {
            message: format!("Using temp directory: {:?}", temp_luna_dir),
        });

        if !temp_luna_dir.exists() {
            return StepResult {
                success: false,
                message: "Temporary Luna directory does not exist".into(),
            };
        }

        sublog_callback(SubLog {
            message: "Copying Luna files into Tidal app directory".into(),
        });

        if let Err(err) = fs::create_dir_all(&destination_path).await {
            return StepResult {
                success: false,
                message: format!("Failed to create destination directory: {}", err),
            };
        }

        if let Err(err) = copy_dir_recursive(&temp_luna_dir, &destination_path).await {
            return StepResult {
                success: false,
                message: format!("Failed to copy Luna files: {}", err),
            };
        }

        sublog_callback(SubLog {
            message: "Luna files copied successfully".into(),
        });

        sublog_callback(SubLog {
            message: "Cleaning up temporary files".into(),
        });

        if let Err(err) = fs::remove_dir_all(&temp_dir).await {
            return StepResult {
                success: false,
                message: format!("Failed to clean up temporary files: {}", err),
            };
        }

        sublog_callback(SubLog {
            message: "Temporary files cleaned up successfully".into(),
        });

        StepResult {
            success: true,
            message: "Insert Luna step completed successfully".into(),
        }
    }
}

pub async fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    let mut queue = VecDeque::new();
    queue.push_back((src.clone(), dst.clone()));

    while let Some((current_src, current_dst)) = queue.pop_front() {
        fs::create_dir_all(&current_dst).await?;

        let mut entries = fs::read_dir(&current_src).await?;
        while let Some(entry) = entries.next_entry().await? {
            let file_type = entry.file_type().await?;
            let src_path = entry.path();
            let dst_path = current_dst.join(entry.file_name());

            if file_type.is_dir() {
                queue.push_back((src_path, dst_path));
            } else {
                fs::copy(&src_path, &dst_path).await?;
            }
        }
    }

    Ok(())
}
