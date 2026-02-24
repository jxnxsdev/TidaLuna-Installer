use std::path::{Path, PathBuf};
use tokio::io;
use std::env;

fn os_str_eq_ignore_ascii_case(value: Option<&std::ffi::OsStr>, expected: &str) -> bool {
    value
        .map(|v| v.to_string_lossy().eq_ignore_ascii_case(expected))
        .unwrap_or(false)
}

fn first_existing_path(paths: &[PathBuf]) -> Option<PathBuf> {
    paths.iter().find(|p| p.exists()).cloned()
}

fn find_resources_in_app_versions(base_dir: &Path) -> Option<PathBuf> {
    let entries = std::fs::read_dir(base_dir).ok()?;
    let mut app_dirs: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with("app-") {
                return None;
            }

            let resources = entry.path().join("resources");
            let resources_alt = entry.path().join("Resources");
            if is_tidal_resources_directory(&resources) || is_tidal_resources_directory(&resources_alt) {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    app_dirs.sort();

    app_dirs.last().and_then(|latest| {
        let resources = base_dir.join(latest).join("resources");
        if is_tidal_resources_directory(&resources) {
            return Some(resources);
        }

        let resources_alt = base_dir.join(latest).join("Resources");
        if is_tidal_resources_directory(&resources_alt) {
            return Some(resources_alt);
        }

        None
    })
}

pub fn normalize_tidal_resources_path(input: PathBuf) -> PathBuf {
    if input.as_os_str().is_empty() {
        return input;
    }

    if os_str_eq_ignore_ascii_case(input.file_name(), "app.asar")
        || os_str_eq_ignore_ascii_case(input.file_name(), "original.asar")
    {
        if let Some(parent) = input.parent() {
            return parent.to_path_buf();
        }
    }

    if os_str_eq_ignore_ascii_case(input.file_name(), "resources") {
        return input;
    }

    if os_str_eq_ignore_ascii_case(input.file_name(), "contents") {
        return input.join("Resources");
    }

    if input
        .extension()
        .map(|ext| ext.to_string_lossy().eq_ignore_ascii_case("app"))
        .unwrap_or(false)
    {
        return input.join("Contents").join("Resources");
    }

    let candidates = vec![input.join("resources"), input.join("Resources")];
    if let Some(path) = first_existing_path(&candidates) {
        return path;
    }

    if let Some(path) = find_resources_in_app_versions(&input) {
        return path;
    }

    input.join("resources")
}

pub fn is_tidal_resources_directory(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && path.is_dir()
        && (path.join("app.asar").is_file()
            || path.join("original.asar").is_file()
            || path.join("app").is_dir())
}

pub fn has_tidal_app_asar(path: &Path) -> bool {
    !path.as_os_str().is_empty() && path.is_dir() && path.join("app.asar").is_file()
}

/// Get the TIDAL resources directory based on OS
pub async fn get_tidal_directory() -> io::Result<PathBuf> {
    let platform = std::env::consts::OS;

    match platform {
        "windows" => {
            if let Some(local_appdata) = env::var_os("LOCALAPPDATA") {
                let tidal_dir = Path::new(&local_appdata).join("TIDAL");
                if tidal_dir.exists() {
                    if let Some(resources_path) = find_resources_in_app_versions(&tidal_dir) {
                        return Ok(resources_path);
                    }
                }
            }
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "TIDAL resources directory not found on Windows",
            ))
        }
        "macos" => {
            let mut candidates = vec![PathBuf::from("/Applications/TIDAL.app")];
            if let Some(home) = dirs::home_dir() {
                candidates.push(home.join("Applications").join("TIDAL.app"));
                candidates.push(home.join("Applications").join("Tidal.app"));
            }

            for app_bundle in candidates {
                let resources = normalize_tidal_resources_path(app_bundle);
                if is_tidal_resources_directory(&resources) {
                    return Ok(resources);
                }
            }

            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "TIDAL resources directory not found on macOS",
            ))
        }
        "linux" => {
            let mut paths = vec![
                PathBuf::from("/var/lib/flatpak/app/com.mastermindzh.tidal-hifi/current/active/files/lib/tidal-hifi/resources"),
                PathBuf::from("/opt/tidal-hifi/resources"),
                PathBuf::from("/usr/lib/tidal-hifi/resources"),
                PathBuf::from("/usr/share/tidal-hifi/resources"),
            ];

            if let Some(home) = dirs::home_dir() {
                paths.push(
                    home.join(".local")
                        .join("share")
                        .join("flatpak")
                        .join("app")
                        .join("com.mastermindzh.tidal-hifi")
                        .join("current")
                        .join("active")
                        .join("files")
                        .join("lib")
                        .join("tidal-hifi")
                        .join("resources"),
                );
            }

            for p in paths {
                if is_tidal_resources_directory(&p) {
                    return Ok(p);
                }
            }

            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "TIDAL resources directory not found on Linux",
            ))
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Unsupported operating system",
        )),
    }
}

/// Check if Luna is installed
pub async fn is_luna_installed() -> io::Result<bool> {
    let tidal_dir = match get_tidal_directory().await {
        Ok(path) => path,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err),
    };

    let app_dir = tidal_dir.join("app");
    Ok(app_dir.exists())
}

