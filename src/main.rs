mod args;
mod cli;
mod ui;

mod utils;
mod types;

use clap::Parser;
use args::Args;

fn main() {
    let args = Args::parse();

    if args.headless {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(cli::run_cli(args));
    } else {
        ui::run_gui().unwrap();
    }
}
