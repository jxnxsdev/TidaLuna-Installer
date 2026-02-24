use crate::installer::step::{InstallStep, StepResult, SubLog};
use crate::utils::fs_helpers::get_tidal_directory;
use async_trait::async_trait;
use std::path::{Path, PathBuf};
use std::process::Command;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub struct LaunchTidalStep {
    pub overwrite_path: Option<PathBuf>,
    pub suppress_console_window: bool,
}

enum LaunchCandidate {
    Path {
        program: PathBuf,
        args: Vec<String>,
    },
    Command {
        program: String,
        args: Vec<String>,
    },
}

fn build_launch_candidates(resources_path: &Path) -> Vec<LaunchCandidate> {
    match std::env::consts::OS {
        "windows" => {
            let app_dir = resources_path.parent().unwrap_or(resources_path);
            let tidal_root = app_dir.parent().unwrap_or(app_dir);

            vec![
                LaunchCandidate::Path {
                    program: app_dir.join("TIDAL.exe"),
                    args: vec![],
                },
                LaunchCandidate::Path {
                    program: app_dir.join("Tidal.exe"),
                    args: vec![],
                },
                LaunchCandidate::Path {
                    program: tidal_root.join("TIDAL.exe"),
                    args: vec![],
                },
                LaunchCandidate::Path {
                    program: tidal_root.join("Update.exe"),
                    args: vec!["--processStart".into(), "TIDAL.exe".into()],
                },
            ]
        }
        "macos" => {
            let contents_dir = resources_path.parent().unwrap_or(resources_path);
            let app_bundle = contents_dir.parent().unwrap_or(contents_dir);

            vec![
                LaunchCandidate::Path {
                    program: contents_dir.join("MacOS").join("TIDAL"),
                    args: vec![],
                },
                LaunchCandidate::Path {
                    program: contents_dir.join("MacOS").join("Tidal"),
                    args: vec![],
                },
                LaunchCandidate::Command {
                    program: "open".into(),
                    args: vec![app_bundle.to_string_lossy().to_string()],
                },
            ]
        }
        "linux" => {
            let app_dir = resources_path.parent().unwrap_or(resources_path);

            vec![
                LaunchCandidate::Path {
                    program: app_dir.join("tidal-hifi"),
                    args: vec![],
                },
                LaunchCandidate::Path {
                    program: app_dir.join("tidal"),
                    args: vec![],
                },
                LaunchCandidate::Command {
                    program: "tidal-hifi".into(),
                    args: vec![],
                },
                LaunchCandidate::Command {
                    program: "flatpak".into(),
                    args: vec!["run".into(), "com.mastermindzh.tidal-hifi".into()],
                },
            ]
        }
        _ => vec![],
    }
}

fn try_launch(
    candidate: &LaunchCandidate,
    suppress_console_window: bool,
    sublog_callback: &(dyn Fn(SubLog) + Send + Sync),
) -> bool {
    const DETACHED_PROCESS: u32 = 0x00000008;
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let spawn_detached = |cmd: &mut Command| {
        #[cfg(target_os = "windows")]
        {
            let mut flags = DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP;
            if suppress_console_window {
                flags |= CREATE_NO_WINDOW;
            }
            cmd.creation_flags(flags);
        }
        cmd.spawn()
    };

    match candidate {
        LaunchCandidate::Path { program, args } => {
            if !program.exists() {
                return false;
            }

            match spawn_detached(Command::new(program).args(args)) {
                Ok(_) => {
                    sublog_callback(SubLog {
                        message: format!("TIDAL launched detached with {:?}", program),
                    });
                    true
                }
                Err(err) => {
                    sublog_callback(SubLog {
                        message: format!("Failed launching {:?}: {}", program, err),
                    });
                    false
                }
            }
        }
        LaunchCandidate::Command { program, args } => match spawn_detached(Command::new(program).args(args)) {
            Ok(_) => {
                sublog_callback(SubLog {
                    message: format!("TIDAL launched detached with command '{} {}'", program, args.join(" ")),
                });
                true
            }
            Err(err) => {
                sublog_callback(SubLog {
                    message: format!("Failed launching '{}' command: {}", program, err),
                });
                false
            }
        },
    }
}

#[async_trait]
impl InstallStep for LaunchTidalStep {
    fn name(&self) -> &str {
        "Launch TIDAL"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let resources_path = if let Some(path) = &self.overwrite_path {
            path.clone()
        } else {
            match get_tidal_directory().await {
                Ok(path) => path,
                Err(err) => {
                    sublog_callback(SubLog {
                        message: format!("Skipping launch: could not resolve TIDAL path ({})", err),
                    });
                    return StepResult {
                        success: true,
                        message: "Installation finished; TIDAL auto-launch skipped".into(),
                    };
                }
            }
        };

        if resources_path.as_os_str().is_empty() || !resources_path.exists() {
            sublog_callback(SubLog {
                message: "Skipping launch: TIDAL resources path is not available".into(),
            });
            return StepResult {
                success: true,
                message: "Installation finished; TIDAL auto-launch skipped".into(),
            };
        }

        sublog_callback(SubLog {
            message: format!("Trying to relaunch TIDAL from {:?}", resources_path),
        });

        let candidates = build_launch_candidates(&resources_path);
        for candidate in &candidates {
            if try_launch(candidate, self.suppress_console_window, sublog_callback) {
                return StepResult {
                    success: true,
                    message: "TIDAL relaunched successfully".into(),
                };
            }
        }

        StepResult {
            success: true,
            message: "Installation finished; no runnable TIDAL binary found for auto-launch".into(),
        }
    }
}
