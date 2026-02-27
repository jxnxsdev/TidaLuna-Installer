<div align="center">

# TidaLuna Installer

# NOTE: If you are facing issues with the current installer not working, please use version 2.1.17 and refer to the guide at https://luna-wiki.jxnxsdev.de!


<p align="center">
  <a href="https://github.com/jxnxsdev/TidaLuna-Installer/releases">
    <img src="https://img.shields.io/github/v/release/jxnxsdev/TidaLuna-Installer?style=for-the-badge" />
  </a>
  <a href="https://github.com/jxnxsdev/TidaLuna-Installer">
    <img src="https://img.shields.io/github/downloads/jxnxsdev/TidaLuna-Installer/total?style=for-the-badge" />
  </a>
  <a href="https://github.com/jxnxsdev/TidaLuna-Installer/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/jxnxsdev/TidaLuna-Installer?style=for-the-badge" />
  </a>
</p>

<p align="center">
Installer for <a href="https://github.com/Inrixia/TidaLuna">TidaLuna</a>, a TIDAL modification / plugin loader.<br/>
Handles installation and automatically removes Neptune if present.
</p>

</div>

## Overview

TidaLuna Installer is a cross-platform application that makes installing and managing TidaLuna easy. It provides both a graphical user interface (GUI) for regular users and a command-line interface (CLI) for advanced users and automation.

## Graphical User Interface (GUI)

### Getting Started:
1. Download the appropriate binary for your platform from the [Releases page](https://github.com/jxnxsdev/TidaLuna-Installer/releases)
2. Run the installer according to your platform (see below for platform-specific instructions)
3. Select your preferred release channel and version
4. Click "Install" to begin the installation process

The installer will automatically handle all necessary steps including:
- Detecting existing TIDAL installations
- Removing previous TidaLuna/Neptune installations
- Downloading and extracting the selected version
- Applying the modifications to TIDAL
- Signing the application (on macOS)

### Platform-specific Instructions:

#### Windows
- Double-click the downloaded `installer-windows-x86_64-vX.X.X.exe` to run.

#### Linux
- Open a terminal in the directory where you downloaded the installer.
- Make the installer executable (if needed): `chmod +x installer-linux-x86_64-vX.X.X`
- Run the installer: `./installer-linux-x86_64-vX.X.X`

#### macOS
1. Download the installer for your architecture (Intel or Apple Silicon) from the Releases page.
2. Open a terminal and navigate to the directory where the installer was downloaded.
3. You will need to give the installer system access executable permission. Run the following commands (replace `installer-macOS-x86_64-vX.X.X` with the actual file name for Intel, or `installer-macOS-aarch64-vX.X.X` for Apple Silicon):

   ```bash
   xattr -d com.apple.quarantine installer-macOS-x86_64-vX.X.X
   codesign -s - -f --deep installer-macOS-x86_64-vX.X.X
   chmod +x installer-macOS-x86_64-vX.X.X
   ```

   **WARNING**: If you skip this step, you will get an error when trying to run the installer: `zsh: killed installer-macOS-x86_64-vX.X.X`

4. Allow the installer to access and modify the TIDAL app. For that, do the following:
   - Open the Settings app of your Mac.
   - Navigate to Privacy and Security → App Management
   - Check "Terminal" to allow it to update / delete other applications.

5. Now you can run the installer by executing the following command in the terminal (again, replace the file name with the actual one):

   ```bash
   ./installer-macOS-x86_64-vX.X.X
   ```

## Downloads

Precompiled binaries are available on the GitHub Releases page:

[https://github.com/jxnxsdev/TidaLuna-Installer/releases](https://github.com/jxnxsdev/TidaLuna-Installer/releases)

Select the build that matches your operating system and architecture:

- **Windows**: `installer-windows-x86_64-vX.X.X.exe`
- **Linux (double-click install)**: `installer-linux-x86_64-vX.X.X.deb`
- **Linux (portable AppImage)**: `installer-linux-x86_64-vX.X.X.AppImage`
- **Linux (portable binary)**: `installer-linux-x86_64-vX.X.X`
- **macOS (Intel, app bundle)**: `installer-macOS-x86_64-vX.X.X.app.zip`
- **macOS (Apple Silicon, app bundle)**: `installer-macOS-aarch64-vX.X.X.app.zip`
- **macOS (raw binary)**: `installer-macOS-<arch>-vX.X.X`

### GUI Launch Notes

- On **macOS**, extract the `.app.zip` and double-click the `.app`.
- On **Linux**, double-click the `.deb` to install, then launch **TidaLuna Installer** from your app menu.
- On **Linux**, you can also use the `.AppImage` directly (may require `chmod +x` once depending on your desktop/browser).
- Linux portable binaries may still require `chmod +x` if your desktop/browser strips executable permission on download.

## Command Line Interface (CLI) Usage

The TidaLuna Installer includes a command line interface for advanced users, automation, and scripting. To use CLI mode, you must append the `--headless` argument before any other commands.

### Basic Usage

```bash
# Run the installer with GUI (default)
./tidaluna-installer

# Run in CLI mode (requires --headless argument)
./tidaluna-installer --headless --install
./tidaluna-installer --headless --uninstall
```

### Available Commands

#### 1. **List Available Releases**
Display all available release channels and versions:

```bash
./tidaluna-installer --headless --list-versions
```

Example output:
```
Available releases:

Channel: stable
 - 2.1.18 (https://github.com/...)
 - 2.1.17 (https://github.com/...)

Channel: beta
 - 2.2.0-beta.1 (https://github.com/...)

Channel: alpha
 - 2.3.0-alpha.3 (https://github.com/...)
```

#### 2. **Install TidaLuna**
Install TidaLuna with optional version and installation path:

```bash
# Install with default settings (latest stable version, default Tidal directory)
./tidaluna-installer --headless --install

# Install a specific version
./tidaluna-installer --headless --install --version 2.1.17

# Install to a custom directory
./tidaluna-installer --headless --install --path "/path/to/tidal/directory"

# Install specific version to custom directory
./tidaluna-installer --headless --install --version 2.1.17 --path "/path/to/tidal/directory"
```

#### 3. **Uninstall TidaLuna**
Remove TidaLuna from your system:

```bash
# Uninstall from default Tidal directory
./tidaluna-installer --headless --uninstall

# Uninstall from a custom directory
./tidaluna-installer --headless --uninstall --path "/path/to/tidal/directory"
```

### Command Line Options

| Option | Description | Example |
|--------|-------------|---------|
| `--headless` | Run in CLI mode (required for CLI) | `--headless` |
| `--install` | Install TidaLuna | `--headless --install` |
| `--uninstall` | Uninstall TidaLuna | `--headless --uninstall` |
| `--list-versions` | List all available releases | `--headless --list-versions` |
| `--version <VERSION>` | Specify version to install | `--version 2.1.17` |
| `--path <PATH>` | Custom installation path | `--path "/Applications/TIDAL.app/Contents/Resources"` |
| `--help` | Show help message | `--help` |

### Platform-Specific Examples

#### **Windows (PowerShell)**
```powershell
# List available versions
.\tidaluna-installer.exe --headless --list-versions

# Install latest stable version
.\tidaluna-installer.exe --headless --install

# Install to custom location
.\tidaluna-installer.exe --headless --install --path "C:\Program Files\TIDAL\resources"
```

#### **Linux/macOS (Terminal)**
```bash
# Make executable (if needed)
chmod +x tidaluna-installer

# List available versions
./tidaluna-installer --headless --list-versions

# Install latest stable version
./tidaluna-installer --headless --install

# Install to custom location (macOS example)
./tidaluna-installer --headless --install --path "/Applications/TIDAL.app/Contents/Resources"
```

### Notes

- **Important**: CLI mode requires the `--headless` argument before any other commands
- The CLI will automatically detect if TidaLuna/Neptune is already installed
- If no version is specified, the latest stable version is used
- If no path is specified, the default Tidal installation directory is used
- The installer automatically handles killing TIDAL processes when needed
- Installation steps are logged in real-time for debugging

## Support & Community

If you run into issues or want to stay up to date with development, join the community Discord:

[https://discord.gg/jK3uHrJGx4](https://discord.gg/jK3uHrJGx4)

## License

This project is licensed under the **MIT License**.

See the LICENSE file for full details:
[https://github.com/jxnxsdev/TidaLuna-Installer/blob/main/LICENSE](https://github.com/jxnxsdev/TidaLuna-Installer/blob/main/LICENSE)

---

**Note for macOS users**: The additional steps (xattr, codesign, chmod, and App Management permission) are required due to macOS security features. These steps ensure the installer can run properly and modify the TIDAL application.
