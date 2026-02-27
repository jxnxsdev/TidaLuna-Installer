use semver::Version;
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

fn resolve_resources_dir(base_dir: &Path) -> Option<PathBuf> {
    let resources = base_dir.join("resources");
    if is_tidal_resources_directory(&resources) {
        return Some(resources);
    }

    let resources_alt = base_dir.join("Resources");
    if is_tidal_resources_directory(&resources_alt) {
        return Some(resources_alt);
    }

    None
}

fn parse_app_dir_version(app_dir_name: &str) -> Option<Version> {
    app_dir_name
        .strip_prefix("app-")
        .and_then(|version| Version::parse(version).ok())
}

fn collect_resources_in_app_versions(base_dir: &Path) -> Vec<PathBuf> {
    let entries = match std::fs::read_dir(base_dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut app_dirs: Vec<String> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with("app-") {
                return None;
            }

            resolve_resources_dir(&entry.path()).map(|_| name)
        })
        .collect();

    app_dirs.sort_by(|left, right| {
        let left_ver = parse_app_dir_version(left);
        let right_ver = parse_app_dir_version(right);

        match (left_ver, right_ver) {
            (Some(left_ver), Some(right_ver)) => right_ver.cmp(&left_ver),
            _ => right.cmp(left),
        }
    });

    app_dirs
        .into_iter()
        .filter_map(|dir_name| resolve_resources_dir(&base_dir.join(dir_name)))
        .collect()
}

fn find_resources_in_app_versions(base_dir: &Path) -> Option<PathBuf> {
    collect_resources_in_app_versions(base_dir).into_iter().next()
}

fn dedup_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduped = Vec::new();

    for path in paths {
        if !deduped.iter().any(|existing| existing == &path) {
            deduped.push(path);
        }
    }

    deduped
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
    let candidates = find_tidal_directories().await?;
    if let Some(path) = candidates.first() {
        return Ok(path.clone());
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "TIDAL resources directory not found",
    ))
}

pub async fn find_tidal_directories() -> io::Result<Vec<PathBuf>> {
    let platform = std::env::consts::OS;

    match platform {
        "windows" => {
            let mut matches = Vec::new();
            if let Some(local_appdata) = env::var_os("LOCALAPPDATA") {
                let tidal_dir = Path::new(&local_appdata).join("TIDAL");
                if tidal_dir.exists() {
                    matches.extend(collect_resources_in_app_versions(&tidal_dir));
                }
            }

            let matches = dedup_paths(matches);
            if matches.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "TIDAL resources directory not found on Windows",
                ))
            } else {
                Ok(matches)
            }
        }
        "macos" => {
            let mut candidates = vec![
                PathBuf::from("/Applications/TIDAL.app"),
                PathBuf::from("/Applications/Tidal.app"),
                PathBuf::from("/Applications/tidal.app"),
            ];
            if let Some(home) = dirs::home_dir() {
                candidates.push(home.join("Applications").join("TIDAL.app"));
                candidates.push(home.join("Applications").join("Tidal.app"));
                candidates.push(home.join("Applications").join("tidal.app"));
            }

            let mut matches = Vec::new();
            for app_bundle in candidates {
                let resources = normalize_tidal_resources_path(app_bundle);
                if is_tidal_resources_directory(&resources) {
                    matches.push(resources);
                }
            }

            let matches = dedup_paths(matches);
            if matches.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "TIDAL resources directory not found on macOS",
                ))
            } else {
                Ok(matches)
            }
        }
        "linux" => {
            let mut paths = vec![
                PathBuf::from("/var/lib/flatpak/app/com.mastermindzh.tidal-hifi/current/active/files/lib/tidal-hifi/resources"),
                PathBuf::from("/var/lib/flatpak/app/com.mastermindzh.tidal-hifi/x86_64/stable/active/files/lib/tidal-hifi/resources"),
                PathBuf::from("/var/lib/flatpak/app/com.mastermindzh.tidal-hifi/x86_64/beta/active/files/lib/tidal-hifi/resources"),
                PathBuf::from("/opt/tidal-hifi/resources"),
                PathBuf::from("/opt/TIDAL/resources"),
                PathBuf::from("/usr/lib/tidal-hifi/resources"),
                PathBuf::from("/usr/lib/TIDAL/resources"),
                PathBuf::from("/usr/share/tidal-hifi/resources"),
                PathBuf::from("/usr/share/TIDAL/resources"),
                PathBuf::from("/app/extra/tidal-hifi/resources"),
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
                paths.push(
                    home.join(".local")
                        .join("share")
                        .join("flatpak")
                        .join("app")
                        .join("com.mastermindzh.tidal-hifi")
                        .join("x86_64")
                        .join("stable")
                        .join("active")
                        .join("files")
                        .join("lib")
                        .join("tidal-hifi")
                        .join("resources"),
                );
                paths.push(
                    home.join(".local")
                        .join("share")
                        .join("flatpak")
                        .join("app")
                        .join("com.mastermindzh.tidal-hifi")
                        .join("x86_64")
                        .join("beta")
                        .join("active")
                        .join("files")
                        .join("lib")
                        .join("tidal-hifi")
                        .join("resources"),
                );
            }

            let mut matches = Vec::new();
            for p in paths {
                if is_tidal_resources_directory(&p) {
                    matches.push(p);
                }
            }

            let matches = dedup_paths(matches);
            if matches.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "TIDAL resources directory not found on Linux",
                ))
            } else {
                Ok(matches)
            }
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "Unsupported operating system",
        )),
    }
}

/// Check if Luna is installed
pub async fn is_luna_installed() -> io::Result<bool> {
    let tidal_dirs = match find_tidal_directories().await {
        Ok(paths) => paths,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(false),
        Err(err) => return Err(err),
    };

    Ok(tidal_dirs.iter().any(|path| path.join("app").exists()))
}

