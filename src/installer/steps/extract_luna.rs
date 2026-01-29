use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::fs;
use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

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

        sublog_callback(SubLog { message: format!("Ensuring extract path exists: {:?}", extract_path) });
        
        if !extract_path.exists() {
            if let Err(e) = fs::create_dir_all(&extract_path) {
                return StepResult { success: false, message: format!("Failed to create extract dir: {}", e) };
            }
        }

        sublog_callback(SubLog { message: "Extracting Luna...".into() });
        
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
            let out_path = extract_path.join(file_in_zip.name());

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
