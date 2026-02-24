use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::process::Command;

fn run_command(program: &str, args: &[&str]) -> Option<std::process::Output> {
    Command::new(program).args(args).output().ok()
}

pub struct KillTidalStep;

#[async_trait]
impl InstallStep for KillTidalStep {
    fn name(&self) -> &str {
        "Kill TIDAL"
    }

    async fn run(&self, sublog_callback: &(dyn Fn(SubLog) + Send + Sync)) -> StepResult {
        let os = std::env::consts::OS;

        sublog_callback(SubLog {
            message: format!("Detected OS: {}", os),
        });

        let mut executed = false;
        let mut killed_any = false;

        match os {
            "windows" => {
                sublog_callback(SubLog {
                    message: "Killing TIDAL process(es) (Windows)".into(),
                });

                for image in ["TIDAL.exe", "Tidal.exe", "tidal.exe", "Update.exe"] {
                    if let Some(output) = run_command("taskkill", &["/IM", image, "/T", "/F"]) {
                        executed = true;
                        if output.status.success() {
                            killed_any = true;
                            sublog_callback(SubLog {
                                message: format!("Stopped process image: {}", image),
                            });
                        }
                    }
                }
            }
            "macos" => {
                sublog_callback(SubLog {
                    message: "Killing TIDAL process(es) (macOS)".into(),
                });

                for pattern in ["TIDAL", "Tidal"] {
                    if let Some(output) = run_command("pkill", &["-f", pattern]) {
                        executed = true;
                        if output.status.success() {
                            killed_any = true;
                            sublog_callback(SubLog {
                                message: format!("Stopped process pattern: {}", pattern),
                            });
                        }
                    }
                }
            }
            "linux" => {
                sublog_callback(SubLog {
                    message: "Killing TIDAL process(es) (Linux)".into(),
                });

                for pattern in ["tidal-hifi", "tidal"] {
                    if let Some(output) = run_command("pkill", &["-f", pattern]) {
                        executed = true;
                        if output.status.success() {
                            killed_any = true;
                            sublog_callback(SubLog {
                                message: format!("Stopped process pattern: {}", pattern),
                            });
                        }
                    }
                }
            }
            _ => {
                return StepResult {
                    success: false,
                    message: "Unsupported operating system".into(),
                };
            }
        }

        if !executed {
            sublog_callback(SubLog {
                message: "Warning: no kill command could be executed on this system".into(),
            });
        } else if !killed_any {
            sublog_callback(SubLog {
                message: "No running TIDAL process found to kill".into(),
            });
        }

        sublog_callback(SubLog {
            message: "Kill TIDAL step completed".into(),
        });

        StepResult {
            success: true,
            message: "Kill TIDAL completed (non-fatal)".into(),
        }
    }
}