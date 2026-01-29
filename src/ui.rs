use iced::{
    executor, Alignment, Application, Color, Command, Element, Length, Settings, Size, Subscription,
    Theme,
};
use iced::widget::{
    button, checkbox, column, combo_box, horizontal_space, progress_bar, row, scrollable, text,
    text_input, vertical_space, Column, Container, Row, Scrollable, Space,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::installer::{
    steps::{
        copy_asar_install::CopyAsarInstallStep, copy_asar_uninstall::CopyAsarUninstallStep,
        download_luna::DownloadLunaStep, extract_luna::ExtractLunaStep,
        insert_luna::InsertLunaStep, kill_tidal::KillTidalStep, setup::SetupStep,
        sign_tidal::SignTidalStep, uninstall::UninstallStep,
    },
    manager::InstallManager,
};
use crate::utils::{
    fs_helpers::{get_tidal_directory, is_luna_installed},
    release_loader::ReleaseLoader,
};
use semver::Version;
use std::path::PathBuf;
use std::sync::Arc;


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
    ReleaseChannelSelected(String),
    VersionSelected(String),
    InstallPathChanged(String),
    Install,
    Uninstall,
    InstallationStep(String),
    InstallationSubStep(String),
    InstallationComplete(Result<(), String>),
    CheckInstallationStatus,
    InstallationStatus(bool),
    ToggleAdvancedOptions(bool),
    ClearLog,
}


#[derive(Debug, Clone)]
pub struct MyApp {
    releases: Vec<AppRelease>,
    selected_channel: String,
    selected_version: String,
    install_path: String,
    isLoading: bool,
    isInstalling: bool,
    isUninstalling: bool,
    isAdvancedOpen: bool,
    isLunaInstalled: bool,

    channel_pick_list: combo_box::State<String>,
    version_pick_list: combo_box::State<String>,

    log_entries: Vec<LogEntry>,

    sender: Option<UnboundedSender<Message>>,
    
    runtime: Arc<Runtime>,
}

#[derive(Debug, Clone)]
struct LogEntry {
    timestamp: u64,
    message: String,
    level: LogLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
    Step,
    SubStep,
}

impl Default for MyApp {
    fn default() -> Self {
        let channel_pick_list = combo_box::State::new(vec![]);
        let version_pick_list = combo_box::State::new(vec![]);

        Self {
            releases: Vec::new(),
            selected_channel: String::new(),
            selected_version: String::new(),
            install_path: String::new(),
            isLoading: true,
            isInstalling: false,
            isUninstalling: false,
            isAdvancedOpen: false,
            isLunaInstalled: false,
            channel_pick_list,
            version_pick_list,
            log_entries: Vec::new(),
            sender: None,
            runtime: Arc::new(
                Runtime::new().unwrap_or_else(|e| {
                    panic!("Failed to create Tokio runtime: {}", e);
                })
            ),
        }
    }
}


impl Application for MyApp {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        let (sender, _receiver) = mpsc::unbounded_channel();

        let app = Self {
            sender: Some(sender),
            ..Self::default()
        };

        let load_releases = Command::perform(load_releases_async(app.runtime.clone()), Message::ReleasesLoaded);
        let check_installation = Command::perform(check_installation_async(app.runtime.clone()), |is_installed| {
            Message::InstallationStatus(is_installed)
        });

        let cmd = Command::batch(vec![
            Command::perform(async {}, |_| Message::LoadReleases),
            load_releases,
            check_installation,
        ]);

        (app, cmd)
    }
    
    fn title(&self) -> String {
        "TidaLuna Installer".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::LoadReleases => {
                self.isLoading = true;
                self.add_log("Loading releases...", LogLevel::Info);
                let runtime = self.runtime.clone();
                Command::perform(load_releases_async(runtime), Message::ReleasesLoaded)
            }

            Message::ReleasesLoaded(result) => {
                self.isLoading = false;
                match result {
                    Ok(releases) => {
                        self.releases = releases;
                        self.add_log("Releases loaded successfully", LogLevel::Success);

                        let channel_options: Vec<String> = self.releases.iter()
                            .map(|r| r.name.clone())
                            .collect();
                        self.channel_pick_list = combo_box::State::new(channel_options);

                        let default_channel = self
                            .releases
                            .iter()
                            .find(|r| r.name == "stable")
                            .or_else(|| self.releases.iter().find(|r| r.name == "beta"))
                            .or_else(|| self.releases.iter().find(|r| r.name == "alpha"))
                            .map(|r| r.name.clone())
                            .unwrap_or_default();

                        if !default_channel.is_empty() {
                            return self.update(Message::ReleaseChannelSelected(default_channel));
                        }
                    }
                    Err(err) => {
                        self.add_log(
                            &format!("Failed to load releases: {}", err),
                            LogLevel::Error,
                        );
                    }
                }
                Command::none()
            }

            Message::ReleaseChannelSelected(channel) => {
                self.selected_channel = channel.clone();
                self.add_log(&format!("Selected channel: {}", channel), LogLevel::Info);

                if let Some(release) = self.releases.iter().find(|r| r.name == channel) {
                    let mut versions: Vec<String> = release
                        .versions
                        .iter()
                        .map(|v| v.version.clone())
                        .collect();

                    versions.sort_by(|a, b| {
                        let va = Version::parse(a).unwrap_or_else(|_| Version::new(0, 0, 0));
                        let vb = Version::parse(b).unwrap_or_else(|_| Version::new(0, 0, 0));
                        vb.cmp(&va)
                    });

                    self.version_pick_list = combo_box::State::new(versions.clone());

                    if let Some(latest) = versions.first() {
                        self.selected_version = latest.clone();
                        self.add_log(
                            &format!("Auto-selected version: {}", latest),
                            LogLevel::Info,
                        );
                    }
                }

                Command::none()
            }

            Message::VersionSelected(version) => {
                self.selected_version = version.clone();
                self.add_log(&format!("Selected version: {}", version), LogLevel::Info);
                Command::none()
            }

            Message::InstallPathChanged(path) => {
                self.install_path = path;
                Command::none()
            }

            Message::Install => {
                self.isInstalling = true;
                self.clear_log();
                self.add_log("Starting installation...", LogLevel::Step);

                let releases = self.releases.clone();
                let channel = self.selected_channel.clone();
                let version = self.selected_version.clone();
                let path = self.install_path.clone();
                let sender = self.sender.clone();
                let runtime = self.runtime.clone();

                Command::perform(
                    install_async(releases, channel, version, path, sender, runtime),
                    Message::InstallationComplete,
                )
            }

            Message::Uninstall => {
                self.isUninstalling = true;
                self.clear_log();
                self.add_log("Starting uninstallation...", LogLevel::Step);

                let path = self.install_path.clone();
                let sender = self.sender.clone();
                let runtime = self.runtime.clone();

                Command::perform(
                    uninstall_async(path, sender, runtime),
                    Message::InstallationComplete,
                )
            }

            Message::InstallationStep(step) => {
                self.add_log(&step, LogLevel::Step);
                Command::none()
            }

            Message::InstallationSubStep(substep) => {
                self.add_log(&format!("  {}", substep), LogLevel::SubStep);
                Command::none()
            }

            Message::InstallationComplete(result) => {
                self.isInstalling = false;
                self.isUninstalling = false;

                match result {
                    Ok(_) => {
                        self.add_log("Operation completed successfully!", LogLevel::Success);
                    }
                    Err(err) => {
                        self.add_log(&format!("Operation failed: {}", err), LogLevel::Error);
                    }
                }

                let runtime = self.runtime.clone();
                Command::perform(check_installation_async(runtime), |is_installed| {
                    Message::InstallationStatus(is_installed)
                })
            }

            Message::CheckInstallationStatus => {
                let runtime = self.runtime.clone();
                Command::perform(check_installation_async(runtime), |is_installed| {
                    Message::InstallationStatus(is_installed)
                })
            }

            Message::InstallationStatus(is_installed) => {
                self.isLunaInstalled = is_installed;
                if is_installed {
                    self.add_log("TidaLuna is already installed", LogLevel::Info);
                }
                Command::none()
            }

            Message::ToggleAdvancedOptions(is_open) => {
                self.isAdvancedOpen = is_open;
                Command::none()
            }

            Message::ClearLog => {
                self.clear_log();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let title = text("TidaLuna Installer")
            .size(32)
            .style(iced::theme::Text::Color(Color::from_rgb(0.2, 0.5, 0.8)));

        let status_text = if self.isLunaInstalled {
            text("Status: Installed ✓")
                .size(16)
                .style(iced::theme::Text::Color(Color::from_rgb(0.0, 0.7, 0.0)))
        } else {
            text("Status: Not installed")
                .size(16)
                .style(iced::theme::Text::Color(Color::from_rgb(0.7, 0.7, 0.7)))
        };

        let channel_label = text("Release Channel").size(16);

        let channel_pick = if self.isLoading {
            combo_box(
                &self.channel_pick_list,
                "Loading...",
                Some(&self.selected_channel),
                Message::ReleaseChannelSelected,
            )
            .width(Length::Fill)
            .padding(10)
        } else {
            combo_box(
                &self.channel_pick_list,
                "Select a release channel...",
                Some(&self.selected_channel),
                Message::ReleaseChannelSelected,
            )
            .width(Length::Fill)
            .padding(10)
        };

        let version_label = text("Version").size(16);

        let version_pick = if self.selected_channel.is_empty() {
            combo_box(
                &self.version_pick_list,
                "Select a channel first...",
                Some(&self.selected_version),
                Message::VersionSelected,
            )
            .width(Length::Fill)
            .padding(10)
        } else {
            combo_box(
                &self.version_pick_list,
                "Select a version...",
                Some(&self.selected_version),
                Message::VersionSelected,
            )
            .width(Length::Fill)
            .padding(10)
        };

        let path_label = text("Installation Path (optional)")
            .size(16);

        let path_input = text_input(
            "Leave empty for default Tidal directory",
            &self.install_path,
        )
        .on_input(Message::InstallPathChanged)
        .padding(10)
        .width(Length::Fill);

        let advanced_toggle = checkbox("Show advanced options", self.isAdvancedOpen)
            .on_toggle(|checked| Message::ToggleAdvancedOptions(checked))
            .size(16);

        let advanced_section = if self.isAdvancedOpen {
            Row::new()
                .spacing(10)
                .push(path_label.width(180))
                .push(path_input)
        } else {
            Row::new()
        };

        let install_button_text = if self.isLunaInstalled {
            "Reinstall"
        } else {
            "Install"
        };
        let install_button = if self.isInstalling || self.isUninstalling {
            button(
                text(install_button_text)
                    .size(16)
                    .style(iced::theme::Text::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            )
            .padding(15)
            .width(150)
        } else {
            button(
                text(install_button_text)
                    .size(16)
                    .style(iced::theme::Text::Color(Color::WHITE)),
            )
            .on_press(Message::Install)
            .padding(15)
            .width(150)
            .style(iced::theme::Button::Primary)
        };

        let uninstall_button = if !self.isLunaInstalled
            || self.isInstalling
            || self.isUninstalling
        {
            button(
                text("Uninstall")
                    .size(16)
                    .style(iced::theme::Text::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            )
            .padding(15)
            .width(150)
        } else {
            button(
                text("Uninstall")
                    .size(16)
                    .style(iced::theme::Text::Color(Color::WHITE)),
            )
            .on_press(Message::Uninstall)
            .padding(15)
            .width(150)
            .style(iced::theme::Button::Destructive)
        };

        let progress_indicator = if self.isInstalling || self.isUninstalling {
            Row::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(
                    progress_bar(0.0..=100.0, if self.isInstalling { 50.0 } else { 50.0 })
                        .width(200),
                )
                .push(
                    text(if self.isInstalling {
                        "Installing..."
                    } else {
                        "Uninstalling..."
                    })
                    .size(14)
                    .style(iced::theme::Text::Color(Color::from_rgb(0.5, 0.5, 0.5))),
                )
        } else {
            Row::new()
        };

        let log_title = text("Installation Log")
            .size(18)
            .style(iced::theme::Text::Color(Color::from_rgb(0.3, 0.3, 0.3)));

        let log_entries: Vec<Element<Message>> = self.log_entries
            .iter()
            .map(|entry| {
                let color = match entry.level {
                    LogLevel::Info => Color::from_rgb(0.3, 0.3, 0.3),
                    LogLevel::Success => Color::from_rgb(0.0, 0.6, 0.0),
                    LogLevel::Warning => Color::from_rgb(0.8, 0.5, 0.0),
                    LogLevel::Error => Color::from_rgb(0.8, 0.2, 0.2),
                    LogLevel::Step => Color::from_rgb(0.2, 0.4, 0.8),
                    LogLevel::SubStep => Color::from_rgb(0.4, 0.4, 0.4),
                };

                let prefix = match entry.level {
                    LogLevel::Step => "▶ ",
                    LogLevel::SubStep => "  → ",
                    _ => "• ",
                };

                let timestamp_secs = entry.timestamp / 1000;
                let minutes = (timestamp_secs / 60) % 60;
                let seconds = timestamp_secs % 60;
                let millis = entry.timestamp % 1000;
                let timestamp = format!("{:02}:{:02}.{:03}", minutes, seconds, millis);

                Container::new(
                    Row::new()
                        .spacing(10)
                        .align_items(Alignment::Center)
                        .push(
                            text(timestamp)
                                .size(12)
                                .style(iced::theme::Text::Color(Color::from_rgb(
                                    0.5, 0.5, 0.5,
                                )))
                                .width(80),
                        )
                        .push(
                            text(format!("{}{}", prefix, entry.message))
                                .size(14)
                                .style(iced::theme::Text::Color(color)),
                        ),
                )
                .padding(5)
                .width(Length::Fill)
                .into()
            })
            .collect();

        let log_content = scrollable(
            Column::with_children(log_entries)
                .spacing(2),
        )
        .height(200);

        let clear_log_button = if !self.isInstalling && !self.isUninstalling {
            button(text("Clear Log").size(14))
                .on_press(Message::ClearLog)
                .padding(8)
                .style(iced::theme::Button::Secondary)
        } else {
            button(
                text("Clear Log")
                    .size(14)
                    .style(iced::theme::Text::Color(Color::from_rgb(0.5, 0.5, 0.5))),
            )
            .padding(8)
        };

        let content = Column::new()
            .spacing(15)
            .padding(25)
            .max_width(900)
            .push(
                Row::new()
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(title)
                    .push(horizontal_space())
                    .push(status_text)
            )
            .push(Space::with_height(15))
            
            .push(
                Row::new()
                    .spacing(20)
                    .align_items(Alignment::End)
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(250)
                            .push(channel_label)
                            .push(channel_pick)
                    )
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(250)
                            .push(version_label)
                            .push(version_pick)
                    )
                    .push(
                        Column::new()
                            .spacing(5)
                            .width(200)
                            .push(advanced_toggle)
                    )
            )
            
            .push(if self.isAdvancedOpen {
                Row::new()
                    .spacing(10)
                    .push(advanced_section)
            } else {
                Row::new()
            })
            
            .push(Space::with_height(20))
            
            .push(
                Row::new()
                    .spacing(15)
                    .align_items(Alignment::Center)
                    .push(install_button)
                    .push(uninstall_button)
                    .push(horizontal_space())
                    .push(progress_indicator)
            )
            
            .push(Space::with_height(20))
            
            .push(
                Column::new()
                    .spacing(5)
                    .push(
                        Row::new()
                            .spacing(10)
                            .align_items(Alignment::Center)
                            .push(log_title)
                            .push(horizontal_space())
                            .push(clear_log_button)
                    )
                    .push(log_content)
            );

        Container::new(
            Scrollable::new(Container::new(content).width(Length::Fill).center_x()),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}


impl MyApp {
    fn add_log(&mut self, message: &str, level: LogLevel) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        self.log_entries.push(LogEntry {
            timestamp,
            message: message.to_string(),
            level,
        });

        if self.log_entries.len() > 1000 {
            self.log_entries.remove(0);
        }
    }

    fn clear_log(&mut self) {
        self.log_entries.clear();
        self.add_log("Log cleared", LogLevel::Info);
    }
}


async fn load_releases_async(runtime: Arc<Runtime>) -> Result<Vec<AppRelease>, String> {
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

async fn check_installation_async(runtime: Arc<Runtime>) -> bool {
    let result = runtime.spawn(async move {
        is_luna_installed().await.unwrap_or(false)
    }).await;

    result.unwrap_or(false)
}

async fn install_async(
    releases: Vec<AppRelease>,
    channel: String,
    version: String,
    path: String,
    sender: Option<UnboundedSender<Message>>,
    runtime: Arc<Runtime>,
) -> Result<(), String> {
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

        let install_path = if !path.trim().is_empty() {
            PathBuf::from(path)
        } else {
            get_tidal_directory()
                .await
                .map_err(|e| format!("Failed to get Tidal directory: {}", e))?
        };

        let mut final_path = install_path;
        if !final_path.ends_with("resources") {
            final_path.push("resources");
        }

        let mut manager = InstallManager::new();

        manager.add_step(Box::new(SetupStep {
            overwrite_path: Some(final_path.clone()),
        }));
        manager.add_step(Box::new(KillTidalStep));
        manager.add_step(Box::new(UninstallStep {
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

        manager
            .run(
                |sublog| {
                    if let Some(sender) = &sender {
                        let _ = sender.send(Message::InstallationSubStep(sublog.to_string()));
                    }
                },
                |steplog| {
                    if let Some(sender) = &sender {
                        let _ = sender.send(Message::InstallationStep(steplog.to_string()));
                    }
                },
                |step_name| {
                    if let Some(sender) = &sender {
                        let _ = sender.send(Message::InstallationStep(format!(
                            "=== {} ===",
                            step_name
                        )));
                    }
                },
                |success| {
                    if let Some(sender) = &sender {
                        if !success {
                            let _ = sender.send(Message::InstallationStep("Step failed!".to_string()));
                        }
                    }
                },
            )
            .await;

        Ok(())
    }).await;

    match result {
        Ok(inner_result) => inner_result,
        Err(_) => Err("Installation task cancelled".to_string()),
    }
}

async fn uninstall_async(
    path: String,
    sender: Option<UnboundedSender<Message>>,
    runtime: Arc<Runtime>,
) -> Result<(), String> {
    let result = runtime.spawn(async move {
        let uninstall_path = if !path.trim().is_empty() {
            PathBuf::from(path)
        } else {
            get_tidal_directory()
                .await
                .map_err(|e| format!("Failed to get Tidal directory: {}", e))?
        };

        let mut final_path = uninstall_path;
        if !final_path.ends_with("resources") {
            final_path.push("resources");
        }

        let mut manager = InstallManager::new();

        manager.add_step(Box::new(KillTidalStep));
        manager.add_step(Box::new(CopyAsarUninstallStep {
            overwrite_path: Some(final_path.clone()),
        }));
        manager.add_step(Box::new(UninstallStep {
            overwrite_path: Some(final_path.clone()),
        }));
        manager.add_step(Box::new(SignTidalStep));

        manager
            .run(
                |sublog| {
                    if let Some(sender) = &sender {
                        let _ = sender.send(Message::InstallationSubStep(sublog.to_string()));
                    }
                },
                |steplog| {
                    if let Some(sender) = &sender {
                        let _ = sender.send(Message::InstallationStep(steplog.to_string()));
                    }
                },
                |step_name| {
                    if let Some(sender) = &sender {
                        let _ = sender.send(Message::InstallationStep(format!(
                            "=== {} ===",
                            step_name
                        )));
                    }
                },
                |success| {
                    if let Some(sender) = &sender {
                        if !success {
                            let _ = sender.send(Message::InstallationStep("Step failed!".to_string()));
                        }
                    }
                },
            )
            .await;

        Ok(())
    }).await;

    match result {
        Ok(inner_result) => inner_result,
        Err(_) => Err("Uninstallation task cancelled".to_string()),
    }
}


pub fn run_gui() -> iced::Result {
    MyApp::run(Settings {
        window: iced::window::Settings {
            size: Size::new(1000.0, 700.0),
            min_size: Some(Size::new(800.0, 500.0)),
            ..iced::window::Settings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}