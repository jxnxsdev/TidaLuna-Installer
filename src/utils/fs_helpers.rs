use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io;
use std::env;

/// Get the TIDAL resources directory based on OS
pub async fn get_tidal_directory() -> io::Result<PathBuf> {
    let platform = std::env::consts::OS;

    match platform {
        "windows" => {
            if let Some(local_appdata) = env::var_os("LOCALAPPDATA") {
                let tidal_dir = Path::new(&local_appdata).join("TIDAL");
                if tidal_dir.exists() {
                    let mut app_dirs: Vec<String> = Vec::new();
                    let mut entries = fs::read_dir(&tidal_dir).await?;
                    
                    while let Some(entry) = entries.next_entry().await? {
                        let name = entry.file_name();
                        let name_str = name.to_string_lossy();
                        if name_str.starts_with("app-") {
                            app_dirs.push(name_str.to_string());
                        }
                    }

                    app_dirs.sort();
                    if let Some(latest) = app_dirs.pop() {
                        return Ok(tidal_dir.join(latest).join("resources"));
                    }
                }
            }
            Ok(PathBuf::new())
        }
        "macos" => Ok(PathBuf::from("/Applications/TIDAL.app/Contents/Resources")),
        "linux" => {
            let paths = vec![
                "/var/lib/flatpak/app/com.mastermindzh.tidal-hifi/current/active/files/lib/tidal-hifi/resources/",
                "/opt/tidal-hifi/resources/",
            ];

            for p in paths {
                if fs::metadata(p).await.is_ok() {
                    return Ok(PathBuf::from(p));
                }
            }
            Ok(PathBuf::new())
        }
        _ => Ok(PathBuf::new()),
    }
}

/// Check if Luna is installed
pub async fn is_luna_installed() -> io::Result<bool> {
    let tidal_dir = get_tidal_directory().await?;
    if tidal_dir.as_os_str().is_empty() {
        return Ok(false);
    }
    let app_dir = tidal_dir.join("app");
    Ok(app_dir.exists())
}

/// Get the application data path for TidaLunaInstaller
pub fn get_appdata_path() -> PathBuf {
    let platform = std::env::consts::OS;

    match platform {
        "windows" => {
            if let Some(appdata) = env::var_os("APPDATA") {
                Path::new(&appdata).join("TidaLunaInstaller")
            } else {
                PathBuf::new()
            }
        }
        "macos" => {
            dirs::home_dir()
                .unwrap_or_default()
                .join("Library")
                .join("Application Support")
                .join("TidaLunaInstaller")
        }
        "linux" => {
            dirs::home_dir()
                .unwrap_or_default()
                .join(".config")
                .join("TidaLunaInstaller")
        }
        _ => PathBuf::new(),
    }
}
