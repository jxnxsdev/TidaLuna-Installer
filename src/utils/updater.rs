use semver::Version;
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const INSTALLER_RELEASES_API: &str = "https://api.github.com/repos/jxnxsdev/TidaLuna-Installer/releases/latest";

pub fn current_installer_version() -> String {
    option_env!("TIDALUNA_INSTALLER_VERSION")
        .unwrap_or(env!("CARGO_PKG_VERSION"))
        .to_string()
}

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub version: String,
    pub download_url: String,
}

#[derive(Debug, Clone)]
pub struct UpdateApplyResult {
    pub message: String,
    pub should_exit: bool,
}

#[derive(Debug, Deserialize)]
struct GitHubReleaseAsset {
    name: String,
    browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubLatestRelease {
    tag_name: String,
    assets: Vec<GitHubReleaseAsset>,
}

fn parse_version(value: &str) -> Option<Version> {
    Version::parse(value.trim_start_matches('v')).ok()
}

fn is_linux_appimage_path(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.eq_ignore_ascii_case("appimage"))
        .unwrap_or(false)
}

fn pick_update_asset(assets: &[GitHubReleaseAsset]) -> Option<String> {
    match std::env::consts::OS {
        "windows" => assets
            .iter()
            .find(|asset| {
                asset.name.starts_with("installer-windows-x86_64-v") && asset.name.ends_with(".exe")
            })
            .map(|asset| asset.browser_download_url.clone()),
        "macos" => {
            let prefix = if std::env::consts::ARCH == "aarch64" {
                "installer-macOS-aarch64-v"
            } else {
                "installer-macOS-x86_64-v"
            };

            assets
                .iter()
                .find(|asset| asset.name.starts_with(prefix) && !asset.name.ends_with(".app.zip"))
                .map(|asset| asset.browser_download_url.clone())
        }
        "linux" => {
            let exe = std::env::current_exe().ok();
            let appimage_mode = exe
                .as_ref()
                .map(|path| is_linux_appimage_path(path))
                .unwrap_or(false);

            if appimage_mode {
                assets
                    .iter()
                    .find(|asset| {
                        asset.name.starts_with("installer-linux-x86_64-v")
                            && asset.name.ends_with(".AppImage")
                    })
                    .map(|asset| asset.browser_download_url.clone())
            } else {
                assets
                    .iter()
                    .find(|asset| {
                        asset.name.starts_with("installer-linux-x86_64-v")
                            && !asset.name.ends_with(".deb")
                            && !asset.name.ends_with(".AppImage")
                    })
                    .map(|asset| asset.browser_download_url.clone())
            }
        }
        _ => None,
    }
}

fn make_temp_update_path(target: &std::path::Path) -> PathBuf {
    let file_name = target
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or("tidaluna-installer");
    target.with_file_name(format!("{}.update", file_name))
}

#[cfg(not(target_os = "windows"))]
fn replace_binary_unix(target: &std::path::Path, bytes: &[u8]) -> Result<(), String> {
    let tmp_path = make_temp_update_path(target);
    fs::write(&tmp_path, bytes).map_err(|error| format!("failed to write update file: {}", error))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(target)
            .map(|metadata| metadata.permissions())
            .unwrap_or_else(|_| fs::Permissions::from_mode(0o755));
        permissions.set_mode(0o755);
        fs::set_permissions(&tmp_path, permissions)
            .map_err(|error| format!("failed to set executable permissions: {}", error))?;
    }

    fs::rename(&tmp_path, target)
        .map_err(|error| format!("failed to replace executable: {}", error))?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn replace_binary_windows(
    target: &std::path::Path,
    bytes: &[u8],
    relaunch_after: bool,
) -> Result<UpdateApplyResult, String> {
    let update_path = make_temp_update_path(target);
    fs::write(&update_path, bytes).map_err(|error| format!("failed to write update file: {}", error))?;

    let script_path = std::env::temp_dir().join("tidaluna-self-update.cmd");
    let mut script = fs::File::create(&script_path)
        .map_err(|error| format!("failed to create updater script: {}", error))?;

    let target_str = target.to_string_lossy();
    let update_str = update_path.to_string_lossy();

    let relaunch_command = if relaunch_after {
        format!("start \"\" \"{}\"\n", target_str)
    } else {
        String::new()
    };

    let script_contents = format!(
        "@echo off\nsetlocal\n:retry\nmove /Y \"{update}\" \"{target}\" >nul 2>nul\nif errorlevel 1 (\n  timeout /t 1 /nobreak >nul\n  goto retry\n)\n{relaunch}",
        update = update_str,
        target = target_str,
        relaunch = relaunch_command,
    );

    script
        .write_all(script_contents.as_bytes())
        .map_err(|error| format!("failed to write updater script: {}", error))?;

    std::process::Command::new("cmd")
        .args(["/C", script_path.to_string_lossy().as_ref()])
        .spawn()
        .map_err(|error| format!("failed to start updater script: {}", error))?;

    Ok(UpdateApplyResult {
        message: if relaunch_after {
            "Update downloaded. The installer will now close and relaunch updated.".to_string()
        } else {
            "Update downloaded. Exit and reopen the installer to use the new version.".to_string()
        },
        should_exit: true,
    })
}

pub async fn check_for_update(current_version: &str) -> Result<Option<UpdateInfo>, String> {
    let current = parse_version(current_version)
        .ok_or_else(|| format!("invalid current installer version: {}", current_version))?;

    let client = reqwest::Client::builder()
        .user_agent("tidaluna-installer")
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .map_err(|error| format!("failed to create updater http client: {}", error))?;

    let response = client
        .get(INSTALLER_RELEASES_API)
        .send()
        .await
        .map_err(|error| format!("failed to check latest installer release: {}", error))?;

    if !response.status().is_success() {
        return Err(format!("update check request returned {}", response.status()));
    }

    let release: GitHubLatestRelease = response
        .json()
        .await
        .map_err(|error| format!("failed to parse latest release response: {}", error))?;

    let latest = match parse_version(&release.tag_name) {
        Some(version) => version,
        None => return Ok(None),
    };

    if latest <= current {
        return Ok(None);
    }

    let Some(download_url) = pick_update_asset(&release.assets) else {
        return Ok(None);
    };

    Ok(Some(UpdateInfo {
        version: latest.to_string(),
        download_url,
    }))
}

pub async fn apply_update(download_url: &str, relaunch_after: bool) -> Result<UpdateApplyResult, String> {
    let current_exe = std::env::current_exe()
        .map_err(|error| format!("failed to resolve current executable path: {}", error))?;

    let client = reqwest::Client::builder()
        .user_agent("tidaluna-installer")
        .timeout(std::time::Duration::from_secs(90))
        .build()
        .map_err(|error| format!("failed to create updater http client: {}", error))?;

    let response = client
        .get(download_url)
        .send()
        .await
        .map_err(|error| format!("failed to download installer update: {}", error))?;

    if !response.status().is_success() {
        return Err(format!("installer update download returned {}", response.status()));
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|error| format!("failed reading installer update bytes: {}", error))?;

    if bytes.is_empty() {
        return Err("installer update download is empty".to_string());
    }

    #[cfg(target_os = "windows")]
    {
        replace_binary_windows(&current_exe, &bytes, relaunch_after)
    }

    #[cfg(not(target_os = "windows"))]
    {
        let _ = relaunch_after;
        replace_binary_unix(&current_exe, &bytes)?;
        Ok(UpdateApplyResult {
            message: "Installer updated successfully. Please restart the installer.".to_string(),
            should_exit: false,
        })
    }
}
