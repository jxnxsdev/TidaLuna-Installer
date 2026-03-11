use std::io;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
fn startup_folder() -> io::Result<PathBuf> {
    let appdata = std::env::var("APPDATA").map_err(|_| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "APPDATA environment variable not set",
        )
    })?;

    Ok(PathBuf::from(appdata)
        .join("Microsoft")
        .join("Windows")
        .join("Start Menu")
        .join("Programs")
        .join("Startup"))
}

#[cfg(target_os = "windows")]
fn guard_script_path() -> io::Result<PathBuf> {
    Ok(startup_folder()?.join("TidaLuna-Guard.cmd"))
}

#[cfg(target_os = "windows")]
pub fn is_guard_enabled() -> io::Result<bool> {
    Ok(guard_script_path()?.is_file())
}

#[cfg(target_os = "windows")]
pub fn set_guard_enabled(enabled: bool) -> io::Result<bool> {
    let guard_path = guard_script_path()?;

    if enabled {
        let exe_path = std::env::current_exe()?;
        let exe_path = exe_path.to_string_lossy();
        let script = format!(
            "@echo off\r\nset \"EXE={}\"\r\nif not exist \"%EXE%\" exit /b 0\r\n\"%EXE%\" --headless --guard\r\n",
            exe_path
        );

        std::fs::create_dir_all(
            guard_path
                .parent()
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid startup path"))?,
        )?;
        std::fs::write(&guard_path, script)?;
    } else if guard_path.exists() {
        std::fs::remove_file(&guard_path)?;
    }

    is_guard_enabled()
}

#[cfg(not(target_os = "windows"))]
pub fn is_guard_enabled() -> io::Result<bool> {
    Ok(false)
}

#[cfg(not(target_os = "windows"))]
pub fn set_guard_enabled(_enabled: bool) -> io::Result<bool> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Startup guard is only supported on Windows",
    ))
}
