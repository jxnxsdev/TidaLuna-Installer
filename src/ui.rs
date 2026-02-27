use iced::{
    executor, Alignment, Application, Background, Border, Color, Command, Element, Length,
    Settings, Shadow, Size, Subscription, Theme, Vector,
};
use iced::widget::{
    button, checkbox, combo_box, horizontal_space, progress_bar, scrollable, text, text_input,
    Column, Container, Row, Scrollable,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::runtime::Runtime;

use crate::installer::{
    steps::{
        copy_asar_install::CopyAsarInstallStep, copy_asar_uninstall::CopyAsarUninstallStep,
        download_luna::DownloadLunaStep, extract_luna::ExtractLunaStep,
        insert_luna::InsertLunaStep, kill_tidal::KillTidalStep, setup::SetupStep,
        sign_tidal::SignTidalStep, launch_tidal::LaunchTidalStep, reinstall_cleanup::ReinstallCleanupStep, uninstall::UninstallStep,
    },
    manager::InstallManager,
};
use crate::utils::{
    fs_helpers::{find_tidal_directories, is_luna_installed, normalize_tidal_resources_path},
    release_loader::ReleaseLoader,
};
use semver::Version;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};


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
    InstallPathOptionSelected(String),
    TidalPathsDetected(Result<Vec<String>, String>),
    Install,
    Uninstall,
    InstallationComplete(Result<InstallExecutionResult, String>),
    InstallationStatus(bool),
    ToggleAdvancedOptions(bool),
    ClearLog,
}

#[derive(Debug, Clone)]
pub struct InstallExecutionLog {
    message: String,
    is_substep: bool,
}

#[derive(Debug, Clone)]
pub struct InstallExecutionResult {
    logs: Vec<InstallExecutionLog>,
    success: bool,
}


#[derive(Debug, Clone)]
pub struct MyApp {
    releases: Vec<AppRelease>,
    selected_channel: String,
    selected_version: String,
    selected_install_path: String,
    custom_install_path: String,
    is_loading: bool,
    is_installing: bool,
    is_uninstalling: bool,
    is_advanced_open: bool,
    is_luna_installed: bool,

    channel_pick_list: combo_box::State<String>,
    version_pick_list: combo_box::State<String>,
    install_path_pick_list: combo_box::State<String>,
    install_path_options: Vec<String>,

    log_entries: Vec<LogEntry>,
    
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
    Error,
    Step,
    SubStep,
}

impl Default for MyApp {
    fn default() -> Self {
        let channel_pick_list = combo_box::State::new(vec![]);
        let version_pick_list = combo_box::State::new(vec![]);
        let install_path_pick_list = combo_box::State::new(vec![]);

        Self {
            releases: Vec::new(),
            selected_channel: String::new(),
            selected_version: String::new(),
            selected_install_path: String::new(),
            custom_install_path: String::new(),
            is_loading: true,
            is_installing: false,
            is_uninstalling: false,
            is_advanced_open: false,
            is_luna_installed: false,
            channel_pick_list,
            version_pick_list,
            install_path_pick_list,
            install_path_options: Vec::new(),
            log_entries: Vec::new(),
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
        let app = Self::default();

        let load_releases = Command::perform(load_releases_async(app.runtime.clone()), Message::ReleasesLoaded);
        let check_installation = Command::perform(check_installation_async(app.runtime.clone()), |is_installed| {
            Message::InstallationStatus(is_installed)
        });
        let detect_paths = Command::perform(detect_tidal_paths_async(app.runtime.clone()), Message::TidalPathsDetected);

        let cmd = Command::batch(vec![
            Command::perform(async {}, |_| Message::LoadReleases),
            load_releases,
            check_installation,
            detect_paths,
        ]);

        (app, cmd)
    }
    
    fn title(&self) -> String {
        "TidaLuna Installer".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::LoadReleases => {
                self.is_loading = true;
                self.add_log("Loading releases...", LogLevel::Info);
                let runtime = self.runtime.clone();
                Command::perform(load_releases_async(runtime), Message::ReleasesLoaded)
            }

            Message::ReleasesLoaded(result) => {
                self.is_loading = false;
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
                self.custom_install_path = path;
                Command::none()
            }

            Message::InstallPathOptionSelected(path) => {
                self.selected_install_path = path;
                Command::none()
            }

            Message::TidalPathsDetected(result) => {
                match result {
                    Ok(paths) => {
                        self.install_path_options = paths.clone();
                        self.install_path_pick_list = combo_box::State::new(paths.clone());

                        if paths.len() == 1 && self.selected_install_path.trim().is_empty() {
                            self.selected_install_path = paths[0].clone();
                            self.add_log("Detected one TIDAL installation path automatically", LogLevel::Info);
                        } else if paths.len() > 1 {
                            self.selected_install_path = paths[0].clone();
                            self.add_log("Multiple TIDAL installations detected. Auto-selected the latest path; change it in Advanced Options if needed.", LogLevel::Info);
                        }
                    }
                    Err(err) => {
                        self.add_log(&format!("Could not auto-detect TIDAL paths: {}", err), LogLevel::Info);
                    }
                }
                Command::none()
            }

            Message::Install => {
                if self.custom_install_path.trim().is_empty() && self.selected_install_path.trim().is_empty() {
                    self.add_log("No TIDAL path selected. Choose one from the dropdown or enter a custom path in Advanced Options.", LogLevel::Error);
                    return Command::none();
                }

                self.is_installing = true;
                self.clear_log();
                self.add_log("Starting installation...", LogLevel::Step);

                let releases = self.releases.clone();
                let channel = self.selected_channel.clone();
                let version = self.selected_version.clone();
                let selected_path = self.selected_install_path.clone();
                let custom_path = self.custom_install_path.clone();
                let reinstall_mode = self.is_luna_installed;
                let runtime = self.runtime.clone();

                Command::perform(
                    install_async(releases, channel, version, selected_path, custom_path, reinstall_mode, runtime),
                    Message::InstallationComplete,
                )
            }

            Message::Uninstall => {
                if self.custom_install_path.trim().is_empty() && self.selected_install_path.trim().is_empty() {
                    self.add_log("No TIDAL path selected. Choose one from the dropdown or enter a custom path in Advanced Options.", LogLevel::Error);
                    return Command::none();
                }

                self.is_uninstalling = true;
                self.clear_log();
                self.add_log("Starting uninstallation...", LogLevel::Step);

                let selected_path = self.selected_install_path.clone();
                let custom_path = self.custom_install_path.clone();
                let runtime = self.runtime.clone();

                Command::perform(
                    uninstall_async(selected_path, custom_path, runtime),
                    Message::InstallationComplete,
                )
            }

            Message::InstallationComplete(result) => {
                self.is_installing = false;
                self.is_uninstalling = false;

                match result {
                    Ok(execution) => {
                        for log in execution.logs {
                            if log.is_substep {
                                self.add_log(&format!("  {}", log.message), LogLevel::SubStep);
                            } else {
                                self.add_log(&log.message, LogLevel::Step);
                            }
                        }

                        if execution.success {
                            self.add_log("Operation completed successfully!", LogLevel::Success);
                        } else {
                            self.add_log("Operation failed: one or more steps failed", LogLevel::Error);
                        }
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

            Message::InstallationStatus(is_installed) => {
                self.is_luna_installed = is_installed;
                if is_installed {
                    self.add_log("TidaLuna is already installed", LogLevel::Info);
                }
                Command::none()
            }

            Message::ToggleAdvancedOptions(is_open) => {
                self.is_advanced_open = is_open;
                Command::none()
            }

            Message::ClearLog => {
                self.clear_log();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let card_style = |_: &Theme| iced::widget::container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgba(0.11, 0.12, 0.16, 0.94))),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: Color::from_rgba(0.45, 0.55, 0.9, 0.18),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.45),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 18.0,
            },
        };

        let warning_style = |_: &Theme| iced::widget::container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgba(0.26, 0.19, 0.08, 0.95))),
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: Color::from_rgba(0.95, 0.75, 0.25, 0.28),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.38),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 16.0,
            },
        };

        let background_style = |_: &Theme| iced::widget::container::Appearance {
            text_color: None,
            background: Some(Background::Color(Color::from_rgb(0.06, 0.07, 0.10))),
            border: Border::default(),
            shadow: Shadow::default(),
        };

        let title = text("TidaLuna Installer")
            .size(36)
            .style(iced::theme::Text::Color(Color::from_rgb(0.55, 0.76, 0.96)));

        let subtitle = text("Install or remove TidaLuna from a TIDAL installation.")
            .size(14)
            .style(iced::theme::Text::Color(Color::from_rgb(0.74, 0.76, 0.84)));

        let subsubtitle = text("Works for the official TIDAL app and tidal-hifi. Note: If you are using the windows store version of TIDAL, please uninstall it and install the version from the official website.")
            .size(12)
            .style(iced::theme::Text::Color(Color::from_rgb(0.74, 0.76, 0.84)));

        let status_text_label = if self.is_luna_installed {
            let installed_label = if cfg!(target_os = "windows") {
                "Installed"
            } else {
                "Installed ✓"
            };

            text(installed_label)
                .size(15)
                .style(iced::theme::Text::Color(Color::from_rgb(0.64, 0.95, 0.68)))
        } else {
            text("Not installed")
                .size(15)
                .style(iced::theme::Text::Color(Color::from_rgb(0.86, 0.86, 0.90)))
        };

        let status_text = Container::new(
            Row::new()
                .spacing(8)
                .align_items(Alignment::Center)
                .push(
                    text("Status")
                        .size(13)
                        .style(iced::theme::Text::Color(Color::from_rgb(0.66, 0.70, 0.80))),
                )
                .push(status_text_label),
        )
        .padding([6, 10]);

        let channel_label = text("Release Channel").size(16);

        let channel_pick = if self.is_loading {
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

        let path_label = text("Custom Path (optional)")
            .size(16);

        let detected_path_label = text("Detected TIDAL installation path")
            .size(16);
        let detected_path_description = text("Used by default. If you enter a custom path below (click advanced options first), it overrides this selection.")
            .size(12)
            .style(iced::theme::Text::Color(Color::from_rgb(0.7, 0.72, 0.78)));

        let selected_detected_path = if self.selected_install_path.trim().is_empty() {
            None
        } else {
            Some(&self.selected_install_path)
        };

        let detected_path_pick = combo_box(
            &self.install_path_pick_list,
            "No detected TIDAL installation paths",
            selected_detected_path,
            Message::InstallPathOptionSelected,
        )
        .width(Length::Fill)
        .padding(10);

        let path_input = text_input(
            "Leave empty for default Tidal directory",
            &self.custom_install_path,
        )
        .on_input(Message::InstallPathChanged)
        .padding(10)
        .width(Length::Fill);

        let advanced_toggle = checkbox("Show advanced options", self.is_advanced_open)
            .on_toggle(|checked| Message::ToggleAdvancedOptions(checked))
            .size(16);

        let advanced_section = if self.is_advanced_open {
            Row::new()
                .spacing(10)
                .push(path_label.width(180))
                .push(path_input)
        } else {
            Row::new()
        };

        let install_button_text = if self.is_luna_installed {
            "Reinstall"
        } else {
            "Install"
        };
        let install_button = if self.is_installing || self.is_uninstalling {
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

        let uninstall_button = if !self.is_luna_installed
            || self.is_installing
            || self.is_uninstalling
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

        let progress_indicator = if self.is_installing || self.is_uninstalling {
            Row::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(
                    progress_bar(0.0..=100.0, if self.is_installing { 50.0 } else { 50.0 })
                        .width(200),
                )
                .push(
                    text(if self.is_installing {
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

        let legal_warning = Container::new(
            Column::new()
                .spacing(4)
                .push(
                    text("Usage Warning")
                        .size(16)
                        .style(iced::theme::Text::Color(Color::from_rgb(0.98, 0.80, 0.35))),
                )
                .push(
                    text("You cannot pirate media with TidaLuna or bypass TIDAL's free-tier limitations.")
                        .size(14)
                        .style(iced::theme::Text::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                )
                .push(
                    text("Use only with a legitimate account. We will not help with issues related to piracy.")
                        .size(13)
                        .style(iced::theme::Text::Color(Color::from_rgb(0.7, 0.7, 0.7))),
                ),
        )
        .padding(12)
        .width(Length::Fill)
        .style(warning_style);

        let log_title = text("Installation Log")
            .size(18)
            .style(iced::theme::Text::Color(Color::from_rgb(0.3, 0.3, 0.3)));

        let log_entries: Vec<Element<Message>> = self.log_entries
            .iter()
            .map(|entry| {
                let color = match entry.level {
                    LogLevel::Info => Color::from_rgb(0.3, 0.3, 0.3),
                    LogLevel::Success => Color::from_rgb(0.0, 0.6, 0.0),
                    LogLevel::Error => Color::from_rgb(0.8, 0.2, 0.2),
                    LogLevel::Step => Color::from_rgb(0.2, 0.4, 0.8),
                    LogLevel::SubStep => Color::from_rgb(0.4, 0.4, 0.4),
                };

                let prefix = if cfg!(target_os = "windows") {
                    match entry.level {
                        LogLevel::Step => ">> ",
                        LogLevel::SubStep => "  -> ",
                        _ => "* ",
                    }
                } else {
                    match entry.level {
                        LogLevel::Step => "▶ ",
                        LogLevel::SubStep => "  → ",
                        _ => "• ",
                    }
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
                .padding([6, 8])
                .width(Length::Fill)
                .into()
            })
            .collect();

        let log_content = scrollable(
            Column::with_children(log_entries)
                .spacing(2),
        )
        .height(200);

        let clear_log_button = if !self.is_installing && !self.is_uninstalling {
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

        let header_box = Container::new(
            Column::new()
                .spacing(10)
                .push(
                    Row::new()
                        .spacing(10)
                        .align_items(Alignment::Center)
                        .push(title)
                        .push(horizontal_space())
                        .push(status_text),
                )
                .push(subtitle)
                .push(subsubtitle),
        )
        .padding(18)
        .width(Length::Fill)
        .style(card_style);

        let main_box = Container::new(
            Column::new()
                .spacing(18)
                .push(
                    Row::new()
                        .spacing(20)
                        .align_items(Alignment::End)
                        .push(
                            Column::new()
                                .spacing(6)
                                .width(260)
                                .push(channel_label)
                                .push(channel_pick),
                        )
                        .push(
                            Column::new()
                                .spacing(6)
                                .width(260)
                                .push(version_label)
                                .push(version_pick),
                        )
                        .push(
                            Column::new()
                                .spacing(6)
                                .width(200)
                                .push(advanced_toggle),
                        ),
                )
                .push(
                    Column::new()
                        .spacing(6)
                        .push(detected_path_label)
                        .push(detected_path_description)
                        .push(detected_path_pick),
                )
                .push(if self.is_advanced_open {
                    Row::new().spacing(10).push(advanced_section)
                } else {
                    Row::new()
                })
                .push(
                    Row::new()
                        .spacing(15)
                        .align_items(Alignment::Center)
                        .push(install_button)
                        .push(uninstall_button)
                        .push(horizontal_space())
                        .push(progress_indicator),
                )
                .push(
                    Column::new()
                        .spacing(10)
                        .push(
                            Row::new()
                                .spacing(10)
                                .align_items(Alignment::Center)
                                .push(log_title)
                                .push(horizontal_space())
                                .push(clear_log_button),
                        )
                        .push(log_content),
                ),
        )
        .padding(18)
        .width(Length::Fill)
        .style(card_style);

        let content = Column::new()
            .spacing(16)
            .padding(24)
            .width(Length::Fill)
            .push(header_box)
            .push(legal_warning)
            .push(main_box);

        Container::new(
            Scrollable::new(Container::new(content).width(Length::Fill).center_x()),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(background_style)
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

async fn detect_tidal_paths_async(runtime: Arc<Runtime>) -> Result<Vec<String>, String> {
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

async fn install_async(
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

async fn uninstall_async(
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