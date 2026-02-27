use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::io::Cursor;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;

fn has_zip_signature(bytes: &[u8]) -> bool {
    const PK_LOCAL_FILE_HEADER: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];
    const PK_EMPTY_ARCHIVE: [u8; 4] = [0x50, 0x4B, 0x05, 0x06];
    const PK_SPANNED_ARCHIVE: [u8; 4] = [0x50, 0x4B, 0x07, 0x08];

    bytes.starts_with(&PK_LOCAL_FILE_HEADER)
        || bytes.starts_with(&PK_EMPTY_ARCHIVE)
        || bytes.starts_with(&PK_SPANNED_ARCHIVE)
}

fn has_zip_end_of_central_directory(bytes: &[u8]) -> bool {
    const EOCD_SIGNATURE: [u8; 4] = [0x50, 0x4B, 0x05, 0x06];

    if bytes.len() < 22 {
        return false;
    }

    let search_start = bytes.len().saturating_sub(65_557);
    bytes[search_start..]
        .windows(4)
        .rev()
        .any(|window| window == EOCD_SIGNATURE)
}

fn validate_zip_bytes(bytes: &[u8]) -> Result<(), String> {
    if !has_zip_signature(bytes) {
        return Err("missing ZIP signature".to_string());
    }

    if !has_zip_end_of_central_directory(bytes) {
        return Err("missing ZIP central directory".to_string());
    }

    let cursor = Cursor::new(bytes);
    ZipArchive::new(cursor)
        .map(|_| ())
        .map_err(|err| format!("ZIP parse validation failed: {}", err))
}

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

        let client = match reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
        {
            Ok(client) => client,
            Err(err) => {
                return StepResult {
                    success: false,
                    message: format!("Failed to build HTTP client: {}", err),
                };
            }
        };

        let response = match client.get(&self.download_url).send().await {
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

        if bytes.is_empty() {
            return StepResult {
                success: false,
                message: "Download completed but returned an empty file".into(),
            };
        }

        if let Err(err) = validate_zip_bytes(&bytes) {
            let preview = String::from_utf8_lossy(&bytes[..bytes.len().min(180)])
                .replace('\n', " ")
                .replace('\r', " ");

            return StepResult {
                success: false,
                message: format!(
                    "Downloaded file failed ZIP validation ({}). URL: {}. Response preview: {}",
                    err, self.download_url, preview
                ),
            };
        }

        let part_path = temp_dir.join("Luna.zip.part");

        if part_path.exists() {
            let _ = tokio::fs::remove_file(&part_path).await;
        }

        match File::create(&part_path).await {
            Ok(mut file) => {
                if let Err(err) = file.write_all(&bytes).await {
                    return StepResult {
                        success: false,
                        message: format!("Failed to write Luna.zip.part: {}", err),
                    };
                }

                if let Err(err) = file.flush().await {
                    return StepResult {
                        success: false,
                        message: format!("Failed to flush Luna.zip.part: {}", err),
                    };
                }

                if let Err(err) = file.sync_all().await {
                    return StepResult {
                        success: false,
                        message: format!("Failed to sync Luna.zip.part: {}", err),
                    };
                }
            }
            Err(err) => {
                return StepResult {
                    success: false,
                    message: format!("Failed to create Luna.zip.part: {}", err),
                };
            }
        }

        if let Err(err) = tokio::fs::rename(&part_path, &zip_path).await {
            return StepResult {
                success: false,
                message: format!("Failed to finalize Luna.zip: {}", err),
            };
        }

        let written_size = match tokio::fs::metadata(&zip_path).await {
            Ok(metadata) => metadata.len(),
            Err(err) => {
                return StepResult {
                    success: false,
                    message: format!("Failed to verify Luna.zip metadata: {}", err),
                };
            }
        };

        if written_size != bytes.len() as u64 {
            return StepResult {
                success: false,
                message: format!(
                    "Written Luna.zip size mismatch: wrote {} bytes but expected {}",
                    written_size,
                    bytes.len()
                ),
            };
        }

        sublog_callback(SubLog {
            message: format!(
                "Luna downloaded successfully to {:?} ({} bytes)",
                zip_path,
                written_size
            ),
        });

        StepResult {
            success: true,
            message: "Download completed successfully".into(),
        }
    }
}
