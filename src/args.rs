use clap::Parser;

/// TidaLuna Installer CLI
#[derive(Parser, Debug)]
#[command(author, version, about, disable_version_flag = true)]
pub struct Args {
    /// Run in headless CLI mode
    #[arg(long)]
    pub headless: bool,

    /// Install action
    #[arg(short, long)]
    pub install: bool,

    /// Reinstall action (tries uninstall first, then installs)
    #[arg(long)]
    pub reinstall: bool,

    /// Uninstall action
    #[arg(short, long)]
    pub uninstall: bool,

    /// Version (optional, used with install)
    #[arg(short, long)]
    pub version: Option<String>,

    /// Path (optional, used with install or uninstall)
    #[arg(short, long)]
    pub path: Option<String>,

    /// List available versions
    #[arg(short = 'l', long)]
    pub list_versions: bool,

    /// Update the installer binary to the latest release
    #[arg(long)]
    pub update: bool,
}
