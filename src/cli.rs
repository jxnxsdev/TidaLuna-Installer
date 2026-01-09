use crate::args::Args;
use crate::utils::release_loader::ReleaseLoader;
use std::path::Path;
use tokio::fs;
use anyhow::Result;

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
        println!("Available releases and versions:\n");
        for release in releases {
            println!("Channel: {}", release.name);
            for version in &release.versions {
                println!("  - {} ({})", version.version, version.download);
            }
        }
        return;
    }

    // INSTALL
    if args.install {
        let version_to_install = args.version.as_deref().unwrap_or("stable");
        let path = args.path.as_deref().unwrap_or(".");

        println!("Installing version '{}' to '{}'", version_to_install, path);

        // Find the version in releases
        let mut found = None;
        for release in releases {
            if release.name == version_to_install {
                found = release.versions.first();
                break;
            }
        }

        if let Some(v) = found {
            println!("Downloading from: {}", v.download);
            // TODO: download & unpack logic here
        } else {
            eprintln!("Version '{}' not found!", version_to_install);
        }
        return;
    }

    // UNINSTALL
    if args.uninstall {
        let path = args.path.as_deref().unwrap_or(".");
        println!("Uninstalling from '{}'", path);

        // TODO: implement uninstall logic (delete installed files)
        return;
    }

    println!("No action specified. Use -i, -u, or -l.");
}
