use crate::installer::step::{InstallStep, StepResult, SubLog};
use async_trait::async_trait;
use std::process::Command;

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

        let result = match os {
            "windows" => {
                sublog_callback(SubLog {
                    message: "Killing TIDAL process (Windows)".into(),
                });
                Command::new("taskkill").args(["/IM", "TIDAL.exe", "/F"]).output()
            }
            "macos" => {
                sublog_callback(SubLog {
                    message: "Killing TIDAL process (macOS)".into(),
                });
                Command::new("pkill").args(["-f", "TIDAL"]).output()
            }
            "linux" => {
                sublog_callback(SubLog {
                    message: "Killing TIDAL process (Linux)".into(),
                });
                Command::new("pkill").args(["-f", "tidal-hifi"]).output()
            }
            _ => {
                return StepResult {
                    success: false,
                    message: "Unsupported operating system".into(),
                };
            }
        };

        match result {
            Ok(out) => {
                if !out.stdout.is_empty() {
                    sublog_callback(SubLog {
                        message: String::from_utf8_lossy(&out.stdout).to_string(),
                    });
                }

                if !out.stderr.is_empty() {
                    sublog_callback(SubLog {
                        message: format!("Warning while killing TIDAL: {}", String::from_utf8_lossy(&out.stderr)),
                    });
                }

                sublog_callback(SubLog {
                    message: "Kill TIDAL step completed (process may or may not have been running)".into(),
                });
            }
            Err(err) => {
                sublog_callback(SubLog {
                    message: format!("Warning: failed to execute kill command: {}", err),
                });
            }
        }

        StepResult {
            success: true,
            message: "Kill TIDAL completed (non-fatal)".into(),
        }
    }
}