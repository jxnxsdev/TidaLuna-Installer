use clap::Parser;

/// TidaLuna Installer CLI
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Run in headless CLI mode
    #[arg(long)]
    pub headless: bool,

    /// Install action
    #[arg(short, long)]
    pub install: bool,

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
}
