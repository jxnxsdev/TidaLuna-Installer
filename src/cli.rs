use crate::args::Args;
use crate::utils::{
    release_loader::ReleaseLoader,
    fs_helpers::{find_tidal_directories, is_luna_installed, normalize_tidal_resources_path},
};
use std::io::{self, Write};
use std::path::PathBuf;
use semver::Version;

use crate::installer::{
    steps::setup::SetupStep,
    steps::download_luna::DownloadLunaStep,
    steps::extract_luna::ExtractLunaStep,
    steps::copy_asar_install::CopyAsarInstallStep,
    steps::insert_luna::InsertLunaStep,
    steps::kill_tidal::KillTidalStep,
    steps::sign_tidal::SignTidalStep,
    steps::launch_tidal::LaunchTidalStep,
    steps::reinstall_cleanup::ReinstallCleanupStep,
    steps::uninstall::UninstallStep,
    steps::copy_asar_uninstall::CopyAsarUninstallStep,
    manager::InstallManager,
};

fn print_step_separator(step_name: &str) {
    println!("\n{}", "=".repeat(60));
    println!("== {} ", step_name);
    println!("{}", "=".repeat(60));
}

fn print_failure_banner(step_name: &str, message: &str) {
    println!("\n{}", "!".repeat(60));
    println!("!! STEP FAILED: {} !!", step_name);
    println!("!! {} !!", message);
    println!("{}", "!".repeat(60));
}

fn prompt_user_for_tidal_path(paths: &[PathBuf]) -> io::Result<PathBuf> {
    println!("Multiple TIDAL installations were found. Please choose one path:\n");
    for (index, path) in paths.iter().enumerate() {
        println!("  [{}] {}", index + 1, path.to_string_lossy());
    }

    loop {
        print!("\nEnter selection (1-{}): ", paths.len());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let trimmed = input.trim();
        if let Ok(choice) = trimmed.parse::<usize>() {
            if (1..=paths.len()).contains(&choice) {
                return Ok(paths[choice - 1].clone());
            }
        }

        println!("Invalid selection. Please enter a number between 1 and {}.", paths.len());
    }
}

async fn resolve_cli_tidal_path(user_path: &Option<String>) -> io::Result<PathBuf> {
    if let Some(path) = user_path {
        return Ok(normalize_tidal_resources_path(PathBuf::from(path)));
    }

    let found_paths = find_tidal_directories().await?;
    if found_paths.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to find TIDAL resources directory",
        ));
    }

    if found_paths.len() == 1 {
        return Ok(found_paths[0].clone());
    }

    prompt_user_for_tidal_path(&found_paths)
}

pub async fn run_cli(args: Args) {
    println!("TidaLuna Installer CLI\n");

    // Initialize release loader
    let mut loader = ReleaseLoader::new(
        "https://raw.githubusercontent.com/jxnxsdev/TidaLuna-Installer/main/resources/sources.json",
    );

    // Load releases from sources
    let releases = match loader.load_releases().await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to load releases: {}", e);
            return;
        }
    };

    // LIST VERSIONS
    if args.list_versions {
        println!("Available releases:\n");
        for release in releases.iter() {
            println!("Channel: {}", release.name);
            for version in &release.versions {
                println!("  - {} ({})", version.version, version.download);
            }
        }
        return;
    }

    // INSTALL
    if args.install || args.reinstall {
        let mut reinstall_mode = args.reinstall;

        if is_luna_installed().await.unwrap_or(false) {
            println!("TidaLuna / Neptune is already installed. Continuing with reinstall.");
            reinstall_mode = true;
        }

        let version_to_install = args.version.clone();

        // Determine release channel if no version provided: stable > beta > alpha
        let selected_release = if let Some(ver) = &version_to_install {
            releases.iter().find(|r| r.name == *ver)
        } else {
            releases.iter().find(|r| r.name == "stable")
                .or_else(|| releases.iter().find(|r| r.name == "beta"))
                .or_else(|| releases.iter().find(|r| r.name == "alpha"))
        };

        let selected_release = match selected_release {
            Some(r) => r,
            None => {
                eprintln!("No release found to install!");
                return;
            }
        };

        // Pick the newest version using semver
        let latest_version = selected_release
            .versions
            .iter()
            .max_by(|a, b| {
                let va = Version::parse(&a.version).unwrap_or_else(|_| Version::new(0, 0, 0));
                let vb = Version::parse(&b.version).unwrap_or_else(|_| Version::new(0, 0, 0));
                va.cmp(&vb)
            });

        let latest_version = match latest_version {
            Some(version) => version,
            None => {
                eprintln!("Selected release channel '{}' has no versions", selected_release.name);
                return;
            }
        };

        // Determine install path
        let path: PathBuf = match resolve_cli_tidal_path(&args.path).await {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Failed to find TIDAL resources directory: {}", e);
                return;
            }
        };

        println!(
            "\nInstalling {} version {} to {:?}\n",
            selected_release.name, latest_version.version, path
        );

        let mut manager = InstallManager::new();
        manager.add_step(Box::new(KillTidalStep));
        if reinstall_mode {
            manager.add_step(Box::new(ReinstallCleanupStep { overwrite_path: Some(path.clone()) }));
        }
        manager.add_step(Box::new(SetupStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(DownloadLunaStep { download_url: latest_version.download.clone() }));
        manager.add_step(Box::new(ExtractLunaStep));
        manager.add_step(Box::new(CopyAsarInstallStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(InsertLunaStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(SignTidalStep));
        manager.add_step(Box::new(LaunchTidalStep {
            overwrite_path: Some(path.clone()),
            suppress_console_window: false,
        }));

        // Run steps with nice console output
        manager.run(
            |sublog| println!("    {}", sublog),
            |steplog| println!("{}", steplog),
            |step_name| print_step_separator(&step_name),
            |success| {
                if !success {
                    print_failure_banner("Step Failed", "See above for details.");
                }
            },
        ).await;

        return;
    }

    // UNINSTALL
    if args.uninstall {
        let path: PathBuf = match resolve_cli_tidal_path(&args.path).await {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Failed to find TIDAL resources directory: {}", e);
                return;
            }
        };

        println!("\nUninstalling from {:?}\n", path);

        let mut manager = InstallManager::new();
        manager.add_step(Box::new(KillTidalStep));
        manager.add_step(Box::new(CopyAsarUninstallStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(UninstallStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(SignTidalStep));
        manager.add_step(Box::new(LaunchTidalStep {
            overwrite_path: Some(path.clone()),
            suppress_console_window: false,
        }));

        manager.run(
            |sublog| println!("    {}", sublog),
            |steplog| {
                if steplog.contains("failed") || steplog.contains("Failed") {
                    print_failure_banner("Step Failed", &steplog);
                } else {
                    println!("{}", steplog);
                }
            },
            |step_name| print_step_separator(&step_name),
            |success| {
                if !success {
                    print_failure_banner("Step Failed", "See above for details.");
                }
            },
        ).await;

        return;
    }

    println!("No valid command provided. Use --help for usage information.");
}
