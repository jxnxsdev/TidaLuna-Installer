# TidaLuna Installer

Installer for the [TidaLuna](https://github.com/Inrixia/TidaLuna) Tidal Mod. Automatically uninstalls Neptune if it's installed.

![GitHub Downloads (specific asset, all releases)](https://img.shields.io/github/downloads/jxnxsdev/TidaLuna-Installer/installer-windows.exe)

---

## 🧰 How to Use the Installer

The installer should™ work on **Windows**, **Linux**, and **macOS**.

- On **Windows** and **Linux**, precompiled binaries are available on the [Releases page](https://github.com/jxnxsdev/TidaLuna-Installer/releases):
  - `installer-windows.exe`
  - `installer-linux`
- On **macOS**, the installer must currently be run from source.  
  _(macOS support is untested but should work—contributions welcome!)_

Regardless of platform, usage is the same:

1. **Close Tidal** before installing or uninstalling.
2. Launch the installer.
3. A browser window should open. If not, navigate to `http://localhost:3013` manually.
4. Choose a **release channel**: `Stable`, `Pre-release`, or `Dev`.
5. (Optional) Use **Advanced Options** to manually configure paths or debug.
6. Click **Install**, **Reinstall**, or **Uninstall** as needed.

---

## 🚀 Running the Installer

### 🪟 Windows

1. Download `installer-windows.exe` from the [Releases page](https://github.com/jxnxsdev/TidaLuna-Installer/releases)  
   ![Download Release](images/github_compiled_download.png)

2. Run the `.exe`. You may get a SmartScreen warning:
   - Click **"More info"**, then **"Run anyway"**  
     ![SmartScreen Warning](images/smartscreen.png)  
     ![SmartScreen More Info](images/smartscreen_more.png)

---

### 🐧 Linux

1. Download `installer-linux` from the [Releases page](https://github.com/jxnxsdev/TidaLuna-Installer/releases)
2. Make it executable:

   ```bash
   chmod +x installer-linux
   ```

3. Run the binary:

   ```bash
   ./installer-linux
   ```

---

### 🍏 macOS (from source)

> ⚠️ macOS support is currently **untested**.

#### 📦 Requirements

- Node.js v20
- npm (comes with Node.js)
- Project cloned or downloaded

#### 🛠️ Steps

1. Download the project:

   - Go to the [repository](https://github.com/jxnxsdev/TidaLuna-Installer)
   - Click **Code** → **Download ZIP**
     ![Download ZIP](images/github_download.png)

2. Open a terminal in the project folder and run:

   ```bash
   npm install
   npm run tsc
   node ./build/index.js
   ```

---

## ⚙️ Advanced Options

If the installer cannot locate your Tidal installation:

1. Manually find your **Tidal install directory**
2. Navigate into the `app-*` version folder
3. Copy the path to the `resources` folder
   ![Advanced Options](images/advanced_options.png)
   ![Tidal Folder](images/tidal_folder.png)
   ![Tidal Resources Folder](images/tidal_resources.png)

Not sure what version you're looking at? [semver.org](https://semver.org/) explains version formats.

---

## 📎 License

MIT — see [LICENSE](./LICENSE) for details.
