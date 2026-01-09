use std::path::Path;
use crate::utils::fs_helpers::get_appdata_path;
use std::fs;

/// Remove a single file
pub fn remove_file(file_path: &Path) -> std::io::Result<()> {
    if file_path.exists() {
        fs::remove_file(file_path)?;
    }
    Ok(())
}

/// Remove a directory recursively
pub fn remove_dir(dir_path: &Path) -> std::io::Result<()> {
    if dir_path.exists() {
        fs::remove_dir_all(dir_path)?;
    }
    Ok(())
}

/// Create the application data directory if it does not exist
pub fn create_appdata_dir() -> std::io::Result<()> {
    let app_data_path = get_appdata_path();
    if !app_data_path.exists() {
        fs::create_dir_all(&app_data_path)?;
    }
    Ok(())
}
