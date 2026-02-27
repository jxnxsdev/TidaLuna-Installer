use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Component, Path};
use zip::ZipArchive;

const MAX_ZIP_ENTRY_SIZE: u64 = 100 * 1024 * 1024;

fn safe_zip_join(base: &Path, zip_entry_name: &str) -> Option<std::path::PathBuf> {
    let entry_path = Path::new(zip_entry_name);

    let mut safe_relative = std::path::PathBuf::new();
    for component in entry_path.components() {
        match component {
            Component::Normal(segment) => safe_relative.push(segment),
            Component::CurDir => {}
            Component::ParentDir | Component::RootDir | Component::Prefix(_) => return None,
        }
    }

    Some(base.join(safe_relative))
}

pub struct ExtractLunaStep;

#[async_trait]
impl InstallStep for ExtractLunaStep {
    fn name(&self) -> &str {
        "Extract Luna"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let temp_dir = std::env::temp_dir().join("TidaLunaInstaller");
        let zip_path = temp_dir.join("Luna.zip");
        let extract_path = temp_dir.join("LunaExtracted");

        if extract_path.exists() {
            sublog_callback(SubLog { message: format!("Cleaning existing extract path: {:?}", extract_path) });
            if let Err(e) = fs::remove_dir_all(&extract_path) {
                return StepResult { success: false, message: format!("Failed to clean extract dir: {}", e) };
            }
        }

        sublog_callback(SubLog { message: format!("Ensuring extract path exists: {:?}", extract_path) });

        if let Err(e) = fs::create_dir_all(&extract_path) {
            return StepResult { success: false, message: format!("Failed to create extract dir: {}", e) };
        }

        sublog_callback(SubLog { message: "Extracting Luna...".into() });

        let mut raw_bytes = Vec::new();
        match File::open(&zip_path) {
            Ok(mut zip_file) => {
                if let Err(e) = zip_file.read_to_end(&mut raw_bytes) {
                    return StepResult {
                        success: false,
                        message: format!("Failed to read zip bytes before extraction: {}", e),
                    };
                }
            }
            Err(e) => {
                return StepResult {
                    success: false,
                    message: format!("Failed to open zip for validation: {}", e),
                };
            }
        }

        if raw_bytes.len() < 22 {
            return StepResult {
                success: false,
                message: format!("Zip file is unexpectedly small: {} bytes", raw_bytes.len()),
            };
        }

        if let Err(e) = ZipArchive::new(std::io::Cursor::new(&raw_bytes)) {
            return StepResult {
                success: false,
                message: format!(
                    "Failed ZIP validation before extraction: {} (size: {} bytes)",
                    e,
                    raw_bytes.len()
                ),
            };
        }
        
        let file = match File::open(&zip_path) {
            Ok(f) => f,
            Err(e) => return StepResult { success: false, message: format!("Failed to open zip: {}", e) },
        };
        
        let mut archive = match ZipArchive::new(file) {
            Ok(a) => a,
            Err(e) => return StepResult { success: false, message: format!("Failed to read zip archive: {}", e) },
        };

        for i in 0..archive.len() {
            let mut file_in_zip = match archive.by_index(i) {
                Ok(f) => f,
                Err(e) => return StepResult { success: false, message: format!("Failed to access zip entry: {}", e) },
            };

            if file_in_zip.size() > MAX_ZIP_ENTRY_SIZE {
                return StepResult {
                    success: false,
                    message: format!(
                        "Zip entry is too large (>{} bytes): {}",
                        MAX_ZIP_ENTRY_SIZE,
                        file_in_zip.name()
                    ),
                };
            }

            let out_path = match safe_zip_join(&extract_path, file_in_zip.name()) {
                Some(path) => path,
                None => {
                    return StepResult {
                        success: false,
                        message: format!("Unsafe zip entry path rejected: {}", file_in_zip.name()),
                    }
                }
            };

            if file_in_zip.is_dir() {
                if let Err(e) = fs::create_dir_all(&out_path) {
                    return StepResult { success: false, message: format!("Failed to create dir: {}", e) };
                }
            } else {
                if let Some(p) = out_path.parent() {
                    if let Err(e) = fs::create_dir_all(p) {
                        return StepResult { success: false, message: format!("Failed to create parent dir: {}", e) };
                    }
                }
                let mut outfile = match fs::File::create(&out_path) {
                    Ok(f) => f,
                    Err(e) => return StepResult { success: false, message: format!("Failed to create file: {}", e) },
                };
                if let Err(e) = std::io::copy(&mut file_in_zip, &mut outfile) {
                    return StepResult { success: false, message: format!("Failed to write file: {}", e) };
                }
            }
        }

        sublog_callback(SubLog { message: "Luna extracted successfully".into() });
        StepResult { success: true, message: "Extraction completed".into() }
    }
}
