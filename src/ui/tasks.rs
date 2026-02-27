use iced::widget::image;
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::sync::Semaphore;
use tokio::runtime::Runtime;

use crate::installer::{
    manager::InstallManager,
    steps::{
        copy_asar_install::CopyAsarInstallStep, copy_asar_uninstall::CopyAsarUninstallStep,
        download_luna::DownloadLunaStep, extract_luna::ExtractLunaStep,
        insert_luna::InsertLunaStep, kill_tidal::KillTidalStep, launch_tidal::LaunchTidalStep,
        reinstall_cleanup::ReinstallCleanupStep, setup::SetupStep, sign_tidal::SignTidalStep,
        uninstall::UninstallStep,
    },
};
use crate::utils::{
    fs_helpers::{find_tidal_directories, is_luna_installed, normalize_tidal_resources_path},
    release_loader::ReleaseLoader,
    updater,
};

use super::models::{
    AppRelease, AppVersionInfo, InstallExecutionLog, InstallExecutionResult,
    InstallerUpdateApplyResult, InstallerUpdateInfo, Stargazer,
};

pub async fn check_installer_update_async(
    runtime: Arc<Runtime>,
    current_version: String,
) -> Result<Option<InstallerUpdateInfo>, String> {
    let result = runtime
        .spawn(async move { updater::check_for_update(&current_version).await })
        .await;

    match result {
        Ok(inner) => inner,
        Err(_) => Err("Failed to check for installer updates: task cancelled".to_string()),
    }
}

pub async fn apply_installer_update_async(
    runtime: Arc<Runtime>,
    download_url: String,
) -> Result<InstallerUpdateApplyResult, String> {
    let result = runtime
        .spawn(async move { updater::apply_update(&download_url, true).await })
        .await;

    match result {
        Ok(inner) => inner,
        Err(_) => Err("Failed to apply installer update: task cancelled".to_string()),
    }
}

#[derive(Debug, Deserialize)]
struct GitHubStargazer {
    login: String,
    html_url: String,
    avatar_url: String,
}

fn sanitize_login(login: &str) -> Option<String> {
    let value = login.trim();

    if value.is_empty() || value.len() > 39 {
        return None;
    }

    if value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '-')
    {
        Some(value.to_string())
    } else {
        None
    }
}

fn sanitize_profile_url(url: &str, fallback_login: &str) -> String {
    let fallback = format!("https://github.com/{}", fallback_login);
    let value = url.trim();

    if !value.starts_with("https://github.com/") {
        return fallback;
    }

    if value.chars().any(|character| character.is_control()) {
        return fallback;
    }

    value.to_string()
}

pub async fn load_releases_async(runtime: Arc<Runtime>) -> Result<Vec<AppRelease>, String> {
    let result = runtime.spawn(async move {
        let mut loader = ReleaseLoader::new(
            "https://raw.githubusercontent.com/jxnxsdev/TidaLuna-Installer/main/resources/sources.json",
        );

        match loader.load_releases().await {
            Ok(releases) => {
                let app_releases = releases
                    .into_iter()
                    .map(|release| AppRelease {
                        name: release.name.clone(),
                        versions: release
                            .versions
                            .iter()
                            .map(|version| AppVersionInfo {
                                version: version.version.clone(),
                                download: version.download.clone(),
                            })
                            .collect(),
                    })
                    .collect();
                Ok(app_releases)
            }
            Err(e) => Err(format!("Failed to load releases: {}", e)),
        }
    }).await;

    match result {
        Ok(inner_result) => inner_result,
        Err(_) => Err("Failed to load releases: task cancelled".to_string()),
    }
}

pub async fn load_stargazers_async(runtime: Arc<Runtime>) -> Result<Vec<Stargazer>, String> {
    let result = runtime
        .spawn(async move {
            let client = reqwest::Client::builder()
                .user_agent("tidaluna-installer")
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .map_err(|error| format!("failed to create stargazer http client: {}", error))?;

            let mut page = 1;
            let mut users = Vec::<GitHubStargazer>::new();

            loop {
                let response = client
                    .get("https://api.github.com/repos/jxnxsdev/TidaLuna-Installer/stargazers")
                    .query(&[("per_page", "100"), ("page", &page.to_string())])
                    .send()
                    .await
                    .map_err(|error| format!("stargazers request failed: {}", error))?;

                if !response.status().is_success() {
                    return Err(format!("stargazers request returned {}", response.status()));
                }

                let batch: Vec<GitHubStargazer> = response
                    .json()
                    .await
                    .map_err(|error| format!("failed to parse stargazers response: {}", error))?;

                if batch.is_empty() {
                    break;
                }

                let count = batch.len();
                users.extend(batch);

                if count < 100 {
                    break;
                }

                page += 1;
                if page > 20 {
                    break;
                }
            }

            let mut candidates = Vec::<(String, String, String)>::new();

            for user in users {
                let Some(login) = sanitize_login(&user.login) else {
                    continue;
                };

                let profile_url = sanitize_profile_url(&user.html_url, &login);
                let avatar_url = user.avatar_url.trim();

                if !avatar_url.starts_with("https://") || avatar_url.chars().any(|character| character.is_control()) {
                    continue;
                }

                candidates.push((login, profile_url, avatar_url.to_string()));
            }

            let semaphore = Arc::new(Semaphore::new(12));
            let mut join_set = tokio::task::JoinSet::new();

            for (login, profile_url, avatar_url) in candidates {
                let client = client.clone();
                let semaphore = semaphore.clone();

                join_set.spawn(async move {
                    let Ok(_permit) = semaphore.acquire_owned().await else {
                        return None;
                    };

                    let avatar_response = match client.get(&avatar_url).send().await {
                        Ok(response) => response,
                        Err(_) => return None,
                    };

                    if !avatar_response.status().is_success() {
                        return None;
                    }

                    let avatar_bytes = match avatar_response.bytes().await {
                        Ok(bytes) => bytes,
                        Err(_) => return None,
                    };

                    Some(Stargazer {
                        tooltip_text: format!("{}\n{}", login, profile_url),
                        login,
                        avatar: image::Handle::from_memory(avatar_bytes.to_vec()),
                    })
                });
            }

            let mut stargazers = Vec::new();

            while let Some(joined) = join_set.join_next().await {
                if let Ok(Some(stargazer)) = joined {
                    stargazers.push(stargazer);
                }
            }

            stargazers.sort_by(|a, b| a.login.to_lowercase().cmp(&b.login.to_lowercase()));

            Ok(stargazers)
        })
        .await;

    match result {
        Ok(inner_result) => inner_result,
        Err(_) => Err("Failed to load stargazers: task cancelled".to_string()),
    }
}

pub async fn check_installation_async(runtime: Arc<Runtime>) -> bool {
    let result = runtime.spawn(async move {
        is_luna_installed().await.unwrap_or(false)
    }).await;

    result.unwrap_or(false)
}

pub async fn detect_tidal_paths_async(runtime: Arc<Runtime>) -> Result<Vec<String>, String> {
    let result = runtime
        .spawn(async move {
            find_tidal_directories()
                .await
                .map(|paths| paths.into_iter().map(|p| p.to_string_lossy().to_string()).collect())
        })
        .await;

    match result {
        Ok(Ok(paths)) => Ok(paths),
        Ok(Err(err)) => Err(err.to_string()),
        Err(_) => Err("Failed to detect TIDAL paths: task cancelled".to_string()),
    }
}

pub async fn install_async(
    releases: Vec<AppRelease>,
    channel: String,
    version: String,
    selected_path: String,
    custom_path: String,
    reinstall_mode: bool,
    runtime: Arc<Runtime>,
) -> Result<InstallExecutionResult, String> {
    let result = runtime.spawn(async move {
        let selected_release = releases
            .iter()
            .find(|r| r.name == channel)
            .ok_or_else(|| format!("Release channel '{}' not found", channel))?;

        let selected_version = selected_release
            .versions
            .iter()
            .find(|v| v.version == version)
            .ok_or_else(|| format!("Version '{}' not found in channel '{}'", version, channel))?;

        let final_path = if !custom_path.trim().is_empty() {
            normalize_tidal_resources_path(PathBuf::from(custom_path))
        } else if !selected_path.trim().is_empty() {
            normalize_tidal_resources_path(PathBuf::from(selected_path))
        } else {
            return Err("No TIDAL path selected".to_string());
        };

        let mut manager = InstallManager::new();
        let collected_logs = Arc::new(Mutex::new(Vec::<InstallExecutionLog>::new()));
        let install_success = Arc::new(Mutex::new(true));

        manager.add_step(Box::new(KillTidalStep));
        if reinstall_mode {
            manager.add_step(Box::new(ReinstallCleanupStep {
                overwrite_path: Some(final_path.clone()),
            }));
        }
        manager.add_step(Box::new(SetupStep {
            overwrite_path: Some(final_path.clone()),
        }));
        manager.add_step(Box::new(DownloadLunaStep {
            download_url: selected_version.download.clone(),
        }));
        manager.add_step(Box::new(ExtractLunaStep));
        manager.add_step(Box::new(CopyAsarInstallStep {
            overwrite_path: Some(final_path.clone()),
        }));
        manager.add_step(Box::new(InsertLunaStep {
            overwrite_path: Some(final_path.clone()),
        }));
        manager.add_step(Box::new(SignTidalStep));
        manager.add_step(Box::new(LaunchTidalStep {
            overwrite_path: Some(final_path.clone()),
            suppress_console_window: true,
        }));

        let logs_for_sub = Arc::clone(&collected_logs);
        let logs_for_step = Arc::clone(&collected_logs);
        let logs_for_start = Arc::clone(&collected_logs);
        let success_for_end = Arc::clone(&install_success);

        manager
            .run(
                |sublog| {
                    if let Ok(mut logs) = logs_for_sub.lock() {
                        logs.push(InstallExecutionLog {
                            message: sublog,
                            is_substep: true,
                        });
                    }
                },
                |steplog| {
                    if let Ok(mut logs) = logs_for_step.lock() {
                        logs.push(InstallExecutionLog {
                            message: steplog,
                            is_substep: false,
                        });
                    }
                },
                |step_name| {
                    if let Ok(mut logs) = logs_for_start.lock() {
                        logs.push(InstallExecutionLog {
                            message: format!("=== {} ===", step_name),
                            is_substep: false,
                        });
                    }
                },
                |success| {
                    if !success {
                        if let Ok(mut flag) = success_for_end.lock() {
                            *flag = false;
                        }
                    }
                },
            )
            .await;

        let logs = collected_logs.lock().map(|logs| logs.clone()).unwrap_or_default();
        let success = install_success.lock().map(|flag| *flag).unwrap_or(false);

        Ok(InstallExecutionResult { logs, success })
    }).await;

    match result {
        Ok(inner_result) => inner_result,
        Err(_) => Err("Installation task cancelled".to_string()),
    }
}

pub async fn uninstall_async(
    selected_path: String,
    custom_path: String,
    runtime: Arc<Runtime>,
) -> Result<InstallExecutionResult, String> {
    let result = runtime.spawn(async move {
        let final_path = if !custom_path.trim().is_empty() {
            normalize_tidal_resources_path(PathBuf::from(custom_path))
        } else if !selected_path.trim().is_empty() {
            normalize_tidal_resources_path(PathBuf::from(selected_path))
        } else {
            return Err("No TIDAL path selected".to_string());
        };

        let mut manager = InstallManager::new();
        let collected_logs = Arc::new(Mutex::new(Vec::<InstallExecutionLog>::new()));
        let uninstall_success = Arc::new(Mutex::new(true));

        manager.add_step(Box::new(KillTidalStep));
        manager.add_step(Box::new(CopyAsarUninstallStep {
            overwrite_path: Some(final_path.clone()),
        }));
        manager.add_step(Box::new(UninstallStep {
            overwrite_path: Some(final_path.clone()),
        }));
        manager.add_step(Box::new(SignTidalStep));
        manager.add_step(Box::new(LaunchTidalStep {
            overwrite_path: Some(final_path.clone()),
            suppress_console_window: true,
        }));

        let logs_for_sub = Arc::clone(&collected_logs);
        let logs_for_step = Arc::clone(&collected_logs);
        let logs_for_start = Arc::clone(&collected_logs);
        let success_for_end = Arc::clone(&uninstall_success);

        manager
            .run(
                |sublog| {
                    if let Ok(mut logs) = logs_for_sub.lock() {
                        logs.push(InstallExecutionLog {
                            message: sublog,
                            is_substep: true,
                        });
                    }
                },
                |steplog| {
                    if let Ok(mut logs) = logs_for_step.lock() {
                        logs.push(InstallExecutionLog {
                            message: steplog,
                            is_substep: false,
                        });
                    }
                },
                |step_name| {
                    if let Ok(mut logs) = logs_for_start.lock() {
                        logs.push(InstallExecutionLog {
                            message: format!("=== {} ===", step_name),
                            is_substep: false,
                        });
                    }
                },
                |success| {
                    if !success {
                        if let Ok(mut flag) = success_for_end.lock() {
                            *flag = false;
                        }
                    }
                },
            )
            .await;

        let logs = collected_logs.lock().map(|logs| logs.clone()).unwrap_or_default();
        let success = uninstall_success.lock().map(|flag| *flag).unwrap_or(false);

        Ok(InstallExecutionResult { logs, success })
    }).await;

    match result {
        Ok(inner_result) => inner_result,
        Err(_) => Err("Uninstallation task cancelled".to_string()),
    }
}
