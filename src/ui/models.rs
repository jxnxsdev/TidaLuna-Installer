use iced::widget::{combo_box, image};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::runtime::Runtime;

pub type InstallerUpdateInfo = crate::utils::updater::UpdateInfo;
pub type InstallerUpdateApplyResult = crate::utils::updater::UpdateApplyResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRelease {
    pub name: String,
    pub versions: Vec<AppVersionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppVersionInfo {
    pub version: String,
    pub download: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadReleases,
    ReleasesLoaded(Result<Vec<AppRelease>, String>),
    InstallerUpdateChecked(Result<Option<InstallerUpdateInfo>, String>),
    AcceptInstallerUpdate,
    DeclineInstallerUpdate,
    InstallerUpdateApplied(Result<InstallerUpdateApplyResult, String>),
    StargazersLoaded(Result<Vec<Stargazer>, String>),
    ReleaseChannelSelected(String),
    VersionSelected(String),
    InstallPathChanged(String),
    InstallPathOptionSelected(String),
    TidalPathsDetected(Result<Vec<String>, String>),
    Install,
    Uninstall,
    InstallationComplete(Result<InstallExecutionResult, String>),
    InstallationStatus(bool),
    ToggleAdvancedOptions(bool),
    PrevStargazersPage,
    NextStargazersPage,
    ClearLog,
}

#[derive(Debug, Clone)]
pub struct InstallExecutionLog {
    pub message: String,
    pub is_substep: bool,
}

#[derive(Debug, Clone)]
pub struct InstallExecutionResult {
    pub logs: Vec<InstallExecutionLog>,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct Stargazer {
    pub login: String,
    pub tooltip_text: String,
    pub avatar: image::Handle,
}

#[derive(Debug, Clone)]
pub struct MyApp {
    pub releases: Vec<AppRelease>,
    pub selected_channel: String,
    pub selected_version: String,
    pub selected_install_path: String,
    pub custom_install_path: String,
    pub is_loading: bool,
    pub is_installing: bool,
    pub is_uninstalling: bool,
    pub is_advanced_open: bool,
    pub is_luna_installed: bool,
    pub is_loading_stargazers: bool,

    pub channel_pick_list: combo_box::State<String>,
    pub version_pick_list: combo_box::State<String>,
    pub install_path_pick_list: combo_box::State<String>,
    pub install_path_options: Vec<String>,
    pub stargazers: Vec<Stargazer>,
    pub stargazers_error: Option<String>,
    pub stargazers_page: usize,
    pub current_installer_version: String,
    pub available_installer_update: Option<InstallerUpdateInfo>,
    pub show_installer_update_prompt: bool,
    pub is_applying_installer_update: bool,

    pub log_entries: Vec<LogEntry>,
    pub runtime: Arc<Runtime>,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: u64,
    pub message: String,
    pub level: LogLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Success,
    Error,
    Step,
    SubStep,
}
