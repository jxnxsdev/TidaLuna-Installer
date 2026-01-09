use crate::args::Args;
use crate::utils::{release_loader::ReleaseLoader, fs_helpers::get_tidal_directory};
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
    if args.install {
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
            })
            .unwrap();

        // Determine install path
        let mut path: PathBuf = if let Some(p) = &args.path {
            PathBuf::from(p)
        } else {
            get_tidal_directory().await.unwrap_or_else(|_| PathBuf::from("."))
        };

        if !path.ends_with("resources") {
            path.push("resources");
        }

        println!(
            "\nInstalling {} version {} to {:?}\n",
            selected_release.name, latest_version.version, path
        );

        let mut manager = InstallManager::new();
        manager.add_step(Box::new(SetupStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(KillTidalStep));
        manager.add_step(Box::new(UninstallStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(DownloadLunaStep { download_url: latest_version.download.clone() }));
        manager.add_step(Box::new(ExtractLunaStep));
        manager.add_step(Box::new(CopyAsarInstallStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(InsertLunaStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(SignTidalStep));

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
        let mut path: PathBuf = if let Some(p) = &args.path {
            PathBuf::from(p)
        } else {
            get_tidal_directory().await.unwrap_or_else(|_| PathBuf::from("."))
        };

        if !path.ends_with("resources") {
            path.push("resources");
        }

        println!("\nUninstalling from {:?}\n", path);

        let mut manager = InstallManager::new();
        manager.add_step(Box::new(KillTidalStep));
        manager.add_step(Box::new(CopyAsarUninstallStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(UninstallStep { overwrite_path: Some(path.clone()) }));
        manager.add_step(Box::new(SignTidalStep));

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
