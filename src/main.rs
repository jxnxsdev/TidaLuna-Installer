mod args;
mod cli;
mod installer;
mod types;
mod ui;
mod utils;

use clap::Parser;
use args::Args;

fn main() {
    let args = Args::parse();

    if args.headless {
        match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(runtime) => runtime.block_on(cli::run_cli(args)),
            Err(err) => {
                eprintln!("Failed to initialize async runtime: {}", err);
            }
        }
    } else {
        if let Err(err) = ui::run_gui() {
            eprintln!("Failed to launch GUI: {}", err);
        }
    }
}
